use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordVerifier},
};
use axum::{
    Json,
    extract::{ConnectInfo, State},
    http::HeaderMap,
};
use std::{
    collections::HashSet,
    net::{IpAddr, SocketAddr},
};
use tower_cookies::{Cookie, Cookies, cookie::SameSite};
use ulid::Ulid;

use kphis_api_core::{
    open_api::{DocOne, DocOpt},
    state::{ApiState, UserState, get_state_id},
    token::{TokenType, gen_token_public, get_claim_and_verify_public},
};
use kphis_api_query::user::{config, login, totp};
use kphis_model::{
    app::AppStatus,
    user::{
        his::{CurrentUserRole, LoginResponse, UserDb, UserRequest, UserRequest2fa, UserRequestFull},
        permission::Permission,
    },
};
use kphis_util::{
    datetime::get_timestamp_server,
    error::{AppError, ErrorTitle, Source},
};

pub const COOKIE_TOKEN_NAME: &str = "REFRESH";

// from SessionManager.php::checklogin() plus access token and refresh token cookie
// return 401 Unauthorized when user not found and failed password checking
// return 500 Internal Server Error for others error occured
/// /api/user
///
/// Tries to Log-In with password, return single Login Response (Access Token insided) and new Refersh Token in Secure Cookies<br>
/// or NULL for next step
#[utoipa::path(
    post,
    path = "/user",
    request_body = UserRequest,
    responses(DocOpt<LoginResponse>),
)]
pub async fn check_login(
    ConnectInfo(socket_addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    State(app): State<ApiState>,
    cookies: Cookies,
    Json(payload): Json<UserRequest>,
) -> Result<Json<Option<LoginResponse>>, AppError> {
    let user = login::get_user(&payload.username, &app.db_pool, &app.hosxp(), &app.kphis(), &app.kphis_extra())
        .await?
        .ok_or_else(|| AppError::app_401("Check Login").with_title(ErrorTitle::Security))?;

    // check password
    if let Err(e) = verify_password(&user.passweb, &payload.password) {
        tracing::warn!("user {} failed to login with {}", user.name, e.message);
        return Err(e);
    }

    // prepare TS for TOTP
    let response = if user.totp_done.unwrap_or_default() {
        if config::update_ts(&user.loginname, &app.db_pool, &app.kphis_extra()).await?.rows_affected() > 0 {
            None
        } else {
            return Err(AppError::app_401("Check Login").with_title(ErrorTitle::Security));
        }
    } else {
        let real_addr = app
            .app_config
            .real_ip_header
            .as_ref()
            .and_then(|real_ip_header| headers.get(real_ip_header))
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse::<IpAddr>().ok())
            .map(|ip| SocketAddr::new(ip, socket_addr.port()))
            .unwrap_or(socket_addr);
        let response = login_response(real_addr, &user, &cookies, &app).await?;
        tracing::info!("User {} Log-in from {}", &user.name, real_addr.to_string());

        Some(response)
    };

    Ok(Json(response))
}

// return 401 Unauthorized when user not found and failed TOTP checking
// return 500 Internal Server Error for others error occured
/// /api/user
///
/// Tries to Log-In with TOTP, return single Login Response (Access Token insided) and new Refersh Token in Secure Cookies<br>
/// or NULL if timeout
#[utoipa::path(
    patch,
    path = "/user",
    request_body = UserRequest2fa,
    responses(DocOpt<LoginResponse>),
)]
pub async fn check_totp(
    ConnectInfo(socket_addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    State(app): State<ApiState>,
    cookies: Cookies,
    Json(payload): Json<UserRequest2fa>,
) -> Result<Json<Option<LoginResponse>>, AppError> {
    let loginname = if payload.is_sub {
        let Ulid(state_id) = Ulid::from_string(&payload.username).map_err(|e| Source::UlidDecode.to_error(401, e, "Check TOTP"))?;

        match app.online_get(state_id).await.map(|state| state.user.loginname) {
            Some(uname) => uname,
            None => {
                return Err(AppError::app_401("Check TOTP").with_title(ErrorTitle::Security));
            }
        }
    } else {
        payload.username
    };

    let mut user = login::get_user(&loginname, &app.db_pool, &app.hosxp(), &app.kphis(), &app.kphis_extra())
        .await?
        .ok_or_else(|| AppError::app_401("Check TOTP").with_title(ErrorTitle::Security))?;

    // check TOTP
    if let (Some(totp_pk), Some(ts), Ok(now)) = (&user.totp, user.ts, get_timestamp_server()) {
        // check TS
        if ts.saturating_add(app.app_config.handshake_2fa_timeout_second) > now {
            if !totp::verify_totp_encoded_key(&user.loginname, &payload.token_2fa, totp_pk, "KPHIS")? {
                tracing::warn!("user {} failed to login with wrong TOTP", user.name);
                if payload.is_sub {
                    // we avoid 401 to prevent user dropped in client
                    return Err(Source::App.to_error(200, "Try Again", "Check TOTP").with_title(ErrorTitle::Security));
                } else {
                    return Err(AppError::app_401("Check TOTP").with_title(ErrorTitle::Security));
                }
            } else if payload.is_sub {
                if config::update_totp_done(&user.loginname, &app.db_pool, &app.kphis_extra()).await?.rows_affected() > 0 {
                    user.totp_done = Some(true);
                }
            }
        } else {
            tracing::warn!("user {} timeout for login with TOTP", user.name);
            return Ok(Json(None));
        }
    } else {
        return Err(AppError::app_401("Check TOTP").with_title(ErrorTitle::Security));
    }

    let real_addr = app
        .app_config
        .real_ip_header
        .as_ref()
        .and_then(|real_ip_header| headers.get(real_ip_header))
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse::<IpAddr>().ok())
        .map(|ip| SocketAddr::new(ip, socket_addr.port()))
        .unwrap_or(socket_addr);
    let response = login_response(real_addr, &user, &cookies, &app).await?;
    tracing::info!("User {} Log-in from {}", &user.name, real_addr.to_string());

    Ok(Json(Some(response)))
}

async fn login_response(socket_addr: SocketAddr, user: &UserDb, cookies: &Cookies, app: &ApiState) -> Result<LoginResponse, AppError> {
    // generate new state_id, access token and refresh token
    let Ulid(state_id) = Ulid::new();
    let access_token = gen_token_public(
        state_id,
        &user.name,
        (app.access_limit(), app.refresh_limit()),
        None, // generate new rexp
        TokenType::Access,
        &app.paseto.secret,
    )?;
    let refresh_token = gen_token_public(
        state_id,
        &user.name,
        (app.refresh_limit(), app.refresh_limit()),
        None, // generate new rexp
        TokenType::Refresh,
        &app.paseto.secret,
    )?;

    // create cookie of refresh token
    let cookie = Cookie::build((COOKIE_TOKEN_NAME, refresh_token)).http_only(true).same_site(SameSite::Strict).secure(true);
    cookies.add(cookie.into());

    // app_status
    let app_status = app_status_from_api_state(&app);

    // roles
    let roles = login::get_user_roles(&user.loginname, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
    let mut role_names: HashSet<String> = HashSet::new();
    {
        let all_roles = app.roles.lock().await;
        for role in roles.iter() {
            if let Some(with_parent) = all_roles.get(&role.role) {
                role_names.extend(with_parent.clone());
            }
        }
    }
    let mut permissions_names: HashSet<Permission> = HashSet::new();
    {
        let all_roles_permissions = app.roles_permissions.lock().await;
        for role_name in role_names {
            if let Some(rp) = all_roles_permissions.get(&role_name) {
                permissions_names.extend(rp.clone());
            }
        }
    }
    let permissions = permissions_names.into_iter().collect::<Vec<Permission>>();

    // set backend users state
    app.online_add(state_id, &user, &roles, &permissions, socket_addr).await;

    let response = LoginResponse {
        token: access_token,
        timestamp: get_timestamp_server()?,
        public: app.paseto_public.to_string(),
        app_status,
        user: user.into(),
        roles,
        permissions,
    };

    Ok(response)
}

pub fn verify_password(password: &str, hash: &str) -> Result<(), AppError> {
    let passweb_byte = hex::decode(password).unwrap_or(password.as_bytes().to_vec());

    let parsed_hash = PasswordHash::new(hash).map_err(|e| Source::PasswordHash.to_error(401, e, "Verify Password"))?;

    // log when failed login
    Argon2::default()
        .verify_password(&passweb_byte, &parsed_hash)
        .map_err(|e| Source::App.to_error(401, e, "Verify Password"))
}

/// /api/user
///
/// Get Access token by Refresh token in Cookie, return single Login Response (new Access Token insided)<br>
/// and new Refersh Token in Secure Cookies
#[utoipa::path(
    get,
    path = "/user",
    responses(DocOne<LoginResponse>),
)]
pub async fn refresh_token(ConnectInfo(socket_addr): ConnectInfo<SocketAddr>, headers: HeaderMap, State(app): State<ApiState>, cookies: Cookies) -> Result<Json<LoginResponse>, AppError> {
    let cookie = cookies.get(COOKIE_TOKEN_NAME).ok_or_else(|| AppError::app_401("Get Token").with_title(ErrorTitle::Security))?;

    let claims = get_claim_and_verify_public(cookie.value(), &app.paseto.public)?;
    if claims.act != "refresh" {
        return Err(AppError::app_401("Get Token").with_title(ErrorTitle::Security));
    }

    let state_id = get_state_id(&claims)?;

    // get user, roles from backend users state
    match app.online_get(state_id).await {
        Some(UserState { user, roles, permissions, addr, .. }) => {
            // check IP address
            let real_addr = app
                .app_config
                .real_ip_header
                .as_ref()
                .and_then(|real_ip_header| headers.get(real_ip_header))
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse::<IpAddr>().ok())
                .unwrap_or(socket_addr.ip());
            if addr.ip() != real_addr {
                tracing::warn!("user {} failed to refresh token because mismatched IP Address", user.name);
                return Err(Source::App.to_error(401, "Mismatched IP Address", "Get Token"));
            }

            let response = refresh_response(state_id, &user, roles, permissions, &cookies, &app)?;
            tracing::info!("User {} request a new access-token from {}", &user.name, real_addr.to_string());

            Ok(Json(response))
        }
        None => Err(AppError::app_401("Get Token").with_title(ErrorTitle::Security)),
    }
}

fn refresh_response(state_id: u128, user: &UserDb, roles: Vec<CurrentUserRole>, permissions: Vec<Permission>, cookies: &Cookies, app: &ApiState) -> Result<LoginResponse, AppError> {
    // generate new access token and new refersh token
    let access_token = gen_token_public(
        state_id,
        &user.name,
        (app.access_limit(), app.refresh_limit()),
        None, // generate new rexp
        TokenType::Access,
        &app.paseto.secret,
    )?;
    let refresh_token = gen_token_public(
        state_id,
        &user.name,
        (app.refresh_limit(), app.refresh_limit()),
        None, // generate new rexp
        TokenType::Refresh,
        &app.paseto.secret,
    )?;
    // create cookie of refresh token
    let cookie = Cookie::build((COOKIE_TOKEN_NAME, refresh_token)).http_only(true).same_site(SameSite::Strict).secure(true);
    // cookies.add() says "If a Cookie with the same name already exists, it is replaced with provided cookie."
    cookies.add(cookie.into());

    let response = LoginResponse {
        token: access_token,
        timestamp: get_timestamp_server()?,
        public: app.paseto_public.to_string(),
        app_status: app_status_from_api_state(&app),
        user: user.into(),
        roles,
        permissions,
    };

    Ok(response)
}

/// /api/user
///
/// Tries create a new refresh cookies, return single Login Response (Access Token insided) and new Refersh Token in Secure Cookies
#[utoipa::path(
    put,
    path = "/user",
    request_body = UserRequestFull,
    responses(DocOne<LoginResponse>),
)]
pub async fn refresh_cookie(
    ConnectInfo(socket_addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    State(app): State<ApiState>,
    cookies: Cookies,
    Json(payload): Json<UserRequestFull>,
) -> Result<Json<LoginResponse>, AppError> {
    // use old state_id
    let Ulid(state_id) = Ulid::from_string(&payload.username).map_err(|e| Source::UlidDecode.to_error(401, e, "RenewRefresh"))?;

    // get user, roles from backend users state
    match app.online_get(state_id).await {
        Some(UserState { user, roles, permissions, addr, .. }) => {
            // check TOTP
            if let Some(totp_pk) = &user.totp {
                if !totp::verify_totp_encoded_key(&user.loginname, &payload.token_2fa, totp_pk, "KPHIS")? {
                    tracing::warn!("user {} failed to login with wrong TOTP", user.name);
                    return Err(AppError::app_401("Check Login").with_title(ErrorTitle::Security));
                }
            }
            // check password
            if let Err(e) = verify_password(&user.passweb, &payload.password) {
                tracing::warn!("user {} failed to login with {}", user.name, e.message);
                return Err(e);
            }
            // check IP address
            let real_addr = app
                .app_config
                .real_ip_header
                .as_ref()
                .and_then(|real_ip_header| headers.get(real_ip_header))
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse::<IpAddr>().ok())
                .unwrap_or(socket_addr.ip());
            if addr.ip() != real_addr {
                tracing::warn!("user {} failed to login because mismatched IP Address", user.name);
                return Err(Source::App.to_error(401, "Mismatched IP Address", "Get Token"));
            }

            let response = refresh_response(state_id, &user, roles, permissions, &cookies, &app)?;
            tracing::info!("User {} request a new refresh-token from {}", &user.name, real_addr.to_string());

            Ok(Json(response))
        }
        None => Err(AppError::app_401("Get Token").with_title(ErrorTitle::Security)),
    }
}

pub fn app_status_from_api_state(app: &ApiState) -> AppStatus {
    AppStatus {
        is_production: app.production(),
        is_read_only_mode: app.read_only(),
        is_checked_pharmacist_can_done: app.app_config.is_checked_pharmacist_can_done,
        has_covid_lab: app.app_config.has_covid_lab,
        allow_insert_his: app.app_config.allow_insert_his,
        can_sign_pdf: app.app_config.can_sign_pdf,
        reauthen_before_refresh_token_expire_minutes: app.app_config.reauthen_before_refresh_token_expire_minutes,
        handshake_2fa_timeout_second: app.app_config.handshake_2fa_timeout_second,

        hosxp_hn_length: app.app_config.hosxp_hn_length,
        hosxp_vn_length: app.app_config.hosxp_vn_length,
        hosxp_an_length: app.app_config.hosxp_an_length,

        hospcode: app.app_config.hospcode.clone(),
        code_name: app.app_config.code_name.clone(),
        hospital_name: app.app_config.hospital_name.clone(),
        hospital_short_name: app.app_config.hospital_short_name.clone(),
        drug_notify_use: app.app_config.drug_notify_use.clone(),
        drug_notify_start_end_marker_use: app.app_config.drug_notify_start_end_marker_use.clone(),
        drug_notify_start_marker: app.app_config.drug_notify_start_marker.clone(),
        drug_notify_end_marker: app.app_config.drug_notify_end_marker.clone(),

        score_ews: app.app_config.score_ews.clone(),
        score_qsofa: app.app_config.score_qsofa.clone(),
        score_sirs: app.app_config.score_sirs.clone(),

        nurse_assign_groups: app.app_config.nurse_assign_groups.clone(),

        shift_day_start: app.app_config.shift_day_start,
        shift_evening_start: app.app_config.shift_evening_start,
        shift_night_start: app.app_config.shift_night_start,

        concat_with_space: app.app_config.concat_with_space,
        report_coercions: app.app_config.report_coercions.clone(),

        hosxp_ivfluid_dosageform: app.app_config.hosxp_ivfluid_dosageform.clone(),
        hosxp_injection_dosageforms: app.app_config.hosxp_injection_dosageforms.clone(),
        hosxp_had_displaycolor: app.app_config.hosxp_had_displaycolor,
        hosxp_lasa_displaycolor: app.app_config.hosxp_lasa_displaycolor,
        hosxp_med_reconcilation_icode: app.app_config.hosxp_med_reconcilation_icode.clone(),
        lab_alerts: app.app_config.lab_alerts.iter().map(|(description, _, _)| description).cloned().collect(),
        has_pacs_host: app.app_config.pacs_config.is_some(),
        pacs_hn_url: app.app_config.pacs_hn_url.clone(),
        ekg_hn_url: app.app_config.ekg_hn_url.clone(),
        scan_hn_url: app.app_config.scan_hn_url.clone(),
        scan_an_url: app.app_config.scan_an_url.clone(),
        cart_vnan_url: app.app_config.cart_vnan_url.clone(),
        food_url: app.app_config.food_url.clone(),
    }
}
