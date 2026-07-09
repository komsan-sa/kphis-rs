use axum::{
    body::Bytes,
    extract::{ConnectInfo, FromRef, FromRequestParts, MatchedPath, State},
    http::{Method, request::Parts, uri::PathAndQuery},
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use base64::{Engine, engine::general_purpose};
use config::Config;
use cryptographic_message_syntax::Oid;
use pasetors::{
    keys::{AsymmetricKeyPair, Generate},
    version4::V4,
};
use sqlx::{MySql, Pool};
use std::{
    collections::{HashMap, HashSet},
    env,
    net::{IpAddr, SocketAddr},
    path::Path,
    str::FromStr,
    sync::{Arc, LazyLock, RwLock},
};
use time::{OffsetDateTime, PrimitiveDateTime, Time, format_description::well_known::Iso8601, macros::offset};
use tokio::sync::{Mutex, broadcast, mpsc};
use tracing::{info, warn};
use typst_library::diag::{EcoString, FileError};
use typst_pdf::PdfSig;
use ulid::Ulid;
use x509_certificate::{CapturedX509Certificate, InMemorySigningKeyPair};

use kphis_api_query::{
    assets::{load_app_asset, new_app_asset},
    log, query_utils,
    transform::trigger::{add_ipt_delete_trigger, add_ipt_insert_trigger, select_exists_trg_kphis_ipt_log_delete, select_exists_trg_kphis_ipt_log_insert},
    user::role::{get_all_role, get_role_permission_list},
};
use kphis_model::{
    API_PREFIX,
    app::AppAsset,
    claim::Claims,
    endpoint::EndPoint,
    pacs::PacsConfig,
    score::SupportedScore,
    select_utils::ColorSelectOption,
    sse::{SseData, SseGroup, SseMessage},
    user::{
        his::{CurrentUserRole, UserDb},
        permission::Permission,
        role::{Role, RolePermissionList, UserRoleParams},
    },
};
use kphis_util::{
    datetime::{get_timestamp_server, now},
    error::{AppError, Source},
    // util::hash_to_base64_string,
};

use crate::{
    pdf::{core::JsonActorHandle, signer::PdfSigner},
    token::get_claim_and_verify_public,
};

pub static BUILD_TIME: LazyLock<PrimitiveDateTime> = LazyLock::new(|| {
    env::var("SOURCE_DATE_EPOCH")
        .ok()
        .and_then(|val| val.parse::<i64>().ok())
        .and_then(|v| OffsetDateTime::from_unix_timestamp(v).ok())
        .map(|utc| {
            let local = utc.to_offset(offset!(+7));
            PrimitiveDateTime::new(local.date(), local.time())
        })
        .unwrap_or(now())
});

#[derive(Clone)]
pub struct ApiState {
    pub app_config: Arc<ApiConfig>,
    pub online_users: Arc<Mutex<HashMap<u128, UserState>>>,                        // state_id : User
    pub sse_users: Arc<Mutex<HashMap<String, mpsc::UnboundedSender<SseMessage>>>>, // doctorcode : Tx channal
    pub sse_wards: Arc<Mutex<HashMap<String, HashSet<String>>>>,                   // ward : Set<doctorcode>
    pub sse_spcltys: Arc<Mutex<HashMap<u32, HashSet<String>>>>,                    // splcty : Set<doctorcode>
    pub sse_anonymous_senders: Arc<Mutex<Vec<mpsc::UnboundedSender<SseMessage>>>>, // for keeping channal alive
    pub app_asset_cache: Arc<Mutex<AppAsset>>,
    pub app_asset_bytes_cache: Arc<Mutex<Vec<u8>>>,
    pub app_asset_cache_exp: u64,
    // pub app_asset_cache_etag: Arc<RwLock<String>>,
    pub roles: Arc<Mutex<HashMap<String, Vec<String>>>>,
    pub roles_permissions: Arc<Mutex<HashMap<String, HashSet<Permission>>>>,
    pub paseto_public: Arc<String>,
    pub paseto: Arc<AsymmetricKeyPair<V4>>,
    pub json_handle: Arc<RwLock<JsonActorHandle>>,
    pub pdf_signer: Option<Arc<PdfSigner>>,
    pub db_pool: Pool<MySql>,
    pub pacs_client: reqwest::Client,
    pub shutdown_sender: broadcast::Sender<()>,
    pub sw_datetime: Arc<RwLock<PrimitiveDateTime>>,
}

impl ApiState {
    pub async fn new(config: &Config, db_pool: Pool<MySql>, json_handle: Arc<RwLock<JsonActorHandle>>, shutdown_sender: broadcast::Sender<()>) -> Self {
        let hosxp_dbname = config.get_string("hosxp-dbname").expect("'hosxp-dbname' not found in config file");
        let kphis_dbname = config.get_string("kphis-dbname").expect("'kphis-dbname' not found in config file");
        let kphis_log_dbname = config.get_string("kphis-log-dbname").expect("'kphis-log-dbname' not found in config file");
        let kphis_extra_dbname = config.get_string("kphis-extra-dbname").expect("'kphis-extra-dbname' not found in config file");

        crate::utils::check_permissions(&db_pool, &kphis_dbname).await.expect("Failed Permissions Check");

        let all_roles = get_all_role(&db_pool, &kphis_dbname).await.expect("Failed Select AllRoles");
        let roles = role_with_parent(&all_roles);

        let role_permission_list = get_role_permission_list(UserRoleParams::default(), &db_pool, &kphis_dbname)
            .await
            .expect("Failed get AllRolesPermissions");
        let roles_permissions = Arc::new(Mutex::new(roles_permissions(&role_permission_list)));

        let request_body_limited_mb = config.get_int("request-body-limited-mb").ok().and_then(|max| u8::try_from(max).ok()).unwrap_or(2);
        let rate_limit_burst_size = config.get_int("rate-limit-burst-size").ok().and_then(|max| u32::try_from(max).ok()).unwrap_or(100);
        let rate_limit_replenish_every_millisecond = config.get_int("rate-limit-replenish-every-millisecond").ok().and_then(|max| u64::try_from(max).ok()).unwrap_or(100);

        let allow_insert_his = config.get_bool("allow-insert-his").unwrap_or(false);
        let can_sign_pdf = config.get_bool("can-sign-pdf").unwrap_or(false);
        let pdf_signer = if can_sign_pdf {
            let pdf_cert_path = config.get_string("pdf-cert-path").expect("'pdf-cert-path' not found in config file");
            let pdf_key_path = config.get_string("pdf-key-path").expect("'pdf-key-path' not found in config file");
            let public_key = std::fs::read_to_string(&pdf_cert_path).unwrap_or_else(|_| panic!("{}", [&pdf_cert_path, " not found"].concat()));
            let private_key = std::fs::read_to_string(&pdf_key_path).unwrap_or_else(|_| panic!("{}", [&pdf_key_path, " not found"].concat()));
            let signing_certificate = CapturedX509Certificate::from_pem(public_key).expect("Error parsing x509 Certificate");
            let signing_key = Arc::new(InMemorySigningKeyPair::from_pkcs8_pem(private_key).expect("Error parsing pkcs#8 key"));
            let tsa = config.get_string("pdf-time-stamp-authority-url").ok();

            let subject = signing_certificate.subject_name();
            let name = subject.iter_common_name().next().and_then(|cn| cn.to_string().ok()).unwrap_or_default();
            let o = subject.iter_organization().next().and_then(|o| o.to_string().ok());
            let l = subject.iter_locality().next().and_then(|l| l.to_string().ok());
            let st = subject.iter_state_province().next().and_then(|st| st.to_string().ok());
            let c = subject.iter_country().next().and_then(|c| c.to_string().ok());
            let location = [o, l, st, c].into_iter().filter(|i| i.is_some()).map(|i| i.unwrap_or_default()).collect::<Vec<String>>().join(", ");
            // x509 Identifies the email address attribute. 1.2.840.113549.1.9.1
            let contact_info = subject
                .iter_by_oid(Oid(Bytes::copy_from_slice(&[42, 134, 72, 134, 247, 13, 1, 9, 1])))
                .next()
                .and_then(|c| c.to_string().ok())
                .unwrap_or_default();
            let sig = PdfSig {
                name,
                location,
                reason: String::from("Medical Information"),
                contact_info,
            };

            Some(Arc::new(PdfSigner {
                signing_certificate,
                signing_key,
                tsa,
                sig,
            }))
        } else {
            None
        };

        let app_asset_cache_minutes = config.get_int("app-asset-cache-minutes").ok().and_then(|max| u64::try_from(max).ok()).unwrap_or(24 * 60);

        let report_coercions = config.get_array("report-coercions").ok().map(|vs| {
            Arc::new(
                vs.into_iter()
                    .flat_map(|v| {
                        let item = v.into_table().expect("Can not parse into table");
                        let system = item.get("system").cloned().map(|v| v.into_string().expect("Can not parse into string"));
                        let custom = item.get("custom").cloned().map(|v| v.into_string().expect("Can not parse into string"));
                        system.zip(custom)
                    })
                    .collect::<HashMap<String, String>>(),
            )
        });

        let fcnote_patient_types = config
            .get_array("fcnote-patient-types")
            .expect("'fcnote-patient-types' not found in config file")
            .into_iter()
            .flat_map(|v| {
                let item = v.into_table().expect("Can not parse into table");
                item.get("key").cloned().map(|key| ColorSelectOption {
                    key: key.into_string().expect("Can not parse into string"),
                    value: item.get("value").cloned().map(|value| value.into_string().expect("Can not parse into string")).unwrap_or_default(),
                    color: item.get("color").cloned().map(|value| value.into_string().expect("Can not parse into string")).unwrap_or_default(),
                })
            })
            .collect::<Vec<ColorSelectOption>>();

        let (
            app_asset_cache_exp,
            // app_asset_cache_etag,
            app_asset_cache,
            app_asset_bytes_cache,
        ) = new_app_asset(&fcnote_patient_types, app_asset_cache_minutes, &db_pool, &hosxp_dbname, &kphis_dbname)
            .await
            .expect("Cannot get AppAsset from database");

        let hosxp_vn_length = config.get_int("hosxp-vn-length").ok().and_then(|max| usize::try_from(max).ok()).unwrap_or(12);
        let hosxp_an_length = config.get_int("hosxp-an-length").ok().and_then(|max| usize::try_from(max).ok()).unwrap_or(9);
        if hosxp_vn_length == hosxp_an_length {
            panic!("VN length must not equal to AN length");
        }
        let pacs_config = config.get_string("pacs-host").ok().map(|pacs_host| PacsConfig {
            pacs_host,
            pacs_host_is_kphis_broker: config.get_bool("pacs-host-is-kphis-broker").expect("'pacs-host-is-kphis-broker' not found in config file"),
            pacs_user: config.get_string("pacs-user").expect("'pacs-user' not found in config file"),
            pacs_password: config.get_string("pacs-password").expect("'pacs-password' not found in config file"),
            pacs_data_source: config.get_string("pacs-data-source").expect("'pacs-data-source' not found in config file"),
        });

        let app_config = Arc::new(ApiConfig {
            access_token_expire_minutes: config.get_int("access-token-expire-minutes").ok().and_then(|max| u64::try_from(max).ok()).unwrap_or(60),
            refresh_token_expire_minutes: config.get_int("refresh-token-expire-minutes").ok().and_then(|max| u64::try_from(max).ok()).unwrap_or(960),
            reauthen_before_refresh_token_expire_minutes: config
                .get_int("reauthen-before-refresh-token-expire-minutes")
                .ok()
                .and_then(|max| u64::try_from(max).ok())
                .unwrap_or(60),
            handshake_2fa_timeout_second: config.get_int("handshake-2fa-timeout-second").ok().and_then(|max| u64::try_from(max).ok()).unwrap_or(60),
            app_asset_cache_minutes,
            request_body_limited_mb,
            rate_limit_burst_size,
            rate_limit_replenish_every_millisecond,
            real_ip_header: config.get_string("real-ip-header").ok(),

            hosxp_dbname,
            kphis_dbname,
            kphis_log_dbname,
            kphis_extra_dbname,
            hosxp_hn_length: config.get_int("hosxp-hn-length").ok().and_then(|max| usize::try_from(max).ok()).unwrap_or(7),
            hosxp_vn_length,
            hosxp_an_length,

            is_production: config.get_bool("is-production").expect("'is-production' not found in config file"),
            is_read_only_mode: config.get_bool("is-read-only-mode").expect("'is-read-only-mode' not found in config file"),
            is_access_log_only_authorized: config.get_bool("is-access-log-only-authorized").expect("'is-access-log-only-authorized' not found in config file"),
            is_checked_pharmacist_can_done: config.get_bool("is-checked-pharmacist-can-done").unwrap_or(false),
            has_covid_lab: query_utils::can_access_table("kph_covid", "lab_result_covid", &db_pool).await.unwrap_or_default(),
            allow_insert_his,
            can_sign_pdf,

            pacs_config,

            hospcode: config.get_string("hospcode").expect("'hospcode' not found in config file"),
            code_name: config.get_string("code-name").unwrap_or_default(),
            hospital_name: config.get_string("hospital-name").expect("'hospital-name' not found in config file"),
            hospital_short_name: config.get_string("hospital-short-name").expect("'hospital-short-name' not found in config file"),
            hospital_info: config.get_string("hospital-info").unwrap_or_default(),
            drug_notify_use: config.get_string("drug-notify-use").expect("'drug-notify-use' not found in config file"),
            drug_notify_start_end_marker_use: config
                .get_string("drug-notify-start-end-marker-use")
                .expect("'drug-notify-start-end-marker-use' not found in config file"),
            drug_notify_start_marker: config.get_string("drug-notify-start-marker").expect("'drug-notify-start-marker' not found in config file"),
            drug_notify_end_marker: config.get_string("drug-notify-end-marker").expect("'drug-notify-end-marker' not found in config file"),

            score_ews: config
                .get_array("score-ews")
                .expect("'score-ews' not found in config file")
                .into_iter()
                .map(|ews| SupportedScore::new(&ews.into_string().expect("Can not parse into string")))
                .collect(),
            score_qsofa: config
                .get_array("score-qsofa")
                .expect("'score-qsofa' not found in config file")
                .into_iter()
                .map(|ews| SupportedScore::new(&ews.into_string().expect("Can not parse into string")))
                .collect(),
            score_sirs: config
                .get_array("score-sirs")
                .expect("'score-sirs' not found in config file")
                .into_iter()
                .map(|ews| SupportedScore::new(&ews.into_string().expect("Can not parse into string")))
                .collect(),

            nurse_assign_groups: config
                .get_array("nurse-assign-groups")
                .expect("'nurse-assign-groups' not found in config file")
                .into_iter()
                .map(|v| v.into_string().expect("Can not parse into string"))
                .collect::<Vec<String>>(),

            shift_day_start: Time::parse(&config.get_string("shift-day-start").expect("'shift-day-start' not found in config file"), &Iso8601::DEFAULT).expect("Fail convert to Time"),
            shift_evening_start: Time::parse(&config.get_string("shift-evening-start").expect("'shift-evening-start' not found in config file"), &Iso8601::DEFAULT).expect("Fail convert to Time"),
            shift_night_start: Time::parse(&config.get_string("shift-night-start").expect("'shift-night-start' not found in config file"), &Iso8601::DEFAULT).expect("Fail convert to Time"),

            hosxp_ivfluid_dosageform: config.get_string("hosxp-ivfluid-dosageform").expect("'hosxp-ivfluid-dosageform' not found in config file"),
            hosxp_injection_dosageforms: config
                .get_array("hosxp-injection-dosageforms")
                .expect("'hosxp-injection-dosageforms' not found in config file")
                .into_iter()
                .map(|dosageform| dosageform.into_string().expect("Can not parse into integer"))
                .collect::<Vec<String>>(),
            hosxp_had_displaycolor: config.get_int("hosxp-had-displaycolor").ok().and_then(|max| i32::try_from(max).ok()),
            hosxp_lasa_displaycolor: config.get_int("hosxp-lasa-displaycolor").ok().and_then(|max| i32::try_from(max).ok()),
            hosxp_med_reconcilation_icode: config.get_string("hosxp-med-reconcilation-icode").expect("'hosxp-med-reconcilation-icode' not found in config file"),
            hosxp_lab_wbc_code: config.get_int("hosxp-lab-wbc-code").expect("'hosxp-lab-wbc-code' not found in config file"),
            hosxp_lab_band_code: config.get_int("hosxp-lab-band-code").expect("'hosxp-lab-band-code' not found in config file"),
            hosxp_operation_success: config
                .get_array("hosxp-operation-success")
                .expect("'hosxp-operation-success' not found in config file")
                .into_iter()
                .map(|code| code.into_uint().expect("Can not parse into integer"))
                .collect::<Vec<u64>>(),
            doctor_intern_roles: config
                .get_array("doctor-intern-roles")
                .expect("'doctor-intern-roles' not found in config file")
                .into_iter()
                .map(|role| role.into_string().expect("Can not parse into string"))
                .collect::<Vec<String>>(),

            concat_with_space: config.get_bool("concat-with-space").expect("'concat-with-space' not found in config file"),
            report_coercions,
            fcnote_patient_types,
            lab_alerts: config
                .get_array("lab-alerts")
                .expect("'lab-alerts' not found in config file")
                .into_iter()
                .flat_map(|v| {
                    let item = v.into_table().expect("Can not parse into table");
                    item.get("description").cloned().map(|description| {
                        (
                            description.into_string().expect("Can not parse into string"),
                            item.get("codes")
                                .cloned()
                                .map(|codes| {
                                    codes
                                        .into_array()
                                        .expect("Can not parse into array")
                                        .into_iter()
                                        .map(|code| code.into_uint().expect("Can not parse into integer"))
                                        .collect::<Vec<u64>>()
                                })
                                .unwrap_or_default(),
                            item.get("where").cloned().map(|w| w.into_string().expect("Can not parse into string")).unwrap_or_default(),
                        )
                    })
                })
                .collect(),
            scr_codes: config
                .get_array("scr-codes")
                .expect("'scr-codes' not found in config file")
                .into_iter()
                .map(|v| v.into_uint().expect("Can not parse into integer"))
                .collect::<Vec<u64>>(),
            egfr_codes: config
                .get_array("egfr-codes")
                .expect("'egfr-codes' not found in config file")
                .into_iter()
                .map(|v| v.into_uint().expect("Can not parse into integer"))
                .collect::<Vec<u64>>(),
            lab_codes: config
                .get_array("lab-codes")
                .expect("'lab-codes' not found in config file")
                .into_iter()
                .flat_map(|v| {
                    let item = v.into_table().expect("Can not parse into table");
                    item.get("name").cloned().map(|name| {
                        (
                            name.into_string().expect("Can not parse into string"),
                            item.get("codes")
                                .cloned()
                                .map(|codes| {
                                    codes
                                        .into_array()
                                        .expect("Can not parse into array")
                                        .into_iter()
                                        .map(|code| code.into_uint().expect("Can not parse into integer"))
                                        .collect::<Vec<u64>>()
                                })
                                .unwrap_or_default(),
                        )
                    })
                })
                .collect(),
            message_icodes: config
                .get_array("message-icodes")
                .expect("'message-icodes' not found in config file")
                .into_iter()
                .flat_map(|v| {
                    let item = v.into_table().expect("Can not parse into table");
                    item.get("message").cloned().map(|name| {
                        (
                            name.into_string().expect("Can not parse into string"),
                            item.get("icodes")
                                .cloned()
                                .map(|codes| {
                                    codes
                                        .into_array()
                                        .expect("Can not parse into array")
                                        .into_iter()
                                        .map(|code| code.into_string().expect("Can not parse into integer"))
                                        .collect::<Vec<String>>()
                                })
                                .unwrap_or_default(),
                        )
                    })
                })
                .collect(),
            message_egfr_icodes: config
                .get_array("message-egfr-icodes")
                .expect("'message-egfr-icodes' not found in config file")
                .into_iter()
                .flat_map(|v| {
                    let item = v.into_table().expect("Can not parse into table");
                    item.get("message").cloned().map(|name| {
                        (
                            name.into_string().expect("Can not parse into string"),
                            item.get("egfr").cloned().map(|egfr| egfr.into_uint().expect("Can not parse into integer")).unwrap_or(30),
                            item.get("icodes")
                                .cloned()
                                .map(|codes| {
                                    codes
                                        .into_array()
                                        .expect("Can not parse into array")
                                        .into_iter()
                                        .map(|code| code.into_string().expect("Can not parse into integer"))
                                        .collect::<Vec<String>>()
                                })
                                .unwrap_or_default(),
                        )
                    })
                })
                .collect(),
            message_crcl_icodes: config
                .get_array("message-crcl-icodes")
                .expect("'message-crcl-icodes' not found in config file")
                .into_iter()
                .flat_map(|v| {
                    let item = v.into_table().expect("Can not parse into table");
                    item.get("message").cloned().map(|name| {
                        (
                            name.into_string().expect("Can not parse into string"),
                            item.get("crcl").cloned().map(|egfr| egfr.into_uint().expect("Can not parse into integer")).unwrap_or(30),
                            item.get("icodes")
                                .cloned()
                                .map(|codes| {
                                    codes
                                        .into_array()
                                        .expect("Can not parse into array")
                                        .into_iter()
                                        .map(|code| code.into_string().expect("Can not parse into integer"))
                                        .collect::<Vec<String>>()
                                })
                                .unwrap_or_default(),
                        )
                    })
                })
                .collect(),

            pacs_hn_url: config.get_string("pacs-hn-url").ok(),
            ekg_hn_url: config.get_string("ekg-hn-url").ok(),
            scan_hn_url: config.get_string("scan-hn-url").ok(),
            scan_an_url: config.get_string("scan-an-url").ok(),
            cart_vnan_url: config.get_string("cart-vnan-url").ok(),
            food_url: config.get_string("food-url").ok(),
        });

        if !app_config.is_shift_valid() {
            panic!("Config's 'shift-xxx-start' time not in order");
        }

        let paseto = Arc::new(AsymmetricKeyPair::<V4>::generate().expect("Fail generating keypair"));
        let paseto_public = Arc::new(general_purpose::URL_SAFE_NO_PAD.encode(paseto.public.as_bytes()));

        let pacs_client = reqwest::Client::builder()
            //.cookie_provider(Arc::new(CookieStoreMutex::new(CookieStore::default())))
            .cookie_store(true)
            .build()
            .expect("Cannot create PACs client");

        Self {
            app_config,
            online_users: Arc::new(Mutex::new(HashMap::new())),
            sse_users: Arc::new(Mutex::new(HashMap::new())),
            sse_wards: Arc::new(Mutex::new(HashMap::new())),
            sse_spcltys: Arc::new(Mutex::new(HashMap::new())),
            sse_anonymous_senders: Arc::new(Mutex::new(Vec::new())),
            app_asset_cache,
            app_asset_bytes_cache,
            app_asset_cache_exp,
            // app_asset_cache_etag,
            roles: Arc::new(Mutex::new(roles)),
            roles_permissions,
            paseto_public,
            paseto,
            json_handle,
            pdf_signer,
            db_pool,
            pacs_client,
            shutdown_sender,
            sw_datetime: Arc::new(RwLock::new(BUILD_TIME.clone())),
        }
    }

    pub fn is_allow(&self, method: &Method, endpoint: &EndPoint, user_state: &UserState, is_pre_admit: bool) -> bool {
        if self.production() {
            let is_method_valid = if self.read_only() { matches!(method, &Method::GET) } else { true };
            is_method_valid && endpoint.is_allow(method, &user_state.permissions, is_pre_admit)
        } else {
            true
        }
    }

    pub fn is_pre_admit(&self, an: &str) -> bool {
        an.len() > self.hosxp_an_len()
    }

    pub fn is_pre_admit_opt(&self, an: &Option<String>) -> bool {
        an.as_ref().map(|s| s.len() > self.hosxp_an_len()).unwrap_or_default()
    }

    /// add user to online_users
    pub async fn online_add(&self, state_id: u128, user: &UserDb, roles: &[CurrentUserRole], permissions: &[Permission], addr: SocketAddr) {
        let mut guard = self.online_users.lock().await;
        // send Log-out message to old state of this user
        if let Some(doctorcode) = &user.doctorcode {
            self.sse_logout(doctorcode, "พบการเข้าใช้งานของท่านจากที่อื่น, หากไม่ใช่ท่าน กรุณาติดต่อผู้ดูแลระบบ").await;
        }
        // remove old user state
        guard.retain(|_, v| v.user.loginname != user.loginname);
        // insert new
        guard.insert(
            state_id,
            UserState {
                state_id,
                user: user.to_owned(),
                roles: roles.to_vec(),
                permissions: permissions.to_vec(),
                addr,
            },
        );
    }
    pub async fn online_get(&self, state_id: u128) -> Option<UserState> {
        let guard = self.online_users.lock().await;
        guard.get(&state_id).cloned()
    }
    /// if changed_totp is Some then change user in online_user as not-done/no TOTP
    pub async fn online_update_user_config(&self, state_id: u128, theme: &Option<String>, wide_screen: &Option<String>, changed_totp: &Option<Option<String>>) {
        let mut guard = self.online_users.lock().await;
        if let Some(user) = guard.get_mut(&state_id) {
            user.user.theme = theme.to_owned();
            user.user.wide_screen = wide_screen.to_owned();
            if let Some(topt) = changed_totp {
                user.user.totp = topt.to_owned();
                user.user.totp_done = None;
            }
        }
    }
    pub async fn online_update_msg_group(&self, state_id: u128, sse_group: &SseGroup) {
        let mut guard = self.online_users.lock().await;
        if let Some(user) = guard.get_mut(&state_id) {
            user.user.wards = sse_group.wards.to_owned();
            user.user.spclty_ids = sse_group.spclty_ids.to_owned();
        }
    }
    pub async fn online_remove_by_loginname(&self, loginname: &str, message: &str) {
        let mut guard = self.online_users.lock().await;
        let mut doctorcodes = Vec::new();
        guard.retain(|_, v| {
            if v.user.loginname == loginname {
                if let Some(doctorcode) = v.user.doctorcode.as_ref() {
                    doctorcodes.push(doctorcode.to_owned());
                }
                false
            } else {
                true
            }
        });
        if !doctorcodes.is_empty() {
            self.sse_logout_many(&doctorcodes, message).await;
        }
    }

    pub async fn online_remove_by_role(&self, role: &str, message: &str) {
        let mut guard = self.online_users.lock().await;
        let mut doctorcodes = Vec::new();
        guard.retain(|_, v| {
            if v.roles.iter().any(|r| r.role == role) {
                if let Some(doctorcode) = v.user.doctorcode.as_ref() {
                    doctorcodes.push(doctorcode.to_owned());
                }
                false
            } else {
                true
            }
        });
        if !doctorcodes.is_empty() {
            self.sse_logout_many(&doctorcodes, message).await;
        }
    }

    pub async fn sse_insert(&self, doctorcode: &str, tx: mpsc::UnboundedSender<SseMessage>) {
        let mut guard = self.sse_users.lock().await;
        guard.insert(doctorcode.to_owned(), tx);
    }
    pub async fn sse_anonymous_insert(&self, tx: mpsc::UnboundedSender<SseMessage>) {
        let mut guard = self.sse_anonymous_senders.lock().await;
        guard.push(tx);
    }
    pub async fn sse_wards_update(&self, doctorcode: &str, wards: &[String]) {
        let mut wd_guard = self.sse_wards.lock().await;
        // add new ward if not exists
        for ward in wards {
            if !wd_guard.contains_key(ward) {
                wd_guard.insert(ward.to_owned(), HashSet::new());
            }
        }
        // update doctorcodes
        wd_guard.iter_mut().for_each(|(ward, doctorcodes)| {
            if wards.contains(ward) {
                doctorcodes.insert(doctorcode.to_owned());
            } else {
                doctorcodes.take(doctorcode);
            }
        })
    }
    pub async fn sse_spcltys_update(&self, doctorcode: &str, spclty_ids: &[u32]) {
        let mut sp_guard = self.sse_spcltys.lock().await;
        // add new spclty if not exists
        for spclty in spclty_ids {
            if !sp_guard.contains_key(spclty) {
                sp_guard.insert(*spclty, HashSet::new());
            }
        }
        // update doctorcodes
        sp_guard.iter_mut().for_each(|(spclty, doctorcodes)| {
            if spclty_ids.contains(spclty) {
                doctorcodes.insert(doctorcode.to_owned());
            } else {
                doctorcodes.take(doctorcode);
            }
        })
    }
    // Send direct message to this target_doctorcode)
    /// return true when send != error
    pub async fn sse_direct_msg(&self, target_doctorcode: &str, data: &SseData) {
        let mut guard = self.sse_users.lock().await;
        guard.retain(|uname, tx| {
            if target_doctorcode == uname {
                // If not `is_ok`, the SSE stream is gone, and so don't retain
                tx.send(SseMessage::DirectMsg(data.to_owned())).is_ok()
            } else {
                // don't send to other user, but do retain
                true
            }
        });
    }
    // We use `retain` instead of a for loop so that we can reap any user that appears to have disconnected.
    pub async fn sse_ward_msg(&self, ward: &str, data: &SseData) {
        let wd_guard = self.sse_wards.lock().await;
        if let Some(target_names) = wd_guard.get(ward) {
            let mut guard = self.sse_users.lock().await;
            for target_name in target_names {
                guard.retain(|uname, tx| {
                    if target_name == uname {
                        // If not `is_ok`, the SSE stream is gone, and so don't retain
                        tx.send(SseMessage::WardMsg(data.to_owned())).is_ok()
                    } else {
                        // don't send to other user, but do retain
                        true
                    }
                });
            }
        }
    }
    // We use `retain` instead of a for loop so that we can reap any user that appears to have disconnected.
    pub async fn sse_spclty_msg(&self, spclty: u32, data: &SseData) {
        let sp_guard = self.sse_spcltys.lock().await;
        if let Some(target_names) = sp_guard.get(&spclty) {
            let mut guard = self.sse_users.lock().await;
            for target_name in target_names {
                guard.retain(|uname, tx| {
                    if target_name == uname {
                        // If not `is_ok`, the SSE stream is gone, and so don't retain
                        tx.send(SseMessage::SpcltyMsg(data.to_owned())).is_ok()
                    } else {
                        // don't send to other user, but do retain
                        true
                    }
                });
            }
        }
    }
    // We use `retain` instead of a for loop so that we can reap any user that appears to have disconnected.
    pub async fn sse_global_msg(&self, data: &SseData) {
        let mut guard = self.sse_users.lock().await;
        guard.retain(|_uname, tx| {
            // If not `is_ok`, the SSE stream is gone, and so don't retain
            tx.send(SseMessage::GlobalMsg(data.to_owned())).is_ok()
        });
    }
    // Send logout message to this user)
    pub async fn sse_logout(&self, doctorcode: &str, message: &str) {
        let mut guard = self.sse_users.lock().await;
        guard.retain(|uid, tx| {
            if doctorcode == uid {
                tx.send(SseMessage::Logout(message.to_owned())).is_ok()
                // If not `is_ok`, the SSE stream is gone, and so don't retain
            } else {
                // don't send to other user, but do retain
                true
            }
        });
        // guard.remove(doctorcode);
    }
    // Send logout message to this user)
    pub async fn sse_logout_many(&self, doctorcodes: &[String], message: &str) {
        let mut guard = self.sse_users.lock().await;
        guard.retain(|uid, tx| {
            if doctorcodes.contains(uid) {
                tx.send(SseMessage::Logout(message.to_owned())).is_ok()
                // If not `is_ok`, the SSE stream is gone, and so don't retain
            } else {
                // don't send to other user, but do retain
                true
            }
        });
        // guard.remove(doctorcode);
    }
    pub async fn sse_clear_all(&self) {
        let mut sse_lock = self.sse_users.lock().await;
        sse_lock.retain(|_uid, tx| tx.send(SseMessage::Logout(String::from("START SHUTDOWN SERVER"))).is_ok());
        sse_lock.clear();
        let mut sse_anonymous_lock = self.sse_anonymous_senders.lock().await;
        for tx in sse_anonymous_lock.iter() {
            let _ = tx.send(SseMessage::Logout(String::from("START SHUTDOWN SERVER"))).is_ok();
        }
        sse_anonymous_lock.clear();
    }

    pub fn hosxp(&self) -> String {
        self.app_config.hosxp_dbname.clone()
    }
    pub fn kphis(&self) -> String {
        self.app_config.kphis_dbname.clone()
    }
    pub fn kphis_log(&self) -> String {
        self.app_config.kphis_log_dbname.clone()
    }
    pub fn kphis_extra(&self) -> String {
        self.app_config.kphis_extra_dbname.clone()
    }
    pub fn hosxp_hn_len(&self) -> usize {
        self.app_config.hosxp_hn_length
    }
    pub fn hosxp_an_len(&self) -> usize {
        self.app_config.hosxp_an_length
    }
    pub fn hosxp_vn_len(&self) -> usize {
        self.app_config.hosxp_vn_length
    }
    pub fn access_limit(&self) -> u64 {
        self.app_config.access_token_expire_minutes
    }
    pub fn refresh_limit(&self) -> u64 {
        self.app_config.refresh_token_expire_minutes
    }
    pub fn production(&self) -> bool {
        self.app_config.is_production
    }
    pub fn read_only(&self) -> bool {
        self.app_config.is_read_only_mode
    }
    pub fn access_log_only_authorized(&self) -> bool {
        self.app_config.is_access_log_only_authorized
    }
    pub fn ivfluid(&self) -> String {
        self.app_config.hosxp_ivfluid_dosageform.clone()
    }
    pub fn operation_success(&self) -> Vec<u64> {
        self.app_config.hosxp_operation_success.clone()
    }
    pub fn scr_codes(&self) -> Vec<u64> {
        self.app_config.scr_codes.clone()
    }
    pub fn egfr_codes(&self) -> Vec<u64> {
        self.app_config.egfr_codes.clone()
    }
    pub fn lab_codes(&self) -> Vec<(String, Vec<u64>)> {
        self.app_config.lab_codes.clone()
    }
    pub fn message_icodes(&self) -> Vec<(String, Vec<String>)> {
        self.app_config.message_icodes.clone()
    }
    pub fn message_egfr_icodes(&self) -> Vec<(String, u64, Vec<String>)> {
        self.app_config.message_egfr_icodes.clone()
    }
    pub fn message_crcl_icodes(&self) -> Vec<(String, u64, Vec<String>)> {
        self.app_config.message_crcl_icodes.clone()
    }

    // pub async fn get_app_asset(&self, etag: &Option<String>) -> Result<AppAsset, AppError> {
    //     let (exp, etag, app_asset) = {
    //         let lock = self.app_asset_cache.lock().await;
    //         (lock.exp, lock.etag.clone(), lock.app_asset.clone())
    //     };
    //     let now = get_timestamp_server()?;
    //     if exp < now {
    //         self.reload_app_asset();
    //     }
    //     Ok(app_asset)
    // }

    pub fn reload_app_asset(&mut self) {
        if let Ok(now) = get_timestamp_server() {
            self.app_asset_cache_exp = now + (self.app_config.app_asset_cache_minutes * 60);
            // let etag = self.app_asset_cache_etag.clone();
            let asset_cache = self.app_asset_cache.clone();
            let asset_bytes_cache = self.app_asset_bytes_cache.clone();
            let pool = self.db_pool.clone();
            let hosxp = self.hosxp();
            let kphis = self.kphis();
            let fcnote_patient_types = self.app_config.fcnote_patient_types.clone();
            tokio::task::spawn(async move {
                // we load from database so is_from_file always be false
                if let Ok((app_asset, app_asset_bytes, _is_from_file)) = load_app_asset(true, &fcnote_patient_types, &pool, &hosxp, &kphis).await {
                    // // update etag
                    // if let Ok(mut etag_lock) = etag.write() {
                    //     *etag_lock = hash_to_base64_string(&app_asset_bytes);
                    // }
                    // update cache
                    {
                        let mut asset_cache_lock = asset_cache.lock().await;
                        *asset_cache_lock = app_asset;
                    }
                    {
                        let mut asset_bytes_cache_lock = asset_bytes_cache.lock().await;
                        *asset_bytes_cache_lock = app_asset_bytes;
                    }
                }
            });
        }
    }

    pub async fn check_and_apply_triggers(&self) {
        match select_exists_trg_kphis_ipt_log_insert(&self.db_pool, &self.hosxp()).await {
            Ok(is_exists) => {
                if !is_exists {
                    if let Err(e) = add_ipt_insert_trigger(&self.db_pool, &self.hosxp(), &self.kphis_log()).await {
                        warn!("Cannot {}: {}", &e.action, &e.message);
                    } else {
                        info!("trg_kphis_ipt_log_insert created");
                    }
                }
            }
            Err(e) => {
                warn!("Cannot {}: {}", &e.action, &e.message);
            }
        }
        match select_exists_trg_kphis_ipt_log_delete(&self.db_pool, &self.hosxp()).await {
            Ok(is_exists) => {
                if !is_exists {
                    if let Err(e) = add_ipt_delete_trigger(&self.db_pool, &self.hosxp(), &self.kphis_log()).await {
                        warn!("Cannot {}: {}", &e.action, &e.message);
                    } else {
                        info!("trg_kphis_ipt_log_delete created");
                    }
                }
            }
            Err(e) => {
                warn!("Cannot {}: {}", &e.action, &e.message);
            }
        }
    }

    pub fn get_api(&self, path: &Path, user: &UserState) -> Result<Vec<u8>, FileError> {
        match self.json_handle.read() {
            Ok(lock) => lock.process_blocking(path, user, self),
            Err(e) => Err(FileError::Other(Some(EcoString::from(e.to_string())))),
        }
    }

    pub async fn update_role_permission(&self) -> Result<(), AppError> {
        let all_roles_new = get_all_role(&self.db_pool, &self.kphis()).await?;
        let roles_new = role_with_parent(&all_roles_new);

        let role_permission_list_new = get_role_permission_list(UserRoleParams::default(), &self.db_pool, &self.kphis()).await?;
        let roles_permissions_new = roles_permissions(&role_permission_list_new);

        *self.roles.lock().await = roles_new;
        *self.roles_permissions.lock().await = roles_permissions_new;

        Ok(())
    }
}

#[derive(Clone)]
pub struct RequestState {
    pub api_state: ApiState,
    pub user_state: UserState,
    matched_path: MatchedPath,
    path_query: Option<PathAndQuery>,
}

impl RequestState {
    /// May error `403`, `500`
    pub async fn authorize_and_access_log(&self, method: &Method, is_pre_admit: bool) -> Result<(), AppError> {
        let endpoint_with_prefix = self.matched_path.as_str();
        let endpoint_striped = endpoint_with_prefix.strip_prefix(API_PREFIX).unwrap_or(endpoint_with_prefix);
        let endpoint = EndPoint::from_str(endpoint_striped).unwrap_or(EndPoint::Unknown);
        // debug!("convert MatchedPath: {} to Endpoint.base(): {}", matched_path.as_str(), endpoint.base());

        let accepted = !self.api_state.production() || ((!self.api_state.read_only() || matches!(method, &Method::GET)) && endpoint.is_allow(method, &self.user_state.permissions, is_pre_admit));

        if accepted || !self.api_state.access_log_only_authorized() {
            let access_detail = access_detail(method, &self.path_query, accepted);

            let result = log::insert_access_log(
                &self.user_state.user.loginname,
                &self.user_state.addr.to_string(),
                &access_detail,
                &self.api_state.db_pool,
                &self.api_state.kphis_log(),
            )
            .await?;
            if result.rows_affected() == 0 {
                return Err(Source::App.to_error(500, "Failed Insert AccessLog", "Insert AccessLog"));
            }
        }
        if accepted {
            Ok(())
        } else {
            Err(Source::App.to_error(403, [&endpoint.to_string(), " not allowed"].concat(), "Authorize and AccessLog"))
        }
    }
}

fn access_detail(method: &Method, path_query: &Option<PathAndQuery>, accepted: bool) -> String {
    let path = path_query.as_ref().map(|pq| pq.to_string()).unwrap_or_default();
    let status = if accepted { "accepted" } else { "rejected" };
    ["{\"method\":\"", method.as_ref(), "\",\"path\":\"", &path, "\",\"status\":\"", status, "\"}"].concat()
}

impl<S> FromRequestParts<S> for RequestState
where
    ApiState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let ConnectInfo(addr): ConnectInfo<SocketAddr> = ConnectInfo::from_request_parts(parts, state)
            .await
            .map_err(|_| Source::App.to_error(500, "SocketAddr Not Found", "ExtractUser"))?;
        let matched_path = MatchedPath::from_request_parts(parts, state)
            .await
            .map_err(|_| Source::App.to_error(500, "MatchedPath Not Found", "ExtractUser"))?;
        let State(api_state): State<ApiState> = State::from_request_parts(parts, state)
            .await
            .map_err(|_| Source::App.to_error(500, "ApiState Not Found", "ExtractUser"))?;
        let TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>> = TypedHeader::from_request_parts(parts, state)
            .await
            .map_err(|_| Source::App.to_error(400, "Token Not Found", "ExtractUser"))?;
        let path_query = parts.uri.path_and_query().cloned();
        let real_addr = api_state
            .app_config
            .real_ip_header
            .as_ref()
            .and_then(|real_ip_header| parts.headers.get(real_ip_header))
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse::<IpAddr>().ok())
            .map(|ip| SocketAddr::new(ip, addr.port()))
            .unwrap_or(addr);
        let user_state = UserState::from_token(bearer.token(), real_addr, &api_state).await?;

        Ok(Self {
            api_state,
            user_state,
            matched_path,
            path_query,
        })
    }
}

#[derive(Clone)]
pub struct UserState {
    pub state_id: u128,
    pub user: UserDb,
    pub roles: Vec<CurrentUserRole>,
    pub permissions: Vec<Permission>,
    pub addr: SocketAddr,
}

impl UserState {
    /// May error 401, 500
    pub async fn from_token(token: &str, addr: SocketAddr, app: &ApiState) -> Result<Self, AppError> {
        let claims = get_claim_and_verify_public(token, &app.paseto.public)?;
        let state_id = get_state_id(&claims)?;
        let user_state = app
            .online_get(state_id)
            .await
            .ok_or_else(|| Source::App.to_error(401, "กรุณาเข้าสู่ระบบใหม่", "Get UserState").with_title(kphis_util::error::ErrorTitle::NoUserState))?;

        if user_state.addr.ip() == addr.ip() {
            Ok(user_state)
        } else {
            Err(Source::App.to_error(401, "Mismatched IP Address", "Get UserState"))
        }
    }

    pub fn trace_req_by(&self) {
        tracing::debug!("requested by {} from {}", self.user.name, self.addr);
    }
}

pub fn get_state_id(claims: &Claims) -> Result<u128, AppError> {
    let Ulid(state_id) = Ulid::from_string(&claims.sub).map_err(|e| Source::UlidDecode.to_error(401, e, "Claims"))?;

    Ok(state_id)
}

pub struct ApiConfig {
    pub access_token_expire_minutes: u64,
    pub refresh_token_expire_minutes: u64,
    pub reauthen_before_refresh_token_expire_minutes: u64,
    pub handshake_2fa_timeout_second: u64,
    pub app_asset_cache_minutes: u64,
    pub request_body_limited_mb: u8,
    pub rate_limit_burst_size: u32,
    pub rate_limit_replenish_every_millisecond: u64,
    pub real_ip_header: Option<String>,

    pub is_production: bool,
    pub is_read_only_mode: bool,
    pub is_access_log_only_authorized: bool,
    pub is_checked_pharmacist_can_done: bool,
    pub has_covid_lab: bool,
    pub allow_insert_his: bool,
    pub can_sign_pdf: bool,

    pub hosxp_dbname: String,
    pub kphis_dbname: String,
    pub kphis_log_dbname: String,
    pub kphis_extra_dbname: String,
    pub hosxp_hn_length: usize,
    pub hosxp_vn_length: usize,
    pub hosxp_an_length: usize,

    pub pacs_config: Option<PacsConfig>,

    pub hospcode: String,
    pub code_name: String,
    pub hospital_name: String,
    pub hospital_short_name: String,
    pub hospital_info: String,
    pub drug_notify_use: String,
    pub drug_notify_start_end_marker_use: String,
    pub drug_notify_start_marker: String,
    pub drug_notify_end_marker: String,

    pub score_ews: Vec<SupportedScore>,
    pub score_qsofa: Vec<SupportedScore>,
    pub score_sirs: Vec<SupportedScore>,

    pub nurse_assign_groups: Vec<String>,

    pub shift_day_start: Time,
    pub shift_evening_start: Time,
    pub shift_night_start: Time,

    pub hosxp_ivfluid_dosageform: String,
    pub hosxp_injection_dosageforms: Vec<String>,
    pub hosxp_had_displaycolor: Option<i32>,
    pub hosxp_lasa_displaycolor: Option<i32>,
    pub hosxp_med_reconcilation_icode: String,
    pub hosxp_lab_wbc_code: i64,
    pub hosxp_lab_band_code: i64,
    pub hosxp_operation_success: Vec<u64>,
    pub doctor_intern_roles: Vec<String>,

    pub concat_with_space: bool,

    pub report_coercions: Option<Arc<HashMap<String, String>>>,
    pub fcnote_patient_types: Vec<ColorSelectOption>,
    pub lab_alerts: Vec<(String, Vec<u64>, String)>,
    pub scr_codes: Vec<u64>,
    pub egfr_codes: Vec<u64>,
    pub lab_codes: Vec<(String, Vec<u64>)>,
    pub message_icodes: Vec<(String, Vec<String>)>,
    pub message_egfr_icodes: Vec<(String, u64, Vec<String>)>,
    pub message_crcl_icodes: Vec<(String, u64, Vec<String>)>,

    pub pacs_hn_url: Option<String>,
    pub ekg_hn_url: Option<String>,
    pub scan_hn_url: Option<String>,
    pub scan_an_url: Option<String>,
    pub cart_vnan_url: Option<String>,
    pub food_url: Option<String>,
}

impl ApiConfig {
    fn is_shift_valid(&self) -> bool {
        let shift_day_start = self.shift_day_start;
        let shift_evening_start = self.shift_evening_start;
        let shift_night_start = self.shift_night_start;
        (shift_day_start > shift_night_start || shift_night_start > shift_evening_start) && shift_evening_start > shift_day_start
        // (shift_night_start > shift_evening_start && shift_evening_start > shift_day_start) ||
        // (shift_evening_start > shift_day_start && shift_day_start > shift_night_start)
    }
}

pub fn role_with_parent(all_roles: &[Role]) -> HashMap<String, Vec<String>> {
    let mut roles: HashMap<String, Vec<String>> = HashMap::new();
    for role in all_roles.iter() {
        let mut with_parent = vec![role.role.clone()];
        role_parent_recursive(role, all_roles, &mut with_parent);
        roles.insert(role.role.clone(), with_parent);
    }
    roles
}
fn role_parent_recursive(role: &Role, all_roles: &[Role], with_parent: &mut Vec<String>) {
    if let Some(parent) = role.parent_role.as_ref() {
        if let Some(role) = all_roles.iter().find(|role| &role.role == parent) {
            with_parent.push(parent.clone());
            role_parent_recursive(role, all_roles, with_parent);
        }
    }
}

pub fn roles_permissions(role_permission_list: &[RolePermissionList]) -> HashMap<String, HashSet<Permission>> {
    let mut roles_permissions: HashMap<String, HashSet<Permission>> = HashMap::new();
    for role_permission in role_permission_list {
        let permissions = HashSet::from_iter(role_permission.permissions.clone().unwrap_or_default());
        match roles_permissions.get_mut(&role_permission.role) {
            Some(rp) => {
                rp.extend(permissions);
            }
            None => {
                roles_permissions.insert(role_permission.role.clone(), permissions);
            }
        }
    }
    roles_permissions
}
