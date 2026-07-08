use derive_demo::Demo;
use js_sys::JsString;
use serde_derive::{Deserialize, Serialize};
use sqlx::{
    FromRow,
    types::time::{Date, PrimitiveDateTime, Time},
};
use std::rc::Rc;
use time::macros::{date, datetime, time};
use utoipa::{IntoParams, ToSchema};
use wasm_bindgen::JsCast;

use kphis_util::{
    datetime::{date_th_opt, datetime_th_opt},
    error::{AppError, Source},
    util::str_some,
};

use crate::{
    app::AppState,
    endpoint::{EndPoint, QueryString, find_qs},
    fetch::{ExecuteResponse, execute_fetch, fetch_json_api},
};

#[derive(Clone, Default, Deserialize, IntoParams)]
pub struct PrescriptionScreenParams {
    pub search: Option<String>,
    pub vn: Option<String>,
    /// for patch
    pub action: Option<String>, // check, done
}

impl QueryString for PrescriptionScreenParams {
    fn from_tuples(params: &[(String, String)]) -> Option<Self> {
        (!params.is_empty()).then(|| Self {
            search: find_qs(params, "search"),
            vn: find_qs(params, "vn"),
            action: find_qs(params, "action"),
        })
    }

    fn query_string(&self) -> String {
        let mut queries = Vec::with_capacity(3);
        if let Some(search) = &self.search {
            queries.push(["search=", search].concat());
        }
        if let Some(vn) = &self.vn {
            queries.push(["vn=", vn].concat());
        }
        if let Some(action) = &self.action {
            queries.push(["action=", action].concat());
        }

        (!queries.is_empty()).then(|| ["?", &queries.join("&")].concat()).unwrap_or_default()
    }
}

/// Prescription Screening data
#[derive(Clone, Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(PrescriptionScreen::demo()))]
pub struct PrescriptionScreen {
    #[Demo(value = "Some(PrescriptionInfo::demo())")]
    pub info: Option<PrescriptionInfo>,
    #[Demo(value = "Some(PrescriptionVn::demo())")]
    pub visit: Option<PrescriptionVn>,
}

impl PrescriptionScreen {
    /// GET `EndPoint::PrescrptionScreen`
    pub async fn call_api_get(params: &PrescriptionScreenParams, app: Rc<AppState>) -> Result<Self, AppError> {
        match fetch_json_api(&[EndPoint::PrescrptionScreen.base(), params.clone().query_string()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Self = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch PrescriptionScreen"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch PrescriptionScreen"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }

    /// POST `EndPoint::PrescrptionScreen`
    pub async fn call_api_post(params: &PrescriptionScreenParams, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        execute_fetch(&[EndPoint::PrescrptionScreen.base(), params.query_string()].concat(), "POST", None, app).await
    }
}

/// Prescription Info
#[derive(Clone, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(PrescriptionInfo::demo()))]
pub struct PrescriptionInfo {
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub vstdate: Option<Date>,
    #[Demo(value = "Some(1)")]
    pub oqueue: Option<i32>,
    #[Demo(value = r#"Some(String::from("0001234"))"#)]
    pub hn: Option<String>,
    #[Demo(value = r#"Some(String::from("661231235959"))"#)]
    pub vn: Option<String>,
    #[Demo(value = r#"Some(String::from("1111111111111"))"#)]
    pub cid: Option<String>,
    #[Demo(value = r#"Some(String::from("Mr.Patient Sicker"))"#)]
    pub fullname: Option<String>,
    #[Demo(value = "Some(33)")]
    pub age_y: Option<i8>,
    #[Demo(value = "Some(3)")]
    pub age_m: Option<i64>,
    #[Demo(value = "Some(3)")]
    pub age_d: Option<i64>,
    #[Demo(value = r#"Some(String::from("ชาย"))"#)]
    pub sex_name: Option<String>,
    #[Demo(value = r#"Some(String::from("88 moo 8"))"#)]
    pub homeaddr: Option<String>,
    #[Demo(value = r#"Some(String::from("888-888-8888"))"#)]
    pub hometel: Option<String>,
    #[sqlx(skip)]
    #[Demo(value = "vec![VisitDate::demo()]")]
    pub dates: Vec<VisitDate>,
    #[sqlx(skip)]
    #[Demo(value = "vec![PtNote::demo()]")]
    pub notes: Vec<PtNote>,
    #[sqlx(skip)]
    #[Demo(value = r#"vec![String::from("PENICILLIN=Rash")]"#)]
    pub drug_allergies: Vec<String>,
    #[sqlx(skip)]
    #[Demo(value = "vec![Lab::demo()]")]
    pub last_labs: Vec<Lab>,
}

/// Visit Date of Prescription Info
#[derive(Clone, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(VisitDate::demo()))]
pub struct VisitDate {
    #[Demo(value = r#"Some(String::from("661231235959"))"#)]
    pub vn: Option<String>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub vstdate: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub vsttime: Option<Time>,
    #[Demo(value = r#"Some(String::from("660001234"))"#)]
    pub an: Option<String>,
}

/// Note from HOSxP
#[derive(Clone, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(PtNote::demo()))]
pub struct PtNote {
    #[Demo(value = "1")]
    pub ptnote_id: i32,
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub note_datetime: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(String::from("Note"))"#)]
    pub plain_text: Option<String>,
}
impl PtNote {
    pub fn string(&self) -> String {
        ["[", &datetime_th_opt(&self.note_datetime), "] : ", &self.plain_text.clone().unwrap_or_default()].concat()
    }
}

/// Lab of Prescription Info
#[derive(Clone, Demo, Default, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(Lab::demo()))]
pub struct Lab {
    #[Demo(value = r#"String::from("WBC")"#)]
    pub lab_name: String,
    #[Demo(value = "Some(1)")]
    pub lab_items_code: Option<i32>,
    #[Demo(value = r#"Some(String::from("WBC"))"#)]
    pub lab_items_name_ref: Option<String>,
    #[Demo(value = r#"Some(String::from("cell/mm3"))"#)]
    pub lab_items_unit: Option<String>,
    #[Demo(value = r#"Some(String::from("1-2"))"#)]
    pub lab_items_normal_value: Option<String>,
    #[Demo(value = "Some(1)")]
    pub lab_order_number: Option<i32>,
    #[Demo(value = r#"Some(String::from("8,888"))"#)]
    pub lab_order_result: Option<String>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub order_date: Option<Date>,
    #[Demo(value = r#"Some(String::from("0001234"))"#)]
    pub hn: Option<String>,
    #[Demo(value = r#"Some(String::from("661231235959"))"#)]
    pub vn: Option<String>,
}

impl Lab {
    pub fn new(name: &str) -> Self {
        Self {
            lab_name: name.to_owned(),
            ..Default::default()
        }
    }
}

/// HosXp Prescription Info
#[derive(Clone, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(PrescriptionVn::demo()))]
pub struct PrescriptionVn {
    #[Demo(value = "Some(1)")]
    pub oqueue: Option<i32>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub vstdate: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub vsttime: Option<Time>,
    #[Demo(value = r#"Some(String::from("0001234"))"#)]
    pub hn: Option<String>,
    #[Demo(value = r#"Some(String::from("661231235959"))"#)]
    pub vn: Option<String>,
    #[Demo(value = r#"Some(String::from("660001234"))"#)]
    pub an: Option<String>,
    #[Demo(value = r#"Some(String::from("Dr.Doctor"))"#)]
    pub doctor_name: Option<String>,
    #[Demo(value = r#"Some(String::from("บุคคลในครอบครัว อสม."))"#)]
    pub pttype_name: Option<String>,
    #[Demo(value = r#"Some(String::from("Sick"))"#)]
    pub cc: Option<String>,
    #[Demo(value = r#"Some(String::from("Present Hx"))"#)]
    pub hpi: Option<String>,
    #[Demo(value = r#"Some(String::from("PE"))"#)]
    pub pe: Option<String>,
    #[Demo(value = r#"Some(String::from("I10 : Essential Hypertension"))"#)]
    pub diag: Option<String>,
    #[Demo(value = "Some(37.5)")]
    pub temperature: Option<f64>,
    #[Demo(value = "Some(120.0)")]
    pub bps: Option<f64>,
    #[Demo(value = "Some(80.0)")]
    pub bpd: Option<f64>,
    #[Demo(value = "Some(50.0)")]
    pub bw: Option<f64>,
    #[Demo(value = "Some(170)")]
    pub height: Option<i32>,
    #[Demo(value = "Some(17.3)")]
    pub bmi: Option<f64>,
    #[Demo(value = "Some(88.8)")]
    pub fbs: Option<f64>,

    #[Demo(value = r#"Some(String::from("Mr.Pharmacist"))"#)]
    pub pharmacist_accept_name: Option<String>,
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub pharmacist_accept_time: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(String::from("Mr.Checker"))"#)]
    pub pharmacist_check_name: Option<String>,
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub pharmacist_check_time: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(String::from("Mr.Payer"))"#)]
    pub pharmacist_done_name: Option<String>,
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub pharmacist_done_time: Option<PrimitiveDateTime>,

    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub postal_status: Option<String>,
    #[Demo(value = r#"Some(String::from("Mr.Payer"))"#)]
    pub postal_doctor_name: Option<String>,
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub postal_time: Option<PrimitiveDateTime>,

    #[Demo(value = r#"Some(String::from("ADD"))"#)]
    pub telemed_add: Option<String>,
    #[Demo(value = r#"Some(String::from("DOSE UP"))"#)]
    pub telemed_dose_up: Option<String>,
    #[Demo(value = r#"Some(String::from("DOSE DOWN"))"#)]
    pub telemed_dose_down: Option<String>,
    #[Demo(value = r#"Some(String::from("OFF"))"#)]
    pub telemed_off: Option<String>,
    #[Demo(value = r#"Some(String::from("OTHER"))"#)]
    pub telemed_other: Option<String>,
    #[Demo(value = r#"Some(String::from("Mr.Payer"))"#)]
    pub telemed_doctor_name: Option<String>,
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub telemed_time: Option<PrimitiveDateTime>,

    #[Demo(value = r#"Some(String::from("Drug precaution"))"#)]
    pub pharmacy_care: Option<String>,
    #[Demo(value = r#"Some(String::from("Mr.Pharm"))"#)]
    pub pharmacy_care_doctor_name: Option<String>,
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub pharmacy_care_time: Option<PrimitiveDateTime>,

    #[sqlx(skip)]
    #[Demo(value = "vec![Medicine::demo()]")]
    pub medicines: Vec<Medicine>,
    #[sqlx(skip)]
    #[Demo(value = "vec![DrugInteraction::demo()]")]
    pub drug_interactions: Vec<DrugInteraction>,
    #[sqlx(skip)]
    #[Demo(value = "vec![Lab::demo()]")]
    pub labs: Vec<Lab>,
    #[sqlx(skip)]
    #[Demo(value = "vec![NextAppointment::demo()]")]
    pub next_app: Vec<NextAppointment>,
    #[sqlx(skip)]
    #[Demo(value = r#"vec![String::from("Interaction: drug:(WARFARIN, PARACETAMOL)")]"#)]
    pub mess_vn: Vec<String>,
}

/// Medicine data of HosXp Prescription Info
#[derive(Clone, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(Medicine::demo()))]
pub struct Medicine {
    #[Demo(value = r#"Some(String::from("PARACETAMOL"))"#)]
    pub name_drugitems: Option<String>,
    #[Demo(value = r#"Some(String::from("PARACETAMOL"))"#)]
    pub generic_name: Option<String>,
    #[Demo(value = r#"Some(String::from("500 mg."))"#)]
    pub strength: Option<String>,
    #[Demo(value = "Some(10)")]
    pub qty: Option<i32>,
    #[Demo(value = r#"Some(String::from("1000227"))"#)]
    pub icode: Option<String>,
    #[Demo(value = r#"Some(datetime!(2023-12-31 23:59:59))"#)]
    pub rxdatetime: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(String::from("0666"))"#)]
    pub drugusage: Option<String>,
    #[Demo(value = r#"Some(String::from("661231235959"))"#)]
    pub vn: Option<String>,
    #[Demo(value = r#"Some(String::from("660001234"))"#)]
    pub an: Option<String>,
    #[Demo(value = r#"Some(String::from("0001234"))"#)]
    pub hn: Option<String>,
    #[Demo(value = r#"Some(String::from("1111111"))"#)]
    pub sp_use: Option<String>,
    #[Demo(value = r#"Some(String::from("22pt (2 เม็ด * 2 PC)"))"#)]
    pub shortlist: Option<String>,
    /// type^code^name_drugitems^strength^qty^icode^datetime^shortlist
    #[Demo(value = r#"Some(String::from("VN^660531084331^PARACETAMOL 500 mg. เม็ด^500 mg.^20^1000227^2023-05-31 09:49:12^22pt (2 เม็ด * 2 PC)"))"#)]
    pub last_prescription: Option<String>,
}

pub struct LastMedicine {
    pub id_type: String,
    pub id: Option<String>,
    pub name_drugitems: Option<String>,
    pub strength: Option<String>,
    pub qty: Option<i32>,
    pub icode: Option<String>,
    pub rxdatetime: Option<String>,
    pub shortlist: Option<String>,
}
impl LastMedicine {
    pub fn new(concat: &Option<String>) -> Option<Self> {
        concat.as_ref().map(|cc| cc.split('^').collect::<Vec<&str>>()).and_then(|lm| {
            if lm.len() == 8 {
                Some(Self {
                    id_type: lm[0].to_owned(),
                    id: str_some(lm[1].to_owned()),
                    name_drugitems: str_some(lm[2].to_owned()),
                    strength: str_some(lm[3].to_owned()),
                    qty: lm[4].parse::<i32>().ok(),
                    icode: str_some(lm[5].to_owned()),
                    rxdatetime: str_some(lm[6].to_owned()),
                    shortlist: str_some(lm[7].to_owned()),
                })
            } else {
                None
            }
        })
    }
}

/// Drug Interaction of HosXp Prescription Info
#[derive(Clone, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(DrugInteraction::demo()))]
pub struct DrugInteraction {
    #[Demo(value = r#"Some(String::from("WARFARIN"))"#)]
    pub drugname1: Option<String>,
    #[Demo(value = r#"Some(String::from("PARACETAMOL"))"#)]
    pub drugname2: Option<String>,
    #[Demo(value = "Some(1)")]
    pub severity: Option<i32>,
    #[Demo(value = r#"Some(String::from("Note"))"#)]
    pub note: Option<String>,
}

/// Next Appointment of HosXp Prescription Info
#[derive(Clone, Debug, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(NextAppointment::demo()))]
pub struct NextAppointment {
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub nextdate: Option<Date>,
    #[Demo(value = r#"Some(String::from("ผู้ป่วยนอก"))"#)]
    pub clinic_name: Option<String>,
    #[Demo(value = "Some(30)")]
    pub days: Option<i32>,
}
impl NextAppointment {
    pub fn string(&self) -> String {
        [&self.days.map(|i| i.to_string()).unwrap_or_default(), " วัน (", &date_th_opt(&self.nextdate), ") ", &self.clinic_name.clone().unwrap_or_default()].concat()
    }
}

/// PresctiptionScreen data for patch
#[derive(Clone, Default, Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(PrescriptionScreenPatch::demo()))]
pub struct PrescriptionScreenPatch {
    #[Demo(value = r#"Some(PostalPatch::demo())"#)]
    pub postal: Option<PostalPatch>,
    #[Demo(value = r#"Some(TelemedPatch::demo())"#)]
    pub telemed: Option<TelemedPatch>,
    #[Demo(value = r#"Some(String::from("Drug precaution"))"#)]
    pub pharmacy_care: Option<String>,
}

impl PrescriptionScreenPatch {
    /// PATCH `EndPoint::PrescrptionScreen`
    pub async fn call_api_patch(&self, params: &PrescriptionScreenParams, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        let body_json = serde_json::to_string(self).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Patch PrescriptionScreen"))?;

        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Patch PrescriptionScreen"))?;

        execute_fetch(&[EndPoint::PrescrptionScreen.base(), params.query_string()].concat(), "PATCH", Some(&body), app).await
    }
}

/// Postal data for save
#[derive(Clone, Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(PostalPatch::demo()))]
pub struct PostalPatch {
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub postal_status: Option<String>,
}

/// Telemed data for save
#[derive(Clone, Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(TelemedPatch::demo()))]
pub struct TelemedPatch {
    #[Demo(value = r#"Some(String::from("ADD"))"#)]
    pub telemed_add: Option<String>,
    #[Demo(value = r#"Some(String::from("DOSE UP"))"#)]
    pub telemed_dose_up: Option<String>,
    #[Demo(value = r#"Some(String::from("DOSE DOWN"))"#)]
    pub telemed_dose_down: Option<String>,
    #[Demo(value = r#"Some(String::from("OFF"))"#)]
    pub telemed_off: Option<String>,
    #[Demo(value = r#"Some(String::from("OTHER"))"#)]
    pub telemed_other: Option<String>,
}
