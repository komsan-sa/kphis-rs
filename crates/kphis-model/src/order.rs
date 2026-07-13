use derive_demo::Demo;
use js_sys::JsString;
use serde::{Deserialize, Serialize};
use sqlx::{
    FromRow, MySql, Pool,
    mysql::MySqlQueryResult,
    types::time::{Date, PrimitiveDateTime, Time},
};
use sqlx_binder::MySqlBinder;
use std::rc::Rc;
use time::{
    format_description::well_known::Iso8601,
    macros::{date, datetime, time},
};
use utoipa::{IntoParams, ToSchema};
use wasm_bindgen::JsCast;

use kphis_util::{
    datetime::{date_8601, date_th, datetime_th, js_now},
    error::{AppError, Source},
    util::zero_none,
};

use crate::{
    app::{AppState, VisitTypeId},
    endpoint::{EndPoint, QueryString, find_qs},
    fetch::{ExecuteResponse, execute_fetch, execute_fetch_vec, execute_fetch_vec_with_u32, fetch_json_api},
    index_plan::{IndexPlan, IndexPlanOnly},
    med_reconcile::MedReconciliationItem,
    pre_order::order::{PreOrder, PreOrderItem, PreOrderItemType},
};

/// IPD Order Date with today marking
#[derive(Clone, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(OrderDate::demo()))]
pub struct OrderDate {
    #[Demo(value = "date!(2023-12-31)")]
    pub order_date: Date,
    #[Demo(value = "true")]
    pub is_today: bool,
}

impl OrderDate {
    pub fn string(&self) -> String {
        [self.order_date.to_string(), if self.is_today { String::from("1") } else { String::from("0") }].join("|")
    }

    pub fn from_string(value: &str) -> Option<Self> {
        let tuple = value.split('|').collect::<Vec<&str>>();
        if tuple.len() == 2 {
            Date::parse(tuple[0], &Iso8601::DEFAULT).ok().map(|date| Self {
                order_date: date,
                is_today: tuple[1] == "1",
            })
        } else {
            None
        }
    }

    /// GET `EndPoint::IpdOrderOrderDateAn`
    pub async fn call_api_get(an: &str, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[&EndPoint::IpdOrderOrderDateAn.base(), an].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch OrderDate"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch OrderDate"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}

impl PartialEq for OrderDate {
    fn eq(&self, other: &Self) -> bool {
        self.order_date == other.order_date
    }
}

#[derive(Clone, Default, Deserialize, IntoParams)]
pub struct OrderParams {
    pub an: Option<String>,
    pub opd_er_order_master_id: Option<u32>,
    pub order_id: Option<u32>,
    pub order_item_id: Option<u32>,
    pub order_type: Option<String>,
    pub current_date: Option<Date>,
    /// for GET `xx/order/item` only
    pub plan_date: Option<Date>,
    pub order_confirm: Option<String>,     // "Y"
    pub order_owner_types: Option<String>, // comma delimited
    pub order_item_types: Option<String>,  // comma delimited
    /// `other` will `without_plan` automatically
    pub view_by: Option<String>,
    pub with_offed: Option<String>,    // "Y"
    pub without_order: Option<String>, // "Y"
    pub without_plan: Option<String>,  // "Y"
    /// for getting Order with `nurse_order_as` without `doctor_confirm_time`
    pub doctor_not_confirm_as: Option<String>, // "Y"
}

impl QueryString for OrderParams {
    fn from_tuples(params: &[(String, String)]) -> Option<Self> {
        (!params.is_empty()).then(|| Self {
            an: find_qs(params, "an"),
            opd_er_order_master_id: find_qs(params, "opd_er_order_master_id").and_then(|s| s.parse::<u32>().ok()),
            order_id: find_qs(params, "order_id").and_then(|s| s.parse::<u32>().ok()),
            order_item_id: find_qs(params, "order_item_id").and_then(|s| s.parse::<u32>().ok()),
            order_type: find_qs(params, "order_type"),
            current_date: find_qs(params, "current_date").and_then(|s| date_8601(&s)),
            plan_date: find_qs(params, "plan_date").and_then(|s| date_8601(&s)),
            order_confirm: find_qs(params, "order_confirm"),
            order_owner_types: find_qs(params, "order_owner_types"),
            order_item_types: find_qs(params, "order_item_types"),
            view_by: find_qs(params, "view_by"),
            with_offed: find_qs(params, "with_offed"),
            without_order: find_qs(params, "without_order"),
            without_plan: find_qs(params, "without_plan"),
            doctor_not_confirm_as: find_qs(params, "doctor_not_confirm_as"),
        })
    }

    fn query_string(&self) -> String {
        let mut queries = Vec::with_capacity(14);
        if let Some(an) = &self.an {
            queries.push(["an=", an].concat());
        }
        if let Some(opd_er_order_master_id) = &self.opd_er_order_master_id {
            queries.push(["opd_er_order_master_id=", &opd_er_order_master_id.to_string()].concat());
        }
        if let Some(order_id) = &self.order_id {
            queries.push(["order_id=", &order_id.to_string()].concat());
        }
        if let Some(order_item_id) = &self.order_item_id {
            queries.push(["order_item_id=", &order_item_id.to_string()].concat());
        }
        if let Some(order_type) = &self.order_type {
            queries.push(["order_type=", order_type].concat());
        }
        if let Some(current_date) = &self.current_date {
            queries.push(["current_date=", &current_date.to_string()].concat());
        }
        if let Some(plan_date) = &self.plan_date {
            queries.push(["plan_date=", &plan_date.to_string()].concat());
        }
        if let Some(order_confirm) = &self.order_confirm {
            queries.push(["order_confirm=", order_confirm].concat());
        }
        if let Some(order_owner_types) = &self.order_owner_types {
            queries.push(["order_owner_types=", order_owner_types].concat());
        }
        if let Some(order_item_types) = &self.order_item_types {
            queries.push(["order_item_types=", order_item_types].concat());
        }
        if let Some(view_by) = &self.view_by {
            queries.push(["view_by=", view_by].concat());
        }
        if let Some(with_offed) = &self.with_offed {
            queries.push(["with_offed=", with_offed].concat());
        }
        if let Some(without_order) = &self.without_order {
            queries.push(["without_order=", without_order].concat());
        }
        if let Some(without_plan) = &self.without_plan {
            queries.push(["without_plan=", without_plan].concat());
        }
        if let Some(doctor_not_confirm_as) = &self.doctor_not_confirm_as {
            queries.push(["doctor_not_confirm_as=", doctor_not_confirm_as].concat());
        }
        (!queries.is_empty()).then(|| ["?", &queries.join("&")].concat()).unwrap_or_default()
    }
}

/// Order with Items
#[derive(Clone, Debug, Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(Order::demo()))]
pub struct Order {
    /// for generic over `an` or `opd_er_order_master_id` only
    #[Demo(value = r#"VisitTypeId::demo_ipd(String::from("660001234"))"#)]
    pub visit_type: VisitTypeId,
    #[Demo(value = "1")]
    pub order_id: u32,
    #[Demo(value = r#"Some(String::from("0001234"))"#)]
    pub hn: Option<String>,
    #[Demo(value = r#"Some(String::from("Mr.Patient Sicker"))"#)]
    pub fullname: Option<String>,
    // ipd
    #[Demo(value = r#"Some(String::from("ตึกชาย"))"#)]
    pub ward_name: Option<String>,
    #[Demo(value = r#"Some(String::from("C01"))"#)]
    pub bedno: Option<String>,
    // opd-er
    #[Demo(value = r#"Some(String::from("1"))"#)]
    pub display_bedno: Option<String>,
    #[Demo(value = r#"Some(String::from("แดง"))"#)]
    pub bed_type_name: Option<String>,
    #[Demo(value = r##"Some(String::from("#e47e7e"))"##)]
    pub bed_type_color: Option<String>,

    #[Demo(value = "date!(2023-12-31)")]
    pub order_date: Date,
    #[Demo(value = "time!(23:59:59)")]
    pub order_time: Time,
    #[Demo(value = r#"String::from("008")"#)]
    pub order_doctor: String,
    #[Demo(value = r#"String::from("oneday")"#)]
    pub order_type: String,
    #[Demo(value = r#"String::from("nurse")"#)]
    pub order_owner_type: String,
    #[Demo(value = r#"String::from("Y")"#)]
    pub order_confirm: String,
    #[Demo(value = r#"Some(String::from("007"))"#)]
    pub nurse_order_as: Option<String>,
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub doctor_confirm_time: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(String::from("008"))"#)]
    pub nurse_accept: Option<String>,
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub nurse_accept_time: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(String::from("009"))"#)]
    pub pharmacist_accept: Option<String>,
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub pharmacist_accept_time: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(String::from("010"))"#)]
    pub pharmacist_check: Option<String>,
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub pharmacist_check_time: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(String::from("011"))"#)]
    pub pharmacist_done: Option<String>,
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub pharmacist_done_time: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(String::from("done"))"#)]
    pub pharmacist_order_status: Option<String>,
    #[Demo(value = "Some(1)")]
    pub pre_order_id: Option<u32>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub pre_order_date: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub pre_order_time: Option<Time>,

    #[Demo(value = r#"Some(String::from("Miss.Nurse"))"#)]
    pub order_doctor_name: Option<String>,
    #[Demo(value = r#"Some(String::from("ว.00000"))"#)]
    pub order_doctor_licenseno: Option<String>,
    #[Demo(value = r#"Some(String::from("Lieutenent"))"#)]
    pub order_doctor_entryposition: Option<String>,
    #[Demo(value = "Some(true)")]
    pub order_doctor_is_intern: Option<bool>,

    #[Demo(value = r#"Some(String::from("Dr.Doctor"))"#)]
    pub nurse_order_as_name: Option<String>,
    #[Demo(value = r#"Some(String::from("ว00000"))"#)]
    pub nurse_order_as_licenseno: Option<String>,
    #[Demo(value = r#"Some(String::from("Lieutenent"))"#)]
    pub nurse_order_as_entryposition: Option<String>,
    #[Demo(value = "Some(true)")]
    pub nurse_order_as_is_intern: Option<bool>,

    #[Demo(value = r#"Some(String::from("Miss.Nurse"))"#)]
    pub nurse_accept_name: Option<String>,
    #[Demo(value = r#"Some(String::from("1234567"))"#)]
    pub nurse_accept_licenseno: Option<String>,
    #[Demo(value = r#"Some(String::from("Lieutenent"))"#)]
    pub nurse_accept_entryposition: Option<String>,

    #[Demo(value = r#"Some(String::from("Mr.Pharmacist"))"#)]
    pub pharmacist_accept_name: Option<String>,
    #[Demo(value = r#"Some(String::from("1234567"))"#)]
    pub pharmacist_accept_licenseno: Option<String>,
    #[Demo(value = r#"Some(String::from("Lieutenent"))"#)]
    pub pharmacist_accept_entryposition: Option<String>,

    #[Demo(value = r#"Some(String::from("Mr.Checker"))"#)]
    pub pharmacist_check_name: Option<String>,
    #[Demo(value = r#"Some(String::from("1234567"))"#)]
    pub pharmacist_check_licenseno: Option<String>,
    #[Demo(value = r#"Some(String::from("Lieutenent"))"#)]
    pub pharmacist_check_entryposition: Option<String>,

    #[Demo(value = r#"Some(String::from("Mr.Payer"))"#)]
    pub pharmacist_done_name: Option<String>,
    #[Demo(value = r#"Some(String::from("1234567"))"#)]
    pub pharmacist_done_licenseno: Option<String>,
    #[Demo(value = r#"Some(String::from("Lieutenent"))"#)]
    pub pharmacist_done_entryposition: Option<String>,

    #[Demo(value = "vec![OrderItemType::demo()]")]
    pub order_item_types: Vec<OrderItemType>,
}

impl Order {
    /// for sending to `OnedayForm::new()` or `ContinuousFomm::new()` only (contain only necessary data)
    pub fn new_from_med_rec_items(items: &[Rc<MedReconciliationItem>]) -> Rc<Self> {
        let now = js_now();
        Rc::new(Self {
            visit_type: VisitTypeId::Ipd(String::new()),
            order_id: 0,
            hn: None,
            fullname: None,
            ward_name: None,
            bedno: None,
            display_bedno: None,
            bed_type_name: None,
            bed_type_color: None,
            order_date: now.date(),                   // not used but cannot None
            order_time: now.time(),                   // not used but cannot None
            order_doctor: String::new(),              // not used but cannot None
            order_type: String::from("continuous"),   // not used but cannot None
            order_owner_type: String::from("doctor"), // not used but cannot None
            order_confirm: String::new(),             // not used but cannot None
            nurse_order_as: None,
            doctor_confirm_time: None,
            nurse_accept: None,
            nurse_accept_time: None,
            pharmacist_accept: None,
            pharmacist_accept_time: None,
            pharmacist_check: None,
            pharmacist_check_time: None,
            pharmacist_done: None,
            pharmacist_done_time: None,
            pharmacist_order_status: None,
            pre_order_id: None,
            pre_order_date: None,
            pre_order_time: None,

            order_doctor_name: None,
            order_doctor_licenseno: None,
            order_doctor_entryposition: None,
            order_doctor_is_intern: None,

            nurse_order_as_name: None,
            nurse_order_as_licenseno: None,
            nurse_order_as_entryposition: None,
            nurse_order_as_is_intern: None,

            nurse_accept_name: None,
            nurse_accept_licenseno: None,
            nurse_accept_entryposition: None,

            pharmacist_accept_name: None,
            pharmacist_accept_licenseno: None,
            pharmacist_accept_entryposition: None,

            pharmacist_check_name: None,
            pharmacist_check_licenseno: None,
            pharmacist_check_entryposition: None,

            pharmacist_done_name: None,
            pharmacist_done_licenseno: None,
            pharmacist_done_entryposition: None,

            order_item_types: vec![OrderItemType {
                order_item_type: OrderTypeName::Med,
                order_items: items.iter().map(|i| OrderItem::from(i)).collect(),
            }],
        })
    }

    /// GET `EndPoint::IpdOrderOrder`
    pub async fn call_api_get_ipd(params: &OrderParams, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        Self::get(&[EndPoint::IpdOrderOrder.base(), params.clone().query_string()].concat(), app).await
    }

    /// GET `EndPoint::OpdErOrderOrder`
    pub async fn call_api_get_opd_er(params: &OrderParams, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        Self::get(&[EndPoint::OpdErOrderOrder.base(), params.clone().query_string()].concat(), app).await
    }

    async fn get(path: &str, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(path, "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch Order"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch Order"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }

    /// DELETE `EndPoint::IpdOrderOrderId`
    /// DELETE `EndPoint::OpdErOrderOrderId`
    pub async fn call_api_delete(is_ipd: bool, order_id: u32, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        let endpoint = if is_ipd { EndPoint::IpdOrderOrderId } else { EndPoint::OpdErOrderOrderId };
        execute_fetch(&[endpoint.base(), order_id.to_string()].concat(), "DELETE", None, app).await
    }

    pub fn is_my_order(&self, doctor_code: &Option<String>) -> bool {
        if let Some(my_code) = doctor_code.as_ref() { my_code == &self.order_doctor } else { false }
    }
    pub fn is_oneday(&self) -> bool {
        self.order_type.as_str() == "oneday"
    }
    pub fn is_confirm(&self) -> bool {
        self.order_confirm.as_str() == "Y"
    }
    /// else is nurse
    pub fn is_by_doctor(&self) -> bool {
        self.order_owner_type.as_str() == "doctor"
    }
    pub fn is_nurse_order_as(&self) -> bool {
        self.nurse_order_as_name.is_some()
    }
    pub fn is_doctor_can_confirm(&self, doctor_code: &Option<String>) -> bool {
        if let (Some(my_code), Some(as_code)) = (doctor_code.as_ref(), self.nurse_order_as.as_ref()) {
            my_code == as_code
        } else {
            false
        }
    }
    pub fn is_doctor_confirm(&self) -> bool {
        self.doctor_confirm_time.is_some()
    }
    pub fn is_nurse_accepted(&self) -> bool {
        self.nurse_accept_time.is_some()
    }
    pub fn is_pharm_accepted(&self) -> bool {
        self.pharmacist_accept_time.is_some()
    }
    pub fn is_pharm_checked(&self) -> bool {
        self.pharmacist_check_time.is_some()
    }
    pub fn is_pharm_can_done(&self, doctor_code: &Option<String>, is_checked_pharmacist_can_done: bool) -> bool {
        if is_checked_pharmacist_can_done {
            true
        } else if let (Some(my_code), Some(ac_code)) = (doctor_code.as_ref(), self.pharmacist_check.as_ref()) {
            my_code != ac_code
        } else {
            false
        }
    }
    pub fn is_pharm_done(&self) -> bool {
        self.pharmacist_done_time.is_some()
    }
    /// get max-datetime of confirmed order for limit customized order-time by user
    pub fn max_datetime(&self, is_pharmacist: bool) -> Option<PrimitiveDateTime> {
        let confirm_time = Some(PrimitiveDateTime::new(self.order_date, self.order_time));
        if is_pharmacist { self.pharmacist_accept_time.or(confirm_time) } else { confirm_time }
    }

    pub fn need_medplan(&self) -> bool {
        self.order_item_types.iter().any(|oit| match oit.order_item_type.need_medplan_and_off() {
            (true, false) => oit.order_items.iter().any(|oi| oi.icode.is_some()),
            (false, true) => oit.order_items.iter().any(|oi| oi.off_icode.is_some()),
            _ => false,
        })
    }
}

impl From<Rc<PreOrder>> for Order {
    fn from(item: Rc<PreOrder>) -> Self {
        Order {
            visit_type: VisitTypeId::Visit(String::new()),
            order_id: item.order_id,
            hn: None,
            fullname: None,
            ward_name: None,
            bedno: None,
            display_bedno: None,
            bed_type_name: None,
            bed_type_color: None,
            order_date: item.order_date,
            order_time: item.order_time,
            order_doctor: item.order_doctor.clone(),
            order_type: item.order_type.clone(),
            order_owner_type: item.order_owner_type.clone(),
            order_confirm: item.order_confirm.clone(),
            nurse_order_as: None,
            doctor_confirm_time: None,
            nurse_accept: item.nurse_accept.clone(),
            nurse_accept_time: item.nurse_accept_time,
            pharmacist_accept: item.pharmacist_accept.clone(),
            pharmacist_accept_time: item.pharmacist_accept_time,
            pharmacist_check: item.pharmacist_check.clone(),
            pharmacist_check_time: item.pharmacist_check_time,
            pharmacist_done: item.pharmacist_done.clone(),
            pharmacist_done_time: item.pharmacist_done_time,
            pharmacist_order_status: item.pharmacist_order_status.clone(),
            pre_order_id: None,
            pre_order_date: None,
            pre_order_time: None,

            order_doctor_name: item.order_doctor_name.clone(),
            order_doctor_licenseno: None,
            order_doctor_entryposition: None,
            order_doctor_is_intern: item.order_doctor_is_intern.clone(),

            nurse_order_as_name: None,
            nurse_order_as_licenseno: None,
            nurse_order_as_entryposition: None,
            nurse_order_as_is_intern: None,

            nurse_accept_name: item.nurse_accept_name.clone(),
            nurse_accept_licenseno: None,
            nurse_accept_entryposition: None,

            pharmacist_accept_name: item.pharmacist_accept_name.clone(),
            pharmacist_accept_licenseno: None,
            pharmacist_accept_entryposition: None,

            pharmacist_check_name: item.pharmacist_check_name.clone(),
            pharmacist_check_licenseno: None,
            pharmacist_check_entryposition: None,

            pharmacist_done_name: item.pharmacist_done_name.clone(),
            pharmacist_done_licenseno: None,
            pharmacist_done_entryposition: None,

            order_item_types: item.order_item_types.clone().into_iter().map(OrderItemType::from).collect(),
        }
    }
}

#[derive(Demo, Deserialize, Serialize, FromRow, MySqlBinder)]
pub struct OrderOnly {
    #[Demo(value = "1")]
    pub order_id: u32,
    #[Demo(value = "date!(2023-12-31)")]
    pub order_date: Date,
    #[Demo(value = "time!(23:59:59)")]
    pub order_time: Time,
    #[Demo(value = r#"String::from("008")"#)]
    pub order_doctor: String,
    #[Demo(value = r#"String::from("oneday")"#)]
    pub order_type: String,
    #[Demo(value = r#"String::from("nurse")"#)]
    pub order_owner_type: String,
    #[Demo(value = r#"String::from("Y")"#)]
    pub order_confirm: String,
    #[Demo(value = r#"Some(String::from("007"))"#)]
    pub nurse_order_as: Option<String>,
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub doctor_confirm_time: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(String::from("008"))"#)]
    pub nurse_accept: Option<String>,
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub nurse_accept_time: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(String::from("009"))"#)]
    pub pharmacist_accept: Option<String>,
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub pharmacist_accept_time: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(String::from("010"))"#)]
    pub pharmacist_check: Option<String>,
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub pharmacist_check_time: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(String::from("011"))"#)]
    pub pharmacist_done: Option<String>,
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub pharmacist_done_time: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(String::from("done"))"#)]
    pub pharmacist_order_status: Option<String>,
    #[Demo(value = "Some(1)")]
    pub pre_order_id: Option<u32>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub pre_order_date: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub pre_order_time: Option<Time>,
    #[Demo(value = r#"String::from("user")"#)]
    pub create_user: String,
    #[Demo(value = "datetime!(2023-12-31 23:59:59)")]
    pub create_datetime: PrimitiveDateTime,
    #[Demo(value = r#"String::from("user")"#)]
    pub update_user: String,
    #[Demo(value = "datetime!(2023-12-31 23:59:59)")]
    pub update_datetime: PrimitiveDateTime,
    #[Demo(value = "1")]
    pub version: i32,

    #[sqlx(skip)]
    #[sqlx_binder(skip)]
    #[Demo(value = "vec![OrderItemOnly::demo()]")]
    pub order_items: Vec<OrderItemOnly>,
}

impl PartialEq for OrderOnly {
    fn eq(&self, other: &Self) -> bool {
        // order_id == other.order_id &&
        self.order_date == other.order_date
            && self.order_time == other.order_time
            && self.order_doctor == other.order_doctor
            && self.order_type == other.order_type
            && self.order_owner_type == other.order_owner_type
            && self.order_confirm == other.order_confirm
            && self.nurse_order_as == other.nurse_order_as
            && self.doctor_confirm_time == other.doctor_confirm_time
            && self.nurse_accept == other.nurse_accept
            && self.nurse_accept_time == other.nurse_accept_time
            && self.pharmacist_accept == other.pharmacist_accept
            && self.pharmacist_accept_time == other.pharmacist_accept_time
            && self.pharmacist_check == other.pharmacist_check
            && self.pharmacist_check_time == other.pharmacist_check_time
            && self.pharmacist_done == other.pharmacist_done
            && self.pharmacist_done_time == other.pharmacist_done_time
            && self.pharmacist_order_status == other.pharmacist_order_status
            && self.pre_order_id == other.pre_order_id
            && self.pre_order_date == other.pre_order_date
            && self.pre_order_time == other.pre_order_time
            && self.create_user == other.create_user
            && self.create_datetime == other.create_datetime
            && self.update_user == other.update_user
            && self.update_datetime == other.update_datetime
            && self.version == other.version
            && if self.order_items.len() == other.order_items.len() {
                self.order_items.iter().zip(other.order_items.iter()).all(|(a, b)| a.eq(b))
            } else {
                false
            }
    }
}

/// Type of Order Item
#[derive(Clone, Debug, Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(OrderItemType::demo()))]
pub struct OrderItemType {
    #[Demo(value = "OrderTypeName::demo_med()")]
    pub order_item_type: OrderTypeName,
    #[Demo(value = "vec![OrderItem::demo()]")]
    pub order_items: Vec<OrderItem>,
}

impl OrderItemType {
    pub fn is_med(&self) -> bool {
        // normal order: has `icode`
        // off order: not has `icode`, has `off_icode`
        // med reconciliation order: not has `icode`, has `med_reconciliation_item_id`
        self.order_items
            .iter()
            .any(|oi| oi.icode.is_some() || oi.off_icode.is_some() || oi.med_reconciliation_item_id.is_some())
    }
    pub fn is_note(&self) -> bool {
        matches!(self.order_item_type, OrderTypeName::Note)
    }
    pub fn is_pharm_notify(&self) -> bool {
        matches!(self.order_item_type, OrderTypeName::Pharm)
    }
    pub fn is_homemed(&self) -> bool {
        matches!(self.order_item_type, OrderTypeName::HomeMedication)
    }
}

impl From<PreOrderItemType> for OrderItemType {
    fn from(item: PreOrderItemType) -> Self {
        OrderItemType {
            order_item_type: item.order_item_type,
            order_items: item.order_items.into_iter().map(OrderItem::from).collect(),
        }
    }
}

/// Item of Order with Action
#[derive(Clone, Debug, Default, Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(OrderItem::demo()))]
pub struct OrderItem {
    /// for generic over `an` or `opd_er_order_master_id` only
    #[Demo(value = r#"VisitTypeId::demo_ipd(String::from("660001234"))"#)]
    pub visit_type: VisitTypeId,
    #[Demo(value = "1")]
    pub order_item_id: u32,
    #[Demo(value = "Some(1)")]
    pub order_id: Option<u32>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub order_date: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub order_time: Option<Time>,
    #[Demo(value = r#"Some(String::from("oneday"))"#)]
    pub order_type: Option<String>,
    #[Demo(value = r#"Some(String::from("nurse"))"#)]
    pub order_owner_type: Option<String>,
    #[Demo(value = r#"Some(String::from("Dr.Doctor"))"#)]
    pub order_doctor_name: Option<String>,
    #[Demo(value = r#"Some(String::from("ว.00000"))"#)]
    pub order_doctor_licenseno: Option<String>,

    #[Demo(value = r#"Some(String::from("med"))"#)]
    pub order_item_type: Option<String>,
    /// OFFed Medicine NOT in hosital-drug-list will store as `med_name`\n`order_item_detail`
    #[Demo(value = r#"Some(String::from("รับประทานครั้งละ 1 เม็ด เวลามีอาการ"))"#)]
    pub order_item_detail: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub stat: Option<String>,
    /// use only order_item_type is `off`
    #[Demo(value = "Some(1)")]
    pub off_order_item_id: Option<u32>,
    /// OFFed Medicine NOT in hosital-drug-list will be NULL here
    #[Demo(value = r#"Some(String::from("1000222"))"#)]
    pub icode: Option<String>,
    #[Demo(value = r#"Some(String::from("Incharge"))"#)]
    pub nurse_assign: Option<String>,

    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub off_by_datetime: Option<PrimitiveDateTime>,
    /// OFFed Medicine NOT in hosital-drug-list will be NULL here
    #[Demo(value = r#"Some(String::from("PARACETAMOL 500 mg. เม็ด"))"#)]
    pub med_name: Option<String>,
    #[Demo(value = "Some(0)")]
    pub displaycolor: Option<i32>,
    #[Demo(value = "Some(2)")]
    pub addict_type_id: Option<i32>,
    #[Demo(value = "Some(2)")]
    pub habit_forming_type: Option<i32>,
    #[Demo(value = r#"Some(String::from("PARACETAMOL"))"#)]
    pub generic_name: Option<String>,
    #[Demo(value = r#"Some(String::from("TABLET"))"#)]
    pub dosageform: Option<String>,

    /// use only order_item_type is `off`
    #[Demo(value = r#"Some(String::from("1000222"))"#)]
    pub off_icode: Option<String>,
    /// use only order_item_type is `off`
    #[Demo(value = r#"Some(String::from("PARACETAMOL 500 mg. เม็ด"))"#)]
    pub off_med_name: Option<String>,
    /// use only order_item_type is `off`
    #[Demo(value = "Some(0)")]
    pub off_displaycolor: Option<i32>,
    /// use only order_item_type is `off`
    #[Demo(value = r#"Some(String::from("รับประทานครั้งละ 1 เม็ด เวลามีอาการ"))"#)]
    pub off_order_item_detail: Option<String>,

    #[Demo(value = r#"Some(String::from("PARACETAMOL=Rash"))"#)]
    pub allergy_agent_symptom: Option<String>,
    #[Demo(value = "Some(10)")]
    pub first_qty: Option<i32>,
    #[Demo(value = "Some(10)")]
    pub qty: Option<i32>,

    #[Demo(value = "Some(1)")]
    pub med_reconciliation_item_id: Option<u32>,
    #[Demo(value = r#"Some(String::from("1 prn"))"#)]
    pub old_drugusage: Option<String>,
    #[Demo(value = r#"Some(String::from("Store"))"#)]
    pub receive_from: Option<String>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub receive_date: Option<Date>,
    #[Demo(value = "Some(10)")]
    pub receive_qty: Option<i32>,
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub last_dose_taken_time: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(String::from("Remark"))"#)]
    pub last_dose_taken_remark: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub used: Option<String>,

    #[Demo(value = r#"Some(String::from("CrCl < 30 dose xx"))"#)]
    pub due_usage: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub due_status: Option<String>,
    #[Demo(value = r#"Some(String::from("N"))"#)]
    pub due_doctor: Option<String>,
    #[Demo(value = r#"Some(String::from("Take risk"))"#)]
    pub due_doctor_note: Option<String>,
    #[Demo(value = r#"Some(String::from("N"))"#)]
    pub due_pharm: Option<String>,
    #[Demo(value = r#"Some(String::from("Risk"))"#)]
    pub due_pharm_note: Option<String>,

    #[Demo(value = r#"Some(String::from("Use me gently"))"#)]
    pub info: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub info_status: Option<String>,

    #[Demo(value = "Some(1)")]
    pub order_duration: Option<i32>,
    #[Demo(value = "Some(7)")]
    pub duration1: Option<i16>,
    #[Demo(value = r#"Some(String::from("gold"))"#)]
    pub exceed_duration1_color: Option<String>,
    #[Demo(value = "Some(15)")]
    pub duration2: Option<i16>,
    #[Demo(value = r#"Some(String::from("pink"))"#)]
    pub exceed_duration2_color: Option<String>,
    #[Demo(value = "Some(30)")]
    pub duration3: Option<i16>,
    #[Demo(value = r#"Some(String::from("red"))"#)]
    pub exceed_duration3_color: Option<String>,

    #[Demo(value = r#"Some(String::from("Record BP q 1 hr"))"#)]
    pub monitor: Option<String>,
    #[Demo(value = "Some(5)")]
    pub monitor_count: Option<u8>,
    /// minutes
    #[Demo(value = "Some(120)")]
    pub monitor_duration: Option<u32>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub monitor_status: Option<String>,

    #[Demo(value = "vec![IndexPlan::demo()]")]
    pub index_plans: Vec<IndexPlan>,
}

impl OrderItem {
    /// GET `EndPoint::IpdOrderItem`
    pub async fn call_api_get_ipd(params: &OrderParams, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        if params.an.is_some() {
            match fetch_json_api(&[EndPoint::IpdOrderItem.base(), params.clone().query_string()].concat(), "GET", None, app).await {
                Ok((response, true)) => {
                    let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch IpdOrderItem"))?;
                    Ok(response)
                }
                Ok((app_error, false)) => {
                    let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch IpdOrderItem"))?;
                    Err(error)
                }
                Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
            }
        } else {
            Ok(Vec::new())
        }
    }
    /// GET `EndPoint::IpdOrderPrevious`
    pub async fn call_api_get_ipd_previous(params: &OrderParams, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[EndPoint::IpdOrderPrevious.base(), params.clone().query_string()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch PreviousOrderItem"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch PreviousOrderItem"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
    /// GET `EndPoint::OpdErOrderItem`
    pub async fn call_api_get_opd_er(params: &OrderParams, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        if params.opd_er_order_master_id.is_some() {
            match fetch_json_api(&[EndPoint::OpdErOrderItem.base(), params.clone().query_string()].concat(), "GET", None, app).await {
                Ok((response, true)) => {
                    let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch OpdErOrderItem"))?;
                    Ok(response)
                }
                Ok((app_error, false)) => {
                    let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch OpdErOrderItem"))?;
                    Err(error)
                }
                Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
            }
        } else {
            Ok(Vec::new())
        }
    }

    pub fn med_rec_info(&self) -> String {
        let (old_usage, now_title) = if self.is_med_rec_change_usage() {
            (self.old_drugusage.as_ref().map(|s| ["วิธีใช้เดิม: ", s, "\n"].concat()).unwrap_or_default(), "วิธีใช้ใหม่: ")
        } else {
            (String::new(), "วิธีใช้: ")
        };
        [
            old_usage,
            self.order_item_detail.as_ref().map(|s| [now_title, s].concat()).unwrap_or_default(),
            self.receive_from.as_ref().map(|s| ["\nได้รับจาก: ", s].concat()).unwrap_or_default(),
            self.receive_qty.map(|i| ["\nปริมาณ: ", &i.to_string()].concat()).unwrap_or_default(),
            self.receive_date.map(|d| ["\nเมื่อวันที่: ", &date_th(&d)].concat()).unwrap_or_default(),
            self.last_dose_taken_time.map(|dt| ["\nรับประทานครั้งล่าสุด: ", &datetime_th(&dt)].concat()).unwrap_or_default(),
            self.last_dose_taken_remark.as_ref().map(|s| ["\nหมายเหตุ: ", s].concat()).unwrap_or_default(),
        ]
        .concat()
    }

    pub fn is_med_rec_change_usage(&self) -> bool {
        if let (Some(old), Some(new)) = (&self.old_drugusage, &self.order_item_detail) {
            old != new
        } else {
            false
        }
    }

    pub fn has_dtx(&self) -> bool {
        self.order_item_detail.as_ref().map(|s| s.to_ascii_lowercase().contains("dtx")).unwrap_or_default()
    }

    pub fn has_hct(&self) -> bool {
        self.order_item_detail.as_ref().map(|s| s.to_ascii_lowercase().contains("hct")).unwrap_or_default()
    }
}

impl From<PreOrderItem> for OrderItem {
    fn from(item: PreOrderItem) -> Self {
        OrderItem {
            visit_type: VisitTypeId::Visit(String::new()),
            order_item_id: item.order_item_id,
            order_id: item.order_id,
            order_date: None,
            order_time: None,
            order_type: None,
            order_owner_type: None,
            order_doctor_name: None,
            order_doctor_licenseno: None,

            order_item_type: item.order_item_type,
            order_item_detail: item.order_item_detail,
            stat: item.stat,
            off_order_item_id: item.off_order_item_id,
            icode: item.icode,
            nurse_assign: None,
            off_by_datetime: None,
            med_name: item.med_name,
            displaycolor: item.displaycolor,
            addict_type_id: None,
            habit_forming_type: None,
            generic_name: item.generic_name,
            dosageform: item.dosageform,
            off_icode: item.off_icode,
            off_med_name: item.off_med_name,
            off_displaycolor: item.off_displaycolor,
            off_order_item_detail: item.off_order_item_detail,
            allergy_agent_symptom: item.allergy_agent_symptom,
            first_qty: None,
            qty: None,

            med_reconciliation_item_id: None,
            old_drugusage: None,
            receive_from: None,
            receive_date: None,
            receive_qty: None,
            last_dose_taken_time: None,
            last_dose_taken_remark: None,
            used: None,

            due_usage: None,
            due_status: None,
            due_doctor: None,
            due_doctor_note: None,
            due_pharm: None,
            due_pharm_note: None,

            info: None,
            info_status: None,

            monitor: None,
            monitor_count: None,
            monitor_duration: None,
            monitor_status: None,

            order_duration: None,
            duration1: None,
            exceed_duration1_color: None,
            duration2: None,
            exceed_duration2_color: None,
            duration3: None,
            exceed_duration3_color: None,
            index_plans: Vec::new(),
        }
    }
}

impl From<&Rc<MedReconciliationItem>> for OrderItem {
    fn from(item: &Rc<MedReconciliationItem>) -> Self {
        OrderItem {
            visit_type: item.visit_type.clone(),
            order_item_id: 0,
            order_id: None,
            order_date: None,
            order_time: None,
            order_type: None,
            order_owner_type: None,
            order_item_type: Some(String::from("med")),
            order_doctor_name: None,
            order_doctor_licenseno: None,

            order_item_detail: item.changed_drugusage.clone().or(item.old_drugusage.clone()),
            stat: Some(String::from("N")),
            off_order_item_id: None,
            icode: item.icode.clone(),
            nurse_assign: None,
            off_by_datetime: None,
            med_name: item.custom_med_name.clone().or(item.med_name.clone()),
            displaycolor: None,
            addict_type_id: None,
            habit_forming_type: None,
            generic_name: item.generic_name.clone(),
            dosageform: item.dosageform.clone(),
            off_icode: None,
            off_med_name: None,
            off_displaycolor: None,
            off_order_item_detail: None,
            allergy_agent_symptom: item.allergy_agent_symptom.clone(),
            first_qty: None,
            qty: None,

            med_reconciliation_item_id: zero_none(item.med_reconciliation_item_id),
            old_drugusage: item.old_drugusage.clone(),
            receive_from: item.receive_from.clone(),
            receive_date: item.receive_date,
            receive_qty: item.receive_qty,
            last_dose_taken_time: item.last_dose_taken_time,
            last_dose_taken_remark: item.last_dose_taken_remark.clone(),
            used: item.used.clone(),

            due_usage: item.due_usage.clone(),
            due_status: item.due_status.clone(),
            due_doctor: None,
            due_doctor_note: None,
            due_pharm: None,
            due_pharm_note: None,

            info: item.info.clone(),
            info_status: item.info_status.clone(),

            monitor: None,
            monitor_count: None,
            monitor_duration: None,
            monitor_status: None,

            order_duration: None,
            duration1: None,
            exceed_duration1_color: None,
            duration2: None,
            exceed_duration2_color: None,
            duration3: None,
            exceed_duration3_color: None,
            index_plans: Vec::new(),
        }
    }
}

#[derive(Clone, Demo, Deserialize, Serialize, FromRow, MySqlBinder)]
pub struct OrderItemOnly {
    #[Demo(value = "1")]
    pub order_item_id: u32,
    #[Demo(value = "Some(1)")]
    pub order_id: Option<u32>,
    #[Demo(value = r#"Some(String::from("med"))"#)]
    pub order_item_type: Option<String>,
    #[Demo(value = r#"Some(String::from("รับประทานครั้งละ 1 เม็ด เวลามีอาการ"))"#)]
    pub order_item_detail: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub stat: Option<String>,
    #[Demo(value = "Some(1)")]
    pub off_order_item_id: Option<u32>,
    #[Demo(value = r#"Some(String::from("1000222"))"#)]
    pub icode: Option<String>,
    #[Demo(value = "Some(1)")]
    pub med_reconciliation_item_id: Option<u32>,
    #[Demo(value = r#"Some(String::from("Incharge"))"#)]
    pub nurse_assign: Option<String>,
    #[Demo(value = "Some(10)")]
    pub first_qty: Option<i32>,
    #[Demo(value = "Some(10)")]
    pub qty: Option<i32>,
    #[Demo(value = r#"Some(String::from("N"))"#)]
    pub due_doctor: Option<String>,
    #[Demo(value = r#"Some(String::from("Take risk"))"#)]
    pub due_doctor_note: Option<String>,
    #[Demo(value = r#"Some(String::from("N"))"#)]
    pub due_pharm: Option<String>,
    #[Demo(value = r#"Some(String::from("Risk"))"#)]
    pub due_pharm_note: Option<String>,
    #[Demo(value = r#"String::from("user")"#)]
    pub create_user: String,
    #[Demo(value = "datetime!(2023-12-31 23:59:59)")]
    pub create_datetime: PrimitiveDateTime,
    #[Demo(value = r#"String::from("user")"#)]
    pub update_user: String,
    #[Demo(value = "datetime!(2023-12-31 23:59:59)")]
    pub update_datetime: PrimitiveDateTime,
    #[Demo(value = "1")]
    pub version: i32,

    #[sqlx(skip)]
    #[sqlx_binder(skip)]
    #[Demo(value = "vec![IndexPlanOnly::demo()]")]
    pub index_plans: Vec<IndexPlanOnly>,
}

impl PartialEq for OrderItemOnly {
    fn eq(&self, other: &Self) -> bool {
        // self.order_item_id == other.order_item_id &&
        // self.order_id == other.order_id &&
        self.order_item_type == other.order_item_type
            && self.order_item_detail == other.order_item_detail
            && self.stat == other.stat
            // && self.off_order_item_id == other.off_order_item_id
            && self.icode == other.icode
            && self.med_reconciliation_item_id == other.med_reconciliation_item_id
            && self.nurse_assign == other.nurse_assign
            && self.first_qty == other.first_qty
            && self.qty == other.qty
            && self.due_doctor == other.due_doctor
            && self.due_doctor_note == other.due_doctor_note
            && self.due_pharm == other.due_pharm
            && self.due_pharm_note == other.due_pharm_note
            && self.create_user == other.create_user
            && self.create_datetime == other.create_datetime
            && self.update_user == other.update_user
            && self.update_datetime == other.update_datetime
            && self.version == other.version
            && if self.index_plans.len() == other.index_plans.len() {
                self.index_plans.iter().zip(other.index_plans.iter()).all(|(a, b)| a.eq(b))
            } else {
                false
            }
    }
}

/// Item of Order (compact)
#[derive(Clone, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(MedOrderItem::demo()))]
pub struct MedOrderItem {
    #[Demo(value = r#"Some(String::from("1000222"))"#)]
    pub icode: Option<String>,
    #[Demo(value = r#"Some(String::from("PARACETAMOL 500 mg. เม็ด"))"#)]
    pub med_name: Option<String>,
    #[Demo(value = r#"Some(String::from("PARACETAMOL"))"#)]
    pub generic_name: Option<String>,
    #[Demo(value = r#"Some(String::from("TABLET"))"#)]
    pub dosageform: Option<String>,
    #[Demo(value = r#"Some(String::from("รับประทานครั้งละ 1 เม็ด เวลามีอาการ"))"#)]
    pub order_item_detail: Option<String>,
    #[Demo(value = r#"Some(String::from("med"))"#)]
    pub order_item_type: Option<String>,
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub off_by_datetime: Option<PrimitiveDateTime>,
    #[Demo(value = "Some(0)")]
    pub displaycolor: Option<i32>,
    #[Demo(value = "Some(2)")]
    pub addict_type_id: Option<i32>,
    #[Demo(value = "Some(2)")]
    pub habit_forming_type: Option<i32>,
    #[Demo(value = r#"Some(String::from("CrCl < 30 dose xx"))"#)]
    pub due_usage: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub due_status: Option<String>,

    #[Demo(value = r#"Some(String::from("Use me gently"))"#)]
    pub info: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub info_status: Option<String>,

    #[Demo(value = "Some(1)")]
    pub med_reconciliation_item_id: Option<u32>,
    #[Demo(value = r#"Some(String::from("1 prn"))"#)]
    pub old_drugusage: Option<String>,
    #[Demo(value = r#"Some(String::from("Store"))"#)]
    pub receive_from: Option<String>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub receive_date: Option<Date>,
    #[Demo(value = "Some(10)")]
    pub receive_qty: Option<i32>,
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub last_dose_taken_time: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(String::from("Remark"))"#)]
    pub last_dose_taken_remark: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub used: Option<String>,
}

impl MedOrderItem {
    /// GET `EndPoint::IpdOrderOnedayPreviousAn`
    pub async fn call_api_get_ipd_oneday_previous(an: &str, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[&EndPoint::IpdOrderOnedayPreviousAn.base(), an].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch PreviousOneDayOrderItem"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch PreviousOneDayOrderItem"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
    /// GET `EndPoint::IpdOrderToHomeMedAn`
    pub async fn call_api_get_ipd_cont_to_home_med(an: &str, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[&EndPoint::IpdOrderToHomeMedAn.base(), an].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch MedOrderItem"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch MedOrderItem"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}

/// Type of Order
#[derive(Copy, Clone, Debug, Demo, PartialEq, PartialOrd, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "kebab-case")]
#[schema(example = json!(OrderTypeName::demo_med()))]
pub enum OrderTypeName {
    Note,
    Off,
    Food,
    Activity,
    Lab,
    Xray,
    Ivfluid,
    Record,
    Med,
    Injection,
    Retain,
    Serial,
    Other,
    Pharm,
    Discharge,
    HomeMedication,
}
impl OrderTypeName {
    pub fn string(&self) -> &'static str {
        match self {
            Self::Note => "Note",
            Self::Off => "Off",
            Self::Food => "Food",
            Self::Activity => "Activity",
            Self::Lab => "Lab",
            Self::Xray => "X-Ray",
            Self::Ivfluid => "IV Fluid",
            Self::Record => "Record",
            Self::Med => "Medication",
            Self::Injection => "Injection",
            Self::Retain => "Retain",
            Self::Serial => "Serial",
            Self::Other => "Other",
            Self::Pharm => "Pharmacist Notify",
            Self::Discharge => "Discharge",
            Self::HomeMedication => "Home Medication",
        }
    }
    pub fn from_string(text: &str) -> Self {
        match text {
            "note" => Self::Note,
            "off" => Self::Off,
            "food" => Self::Food,
            "activity" => Self::Activity,
            "lab" => Self::Lab,
            "xray" => Self::Xray,
            "ivfluid" => Self::Ivfluid,
            "record" => Self::Record,
            "med" => Self::Med,
            "injection" => Self::Injection,
            "retain" => Self::Retain,
            "serial" => Self::Serial,
            "pharm" => Self::Pharm,
            "discharge" => Self::Discharge,
            "home-medication" => Self::HomeMedication,
            _ => Self::Other,
        }
    }

    pub fn need_medplan_and_off(&self) -> (bool, bool) {
        match self {
            Self::Ivfluid | Self::Med | Self::Injection | Self::HomeMedication => (true, false),
            Self::Off => (false, true),
            _ => (false, false),
        }
    }
}

/// Order for save
#[derive(Debug, Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(OrderSave::demo()))]
pub struct OrderSave {
    /// for generic over `an` or `opd_er_order_master_id` only
    #[Demo(value = r#"VisitTypeId::demo_ipd(String::from("660001234"))"#)]
    pub visit_type: VisitTypeId,
    #[Demo(value = "Some(1)")]
    pub order_id: Option<u32>,
    #[Demo(value = r#"String::from("007")"#)]
    pub order_doctor: String,
    #[Demo(value = r#"String::from("oneday")"#)]
    pub order_type: String,
    #[Demo(value = r#"String::from("nurse")"#)]
    pub order_owner_type: String,
    #[Demo(value = "vec![OrderItemSave::demo()]")]
    pub order_items: Vec<OrderItemSave>,
}

impl OrderSave {
    /// - POST `EndPoint::IpdOrderOrder`
    /// - POST `EndPoint::OpdErOrderOrder`
    pub async fn call_api_post(&self, app: Rc<AppState>) -> Result<(u32, Vec<ExecuteResponse>), AppError> {
        let (path, is_valid) = match &self.visit_type {
            VisitTypeId::Ipd(an) | VisitTypeId::PreAdmit(an) => (EndPoint::IpdOrderOrder, !an.is_empty()),
            VisitTypeId::OpdEr(_, opd_er_order_master_id) => (EndPoint::OpdErOrderOrder, *opd_er_order_master_id > 0),
            VisitTypeId::Visit(_) => (EndPoint::Unknown, false),
        };

        if is_valid {
            let body_json = serde_json::to_string(self).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Send OrderSave"))?;
            let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send OrderSave"))?;

            execute_fetch_vec_with_u32(&path.base(), "POST", Some(&body), app).await
        } else {
            Err(AppError::app_400("Check OrderSave"))
        }
    }
}

/// Order for edit
#[derive(Debug, Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(OrderPatch::demo()))]
pub struct OrderPatch {
    #[Demo(value = "1")]
    pub order_id: u32,
    #[Demo(value = "OrderPatchAction::demo_confirm()")]
    pub action: OrderPatchAction,
    #[Demo(value = r#"Some(String::from("007"))"#)]
    pub nurse_order_as: Option<String>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub order_time: Option<Time>,
    #[Demo(value = "vec![MedPlanItem::demo()]")]
    pub medplans: Vec<MedPlanItem>,
    #[Demo(value = "vec![1]")]
    pub off_med_plan_numbers: Vec<i32>,
}

impl OrderPatch {
    /// - PATCH `EndPoint::IpdOrderOrder`
    /// - PATCH `EndPoint::OpdErOrderOrder`
    pub async fn call_api_patch(&self, is_ipd: bool, app: Rc<AppState>) -> Result<Vec<ExecuteResponse>, AppError> {
        let ep = if is_ipd { EndPoint::IpdOrderOrder } else { EndPoint::OpdErOrderOrder };
        let body_json = serde_json::to_string(self).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Send OrderPatch"))?;

        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send OrderPatch"))?;

        execute_fetch_vec(&ep.base(), "PATCH", Some(&body), app).await
    }
}

/// Item for MedPlan
#[derive(Clone, Debug, Default, Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(MedPlanItem::demo()))]
pub struct MedPlanItem {
    #[Demo(value = "1")]
    pub order_item_id: u32,
    #[Demo(value = r#"String::from("660001234")"#)]
    pub an: String,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub order_date: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub order_time: Option<Time>,
    #[Demo(value = r#"Some(String::from("oneday"))"#)]
    pub order_type: Option<String>,
    #[Demo(value = r#"Some(String::from("med"))"#)]
    pub order_item_type: Option<String>,
    #[Demo(value = r#"Some(String::from("PARACETAMOL 500 mg. เม็ด"))"#)]
    pub med_name: Option<String>,
    #[Demo(value = r#"Some(String::from("รับประทาน ครั้งละ 1 เม็ด วันละ 1 เวลา"))"#)]
    pub order_item_detail: Option<String>,
    #[Demo(value = r#"Some(String::from("1000222"))"#)]
    pub icode: Option<String>,
    #[Demo(value = r#"String::from("008")"#)]
    pub order_doctor: String,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub stat: Option<String>,
    #[Demo(value = "Some(1)")]
    pub med_reconciliation_item_id: Option<u32>,
    #[Demo(value = "Some(10)")]
    pub first_qty: Option<i32>,
    #[Demo(value = "Some(10)")]
    pub qty: Option<i32>,
}

/// Order Patching Action
#[derive(Clone, Debug, Demo, Deserialize, PartialEq, Serialize, ToSchema)]
#[serde(rename_all = "kebab-case")]
#[schema(example = json!(OrderPatchAction::demo_confirm()))]
pub enum OrderPatchAction {
    Confirm,
    ConfirmAs,
    EditAs,
    DoctorConfirm,
    NurseAccept,
    PharmacistAccept,
    PharmacistCheck,
    PharmacistDone,
}

/// Order Item Patching Action
#[derive(Clone, Debug, Demo, Deserialize, PartialEq, Serialize, ToSchema)]
#[serde(rename_all = "kebab-case")]
#[schema(example = json!(OrderItemPatchAction::demo_nurse_assign()))]
pub enum OrderItemPatchAction {
    NurseAssign,
    OrderItemType,
    DueDoctor,
    DuePharm,
}

/// Item of Order for save
#[derive(Debug, Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(OrderItemSave::demo()))]
pub struct OrderItemSave {
    /// use when insert pre-order to order
    #[Demo(value = "Some(1)")]
    pub order_id: Option<u32>,
    #[Demo(value = r#"Some(String::from("med"))"#)]
    pub order_item_type: Option<String>,
    #[Demo(value = r#"Some(String::from("รับประทานครั้งละ 1 เม็ด เวลามีอาการ"))"#)]
    pub order_item_detail: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub stat: Option<String>,
    #[Demo(value = "Some(1)")]
    pub off_order_item_id: Option<u32>,
    #[Demo(value = r#"Some(String::from("1000222"))"#)]
    pub icode: Option<String>,
    /// null in pre-order
    #[Demo(value = "Some(1)")]
    pub med_reconciliation_item_id: Option<u32>,
    /// null in pre-order
    #[Demo(value = "Some(10)")]
    pub first_qty: Option<i32>,
    /// null in pre-order
    #[Demo(value = "Some(10)")]
    pub qty: Option<i32>,
}

/// OrderItem for edit
#[derive(Debug, Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(OrderItemPatch::demo()))]
pub struct OrderItemPatch {
    #[Demo(value = "1")]
    pub order_item_id: u32,
    #[Demo(value = "OrderItemPatchAction::demo_nurse_assign()")]
    pub action: OrderItemPatchAction,
    #[Demo(value = r#"Some(String::from("Incharge"))"#)]
    pub nurse_assign: Option<String>,
    #[Demo(value = r#"Some(String::from("med"))"#)]
    pub order_item_type: Option<String>,
    #[Demo(value = r#"Some(String::from("N"))"#)]
    pub due_doctor: Option<String>,
    #[Demo(value = r#"Some(String::from("Take risk"))"#)]
    pub due_doctor_note: Option<String>,
    #[Demo(value = r#"Some(String::from("N"))"#)]
    pub due_pharm: Option<String>,
    #[Demo(value = r#"Some(String::from("Risk"))"#)]
    pub due_pharm_note: Option<String>,
}

impl OrderItemPatch {
    /// - PATCH `EndPoint::IpdOrderItem`
    /// - PATCH `EndPoint::OpdErOrderItem`
    pub async fn call_api_patch(&self, is_ipd: bool, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        let ep = if is_ipd { EndPoint::IpdOrderItem } else { EndPoint::OpdErOrderItem };
        let body_json = serde_json::to_string(self).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Send OrderItemPatch"))?;

        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send OrderItemPatch"))?;

        execute_fetch(&ep.base(), "PATCH", Some(&body), app).await
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderButtons {
    pub word_type: String,
    pub buttons: Vec<Button>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Button {
    pub is_new: bool,
    pub separator: String,
    pub word: String,
    pub minus_from_end: u32,
    pub id: Option<String>,
}

impl OrderButtons {
    /// GET /jsons/ipd-one-day-buttons.json
    /// or GET /jsons/ipd-continuous-buttons.json
    /// or GET /jsons/ipd-progress-note-buttons.json
    pub async fn get(button_type: &str, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        let path = ["/local/jsons/ipd-", button_type, "-buttons.json"].concat();

        match fetch_json_api(&path, "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch OrderButtons"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch OrderButtons"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}
