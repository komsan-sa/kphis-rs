use dominator::{Dom, clone, events, html, with_node};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, always, not},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::{
    collections::HashSet,
    rc::Rc,
    sync::atomic::{AtomicU32, Ordering},
};
use time::{Date, Duration, PrimitiveDateTime, Time};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlButtonElement, HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement};

use kphis_model::{
    app::VisitTypeId,
    endpoint::EndPoint,
    fetch::{Method, call_api_get_exists_key_id},
    image::file_path::ImageUsage,
    index_plan::{IndexActionStatus, IndexPlan, IndexPlanType},
    ipd::his::HisMedPlanIpd,
    med_reconcile::{MedReconciliation, MedReconciliationItem, MedReconciliationParams},
    opd_er::{hosxp_med::OpdMed, order_master::OpdErOrderMasterCheck},
    order::{Button, MedOrderItem, MedPlanItem, Order, OrderDate, OrderItem, OrderItemPatch, OrderItemPatchAction, OrderItemSave, OrderParams, OrderPatch, OrderPatchAction, OrderTypeName},
    patient_info::PatientInfo,
    pre_order::{order::PreOrderItem, progress_note::PreProgressNoteItem},
    progress_note::{ProgressNote, ProgressNoteItem, ProgressNoteItemSave, ProgressNoteParams},
    report::{SystemReport, TypstReport},
    route::Route,
    sse::SsePostMessage,
    tab::Tab,
    timer::Timeout,
    user::permission::Permission,
};
use kphis_ui_app::App;
use kphis_ui_core::{class, doms, mixins};
use kphis_util::{
    datetime::{JsTime, date_8601, date_th, date_th_opt, datetime_th, datetime_th_opt, datetime_th_relative, js_now, time_8601, time_hm, time_hm_opt},
    error::CONTACT_ADMIN,
    util::{sanity_dot_space, sanity_tis620, str_some, zero_none},
};

use crate::{
    admission_note::AdmissionNoteCpn,
    gadget::{image::ImageCpn, pdf_button::{PdfButtons, static_pdf_btn_with_modal}},
    modal::{
        blank_modal,
        index_plan_action_form::{FormType, IndexPlanActionForm, OrderType},
        medplan_form::{MedPlanForm, MedPlanMutable, OffMedPlanMutable},
        pre_order_preview::ToOrderType,
        pre_order_select::{PreOrderSelect, PreOrderType},
    },
    opd_er_medical_history::OpdErMedicalHistoryCpn,
    order_form::{continuous::ContinuousForm, oneday::OneDayForm, progress_note::ProgressNoteForm},
};

pub trait MedSearchable {
    fn order_id(&self) -> Option<u32>;
    fn an(&self) -> Option<String>;
    fn focused(&self) -> Mutable<Option<u32>>;
    fn changed(&self) -> Mutable<bool>;
    fn display_med_searchbox(&self) -> Mutable<bool>;
    fn display_homemed_searchbox(&self) -> Mutable<bool>;
    fn display_ivfluid_searchbox(&self) -> Mutable<bool>;
    fn ivfluids(&self) -> MutableVec<Rc<OrderItemMutable>>;
    fn meds(&self) -> MutableVec<Rc<OrderItemMutable>>;
    fn homemeds(&self) -> MutableVec<Rc<OrderItemMutable>>;
    fn offs(&self) -> MutableVec<Rc<OrderItemMutable>>;
}

static NEXT_ID: AtomicU32 = AtomicU32::new(1);

pub const ORDER_STYLE: &str = r#"
ul.dash {
    list-style: none;
    margin-left: 0;
    padding-left: 1em;
    margin-bottom: 0
}
ul.dash > li:before {
    display: inline-block;
    content: "-";
    width: 1em;
    margin-left: -1em
}
.modal {
    background-color: rgba(0,0,0,0.5)
}"#;

/// - GET `EndPoint::IpdOrderOrderDateAn`
/// - GET `EndPoint::OpdErOrderMasterCheckVn`
/// - GET `EndPoint::IpdOrderOrder`
/// - GET `EndPoint::OpdErOrderOrder`
/// - GET `EndPoint::IpdOrderPrevious`
/// - GET `EndPoint::IpdOrderProgressNote`
/// - GET `EndPoint::OpdErOrderProgressNote`
/// - GET `EndPoint::OpdErHisMedVn` (opd-er only)
/// - GET `EndPoint::IpdMedReconcile`
/// - GET `EndPoint::OpdErMedReconcile`
/// - PATCH `EndPoint::IpdOrderOrder` (self/MedPlanForm, guarded, remove order-action btns, disable med-plan)
/// - PATCH `EndPoint::OpdErOrderOrder` (self/MedPlanForm, guarded, remove order-action btns, disable med-plan)
/// - DELETE `EndPoint::IpdOrderOrderId` (guarded, remove 'Delete' btn)
/// - DELETE `EndPoint::OpdErOrderOrderId` (guarded, remove 'Delete' btn)
/// - DELETE `EndPoint::IpdOrderProgressNoteId` (guarded, remove 'Delete' btn)
/// - DELETE `EndPoint::OpdErOrderProgressNoteId` (guarded, remove 'Delete' btn)
/// - POST `EndPoint::IpdOrderOrder` (OneDayForm/ContinuousForm, guarded, remove '+Add','Edit','Off' btn)
/// - POST `EndPoint::OpdErOrderOrder` (OneDayForm/ContinuousForm, guarded, remove '+Add','Edit','Off' btn)
/// - POST `EndPoint::IpdOrderProgressNote` (ProgressNoteForm, guarded, remove '+Add','Edit' btn)
/// - POST `EndPoint::OpdErOrderProgressNote` (ProgressNoteForm, guarded, remove '+Add','Edit' btn)
/// - GET `EndPoint::IpdOrderItem` (IndexPlanActionForm, guarded, remove '+Plan' btn)
/// - GET `EndPoint::OpdErOrderItem` (IndexPlanActionForm, guarded, remove '+Plan' btn)
/// - GET `EndPoint::HisMedPlanIpdAn` (MedPlanForm, guarded, disable med-plan)
/// - GET `EndPoint::OpdErMedicalHistory` (MedicalHistoryCpn, guarded, remove 'ประวัติผู้ป่วย' btn)
/// - GET `EndPoint::IpdAdmissionNoteDrAn` (AdmissionNoteCpn, guarded, remove 'ประวัติผู้ป่วย' btn)
/// - GET `EndPoint::IpdPreOrderMaster` (PreOrderSelect, guarded, remove 'Template','เลือกใบ Order' btn)
/// - POST `EndPoint::IpdPreOrderOrder` (PreOrderSelect, guarded, remove 'Template','เลือกใบ Order' btn)
/// - POST `EndPoint::IpdPreOrderProgressNote` (PreOrderSelect, guarded, remove 'Template','เลือกใบ Order' btn)
#[derive(Default)]
pub struct OrderCpn {
    is_ipd: bool,
    changed: Mutable<bool>,
    focused: Mutable<bool>,
    focus: Mutable<bool>,
    checker: Mutable<bool>,

    pub patient: Mutable<Option<Rc<PatientInfo>>>,

    date_path: Mutable<String>,
    loaded_order_date: Mutable<bool>,
    loaded_opd_er_check: Mutable<bool>,
    loaded_pre_order_count: Mutable<bool>,

    pub loaded_all: Mutable<bool>,

    pub reload_order_oneday: Mutable<bool>,
    pub reload_order_continuous: Mutable<bool>,
    pub reload_progress_note: Mutable<bool>,
    pub reload_opd_med: Mutable<bool>,

    view_by: Mutable<String>,
    focused_id: Mutable<u32>,
    nurse_order_as_result: Mutable<String>,
    pre_order_exists: Mutable<bool>,

    current_date: Mutable<Option<Rc<OrderDate>>>, // = $('#order_date_select').data('lastSelected', order_date)
    order_dates: MutableVec<Rc<OrderDate>>,
    order_dates_exact: Mutable<Vec<Rc<OrderDate>>>,
    order_dates_page: Mutable<usize>, // start from 0
    er_orders: MutableVec<Rc<OpdErOrderMasterCheck>>,

    is_readonly_yn: Mutable<String>,
    opd_meds: MutableVec<Rc<OpdMed>>,

    oneday: MutableVec<Rc<Order>>,
    continuous: MutableVec<Rc<Order>>,
    progress_note: MutableVec<Rc<ProgressNote>>,

    previous_dc_home_med: MutableVec<Rc<OrderItem>>,
    previous_retain: MutableVec<Rc<OrderItem>>,
    previous_continuous_non_med: MutableVec<Rc<OrderItem>>,
    previous_continuous_med: MutableVec<Rc<OrderItem>>,
    previous_continuous_injection: MutableVec<Rc<OrderItem>>,

    used_med_rec: MutableVec<Rc<MedReconciliationItem>>,
    holded_med_rec: MutableVec<Rc<MedReconciliationItem>>,
    offed_med_rec: MutableVec<Rc<MedReconciliationItem>>,
    missed_med_rec: MutableVec<Rc<MedReconciliationItem>>,

    show_admission_note: Mutable<bool>,
    show_medical_history: Mutable<bool>,

    show_oneday_input: Mutable<bool>,
    show_continuous_input: Mutable<bool>,
    show_progress_note_input: Mutable<bool>,
    show_progress_note_auditor_input: Mutable<bool>,

    // off_detail: Mutable<Option<(u32, String)>>,
    // for using in OrderForm
    offs_by_parent: MutableVec<Rc<OrderItemMutable>>,
    edit_order: Mutable<Option<Rc<Order>>>,
    edit_progress_note: Mutable<Option<Rc<ProgressNote>>>,

    pre_order_select_modal: Mutable<Option<Rc<PreOrderSelect>>>,
    index_plan_action_modal: Mutable<Option<Rc<IndexPlanActionForm>>>,

    pub off_icodes: Mutable<Vec<Rc<OffOrderItem>>>,
    // pub off_med_plan_numbers: MutableVec<i32>,
    pub off_medplans: MutableVec<Rc<OffMedPlanMutable>>,
    pub retain_medplans: MutableVec<Rc<OffMedPlanMutable>>,
    pub medplans: MutableVec<Rc<MedPlanMutable>>,
    medplan_form_modal: Mutable<Option<Rc<MedPlanForm>>>,
}

impl OrderCpn {
    pub fn new(
        is_ipd: bool,
        patient: Mutable<Option<Rc<PatientInfo>>>,
        view_by: Mutable<String>,
        is_readonly_yn: Mutable<String>,
        date_path: Mutable<String>,
        focused_id: Mutable<u32>,
        app: Rc<App>,
    ) -> Rc<Self> {
        let edit_order = app.edit_order.replace(None);
        let show_oneday_input = Mutable::new(false);
        let show_continuous_input = Mutable::new(false);
        if let Some(order_type) = edit_order.as_ref().map(|order| order.order_type.clone()) {
            match order_type.as_str() {
                "oneday" => {
                    show_oneday_input.set(true);
                }
                "continuous" => {
                    show_continuous_input.set(true);
                }
                _ => {}
            }
        }
        let (loaded_all, loaded_order_date, loaded_opd_er_check, reload_opd_med) = if is_ipd {
            // ipd : loaded_order_date -> loaded_all -> loaded_opd_er_check
            (Mutable::new(true), Mutable::new(false), Mutable::new(false), Mutable::new(false))
        } else {
            // opd-er : loaded_all, reload_opd_med needed to be `true` to loaded
            (Mutable::new(false), Mutable::new(true), Mutable::new(true), Mutable::new(true))
        };
        Rc::new(Self {
            is_ipd,
            patient,
            view_by,
            focused_id,
            edit_order: Mutable::new(edit_order),
            show_oneday_input,
            show_continuous_input,
            is_readonly_yn,
            date_path,
            loaded_all,
            loaded_order_date,
            loaded_opd_er_check,
            reload_opd_med,
            ..Default::default()
        })
    }

    fn is_readonly(&self) -> bool {
        self.is_readonly_yn.lock_ref().as_str() == "Y"
    }

    fn is_readonly_signal(&self) -> impl Signal<Item = bool> + use<> {
        self.is_readonly_yn.signal_cloned().map(|readonly| readonly == "Y")
    }

    fn is_not_discharged(&self) -> impl Signal<Item = bool> + use<> {
        self.patient
            .signal_cloned()
            .map(|opt| opt.as_ref().and_then(|patient| patient.lastdate().map(|dchdate| dchdate >= js_now().date())).unwrap_or(true))
    }

    fn is_view_by_doctor(&self) -> impl Signal<Item = bool> + use<> {
        self.view_by.signal_cloned().map(|view_by| view_by == "doctor")
    }

    fn is_view_by_doctor_or_nurse(&self) -> impl Signal<Item = bool> + use<> {
        self.view_by.signal_cloned().map(|view_by| ["doctor", "nurse"].contains(&view_by.as_str()))
    }

    fn is_view_by_doctor_or_nurse_and_not_readonly(&self) -> impl Signal<Item = bool> + use<> {
        map_ref! {
            let is_doctor_or_nurse = self.is_view_by_doctor_or_nurse(),
            let read_only = self.is_readonly_signal() =>
            *is_doctor_or_nurse && !read_only
        }
    }

    fn is_view_by_pharmacist(&self) -> impl Signal<Item = bool> + use<> {
        self.view_by.signal_cloned().map(|view_by| view_by == "pharmacist")
    }

    fn set_checker(&self) {
        self.checker
            .set_neq(!(self.oneday.lock_ref().is_empty() && self.continuous.lock_ref().is_empty() && self.progress_note.lock_ref().is_empty()));
    }

    fn current_is_order_date(&self, order_date: Rc<OrderDate>) -> impl Signal<Item = bool> + use<> {
        self.current_date.signal_cloned().map(move |opt| opt.as_ref().map(|cd| *cd == order_date).unwrap_or_default())
    }

    fn current_is_today(&self) -> impl Signal<Item = bool> + use<> {
        self.current_date.signal_cloned().map(|opt| opt.as_ref().map(|od| od.is_today).unwrap_or(false))
    }

    fn is_today(&self) -> bool {
        self.current_date.lock_ref().as_ref().map(|od| od.is_today).unwrap_or_default()
    }

    fn load_order_date(page: Rc<Self>, app: Rc<App>) {
        if let Some(patient) = page.patient.get_cloned() {
            match patient.visit_type() {
                VisitTypeId::Ipd(an) | VisitTypeId::PreAdmit(an) => {
                    app.async_load(
                        true,
                        clone!(app => async move {
                            // GET `EndPoint::IpdOrderOrderDateAn`
                            match OrderDate::call_api_get(&an, app.state()).await {
                                Ok(response) => {
                                    let mut dates = Vec::new();
                                    let now = js_now().date();
                                    let today = OrderDate { order_date: now, is_today: true };
                                    if !response.iter().any(|od| od.order_date == now) {
                                        // only IPD
                                        if let Some(dchdate) = patient.lastdate() {
                                            if now <= dchdate {
                                                dates.push(today);
                                            }
                                        } else {
                                            dates.push(today);
                                        }
                                    }
                                    dates.extend(response);

                                    let first_date = dates.first().cloned();
                                    let current_date = if !page.focused.get() {
                                        match date_8601(&page.date_path.lock_ref()) {
                                            Some(day) => {
                                                let od_opt = dates.iter().find(|d| d.order_date == day).cloned();
                                                od_opt.or(first_date)
                                            }
                                            None => {
                                                first_date
                                            }
                                        }
                                    } else {
                                        first_date
                                    };
                                    page.current_date.set_neq(current_date.map(Rc::new));

                                    let order_dates = dates.iter().map(|d| Rc::new(d.clone()));
                                    {
                                        let mut lock = page.order_dates.lock_mut();
                                        if !lock.is_empty() {
                                            lock.clear();
                                        }
                                        lock.extend(order_dates.clone());
                                    }
                                    page.order_dates_exact.set(order_dates.collect());
                                    page.loaded_all.set_neq(false);
                                }
                                Err(e) => {
                                    app.alert_app_error(&e).await;
                                }
                            }
                        }),
                    )
                }
                VisitTypeId::OpdEr(_, _) | VisitTypeId::Visit(_) => {}
            }
        }
    }

    fn load_opd_er_check(page: Rc<Self>, app: Rc<App>) {
        let vn_opt = page.patient.lock_ref().as_ref().and_then(|pt| pt.vn());
        if let Some(vn) = vn_opt {
            app.async_load(
                true,
                clone!(app => async move {
                    // GET `EndPoint::OpdErOrderMasterCheckVn`
                    match OpdErOrderMasterCheck::call_api_get(&vn, app.state()).await {
                        Ok(response) => {
                            page.er_orders.lock_mut().extend(response.into_iter().map(Rc::new));
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }),
            )
        }
    }

    fn load_pre_order_count(page: Rc<Self>, app: Rc<App>) {
        let hn_opt = page.patient.lock_ref().as_ref().and_then(|pt| pt.hn());
        if let Some(hn) = hn_opt {
            app.async_load(
                true,
                clone!(app => async move {
                    match call_api_get_exists_key_id("pre-order/", &hn, app.state()).await {
                        Ok(response) => {
                            page.pre_order_exists.set_neq(response);
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }),
            )
        }
    }

    fn load_all(page: Rc<Self>, app: Rc<App>) {
        if let Some(patient) = page.patient.get_cloned() {
            page.used_med_rec.lock_mut().clear();
            page.holded_med_rec.lock_mut().clear();
            page.offed_med_rec.lock_mut().clear();

            page.oneday.lock_mut().clear();
            page.continuous.lock_mut().clear();
            page.progress_note.lock_mut().clear();

            if page.is_ipd {
                page.previous_dc_home_med.lock_mut().clear();
                page.previous_retain.lock_mut().clear();
                page.previous_continuous_non_med.lock_mut().clear();
                page.previous_continuous_med.lock_mut().clear();
                page.previous_continuous_injection.lock_mut().clear();
            } else {
                page.opd_meds.lock_mut().clear();
            }

            app.async_load(
                true,
                clone!(app, page, patient => async move {
                    Self::fetch_used_med_rec(patient.clone(), page.clone(), app.clone()).await;
                    Self::fetch_holded_med_rec(patient.clone(), page.clone(), app.clone()).await;
                    Self::fetch_offed_med_rec(patient.clone(), page.clone(), app.clone()).await;

                    Self::fetch_order_oneday(page.clone(), app.clone()).await;
                    Self::fetch_order_continuous(page.clone(), app.clone()).await;
                    Self::fetch_progress_note(page.clone(), app.clone()).await;
                    page.focus.set(true);
                    if !page.is_ipd {
                        Self::fetch_opd_med(page.clone(), app.clone()).await;
                    }
                }),
            );
        }
    }

    pub async fn load_order_oneday(page: Rc<Self>, app: Rc<App>) {
        page.oneday.lock_mut().clear();
        if page.is_ipd {
            page.previous_dc_home_med.lock_mut().clear();
            page.previous_retain.lock_mut().clear();
        }
        Self::fetch_order_oneday(page, app).await;
    }
    async fn fetch_order_oneday(page: Rc<Self>, app: Rc<App>) {
        let visit_type = page.patient.lock_ref().as_ref().map(|pt| pt.visit_type());
        match visit_type {
            Some(VisitTypeId::Ipd(an)) | Some(VisitTypeId::PreAdmit(an)) => {
                let an_opt = str_some(an);
                let params = OrderParams {
                    an: an_opt.clone(),
                    order_type: Some(String::from("oneday")),
                    current_date: page.current_date.lock_ref().as_ref().map(|cd| cd.order_date),
                    view_by: str_some(page.view_by.get_cloned()),
                    ..Default::default()
                };
                // GET `EndPoint::IpdOrderOrder`
                match Order::call_api_get_ipd(&params, app.state()).await {
                    Ok(orders) => {
                        page.oneday.lock_mut().extend(orders.into_iter().map(Rc::new));
                        page.set_checker();
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
                let home_params = OrderParams {
                    an: an_opt.clone(),
                    order_type: Some(String::from("oneday")),
                    current_date: page.current_date.lock_ref().as_ref().map(|cd| cd.order_date),
                    view_by: str_some(page.view_by.get_cloned()),
                    order_item_types: Some(String::from("discharge,home-medication")),
                    ..Default::default()
                };
                // GET `EndPoint::IpdOrderPrevious`
                match OrderItem::call_api_get_ipd_previous(&home_params, app.state()).await {
                    Ok(order_items) => {
                        page.previous_dc_home_med.lock_mut().extend(order_items.into_iter().map(Rc::new));
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
                let retain_params = OrderParams {
                    an: an_opt.clone(),
                    order_type: Some(String::from("oneday")),
                    current_date: page.current_date.lock_ref().as_ref().map(|cd| cd.order_date),
                    view_by: str_some(page.view_by.get_cloned()),
                    order_item_types: Some(String::from("retain")),
                    ..Default::default()
                };
                // GET `EndPoint::IpdOrderPrevious`
                match OrderItem::call_api_get_ipd_previous(&retain_params, app.state()).await {
                    Ok(order_items) => {
                        page.previous_retain.lock_mut().extend(order_items.into_iter().map(Rc::new));
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }
            Some(VisitTypeId::OpdEr(_vn, opd_er_order_master_id)) => {
                let oneday_params = OrderParams {
                    opd_er_order_master_id: zero_none(opd_er_order_master_id),
                    order_type: Some(String::from("oneday")),
                    view_by: str_some(page.view_by.get_cloned()),
                    ..Default::default()
                };
                // GET `EndPoint::OpdErOrderOrder`
                match Order::call_api_get_opd_er(&oneday_params, app.state()).await {
                    Ok(orders) => {
                        page.oneday.lock_mut().extend(orders.into_iter().map(Rc::new));
                        page.set_checker();
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }
            Some(VisitTypeId::Visit(_)) | None => {}
        }
    }

    pub async fn load_order_continuous(page: Rc<Self>, app: Rc<App>) {
        page.continuous.lock_mut().clear();
        if page.is_ipd {
            page.previous_continuous_non_med.lock_mut().clear();
            page.previous_continuous_med.lock_mut().clear();
            page.previous_continuous_injection.lock_mut().clear();
        }
        Self::fetch_order_continuous(page, app).await;
    }
    async fn fetch_order_continuous(page: Rc<Self>, app: Rc<App>) {
        let visit_type = page.patient.lock_ref().as_ref().map(|pt| pt.visit_type());
        match visit_type {
            Some(VisitTypeId::Ipd(an)) | Some(VisitTypeId::PreAdmit(an)) => {
                let params = OrderParams {
                    an: str_some(an),
                    order_type: Some(String::from("continuous")),
                    current_date: page.current_date.lock_ref().as_ref().map(|cd| cd.order_date),
                    view_by: str_some(page.view_by.get_cloned()),
                    ..Default::default()
                };
                let mut is_err = false;
                // GET `EndPoint::IpdOrderOrder`
                match Order::call_api_get_ipd(&params, app.state()).await {
                    Ok(orders) => {
                        page.continuous.lock_mut().extend(orders.into_iter().map(Rc::new));
                        page.set_checker();
                    }
                    Err(e) => {
                        is_err = true;
                        app.alert_app_error(&e).await;
                    }
                }
                // GET `EndPoint::IpdOrderPrevious`
                match OrderItem::call_api_get_ipd_previous(&params, app.state()).await {
                    Ok(order_items) => {
                        let (med_inj, non_med): (Vec<OrderItem>, Vec<OrderItem>) = order_items
                            .into_iter()
                            .partition(|item| ["med", "injection"].contains(&item.order_item_type.clone().unwrap_or_default().as_str()));
                        let (injection, med): (Vec<OrderItem>, Vec<OrderItem>) = med_inj.into_iter().partition(|item| item.order_item_type.clone().unwrap_or_default().as_str() == "injection");
                        page.previous_continuous_non_med.lock_mut().extend(non_med.into_iter().map(Rc::new));
                        page.previous_continuous_med.lock_mut().extend(med.into_iter().map(Rc::new));
                        page.previous_continuous_injection.lock_mut().extend(injection.into_iter().map(Rc::new));
                    }
                    Err(e) => {
                        is_err = true;
                        app.alert_app_error(&e).await;
                    }
                }
                if !is_err {
                    page.set_missed_med_rec(app.hosxp_medrec_icode());
                }
            }
            Some(VisitTypeId::OpdEr(_vn, opd_er_order_master_id)) => {
                let params = OrderParams {
                    opd_er_order_master_id: zero_none(opd_er_order_master_id),
                    order_type: Some(String::from("continuous")),
                    view_by: str_some(page.view_by.get_cloned()),
                    ..Default::default()
                };
                // GET `EndPoint::OpdErOrderOrder`
                match Order::call_api_get_opd_er(&params, app.state()).await {
                    Ok(orders) => {
                        page.continuous.lock_mut().extend(orders.into_iter().map(Rc::new));
                        page.set_checker();
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }
            Some(VisitTypeId::Visit(_)) | None => {}
        }
    }

    pub async fn load_progress_note(page: Rc<Self>, app: Rc<App>) {
        page.progress_note.lock_mut().clear();
        Self::fetch_progress_note(page, app).await;
    }
    async fn fetch_progress_note(page: Rc<Self>, app: Rc<App>) {
        let visit_type = page.patient.lock_ref().as_ref().map(|pt| pt.visit_type());
        match visit_type {
            Some(VisitTypeId::Ipd(an)) | Some(VisitTypeId::PreAdmit(an)) => {
                let progress_note_params = ProgressNoteParams {
                    an: str_some(an),
                    progress_note_date: page.current_date.lock_ref().as_ref().map(|cd| cd.order_date),
                    ..Default::default()
                };
                // GET `EndPoint::IpdOrderProgressNote`
                match ProgressNote::call_api_get_ipd(&progress_note_params, app.state()).await {
                    Ok(progress_notes) => {
                        page.progress_note.lock_mut().extend(progress_notes.into_iter().map(Rc::new));
                        page.set_checker();
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }
            Some(VisitTypeId::OpdEr(_vn, opd_er_order_master_id)) => {
                let progress_note_params = ProgressNoteParams {
                    opd_er_order_master_id: zero_none(opd_er_order_master_id),
                    ..Default::default()
                };
                // GET `EndPoint::OpdErOrderProgressNote`
                match ProgressNote::call_api_get_opd_er(&progress_note_params, app.state()).await {
                    Ok(progress_notes) => {
                        page.progress_note.lock_mut().extend(progress_notes.into_iter().map(Rc::new));
                        page.set_checker();
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }
            Some(VisitTypeId::Visit(_)) | None => {}
        }
    }

    pub fn load_opd_med(page: Rc<Self>, app: Rc<App>) {
        page.opd_meds.lock_mut().clear();
        app.async_load(
            true,
            clone!(app, page => async move {
                Self::fetch_opd_med(page.clone(), app).await;
            }),
        )
    }
    async fn fetch_opd_med(page: Rc<Self>, app: Rc<App>) {
        let vn_opt = page.patient.lock_ref().as_ref().and_then(|pt| pt.vn());
        if let Some(vn) = vn_opt {
            // GET `EndPoint::OpdErHisMedVn`
            match OpdMed::call_api_get(&vn, app.state()).await {
                Ok(opd_meds) => {
                    page.opd_meds.lock_mut().extend(opd_meds.into_iter().map(Rc::new));
                }
                Err(e) => {
                    app.alert_app_error(&e).await;
                }
            }
        }
    }

    async fn fetch_used_med_rec(patient: Rc<PatientInfo>, page: Rc<Self>, app: Rc<App>) {
        // fetch used med-rec
        let result_opt = match &patient.visit_type {
            VisitTypeId::Ipd(an) | VisitTypeId::PreAdmit(an) => {
                let params = MedReconciliationParams {
                    hn: patient.hn(),
                    an: str_some(an.to_owned()),
                    used: Some(String::from("Y")),
                    ..Default::default()
                };
                // GET `EndPoint::IpdMedReconcile`
                Some(MedReconciliation::call_api_get(true, &params, app.state()).await)
            }
            VisitTypeId::OpdEr(_, opd_er_order_master_id) => {
                let params = MedReconciliationParams {
                    hn: patient.hn(),
                    opd_er_order_master_id: zero_none(*opd_er_order_master_id),
                    used: Some(String::from("Y")),
                    ..Default::default()
                };
                // GET `EndPoint::OpdErMedReconcile`
                Some(MedReconciliation::call_api_get(false, &params, app.state()).await)
            }
            VisitTypeId::Visit(_) => None,
        };

        if let Some(result) = result_opt {
            match result {
                Ok(responses) => {
                    page.used_med_rec.lock_mut().extend(responses.into_iter().flat_map(|res| res.med_reconciliation_items).map(Rc::new));
                }
                Err(e) => {
                    app.alert_app_error(&e).await;
                }
            }
        }
    }

    fn set_missed_med_rec(&self, med_rec_icode: Option<String>) {
        // clear
        self.missed_med_rec.lock_mut().clear();
        // prepared variables
        let used_mrd_rec = self.used_med_rec.lock_ref();
        let continuous = self.continuous.lock_ref();
        let confirmed_cont = continuous.iter().filter(|order| order.is_confirm()).flat_map(|order| {
            order
                .order_item_types
                .iter()
                .flat_map(|oit| oit.order_items.iter().filter(|oi| oi.icode.is_some()).cloned().map(Rc::new))
        });
        let prev_med = self.previous_continuous_med.lock_ref().to_vec();
        let prev_inj = self.previous_continuous_injection.lock_ref().to_vec();
        // partition in-hos / out-hos
        let (out_hos_used, in_hos_used): (Vec<_>, Vec<_>) = used_mrd_rec.iter().partition(clone!(med_rec_icode => move |item| {
            med_rec_icode.as_ref().zip(item.icode.as_ref()).map(|(a, b)| a == b).unwrap_or_default()
        }));
        let (out_hos_oi, in_hos_oi): (Vec<_>, Vec<_>) = confirmed_cont
            .chain(prev_med.into_iter())
            .chain(prev_inj.into_iter())
            .partition(|item| med_rec_icode.as_ref().zip(item.icode.as_ref()).map(|(a, b)| a == b).unwrap_or_default());
        // calculate in-hos items
        let in_hos_icodes = in_hos_oi.iter().filter_map(|item| item.icode.clone()).collect::<HashSet<String>>();
        let in_hos_missed = in_hos_used
            .into_iter()
            .filter(|item| item.icode.as_ref().map(|icode| !in_hos_icodes.contains(icode)).unwrap_or_default());
        // calculate out-hos items
        let out_hos_mednames = out_hos_oi.iter().filter_map(|item| item.med_name.clone()).collect::<HashSet<String>>();
        let out_hos_missed = out_hos_used
            .into_iter()
            .filter(|item| item.custom_med_name.as_ref().map(|cmed_name| !out_hos_mednames.contains(cmed_name)).unwrap_or_default());
        // set
        self.missed_med_rec.lock_mut().extend(in_hos_missed.chain(out_hos_missed).cloned());
    }

    async fn fetch_holded_med_rec(patient: Rc<PatientInfo>, page: Rc<Self>, app: Rc<App>) {
        let result_opt = match &patient.visit_type {
            VisitTypeId::Ipd(an) | VisitTypeId::PreAdmit(an) => {
                let params = MedReconciliationParams {
                    hn: patient.hn(),
                    an: str_some(an.to_owned()),
                    used: Some(String::from("H")),
                    ..Default::default()
                };
                // GET `EndPoint::IpdMedReconcile`
                Some(MedReconciliation::call_api_get(true, &params, app.state()).await)
            }
            VisitTypeId::OpdEr(_, opd_er_order_master_id) => {
                let params = MedReconciliationParams {
                    hn: patient.hn(),
                    opd_er_order_master_id: zero_none(*opd_er_order_master_id),
                    used: Some(String::from("H")),
                    ..Default::default()
                };
                // GET `EndPoint::OpdErMedReconcile`
                Some(MedReconciliation::call_api_get(false, &params, app.state()).await)
            }
            VisitTypeId::Visit(_) => None,
        };

        if let Some(result) = result_opt {
            match result {
                Ok(responses) => {
                    page.holded_med_rec.lock_mut().extend(responses.into_iter().flat_map(|res| res.med_reconciliation_items).map(Rc::new));
                }
                Err(e) => {
                    app.alert_app_error(&e).await;
                }
            }
        }
    }

    async fn fetch_offed_med_rec(patient: Rc<PatientInfo>, page: Rc<Self>, app: Rc<App>) {
        let result_opt = match &patient.visit_type {
            VisitTypeId::Ipd(an) | VisitTypeId::PreAdmit(an) => {
                let params = MedReconciliationParams {
                    hn: patient.hn(),
                    an: str_some(an.to_owned()),
                    used: Some(String::from("N")),
                    ..Default::default()
                };
                // GET `EndPoint::IpdMedReconcile`
                Some(MedReconciliation::call_api_get(true, &params, app.state()).await)
            }
            VisitTypeId::OpdEr(_, opd_er_order_master_id) => {
                let params = MedReconciliationParams {
                    hn: patient.hn(),
                    opd_er_order_master_id: zero_none(*opd_er_order_master_id),
                    used: Some(String::from("N")),
                    ..Default::default()
                };
                // GET `EndPoint::OpdErMedReconcile`
                Some(MedReconciliation::call_api_get(false, &params, app.state()).await)
            }
            VisitTypeId::Visit(_) => None,
        };

        if let Some(result) = result_opt {
            match result {
                Ok(responses) => {
                    page.offed_med_rec.lock_mut().extend(responses.into_iter().flat_map(|res| res.med_reconciliation_items).map(Rc::new));
                }
                Err(e) => {
                    app.alert_app_error(&e).await;
                }
            }
        }
    }

    /// - PATCH `EndPoint::IpdOrderOrder`
    /// - PATCH `EndPoint::OpdErOrderOrder`
    pub fn patch_order(action: OrderPatchAction, order: Rc<Order>, order_time: Option<Time>, dues_opt: Option<Rc<DueMutables>>, is_oneday: bool, page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app, order => async move {
                if app.confirm("ยืนยันรายการ").await {
                    let is_order_as = matches!(action, OrderPatchAction::ConfirmAs | OrderPatchAction::EditAs);
                    let nurse_order_as = str_some(page.nurse_order_as_result.get_cloned());
                    if is_order_as && nurse_order_as.is_none() {
                        app.alert_error_with_closed("กรุณาเลือก รคส", "").await;
                        return;
                    }

                    if let Some(dues) = dues_opt && !dues.items.is_empty() && if page.is_ipd {
                        app.endpoint_is_allow(&Method::PATCH, &EndPoint::IpdOrderItem, false)
                    } else {
                        app.endpoint_is_allow(&Method::PATCH, &EndPoint::OpdErOrderItem, false)
                    } {
                        match action {
                            OrderPatchAction::Confirm | OrderPatchAction::ConfirmAs => {
                                for due in dues.items.iter() {
                                    // - PATCH `EndPoint::IpdOrderItem`
                                    // - PATCH `EndPoint::OpdErOrderItem`
                                    Self::patch_due_doctor(due, page.is_ipd, app.clone()).await;
                                }
                            }
                            _ => {}
                        }
                    }

                    let medplans = if matches!(action, OrderPatchAction::PharmacistAccept) {
                        page.medplans.lock_ref().iter().filter_map(|mp| mp.export()).collect()
                    } else {
                        Vec::new()
                    };
                    let mut off_med_plan_numbers = Vec::new();
                    off_med_plan_numbers.extend(page.off_medplans.lock_ref().to_vec().iter().filter_map(|mp| mp.off_med_plan_number()));
                    off_med_plan_numbers.extend(page.retain_medplans.lock_ref().to_vec().iter().filter_map(|mp| mp.off_med_plan_number()));
                    let order_patch = OrderPatch {
                        action: action.clone(),
                        order_id: order.order_id,
                        nurse_order_as: if is_order_as {nurse_order_as.clone()} else {None},
                        order_time,
                        medplans,
                        off_med_plan_numbers,
                    };
                    // PATCH `EndPoint::IpdOrderOrder`
                    // PATCH `EndPoint::OpdErOrderOrder`
                    match order_patch.call_api_patch(page.is_ipd, app.state()).await {
                        Ok(responses) => {
                            app.alert_execute_responses(&responses, clone!(app => async move {
                                page.changed.set_neq(false);
                                send_sse_by_patch(action, page.patient.get_cloned(), order, nurse_order_as, app.clone());
                                if is_oneday {
                                    Self::load_order_oneday(page, app).await;
                                } else {
                                    Self::load_order_continuous(page, app).await;
                                }
                            })).await;
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }
            }),
        );
    }

    /// - PATCH `EndPoint::IpdOrderItem`
    /// - PATCH `EndPoint::OpdErOrderItem`
    fn patch_due_pharm(dues: Rc<DueMutables>, order: Rc<Order>, page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app => async move {
                if !dues.items.is_empty() && if page.is_ipd {
                    app.endpoint_is_allow(&Method::PATCH, &EndPoint::IpdOrderItem, false)
                } else {
                    app.endpoint_is_allow(&Method::PATCH, &EndPoint::OpdErOrderItem, false)
                } {
                    let mut has_pharm_message = false;
                    for due in dues.items.iter() {
                        let due_pharm = due.due_pharm.get_cloned();
                        let due_pharm_note = due.due_pharm_note.get_cloned();
                        if due_pharm.as_ref().map(|d| d != "Y").unwrap_or_default() && due_pharm_note.is_some() {
                            has_pharm_message = true;
                        }
                        let save = OrderItemPatch {
                            order_item_id: due.order_item_id,
                            action: OrderItemPatchAction::DuePharm,
                            nurse_assign: None,
                            order_item_type: None,
                            due_doctor: None,
                            due_doctor_note: None,
                            due_pharm,
                            due_pharm_note,
                        };
                        // PATCH `EndPoint::IpdOrderItem`
                        // PATCH `EndPoint::OpdErOrderItem`
                        match save.call_api_patch(page.is_ipd, app.state()).await {
                            Ok(response) => {
                                if let Some(error) = &response.error {
                                    app.alert_error_with_clipboard(CONTACT_ADMIN, &["ExecuteResponse: ", error].concat()).await;
                                } else {
                                    dues.changed.set(false);
                                }
                            }
                            Err(e) => {
                                app.alert_app_error(&e).await;
                            }
                        }
                    }
                    if has_pharm_message {
                        let patient = page.patient.get_cloned();
                        if let Some(an) = patient.as_ref().and_then(|pt| pt.an.clone()) {
                            let ward_name = patient.as_ref().and_then(|pt| pt.ward_name.as_ref().map(|ward| [ward, " "].concat())).unwrap_or_default();
                            let bed = patient.as_ref().and_then(|pt| pt.bedno.as_ref().map(|bedno| ["เตียง ", bedno, " "].concat())).unwrap_or_default();
                            let hn = patient.as_ref().and_then(|pt| pt.hn.as_ref().map(|hn| ["HN ", hn, " "].concat())).unwrap_or_default();

                            let (person, view_by) = if order.order_owner_type.as_str() == "nurse" {
                                if order.nurse_order_as.is_some() {
                                    (order.nurse_order_as.clone(), "doctor")
                                } else {
                                    (Some(order.order_doctor.clone()), "nurse")
                                }
                            } else {
                                (Some(order.order_doctor.clone()), "doctor")
                            };
                            let message = SsePostMessage {
                                message: [&ward_name, &bed, &hn, "เภสัชกร มีข้อเสนอแนะเกี่ยวกับคำสั่งของท่าน"].concat(),
                                person,
                                // ward: Some(ward),
                                route: Some(Route::IpdMain {
                                    view_by: view_by.to_owned(),
                                    an,
                                    tab: String::from("order"),
                                    sub: order.order_date.to_string(),
                                    id: order.order_id,
                                }),
                                ..Default::default()
                            };
                            app.send_sse(message);
                        }
                    }
                }
            }),
        )
    }

    /// - PATCH `EndPoint::IpdOrderItem`
    /// - PATCH `EndPoint::OpdErOrderItem`
    async fn patch_due_doctor(due: &DueMutable, is_ipd: bool, app: Rc<App>) {
        let save = OrderItemPatch {
            order_item_id: due.order_item_id,
            action: OrderItemPatchAction::DueDoctor,
            nurse_assign: None,
            order_item_type: None,
            due_doctor: due.due_doctor.get_cloned(),
            due_doctor_note: due.due_doctor_note.get_cloned(),
            due_pharm: None,
            due_pharm_note: None,
        };
        // PATCH `EndPoint::IpdOrderItem`
        // PATCH `EndPoint::OpdErOrderItem`
        match save.call_api_patch(is_ipd, app.state()).await {
            Ok(response) => {
                if let Some(error) = &response.error {
                    app.alert_error_with_clipboard(CONTACT_ADMIN, &["ExecuteResponse: ", error].concat()).await;
                }
            }
            Err(e) => {
                app.alert_app_error(&e).await;
            }
        }
    }

    fn delete_order(order_id: u32, is_oneday: bool, page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app => async move {
                if app.confirm("ยืนยันรายการ").await {
                    // DELETE `EndPoint::IpdOrderOrderId`
                    // DELETE `EndPoint::OpdErOrderOrderId`
                    match Order::call_api_delete(page.is_ipd, order_id, app.state()).await {
                        Ok(response) => {
                            app.alert_execute_response(&response, clone!(app => async move {
                                if is_oneday {
                                    Self::load_order_oneday(page, app).await;
                                } else {
                                    Self::load_order_continuous(page, app).await;
                                }
                            })).await;
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }
            }),
        );
    }

    fn delete_progress_note(progress_note_id: u32, page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app => async move {
                if app.confirm("ยืนยันรายการ").await {
                    // DELETE `EndPoint::IpdOrderProgressNoteId`
                    // DELETE `EndPoint::OpdErOrderProgressNoteId`
                    match ProgressNote::call_api_delete(page.is_ipd, progress_note_id, app.state()).await {
                        Ok(response) => {
                            app.alert_execute_response(&response, clone!(app => async move {
                                Self::load_progress_note(page, app).await;
                            })).await;
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }
            }),
        );
    }

    pub fn render(cpn_id: &'static str, page: Rc<Self>, app: Rc<App>) -> Dom {
        let is_ipd = page.is_ipd;
        let is_pre_admit = page.patient.lock_ref().as_ref().map(|pt| pt.visit_type.is_pre_admit()).unwrap_or_default();
        let has_audit_use = app.has_permission(Permission::DataTypeAuditorUse);
        let allow_pre_order = app.endpoint_is_allow(&Method::GET, &EndPoint::IpdPreOrderMaster, true)
            && app.endpoint_is_allow(&Method::POST, &EndPoint::IpdPreOrderOrder, true)
            && app.endpoint_is_allow(&Method::POST, &EndPoint::IpdPreOrderProgressNote, true);
        let allow_order_form = if is_ipd {
            app.endpoint_is_allow(&Method::POST, &EndPoint::IpdOrderOrder, is_pre_admit)
        } else {
            app.endpoint_is_allow(&Method::POST, &EndPoint::OpdErOrderOrder, false)
        };
        let allow_order_add = if is_ipd {
            app.has_permission(Permission::IpdOrderAdd)
        } else {
            app.has_permission(Permission::OpdErOrderAdd)
        };
        let allow_progress_form = if is_ipd {
            app.endpoint_is_allow(&Method::POST, &EndPoint::IpdOrderProgressNote, is_pre_admit)
        } else {
            app.endpoint_is_allow(&Method::POST, &EndPoint::OpdErOrderProgressNote, false)
        };
        let allow_progress_add = if is_ipd {
            app.has_permission(Permission::ProgressNoteAdd)
        } else {
            app.has_permission(Permission::OpdErProgressNoteAdd)
        };

        html!("div", {
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let loaded = page.loaded_order_date.signal() =>
                !busy && !loaded
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load_order_date(page.clone(), app.clone());
                    page.loaded_order_date.set_neq(true);
                }
                async {}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let loaded = page.loaded_opd_er_check.signal() =>
                !busy && !loaded
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load_opd_er_check(page.clone(), app.clone());
                    page.loaded_opd_er_check.set(true);
                }
                async {}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let loaded = page.loaded_pre_order_count.signal() =>
                !busy && !loaded
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load_pre_order_count(page.clone(), app.clone());
                    page.loaded_pre_order_count.set(true);
                }
                async {}
            })))
            .future(map_ref!{
                let focused = page.focused.signal(),
                let focus = page.focus.signal() =>
                *focus && !focused
            }.for_each(clone!(app, page => move |ready| {
                if ready {
                    if let Some(id) = zero_none(page.focused_id.get()) {
                        app.scroll_into_view(&["order_id_", &id.to_string(), "_div", cpn_id].concat());
                    }
                    // focus only once
                    // page.focus.set(false);
                    page.focused.set(true);
                }
                async {}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let loaded = page.loaded_all.signal() =>
                !busy && !loaded
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load_all(page.clone(), app.clone());
                    page.loaded_all.set_neq(true);
                }
                async {}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let reload = page.reload_order_oneday.signal() =>
                !busy && *reload
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    page.reload_order_oneday.set_neq(false);
                    app.async_load(
                        true,
                        clone!(app, page => async move {
                            Self::load_order_oneday(page.clone(), app.clone()).await;
                        })
                    );
                }
                async {}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let reload = page.reload_order_continuous.signal() =>
                !busy && *reload
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    page.reload_order_continuous.set_neq(false);
                    app.async_load(
                        true,
                        clone!(app, page => async move {
                            Self::load_order_continuous(page.clone(), app.clone()).await;
                        })
                    )
                }
                async {}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let reload = page.reload_progress_note.signal() =>
                !busy && *reload
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    page.reload_progress_note.set_neq(false);
                    app.async_load(
                        true,
                        clone!(app, page => async move {
                            Self::load_progress_note(page.clone(), app.clone()).await;
                        })
                    )
                }
                async {}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let reload = page.reload_opd_med.signal() =>
                !busy && *reload
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load_opd_med(page.clone(), app.clone());
                    page.reload_opd_med.set_neq(false);
                }
                async {}
            })))
            .future(map_ref!{
                let focused = page.focused.signal(),
                let focus = page.focus.signal() =>
                *focus && !focused
            }.for_each(clone!(app, page => move |ready| {
                if ready {
                    if let Some(id) = zero_none(page.focused_id.get()) {
                        app.scroll_into_view(&["order_id_", &id.to_string(), "_div", cpn_id].concat());
                    }
                    // focus only once
                    // page.focus.set(false);
                    page.focused.set(true);
                }
                async {}
            })))
            .children([
                html!("style", { .text(ORDER_STYLE)}),
                html!("div", {
                    .class("order-content")
                    .child_signal(page.show_admission_note.signal_cloned().map(clone!(app, page => move |show| {
                        show.then(|| {
                            html!("div", {
                                .class("row")
                                .child(html!("div", {
                                    .class("col-12")
                                    .style("white-space","pre-wrap")
                                    .child(AdmissionNoteCpn::render(
                                        AdmissionNoteCpn::new(page.patient.clone()),
                                        page.show_admission_note.clone(),
                                        cpn_id == "aside",
                                        app.clone()
                                    ))
                                    // ipd-nurse-index.php::viewDrAdmissionNoteAndNurseIndexNote()
                                    // ipd-nurse-index-print-data.php
                                }))
                            })
                        })
                    })))
                    .children([
                        html!("div", {
                            .class(class::FLEX_WRAP_T)
                            .apply_if(page.is_ipd, |dom| { dom
                                .child(html!("div", {
                                    .class(class::COLA_PY_L)
                                    //.visible_signal(not(page.order_dates.signal_vec_cloned().is_empty()))
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP)
                                        .children([
                                            html!("div", {
                                                .class("input-group-text")
                                                .attr("title","แสดงรายการวันที่ทั้งหมดรวมถึงวันที่ไม่มีข้อมูลด้วย")
                                                .children([
                                                    html!("input" => HtmlInputElement, {
                                                        .attr("type", "checkbox")
                                                        .attr("id", &["show-all-order-date-checkbox", cpn_id].concat())
                                                        .with_node!(element => {
                                                            .event(clone!(page => move |_: events::Click| {
                                                                if element.checked() {
                                                                    let now = js_now().date();
                                                                    let patient = page.patient.lock_ref();
                                                                    let reg = patient.as_ref().and_then(|pt| pt.regdate()).unwrap_or(now);
                                                                    // IPD only ?
                                                                    let dch = patient.as_ref().and_then(|pt| pt.lastdate()).unwrap_or(now);

                                                                    let mut all = Vec::new();
                                                                    let mut i = Some(dch);
                                                                    while let Some(d) = i {
                                                                        if d >= reg {
                                                                            all.push(OrderDate { order_date: d, is_today: d == now });
                                                                            i = d.previous_day();
                                                                        } else {
                                                                            i = None;
                                                                        }
                                                                    }
                                                                    if !(reg..=dch).contains(&now) {
                                                                        all.push(OrderDate { order_date: now, is_today: true });
                                                                    }

                                                                    {
                                                                        let mut lock = page.order_dates.lock_mut();
                                                                        if !lock.is_empty() {
                                                                            lock.clear();
                                                                        }
                                                                        lock.extend(all.iter().map(|d| Rc::new(d.clone())));
                                                                    }

                                                                    if let Some(current) = page.current_date.get_cloned() {
                                                                        if !(reg..=dch).contains(&current.order_date) {
                                                                            page.current_date.set_neq(all.first().cloned().map(Rc::new));
                                                                            page.loaded_all.set_neq(false);
                                                                        }
                                                                    }
                                                                } else {
                                                                    let mut lock = page.order_dates.lock_mut();
                                                                    lock.replace_cloned(page.order_dates_exact.get_cloned());
                                                                }
                                                            }))
                                                        })
                                                        // onclick="onShowAllOrderDateCheckbox(event)
                                                    }),
                                                    html!("label", {
                                                        .class(class::FORM_CHK_LBL_R)
                                                        .attr("for", &["show-all-order-date-checkbox", cpn_id].concat())
                                                        .style("user-select","none")
                                                        .text("ทุกวัน")
                                                    })
                                                ])
                                            }),
                                            html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_GRAY)
                                                .child(html!("i",{.class(class::FA_L_CARET)}))
                                                .event(clone!(page => move |_: events::Click| {
                                                    if let Some(current_date) = page.current_date.get_cloned() {
                                                        let lock = page.order_dates.lock_ref();
                                                        let len = lock.len();
                                                        if let Some(pos) = lock.iter().position(|d| *d == current_date) {
                                                            if pos < len - 1 {
                                                                page.current_date.set_neq(Some((lock[pos + 1]).clone()));
                                                                page.loaded_all.set_neq(false);
                                                            }
                                                        }
                                                    }
                                                }))
                                                // onclick="changeSelectOption(document.getElementById('order_date_select'), 'previous', false)
                                            }),
                                            html!("select" => HtmlSelectElement, {
                                                .class("form-select")
                                                .children_signal_vec(page.order_dates.signal_vec_cloned().map(clone!(page => move |od| {
                                                    html!("option", {
                                                        .attr("value", &od.string())
                                                        .text(&[date_th(&od.order_date), if od.is_today {String::from(" (วันนี้)")} else {String::new()}].concat())
                                                        .apply_if(date_8601(&page.date_path.lock_ref()).map(|dp| dp == od.order_date).unwrap_or_default(), |dom| dom.attr("selected",""))
                                                    })
                                                })))
                                                .prop_signal("value", page.current_date.signal_cloned().map(|opt| opt.as_ref().map(|d| d.string())))
                                                .with_node!(element => {
                                                    .event(clone!(page => move |_: events::Change| {
                                                        page.current_date.set_neq(OrderDate::from_string(&element.value()).map(Rc::new));
                                                        page.loaded_all.set_neq(false);
                                                    }))
                                                })
                                                // onchange="onchange_select_order_date(event)
                                            }),
                                            html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_GRAY)
                                                .child(html!("i",{.class(class::FA_R_CARET)}))
                                                .event(clone!(page => move |_: events::Click| {
                                                    if let Some(current_date) = page.current_date.get_cloned() {
                                                        let lock = page.order_dates.lock_ref();
                                                        if let Some(pos) = lock.iter().position(|d| *d == current_date) {
                                                            if pos > 0 {
                                                                page.current_date.set_neq(Some((lock[pos - 1]).clone()));
                                                                page.loaded_all.set_neq(false);
                                                            }
                                                        }
                                                    }
                                                }))
                                                // onclick="changeSelectOption(document.getElementById('order_date_select'), 'next', false)
                                            }),
                                        ])
                                    }))
                                }))
                                .child_signal(map_ref!{
                                    let order_dates_page = page.order_dates_page.signal(),
                                    let order_dates = page.order_dates.signal_vec_cloned().to_signal_cloned() =>
                                    (*order_dates_page, order_dates.clone())
                                }.map(clone!(page => move |(order_dates_page, order_dates)| {
                                    let max_pages = order_dates.len() / 7;
                                    Some(html!("div", {
                                        .class(class::COLA_P)
                                        .apply_if(order_dates_page > 1, |dom| {
                                            dom.child(html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_L_CYAN)
                                                .child(html!("i",{.class(class::FA_L_ANGLES)}))
                                                .event(clone!(page => move |_: events::Click| {
                                                    page.order_dates_page.set_neq(0);
                                                }))
                                            }))
                                        })
                                        .apply_if(order_dates_page > 0, |dom| {
                                            dom.child(html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_L_CYAN)
                                                .child(html!("i",{.class(class::FA_L_ANGLE)}))
                                                .event(clone!(page => move |_: events::Click| {
                                                    page.order_dates_page.set_neq(page.order_dates_page.get().saturating_sub(1));
                                                }))
                                            }))
                                        })
                                        .children(order_dates.into_iter().enumerate().filter(|(i,_)| {
                                            *i >= (7 * order_dates_page) && *i < (7 * (order_dates_page + 1))
                                        }).map(clone!(page => move |(_, d)| {
                                            html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_L)
                                                .class_signal("btn-primary", page.current_is_order_date(d.clone()))
                                                .class_signal("btn-secondary", not(page.current_is_order_date(d.clone())))
                                                .text(&[date_th(&d.order_date), if d.is_today {String::from(" (วันนี้)")} else {String::new()}].concat())
                                                .event(clone!(page => move |_: events::Click| {
                                                    page.current_date.set_neq(Some(d.clone()));
                                                    page.loaded_all.set_neq(false);
                                                }))
                                            })
                                        })))
                                        .apply_if(order_dates_page < max_pages, |dom| {
                                            dom.child(html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_L_CYAN)
                                                .child(html!("i",{.class(class::FA_R_ANGLE)}))
                                                .event(clone!(page => move |_: events::Click| {
                                                    page.order_dates_page.set_neq(page.order_dates_page.get() + 1);
                                                }))
                                            }))
                                        })
                                        .apply_if(max_pages.saturating_sub(order_dates_page) > 1, |dom| {
                                            dom.child(html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_CYAN)
                                                .child(html!("i",{.class(class::FA_R_ANGLES)}))
                                                .event(clone!(page => move |_: events::Click| {
                                                    page.order_dates_page.set_neq(max_pages);
                                                }))
                                            }))
                                        })
                                    }))
                                })))
                                .apply_if(cpn_id != "aside", |not_aside| not_aside
                                    // opd-er-order-master-check.php
                                    .child(html!("div", {
                                        .class("col-auto")
                                        .children_signal_vec(page.er_orders.signal_vec_cloned().filter_map(clone!(app, page => move |er| {
                                            let route = Route::OpdErMain {
                                                view_by: page.view_by.get_cloned(),
                                                opd_er_order_master_id: er.opd_er_order_master_id,
                                                tab: Tab::Order.str().to_owned(),
                                                id: 0,
                                            };
                                            route.has_permission(app.state()).then(|| {
                                                html!("button", {
                                                    .attr("type", "button")
                                                    .class(class::BTN_GRAY)
                                                    // .text("ER Order")
                                                    .text(&["ER (", &date_th(&er.order_date), ")"].concat())
                                                    .child(html!("i", {.class(class::FA_EXT_LINK_R)}))
                                                    .event(move |_: events::Click| {
                                                        route.hard_redirect();
                                                        // open /opd-er-order.php?view_by=doctor&opd_er_order_master_id=${v.opd_er_order_master_id}
                                                    })
                                                })
                                            })
                                        })))
                                    }))
                                )
                            })

                            .child_signal(map_ref!{
                                let is_readonly = page.is_readonly_signal(),
                                let is_pharmacist = page.is_view_by_pharmacist() =>
                                !is_readonly && *is_pharmacist
                            }.map(clone!(app => move |is_allow| {
                                is_allow.then(|| {
                                    html!("div", {
                                        .class(class::COLA_P)
                                        .child(html!("div", {
                                            .class(class::FORM_CHK_SW)
                                            .class("mx-3")
                                            .style("padding-top","8px")
                                            .attr("title","ค่าปกติ คือ เภสัชกรสามารถรับรายการ ได้เฉพาะรายการที่สามารถบันทึกลง HOSxP ได้ หรือรายการ Pharmacist notify เท่านั้น")
                                            .children([
                                                html!("input" => HtmlInputElement, {
                                                    .attr("type", "checkbox")
                                                    .attr("id", &["pharmacist_allow_non_med_sw", cpn_id].concat())
                                                    .class("form-check-input")
                                                    .attr("role","switch")
                                                    .with_node!(element => {
                                                        .future(app.pharmacist_allow_non_med.signal().for_each(clone!(element => move |v| {
                                                            element.set_checked(v);
                                                            async {}
                                                        })))
                                                        .event(clone!(app => move |_: events::Change| {
                                                            app.pharmacist_allow_non_med.set_neq(element.checked());
                                                        }))
                                                    })
                                                }),
                                                doms::label_check_for(&["pharmacist_allow_non_med_sw", cpn_id].concat(),"รับได้ทุกคำสั่ง"),
                                            ])
                                        }))
                                    })
                                })
                            })))
                            .child(html!("div", {
                                .class(class::PY_RX)
                                .apply_if(cpn_id != "aside", |not_aside| not_aside
                                    .child_signal(page.view_by.signal_cloned().map(clone!(app, page => move |view_by| {
                                        ["nurse","pharmacist"].contains(&view_by.as_str()).then(|| {
                                            html!("div", {
                                                .class("float-end")
                                                .children_signal_vec(clone!(page, app => page.patient.signal_cloned().map(move |opt| {
                                                    if let Some(patient) = opt {
                                                        match patient.visit_type() {
                                                            VisitTypeId::Ipd(an)
                                                            | VisitTypeId::PreAdmit(an) => {
                                                                PdfButtons::buttons(
                                                                    PdfButtons::new(
                                                                        TypstReport::from_system_with_coercion(SystemReport::IpdMAR, &app.state().report_coercions()),
                                                                        Mutable::new(an.clone()),
                                                                        Mutable::new(true),
                                                                        page.changed.clone(),
                                                                        move || {serde_json::json!({
                                                                            "id": an,
                                                                            "patient": patient,
                                                                        }).to_string()}
                                                                    ), "eMAR", None, app.clone()
                                                                )
                                                            },
                                                            VisitTypeId::OpdEr(_, _)
                                                            | VisitTypeId::Visit(_) => Vec::new(),
                                                        }
                                                    } else {
                                                        Vec::new()
                                                    }
                                                }).to_signal_vec()))
                                            })
                                        })
                                    })))
                                    .child_signal(page.view_by.signal_cloned().map(clone!(app, page => move |view_by| {
                                        (view_by == "nurse").then(|| {
                                            html!("div", {
                                                .class("float-end")
                                                .children_signal_vec(clone!(page, app => page.patient.signal_cloned().map(move |opt| {
                                                    if let Some(patient) = opt {
                                                        match patient.visit_type() {
                                                            VisitTypeId::Ipd(an) | VisitTypeId::PreAdmit(an) => {
                                                                PdfButtons::buttons(
                                                                    PdfButtons::new(
                                                                        TypstReport::from_system_with_coercion(SystemReport::IpdOrder, &app.state().report_coercions()),
                                                                        Mutable::new(an.clone()),
                                                                        page.checker.clone(),
                                                                        page.changed.clone(),
                                                                        clone!(page => move || {serde_json::json!({
                                                                            "id": an,
                                                                            "patient": patient,
                                                                            "oneday": page.oneday.lock_ref().to_vec(),
                                                                            "cont": page.continuous.lock_ref().to_vec(),
                                                                            "note": page.progress_note.lock_ref().to_vec(),
                                                                            "doctor": null,
                                                                        }).to_string()})
                                                                    ), "Order(วันนี้)", Some("Order(ทั้งหมด)"), app.clone()
                                                                )
                                                            }
                                                            VisitTypeId::OpdEr(vn, _) | VisitTypeId::Visit(vn) => {
                                                                PdfButtons::buttons(
                                                                    PdfButtons::new(
                                                                        TypstReport::from_system_with_coercion(SystemReport::OpdErOrder, &app.state().report_coercions()),
                                                                        Mutable::new(vn.clone()),
                                                                        page.checker.clone(),
                                                                        page.changed.clone(),
                                                                        clone!(page => move || {serde_json::json!({
                                                                            "id": vn,
                                                                            "patient": patient,
                                                                            "oneday": page.oneday.lock_ref().to_vec(),
                                                                            "cont": page.continuous.lock_ref().to_vec(),
                                                                            "note": page.progress_note.lock_ref().to_vec(),
                                                                        }).to_string()})
                                                                    ), "PDF", None, app.clone()
                                                                )
                                                            }
                                                        }
                                                    } else {
                                                        Vec::new()
                                                    }
                                                }).to_signal_vec()))
                                            })
                                        })
                                    })))
                                    .child_signal(map_ref!{
                                        let is_doctor = page.is_view_by_doctor(),
                                        let is_not_discharge = page.is_not_discharged() =>
                                        *is_doctor && if is_ipd {*is_not_discharge} else {true}
                                    }.map(clone!(page => move |is_doctor_not_discharge| {
                                        (is_doctor_not_discharge && allow_pre_order).then(|| {
                                            html!("div", {
                                                .class(class::FLOAT_RL)
                                                .child(html!("button", {
                                                    .attr("type", "button")
                                                    .class(class::BTN_GRAY)
                                                    .attr("data-bs-toggle", "modal")
                                                    .attr("data-bs-target", &["#selectPreOrderModal", cpn_id].concat())
                                                    .child(html!("i", {.class(class::FA_CLIPBOARD)}))
                                                    .text(" Template")
                                                    .event(clone!(page => move |_: events::Click| {
                                                        match page.patient.lock_ref().as_ref().map(|pt| pt.visit_type()) {
                                                            Some(VisitTypeId::Ipd(an))
                                                            | Some(VisitTypeId::PreAdmit(an)) => {
                                                                page.pre_order_select_modal.set(Some(PreOrderSelect::new(
                                                                    PreOrderType::Template,
                                                                    &an,
                                                                    ToOrderType::Order,
                                                                )));
                                                            }
                                                            Some(VisitTypeId::OpdEr(_vn, opd_er_order_master_id)) => {
                                                                page.pre_order_select_modal.set(Some(PreOrderSelect::new(
                                                                    PreOrderType::Template,
                                                                    &opd_er_order_master_id.to_string(),
                                                                    ToOrderType::OpdErOrder,
                                                                )));
                                                            }
                                                            Some(VisitTypeId::Visit(_))
                                                            | None => {}
                                                        }
                                                    }))
                                                }))
                                            })
                                        })
                                    })))
                                    .child_signal(map_ref!{
                                        let is_doctor_or_nurse = page.is_view_by_doctor_or_nurse(),
                                        let is_not_discharge = page.is_not_discharged() =>
                                        * is_doctor_or_nurse && if is_ipd {*is_not_discharge} else {true}
                                    }.map(clone!(page => move |is_doctor_or_nurse_not_discharge| {
                                        (is_doctor_or_nurse_not_discharge && allow_pre_order).then(|| {
                                            html!("div", {
                                                .class(class::FLOAT_RL)
                                                .child(html!("button", {
                                                    .attr("type", "button")
                                                    .class(class::BTN_GRAY)
                                                    .attr("data-bs-toggle", "modal")
                                                    .attr("data-bs-target", &["#selectPreOrderModal", cpn_id].concat())
                                                    .children([
                                                        html!("span", {
                                                            .class(class::SPIN_SM_GLOW_RED)
                                                            .attr("role", "status")
                                                            .style_signal("display", page.pre_order_exists.signal_cloned().map(|exs| {
                                                                if exs {"inline-block"} else {"none"}
                                                            }))
                                                        }),
                                                        html!("i", {.class(class::FA_CLIPBOARD_R)}),
                                                    ])
                                                    .text(" เลือกใบ Order")
                                                    .event(clone!(page => move |_: events::Click| {
                                                        if let Some(patient) = page.patient.lock_ref().as_ref() {
                                                            match patient.visit_type() {
                                                                VisitTypeId::Ipd(an)
                                                                | VisitTypeId::PreAdmit(an) => {
                                                                    page.pre_order_select_modal.set(Some(PreOrderSelect::new(
                                                                        PreOrderType::PreOrder(patient.hn().unwrap_or_default()),
                                                                        &an,
                                                                        ToOrderType::Order,
                                                                    )));
                                                                }
                                                                VisitTypeId::OpdEr(_vn, opd_er_order_master_id) => {
                                                                    page.pre_order_select_modal.set(Some(PreOrderSelect::new(
                                                                        PreOrderType::PreOrder(patient.hn().unwrap_or_default()),
                                                                        &opd_er_order_master_id.to_string(),
                                                                        ToOrderType::OpdErOrder,
                                                                    )));
                                                                }
                                                                VisitTypeId::Visit(_) => {}
                                                            }
                                                        }
                                                    }))
                                                }))
                                            })
                                        })
                                    })))
                                )
                                .apply(|dom| {
                                    if cpn_id == "aside" { dom
                                        .apply_if(app.endpoint_is_allow(&Method::GET, &EndPoint::IpdAdmissionNoteDrAn, is_pre_admit), |d| d
                                            .child(html!("div", {
                                                .class(class::FLOAT_RL)
                                                .child(html!("button", {
                                                    .attr("type", "button")
                                                    .class(class::BTN_GRAY)
                                                    .child(html!("i", {.class(class::FA_CLIPBOARD)}))
                                                    .text(" ประวัติผู้ป่วย")
                                                    .event(clone!(page => move |_: events::Click| {
                                                        page.show_admission_note.set_neq(!page.show_admission_note.get());
                                                    }))
                                                }))
                                            }))
                                        )
                                    // IPD
                                    } else if page.is_ipd { dom
                                        .child_signal(map_ref!{
                                            let is_doctor = page.is_view_by_doctor(),
                                            let an_opt = page.patient.signal_cloned().map(|opt| match opt.map(|pt| pt.visit_type()) {
                                                Some(VisitTypeId::Ipd(an))
                                                | Some(VisitTypeId::PreAdmit(an)) => Some(an),
                                                Some(VisitTypeId::OpdEr(_, _))
                                                | Some(VisitTypeId::Visit(_))
                                                | None => None,
                                            }) => (*is_doctor, an_opt.clone())
                                        }.map(clone!(app, page => move |(is_doctor, an_opt)| {
                                            if is_doctor && let Some(an) = an_opt {
                                                let route = Route::Summary {view_by: page.view_by.get_cloned(), an};
                                                route.has_permission(app.state()).then(|| {
                                                    html!("div", {
                                                        .class(class::FLOAT_RL)
                                                        .child(html!("button", {
                                                            .attr("type", "button")
                                                            .class(class::BTN_GRAY)
                                                            .child(html!("i", {.class(class::FA_CLIPBOARD)}))
                                                            .text(" IPD Summary")
                                                            .event(move |_: events::Click| {
                                                                route.hard_redirect();
                                                            })
                                                        }))
                                                    })
                                                })
                                            } else {
                                                None
                                            }
                                        })))
                                        .apply_if(app.endpoint_is_allow(&Method::GET, &EndPoint::IpdAdmissionNoteDrAn, is_pre_admit), |d| d
                                            .child_signal(page.view_by.signal_cloned().map(clone!(page => move |view_by| {
                                                ["doctor","nurse","pharmacist"].contains(&view_by.as_str()).then(|| {
                                                    html!("div", {
                                                        .class(class::FLOAT_RL)
                                                        .child(html!("button", {
                                                            .attr("type", "button")
                                                            .class(class::BTN_GRAY)
                                                            .child(html!("i", {.class(class::FA_CLIPBOARD)}))
                                                            .text(" ประวัติผู้ป่วย")
                                                            .event(clone!(page => move |_: events::Click| {
                                                                page.show_admission_note.set_neq(!page.show_admission_note.get());
                                                            }))
                                                        }))
                                                        // .attr("onclick", "onclickViewDrAdmissionNoteButton();")
                                                    })
                                                })
                                            })))
                                        )
                                    // OPD-ER
                                    } else { dom
                                        .apply_if(app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErMedicalHistory, false), |d| d
                                            .child_signal(page.show_medical_history.signal_cloned().map(clone!(page => move |show| {
                                                (!show).then(|| {
                                                    html!("div", {
                                                        .class(class::FLOAT_RR)
                                                        .child(html!("button", {
                                                            .attr("type", "button")
                                                            .class(class::BTN_L_GRAY)
                                                            .child(html!("i", {.class(class::FA_CLIPBOARD)}))
                                                            .text(" ประวัติผู้ป่วย")
                                                            .event(clone!(page => move |_: events::Click| {
                                                                page.show_medical_history.set_neq(true);
                                                                // .attr("onclick", "onclickViewHistoryButton();")
                                                            }))
                                                        }))
                                                    })
                                                })
                                            })))
                                        )
                                        .child_signal(page.patient.signal_cloned().map(clone!(app, page => move |opt| {
                                            opt.as_ref().and_then(|pt| pt.an.clone()).and_then(|an| {
                                                let route = Route::IpdMain {
                                                    view_by: page.view_by.get_cloned(),
                                                    an: an.clone(),
                                                    tab: Tab::Order.str().to_owned(),
                                                    sub: String::new(),
                                                    id: 0,
                                                };
                                                route.has_permission(app.state()).then(|| {
                                                    html!("div", {
                                                        .class(class::FLOAT_RR)
                                                        .child(html!("button", {
                                                            .attr("type", "button")
                                                            .class(class::BTN_GRAY)
                                                            .child(html!("i", {.class(class::FA_EXT_LINK)}))
                                                            .text(" Order ผู้ป่วยใน")
                                                            .event(move |_: events::Click| {
                                                                route.hard_redirect();
                                                            })
                                                        }))
                                                    })
                                                })
                                            })
                                        })))
                                    }
                                })
                            }))
                        }),
                    ])
                    .child_signal(page.show_medical_history.signal_cloned().map(clone!(app, page => move |show| {
                        show.then(|| {
                            html!("div", {
                                .class(class::COL_T)
                                .child(OpdErMedicalHistoryCpn::render(
                                    OpdErMedicalHistoryCpn::new(
                                        // page.order_page.clone(),
                                        page.patient.clone(),
                                        None,
                                    ),
                                    Some(page.show_medical_history.clone()),
                                    app.clone()
                                ))
                                // opd-er-order-medical-history-data.php
                            })
                        })
                    })))
                    .children([
                        html!("div", {
                            .class("row")
                            .child(html!("div", {
                                .class("col-12")
                                .child(html!("div", {
                                    .class("tab-content")
                                    .child(html!("div", {
                                        .class(class::TAB_FADE_SHOW_ACTIVE)
                                        .attr("role", "tabpanel")
                                        .child(html!("div", {
                                            .class("row")
                                            .child(html!("div", {
                                                .class("col")
                                                .apply_if(cpn_id != "aside", |dom| dom
                                                    .child_signal(page.is_view_by_doctor().map(|is_doctor| {
                                                        is_doctor.then(|| {
                                                            html!("div", {
                                                                .class("mb-2")
                                                                .children([
                                                                    doms::badge_info_center("บันทึก Progress note ที่ครอบคลุม S O A P ทุกวันใน 3 วันแรก และทุกครั้งที่มีการเปลี่ยนแปลงอาการ หรือการรักษา หรือให้ยาหรือมีการทำ invasive procedure"),
                                                                    doms::badge_info_center("บันทึกการแปลผล investigation ที่สำคัญ และมีการให้การวินิจฉัยร่วมกับการวางแผนการรักษาเมื่อผล investigation ผิดปกติ"),
                                                                ])
                                                            })
                                                        })
                                                    }))
                                                )
                                                .child(html!("table", {
                                                    .class(class::TABLE_1R)
                                                    .children([
                                                        html!("thead", {
                                                            .child(html!("tr", {
                                                                .children([
                                                                    html!("th", {
                                                                        .attr("scope", "col")
                                                                        .class(class::TXT_C_TOP)
                                                                        .class("bg-secondary-subtle")
                                                                        .style("width","30%")
                                                                        .child_signal(page.is_readonly_signal().map(clone!(page => move |is_readonly| {
                                                                            (!is_readonly && allow_progress_form && allow_progress_add).then(|| {
                                                                                html!("button", {
                                                                                    .attr("type", "button")
                                                                                    .class(class::BTN_FR_R)
                                                                                    .class_signal("btn-primary", not(page.show_progress_note_input.signal()))
                                                                                    .class_signal("btn-secondary", page.show_progress_note_input.signal())
                                                                                    .text_signal(page.show_progress_note_input.signal_cloned().map(|show| {
                                                                                        if show {"Cancel"} else {"+Add"}
                                                                                    }))
                                                                                    .event(clone!(page => move |_: events::Click| {
                                                                                        page.edit_progress_note.set(None);
                                                                                        page.show_progress_note_auditor_input.set_neq(false);
                                                                                        page.show_progress_note_input.set(!page.show_progress_note_input.get());
                                                                                    }))
                                                                                })
                                                                            })
                                                                        })))
                                                                        .child_signal(map_ref!{
                                                                            let is_not_discharged = page.is_not_discharged(),
                                                                            let is_readonly = page.is_readonly_signal() =>
                                                                            !is_not_discharged && !is_readonly
                                                                        }.map(clone!(page => move |ready| {
                                                                            (ready && has_audit_use && allow_progress_form && allow_progress_add).then(|| {
                                                                                html!("button", {
                                                                                    .attr("type", "button")
                                                                                    .class(class::BTN_FR_R)
                                                                                    .class_signal("btn-warning", not(page.show_progress_note_auditor_input.signal()))
                                                                                    .class_signal("btn-secondary", page.show_progress_note_auditor_input.signal())
                                                                                    .text_signal(page.show_progress_note_auditor_input.signal_cloned().map(|show| {
                                                                                        if show {"Cancel"} else {"+Audit"}
                                                                                    }))
                                                                                    .event(clone!(page => move |_: events::Click| {
                                                                                        page.edit_progress_note.set(None);
                                                                                        page.show_progress_note_input.set_neq(false);
                                                                                        page.show_progress_note_auditor_input.set(!page.show_progress_note_auditor_input.get());
                                                                                    }))
                                                                                })
                                                                            })
                                                                        })))
                                                                        .child(html!("div",{.class("mt-2").text("Progress Note")}))
                                                                    }),
                                                                    html!("th", {
                                                                        .attr("scope", "col")
                                                                        .class(class::TXT_C_TOP)
                                                                        .style("width","35%")
                                                                        .child_signal(map_ref!{
                                                                            let is_today = page.current_is_today(),
                                                                            let is_doctor_or_nurse_not_readonly = page.is_view_by_doctor_or_nurse_and_not_readonly() =>
                                                                            if is_ipd {*is_today} else {true} && *is_doctor_or_nurse_not_readonly
                                                                        }.map(clone!(page => move |ready| {
                                                                            (ready && allow_order_form && allow_order_add).then(|| {
                                                                                html!("button", {
                                                                                    .attr("type", "button")
                                                                                    .class(class::BTN_FR_R)
                                                                                    .class_signal("btn-primary", not(page.show_oneday_input.signal()))
                                                                                    .class_signal("btn-secondary", page.show_oneday_input.signal())
                                                                                    .text_signal(page.show_oneday_input.signal_cloned().map(|show| {
                                                                                        if show {"Cancel"} else {"+Add"}
                                                                                    }))
                                                                                    .event(clone!(page => move |_: events::Click| {
                                                                                        page.edit_order.set(None);
                                                                                        page.offs_by_parent.lock_mut().clear();
                                                                                        page.show_continuous_input.set_neq(false);
                                                                                        page.show_oneday_input.set(!page.show_oneday_input.get());
                                                                                    }))
                                                                                })
                                                                            })
                                                                        })))
                                                                        .child(html!("div",{.class("mt-2").text("One Day Order")}))
                                                                    }),
                                                                    html!("th", {
                                                                        .attr("scope", "col")
                                                                        .class(class::TXT_C_TOP)
                                                                        .style("width","35%")
                                                                        .child_signal(map_ref!{
                                                                            let is_today = page.current_is_today(),
                                                                            let is_doctor_or_nurse_not_readonly = page.is_view_by_doctor_or_nurse_and_not_readonly() =>
                                                                            if is_ipd {*is_today} else {true} && *is_doctor_or_nurse_not_readonly
                                                                        }.map(clone!(page => move |ready| {
                                                                            (ready && allow_order_form && allow_order_add).then(|| {
                                                                                html!("button", {
                                                                                    .attr("type", "button")
                                                                                    .class(class::BTN_FR_R)
                                                                                    .class_signal("btn-primary", not(page.show_continuous_input.signal()))
                                                                                    .class_signal("btn-secondary", page.show_continuous_input.signal())
                                                                                    .text_signal(page.show_continuous_input.signal_cloned().map(|show| {
                                                                                        if show {"Cancel"} else {"+Add"}
                                                                                    }))
                                                                                    .event(clone!(page => move |_: events::Click| {
                                                                                        page.edit_order.set(None);
                                                                                        page.offs_by_parent.lock_mut().clear();
                                                                                        page.show_oneday_input.set_neq(false);
                                                                                        page.show_continuous_input.set(!page.show_continuous_input.get());
                                                                                    }))
                                                                                })
                                                                            })
                                                                        })))
                                                                        .child(html!("div",{.class("mt-2").text("Continuous Order")}))
                                                                    }),
                                                                ])
                                                            }))
                                                        }),
                                                        html!("tbody", {
                                                            .child(html!("tr", {
                                                                .children([
                                                                    // Progress Note
                                                                    html!("td", {
                                                                        // ipd-dr-order-progress-note-data.php
                                                                        .class("bg-secondary-subtle")
                                                                        .children_signal_vec(page.progress_note.signal_vec_cloned().map(clone!(app, page => move |progress_note| {
                                                                            Self::render_progress_note(cpn_id, progress_note, page.clone(), app.clone())
                                                                        })))
                                                                        .child(html!("div", {
                                                                            .class("text-end")
                                                                            .child_signal(page.is_readonly_signal().map(clone!(page => move |is_readonly| {
                                                                                (!is_readonly && allow_progress_form && allow_progress_add).then(|| {
                                                                                    html!("button", {
                                                                                        .attr("type", "button")
                                                                                        .class(class::BTN_R)
                                                                                        .class_signal("btn-primary", not(page.show_progress_note_input.signal()))
                                                                                        .class_signal("btn-secondary", page.show_progress_note_input.signal())
                                                                                        .text_signal(page.show_progress_note_input.signal_cloned().map(|show| {
                                                                                            if show {"Cancel"} else {"+Add"}
                                                                                        }))
                                                                                        .event(clone!(page => move |_: events::Click| {
                                                                                            page.edit_progress_note.set(None);
                                                                                            page.show_progress_note_auditor_input.set_neq(false);
                                                                                            page.show_progress_note_input.set(!page.show_progress_note_input.get());
                                                                                        }))
                                                                                    })
                                                                                })
                                                                            })))
                                                                            .child_signal(map_ref!{
                                                                                let is_not_discharged = page.is_not_discharged(),
                                                                                let is_readonly = page.is_readonly_signal() =>
                                                                                !is_not_discharged && !is_readonly
                                                                            }.map(clone!(page => move |ready| {
                                                                                (ready && has_audit_use && allow_progress_form && allow_progress_add).then(|| {
                                                                                    html!("button", {
                                                                                        .attr("type", "button")
                                                                                        .class(class::BTN_R)
                                                                                        .class_signal("btn-warning", not(page.show_progress_note_auditor_input.signal()))
                                                                                        .class_signal("btn-secondary", page.show_progress_note_auditor_input.signal())
                                                                                        .text_signal(page.show_progress_note_auditor_input.signal_cloned().map(|show| {
                                                                                            if show {"Cancel"} else {"+Audit"}
                                                                                        }))
                                                                                        .event(clone!(page => move |_: events::Click| {
                                                                                            page.edit_progress_note.set(None);
                                                                                            page.show_progress_note_input.set_neq(false);
                                                                                            page.show_progress_note_auditor_input.set(!page.show_progress_note_auditor_input.get());
                                                                                        }))
                                                                                    })
                                                                                })
                                                                            })))
                                                                        }))
                                                                        .child_signal(map_ref!{
                                                                            let normal = page.show_progress_note_input.signal(),
                                                                            let auditor = page.show_progress_note_auditor_input.signal() =>
                                                                            (*normal, *auditor)
                                                                        }.map(clone!(app, page => move |(normal, auditor)| {
                                                                            (normal || auditor).then(|| {
                                                                                let form = ProgressNoteForm::new(
                                                                                    auditor,
                                                                                    page.edit_progress_note.get_cloned(),
                                                                                    page.current_date.clone(),
                                                                                    page.view_by.clone(),
                                                                                    page.patient.clone(),
                                                                                    None,
                                                                                    app.user.lock_ref().as_ref().map(|u|u.user.doctorcode.get_cloned()).unwrap_or_default(),
                                                                                );
                                                                                ProgressNoteForm::render(
                                                                                    form,
                                                                                    page.show_progress_note_input.clone(),
                                                                                    page.show_progress_note_auditor_input.clone(),
                                                                                    page.edit_progress_note.clone(),
                                                                                    page.reload_progress_note.clone(),
                                                                                    app.clone(),
                                                                                )
                                                                            })
                                                                        })))
                                                                    }),
                                                                    // One Day Order
                                                                    html!("td", {
                                                                        .child_signal(page.previous_dc_home_med.signal_vec_cloned().is_empty().map(clone!(app, page => move |empty| {
                                                                            (!empty).then(|| {
                                                                                html!("div", {
                                                                                    .class(class::BOX_ROUND_T)
                                                                                    .children([
                                                                                        html!("div", {.class("fw-bold").text("Previous Discharge and Home Medication")}),
                                                                                        html!("ul", {
                                                                                            .class("dash")
                                                                                            .class(class::BORDER_T_T)
                                                                                            .style("white-space","pre-wrap")
                                                                                            .children_signal_vec(page.previous_dc_home_med.signal_vec_cloned().map(clone!(app, page => move |order| {
                                                                                                Self::render_previous_order(cpn_id, &order, OrderType::OneDay, page.clone(), app.clone())
                                                                                            })))
                                                                                        }),
                                                                                    ])
                                                                                })
                                                                            })
                                                                        })))
                                                                        .child_signal(page.previous_retain.signal_vec_cloned().is_empty().map(clone!(app, page => move |empty| {
                                                                            (!empty).then(|| {
                                                                                html!("div", {
                                                                                    .class(class::BOX_ROUND_T)
                                                                                    .children([
                                                                                        html!("div", {.class("fw-bold").text("Previous Retains")}),
                                                                                        html!("ul", {
                                                                                            .class("dash")
                                                                                            .class(class::BORDER_T_T)
                                                                                            .style("white-space","pre-wrap")
                                                                                            .children_signal_vec(page.previous_retain.signal_vec_cloned().map(clone!(app, page => move |order| {
                                                                                                Self::render_previous_order(cpn_id, &order, OrderType::OneDay, page.clone(), app.clone())
                                                                                            })))
                                                                                        }),
                                                                                    ])
                                                                                })
                                                                            })
                                                                        })))
                                                                        .children_signal_vec(page.oneday.signal_vec_cloned().map(clone!(app, page => move |order| {
                                                                            Self::render_order(cpn_id, order, true, page.clone(), app.clone())
                                                                        })))
                                                                        .child(html!("div", {
                                                                            .class("text-end")
                                                                            .child_signal(map_ref!{
                                                                                let is_today = page.current_is_today(),
                                                                                let is_doctor_or_nurse_not_readonly = page.is_view_by_doctor_or_nurse_and_not_readonly() =>
                                                                                (if is_ipd {*is_today} else {true} && *is_doctor_or_nurse_not_readonly)
                                                                            }.map(clone!(page => move |ready| {
                                                                                (ready && allow_order_form && allow_order_add).then(|| {
                                                                                    html!("button", {
                                                                                        .attr("type", "button")
                                                                                        .class(class::BTN_R)
                                                                                        .class_signal("btn-primary", not(page.show_oneday_input.signal()))
                                                                                        .class_signal("btn-secondary", page.show_oneday_input.signal())
                                                                                        .text_signal(page.show_oneday_input.signal_cloned().map(|show| {
                                                                                            if show {"Cancel"} else {"+Add"}
                                                                                        }))
                                                                                        .event(clone!(page => move |_: events::Click| {
                                                                                            page.edit_order.set(None);
                                                                                            page.offs_by_parent.lock_mut().clear();
                                                                                            page.show_continuous_input.set_neq(false);
                                                                                            page.show_oneday_input.set(!page.show_oneday_input.get());
                                                                                        }))
                                                                                    })
                                                                                })
                                                                            })))
                                                                        }))
                                                                        .child_signal(map_ref!{
                                                                            let is_today = page.current_is_today(),
                                                                            let show_input = page.show_oneday_input.signal() =>
                                                                            if is_ipd {*is_today} else {true} && *show_input
                                                                        }.map(clone!(app, page => move |show| {
                                                                            show.then(|| {
                                                                                let form = OneDayForm::new(
                                                                                    page.edit_order.get_cloned(),
                                                                                    page.patient.clone(),
                                                                                    None,
                                                                                    app.user.lock_ref().as_ref().map(|u|u.user.doctorcode.get_cloned()).unwrap_or_default(),
                                                                                    page.view_by.clone(),
                                                                                    page.offs_by_parent.clone(),
                                                                                );
                                                                                OneDayForm::render(
                                                                                    form,
                                                                                    page.show_oneday_input.clone(),
                                                                                    page.edit_order.clone(),
                                                                                    page.reload_order_oneday.clone(),
                                                                                    app.clone(),
                                                                                )
                                                                            })
                                                                        })))
                                                                    }),
                                                                    // Continuous Order
                                                                    html!("td", {
                                                                        // ipd-dr-order-continuous-previous-data.php
                                                                        .child_signal(page.previous_continuous_non_med.signal_vec_cloned().is_empty().map(clone!(app, page => move |empty| {
                                                                            (!empty).then(|| {
                                                                                html!("div", {
                                                                                    .class(class::BOX_ROUND_T)
                                                                                    .children([
                                                                                        html!("div", {.class("fw-bold").text("Current Treatment")}),
                                                                                        html!("ul", {
                                                                                            .class("dash")
                                                                                            .class(class::BORDER_T_T)
                                                                                            .style("white-space","pre-wrap")
                                                                                            .children_signal_vec(page.previous_continuous_non_med.signal_vec_cloned().map(clone!(app, page => move |order| {
                                                                                                Self::render_previous_order(cpn_id, &order, OrderType::Continuous, page.clone(), app.clone())
                                                                                            })))
                                                                                        }),
                                                                                    ])
                                                                                })
                                                                            })
                                                                        })))
                                                                        .child_signal(page.previous_continuous_injection.signal_vec_cloned().is_empty().map(clone!(app, page => move |empty| {
                                                                            (!empty).then(|| {
                                                                                html!("div", {
                                                                                    .class(class::BOX_ROUND_T)
                                                                                    .children([
                                                                                        html!("div", {.class("fw-bold").text("Current Injection")}),
                                                                                        html!("ol", {
                                                                                            .class("dash")
                                                                                            .class(class::BORDER_T_T)
                                                                                            .style("white-space","pre-wrap")
                                                                                            .children_signal_vec(page.previous_continuous_injection.signal_vec_cloned().map(clone!(app, page => move |order| {
                                                                                                Self::render_previous_order(cpn_id, &order, OrderType::Continuous, page.clone(), app.clone())
                                                                                            })))
                                                                                        }),
                                                                                    ])
                                                                                })
                                                                            })
                                                                        })))
                                                                        .child_signal(page.previous_continuous_med.signal_vec_cloned().is_empty().map(clone!(app, page => move |empty| {
                                                                            (!empty).then(|| {
                                                                                html!("div", {
                                                                                    .class(class::BOX_ROUND_T)
                                                                                    .children([
                                                                                        html!("div", {.class("fw-bold").text("Current Medication")}),
                                                                                        html!("ol", {
                                                                                            .class("dash")
                                                                                            .class(class::BORDER_T_T)
                                                                                            .style("white-space","pre-wrap")
                                                                                            .children_signal_vec(page.previous_continuous_med.signal_vec_cloned().map(clone!(app, page => move |order| {
                                                                                                Self::render_previous_order(cpn_id, &order, OrderType::Continuous, page.clone(), app.clone())
                                                                                            })))
                                                                                        }),
                                                                                    ])
                                                                                })
                                                                            })
                                                                        })))
                                                                        .child_signal(page.holded_med_rec.signal_vec_cloned().is_empty().map(clone!(app, page => move |empty| {
                                                                            (!empty).then(|| {
                                                                                html!("div", {
                                                                                    .class(class::BOX_ROUND_T)
                                                                                    .class("bg-secondary-subtle")
                                                                                    .children([
                                                                                        html!("div", {
                                                                                            .class("fw-bold")
                                                                                            .text("Held Med Reconciliation")
                                                                                        }),
                                                                                        html!("ol", {
                                                                                            .class("dash")
                                                                                            .class(class::BORDER_T_T)
                                                                                            .style("white-space","pre-wrap")
                                                                                            .children_signal_vec(page.holded_med_rec.signal_vec_cloned().map(clone!(app => move |med_rec_item| {
                                                                                                Self::render_med_rec(&med_rec_item, app.clone())
                                                                                            })))
                                                                                        }),
                                                                                    ])
                                                                                })
                                                                            })
                                                                        })))
                                                                        .child_signal(page.offed_med_rec.signal_vec_cloned().is_empty().map(clone!(app, page => move |empty| {
                                                                            (!empty).then(|| {
                                                                                html!("div", {
                                                                                    .class(class::BOX_ROUND_T)
                                                                                    .class("bg-secondary-subtle")
                                                                                    .children([
                                                                                        html!("div", {
                                                                                            .class("fw-bold")
                                                                                            .text("Offed Med Reconciliation")
                                                                                        }),
                                                                                        html!("ol", {
                                                                                            .class("dash")
                                                                                            .class(class::BORDER_T_T)
                                                                                            .style("white-space","pre-wrap")
                                                                                            .children_signal_vec(page.offed_med_rec.signal_vec_cloned().map(clone!(app => move |med_rec_item| {
                                                                                                Self::render_med_rec(&med_rec_item, app.clone())
                                                                                            })))
                                                                                        }),
                                                                                    ])
                                                                                })
                                                                            })
                                                                        })))
                                                                        .child_signal(page.missed_med_rec.signal_vec_cloned().is_empty().map(clone!(app, page => move |empty| {
                                                                            (!empty).then(|| {
                                                                                html!("div", {
                                                                                    .class(class::BOX_ROUND_T)
                                                                                    .class("bg-secondary-subtle")
                                                                                    .child_signal(map_ref!{
                                                                                        let show = page.show_continuous_input.signal(),
                                                                                        let is_today = page.current_is_today(),
                                                                                        let is_doctor_or_nurse_not_readonly = page.is_view_by_doctor_or_nurse_and_not_readonly() =>
                                                                                        !show && if is_ipd {*is_today} else {true} && *is_doctor_or_nurse_not_readonly
                                                                                    }.map(clone!(page => move |ready| {
                                                                                        ready.then(|| {
                                                                                            html!("button", {
                                                                                                .attr("type", "button")
                                                                                                .class(class::BTN_SM_FR_BLUEO)
                                                                                                .child(html!("i", {.class(class::FA_PLUS_L)}))
                                                                                                .text("Add")
                                                                                                .event(clone!(page => move |_:events::Click| {
                                                                                                    page.edit_order.set(Some(Order::new_from_med_rec_items(&page.missed_med_rec.lock_ref())));
                                                                                                    page.offs_by_parent.lock_mut().clear();
                                                                                                    page.show_oneday_input.set_neq(false);
                                                                                                    page.show_continuous_input.set(!page.show_continuous_input.get());
                                                                                                }))
                                                                                            })
                                                                                        })
                                                                                    })))
                                                                                    .children([
                                                                                        html!("div", {
                                                                                            .class("fw-bold")
                                                                                            .text("Missed Med Reconciliation")
                                                                                        }),
                                                                                        html!("ol", {
                                                                                            .class("dash")
                                                                                            .class(class::BORDER_T_T)
                                                                                            .style("white-space","pre-wrap")
                                                                                            .children_signal_vec(page.missed_med_rec.signal_vec_cloned().map(clone!(app => move |med_rec_item| {
                                                                                                Self::render_med_rec(&med_rec_item, app.clone())
                                                                                            })))
                                                                                        }),
                                                                                    ])
                                                                                })
                                                                            })
                                                                        })))
                                                                        // ipd-dr-order-continuous-data.php
                                                                        .children_signal_vec(page.continuous.signal_vec_cloned().map(clone!(app, page => move |order| {
                                                                            Self::render_order(cpn_id, order, false, page.clone(), app.clone())
                                                                        })))
                                                                        .child(html!("div", {
                                                                            .class("text-end")
                                                                            .child_signal(map_ref!{
                                                                                let is_today = page.current_is_today(),
                                                                                let is_doctor_or_nurse_not_readonly = page.is_view_by_doctor_or_nurse_and_not_readonly() =>
                                                                                (if is_ipd {*is_today} else {true} && *is_doctor_or_nurse_not_readonly)
                                                                            }.map(clone!(page => move |ready| {
                                                                                (ready && allow_order_form && allow_order_add).then(|| {
                                                                                    html!("button", {
                                                                                        .attr("type", "button")
                                                                                        .class(class::BTN_R)
                                                                                        .class("mb-1")
                                                                                        .class_signal("btn-primary", not(page.show_continuous_input.signal()))
                                                                                        .class_signal("btn-secondary", page.show_continuous_input.signal())
                                                                                        .text_signal(page.show_continuous_input.signal_cloned().map(|show| {
                                                                                            if show {"Cancel"} else {"+Add"}
                                                                                        }))
                                                                                        .event(clone!(page => move |_: events::Click| {
                                                                                            page.edit_order.set(None);
                                                                                            page.offs_by_parent.lock_mut().clear();
                                                                                            page.show_oneday_input.set_neq(false);
                                                                                            page.show_continuous_input.set(!page.show_continuous_input.get());
                                                                                        }))
                                                                                    })
                                                                                })
                                                                            })))
                                                                        }))
                                                                        .child_signal(map_ref!{
                                                                            let is_today = page.current_is_today(),
                                                                            let show_input = page.show_continuous_input.signal() =>
                                                                            if is_ipd {*is_today} else {true} && *show_input
                                                                        }.map(clone!(app, page => move |show| {
                                                                            show.then(|| {
                                                                                let form = ContinuousForm::new(
                                                                                    page.edit_order.get_cloned(),
                                                                                    page.patient.clone(),
                                                                                    None,
                                                                                    app.user.lock_ref().as_ref().map(|u|u.user.doctorcode.get_cloned()).unwrap_or_default(),
                                                                                    page.view_by.clone(),
                                                                                    page.offs_by_parent.clone(),
                                                                                );
                                                                                ContinuousForm::render(
                                                                                    form,
                                                                                    page.show_continuous_input.clone(),
                                                                                    page.edit_order.clone(),
                                                                                    page.reload_order_continuous.clone(),
                                                                                    app.clone(),
                                                                                )
                                                                            })
                                                                        })))
                                                                    }),
                                                                ])
                                                            }))
                                                        }),
                                                    ])
                                                }))
                                            }))
                                        }))
                                    }))
                                }))
                            }))
                        }),
                    ])
                    .apply_if(!is_ipd, |dom| { dom
                        .child(html!("div", {
                            .class("row")
                            .child(html!("div", {
                                .class("col")
                                .child(html!("div", {
                                    .child(html!("table", {
                                        .class(class::TABLE_SM)
                                        .children([
                                            html!("caption", {
                                                .style("caption-side","top")
                                                .text("รายการยา OPD (HOSxP)")
                                                .text_signal(page.opd_meds.signal_vec_cloned().len().map(|len| {
                                                    [" [", &len.to_string(), " รายการ]"].concat()
                                                }))
                                            }),
                                            html!("thead", {
                                                .child(html!("tr", {
                                                    .children([
                                                        html!("th", {.attr("scope", "col").text("#")}),
                                                        html!("th", {.attr("scope", "col").text("ชื่อยา")}),
                                                        html!("th", {.attr("scope", "col").text("วิธีใช้")}),
                                                        html!("th", {.attr("scope", "col").class("text-end").text("จำนวน")}),
                                                        html!("th", {.attr("scope", "col").text("วัน-เวลา ที่บันทึก")}),
                                                    ])
                                                }))
                                            }),
                                            html!("tbody", {
                                                .children_signal_vec(page.opd_meds.signal_vec_cloned().enumerate().map(|(i,opd_med)| {
                                                    Self::render_opd_med(i.get().unwrap_or_default(), opd_med)
                                                }))
                                            }),
                                        ])
                                    }))
                                }))
                            }))
                        }))
                    })
                }),
                html!("div", {
                    .class("modal")
                    .attr("id", &["selectPreOrderModal", cpn_id].concat())
                    .attr("role", "dialog")
                    .attr("tabindex", "-1")
                    .child_signal(page.pre_order_select_modal.signal_cloned().map(clone!(app, page => move |opt| {
                        opt.as_ref().map(clone!(app, page => move |modal| {
                            PreOrderSelect::render(modal.clone(), page.pre_order_select_modal.clone(), Some(page.loaded_all.clone()), Some(page.loaded_pre_order_count.clone()), app)
                        })).or(Some(blank_modal()))
                    })))
                }),
                html!("div", {
                    .class("modal")
                    .attr("id", &["indexPlanActionFormModal", cpn_id].concat())
                    .attr("role", "dialog")
                    .attr("tabindex", "-1")
                    .child_signal(page.index_plan_action_modal.signal_cloned().map(clone!(app, page => move |opt| {
                        opt.as_ref().map(clone!(app, page => move |modal| {
                            let reload = if modal.is_continuous() {
                                page.reload_order_continuous.clone()
                            } else {
                                page.reload_order_oneday.clone()
                            };
                            IndexPlanActionForm::render(
                                modal.clone(),
                                page.index_plan_action_modal.clone(),
                                Some(reload),
                                app,
                            )
                        })).or(Some(blank_modal()))
                    })))
                }),
            ])
        })
    }

    // function oneday_data_to_text(one_day_order)
    // function continuous_data_to_text(continuous_order)
    // we use same code in both ipd_order.rs and opd_er_order.rs
    // TODO make this function generic (opd_er_order::OpdErOrderCpn::render_order())
    pub fn render_order(cpn_id: &'static str, order: Rc<Order>, is_oneday: bool, page: Rc<Self>, app: Rc<App>) -> Dom {
        let flags = OrderFlags::from_order(&order, &page, &app);

        let owner_class = if !flags.is_by_doctor && !flags.is_doctor_confirm { "text-bg-warning" } else { "bg-primary" };
        let order_time_mutable: Mutable<String> = Mutable::new(String::new());
        let due_mutables = DueMutables::new(order.clone());
        // START
        html!("div", {
            .visible_signal(not(page.edit_order.signal_cloned().map(clone!(order => move |opt| opt.as_ref().map(|edit| edit.order_id == order.order_id).unwrap_or_default()))))
            .attr("id", &["order_id_", &order.order_id.to_string(), "_div", cpn_id].concat())
            .class(class::BOX_ROUND_T)
            .style("position","relative")
            .apply_if(page.focused_id.get() == order.order_id, |dom| dom.class(class::BORDER3_RED))
            .class_signal("bg-warning-subtle", app.pharmacist_allow_non_med.signal().map(clone!(flags => move |pharmacist_allow_non_med| {
                flags.is_needed_before_pharmacist
                || flags.can_nurse_edit_as
                || flags.is_order_as_wait_for_doctor
                || (flags.can_pharmacist_accept && (pharmacist_allow_non_med || flags.is_pharm_notify || flags.is_med))
                || (flags.can_pharmacist_check && (pharmacist_allow_non_med || flags.is_med))
            })))
            .apply_if(flags.can_pharmacist_done, |dom| {
                dom.class("bg-info-subtle")
            })
            // DATETIME
            .child(html!("span", {
                .text(&[date_th(&order.order_date), time_hm(&order.order_time)].join(" "))
            }))
            // DOCTOR / NURSE
            .child(html!("span", {
                .class(class::BADGE_R)
                .class(owner_class)
                .style("cursor","default")
                .text(match order.order_owner_type.as_str() {
                    "doctor" => "Doctor",
                    "nurse" => "Nurse",
                    _ => "",
                })
            }))
            // PRE-ORDER
            .apply_if(order.pre_order_date.is_some(), |dom| {
                dom.child(html!("span", {
                    .class(class::BADGE_WRAP_R_GREEN)
                    .style("cursor","default")
                    .text("ล่วงหน้า")
                    .attr("info",&["บันทึกไว้เมื่อ: ", &date_th_opt(&order.pre_order_date), " ", &time_hm_opt(&order.pre_order_time)].concat())
                }))
            })
            .children([
                html!("div", {
                    .attr("id", &["order_id_", &order.order_id.to_string(), "_inner_div", cpn_id].concat())
                    .class(class::BORDER_T_Y)
                    .children({
                        let mut children = Vec::new();
                        order.order_item_types.iter().for_each(|order_item_type| {
                            // show order item type name
                            if ["pharmacist","other"].contains(&flags.view_by.as_str()) || (flags.is_doctor && is_oneday && matches!(order_item_type.order_item_type, OrderTypeName::HomeMedication | OrderTypeName::Discharge)) {
                                children.push(html!("div", {
                                    .class("fw-bold")
                                    .text(order_item_type.order_item_type.string())
                                }))
                            }
                            // ORDER ITEMS
                            let lis = order_item_type.order_items.iter().map(clone!(app, page, order, due_mutables, flags => move |order_item| {
                                Self::render_order_item(cpn_id, Rc::new(order_item.to_owned()), order.clone(), is_oneday, due_mutables.clone(), flags.clone(), page.clone(), app.clone())
                            })).collect::<Vec<Dom>>();
                            let list_tag = if order_item_type.is_homemed() {"ol"} else {"ul"};
                            let list = html!(list_tag, {
                                .class("dash")
                                .style("white-space","pre-wrap")
                                .children(lis)
                            });
                            children.push(list);
                        });
                        children
                    })
                }),
                // Order by - signer
                html!("div", {
                    .class(class::SMALL_R)
                    .apply_if(order.order_doctor_is_intern.unwrap_or_default(), |dom| dom.child(html!("span", {.text("(Intern) ")})))
                    .children([
                        html!("span", {.text(&[&order.order_doctor_name.clone().unwrap_or_default(), ", "].concat())}),
                        html!("span", {
                            .class("text-nowrap")
                            .text(&[date_th(&order.order_date), time_hm(&order.order_time)].join(" "))
                        })
                    ])
                }),
                // Order As - signer
                html!("div", {
                    .apply(|dom| {
                        if flags.can_nurse_edit_as {
                            let doctor_select_option = app.app_asset.lock_ref().as_ref().map(|asset| asset.doctor_select_option.clone()).unwrap_or_default();
                            let order_as = &order.nurse_order_as.clone().unwrap_or_default();
                            dom.class(class::INPUT_GROUP_SM).children([
                                doms::label_group_for(&["nurse_order_as", cpn_id].concat(),"รคส"),
                                html!("div", {
                                    .class(class::FLEX_GROW1)
                                    .child(html!("select" => HtmlSelectElement, {
                                        .class(class::FORM_CTRL_SM)
                                        .attr("id", &["nurse_order_as", cpn_id].concat())
                                        .child(html!("option", {
                                            .attr("value","")
                                            .text("กรุณาเลือก")
                                        }))
                                        .children(doctor_select_option.iter().map(|option| {
                                            doms::select_option(option, order_as)
                                        }))
                                        .prop_signal("value", page.nurse_order_as_result.signal_cloned())
                                        .with_node!(element => {
                                            .event(clone!(page => move |_: events::Change| {
                                                page.nurse_order_as_result.set_neq(element.value());
                                                page.changed.set_neq(true);
                                            }))
                                        })
                                    }))
                                }),
                                html!("button" => HtmlButtonElement, {
                                    .attr("type", "button")
                                    .class(class::BTN_SM_BLUE)
                                    .text("Edit")
                                    .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, page, order, order_time_mutable => move || {
                                        Self::patch_order(OrderPatchAction::EditAs, order.clone(), time_8601(&order_time_mutable.lock_ref()), None, is_oneday, page.clone(), app.clone());
                                    }), clone!(order => map_ref!{
                                        let changed = page.changed.signal(),
                                        let order_as = page.nurse_order_as_result.signal_cloned() =>
                                        !changed || (*changed && order.nurse_order_as.clone().map(|oas| oas == *order_as).unwrap_or(true))
                                    }), app.state()))
                                }),
                            ])
                        } else if let Some(nurse_order_as_name) = &order.nurse_order_as_name {
                            dom.class(class::SMALL_R)
                            .children([
                                html!("span", {.text(&["รคส.", if order.nurse_order_as_is_intern.unwrap_or_default() {"(Intern) "} else {""}, nurse_order_as_name].concat())}),
                                html!("span", {
                                    .class("text-nowrap")
                                    .apply(|d| {
                                        if let Some(doctor_confirm_time) = &order.doctor_confirm_time {
                                            d.text(&[" (ยืนยัน ", &datetime_th(doctor_confirm_time), ")"].concat())
                                        } else {
                                            d.text(" (รอแพทย์ยืนยัน)")
                                        }
                                    })
                                }),
                            ])
                        } else {
                            dom
                        }
                    })
                }),
                // Nurse Accept - signer
                html!("div", {
                    .apply_if(flags.is_nurse_accepted, |dom| {
                        dom.class(class::SMALL_R).children([
                            html!("span", {.text(&["(RN) ", &order.nurse_accept_name.clone().unwrap_or_default(), ", "].concat())}),
                            html!("span", {
                                .class("text-nowrap")
                                .text(&datetime_th_opt(&order.nurse_accept_time))
                            })
                        ])
                    })
                }),
                // Pharmacist Accept - signer
                html!("div", {
                    .apply_if(
                        (flags.is_pharmacist || (!flags.is_pharm_checked && !flags.is_pharm_done))
                        && flags.is_pharm_accepted,
                    |dom| {
                        let note = if flags.is_pharmacist {"(รับรายการ) "} else {"(ห้องยารับรายการ) "};
                        dom.class(class::SMALL_R).children([
                            html!("span", {.text(&[note, &order.pharmacist_accept_name.clone().unwrap_or_default(), ", "].concat())}),
                            html!("span", {
                                .class("text-nowrap")
                                .text(&datetime_th_opt(&order.pharmacist_accept_time))
                            })
                        ])
                    })
                }),
                // Pharmacist Check - signer
                html!("div", {
                    .apply_if(
                        (flags.is_pharmacist || !flags.is_pharm_done)
                        && flags.is_pharm_checked,
                    |dom| {
                        dom.class(class::SMALL_R).children([
                            html!("span", {.text(&["(ตรวจสอบ) ", &order.pharmacist_check_name.clone().unwrap_or_default(), ", "].concat())}),
                            html!("span", {
                                .class("text-nowrap")
                                .text(&datetime_th_opt(&order.pharmacist_check_time))
                            })
                        ])
                    })
                }),
                // Pharmacist Done - signer
                html!("div", {
                    .apply_if(flags.is_pharm_done, |dom| {
                        let done_job = if flags.is_pharmacist {"(จ่ายยา) "} else {"(RX) "};
                        dom.class(class::SMALL_R).children([
                            html!("span", {.text(&[done_job, &order.pharmacist_done_name.clone().unwrap_or_default(), ", "].concat())}),
                            html!("span", {
                                .class("text-nowrap")
                                .text(&datetime_th_opt(&order.pharmacist_done_time))
                            })
                        ])
                    })
                }),
            ])
            // custom order_time
            .child_signal(app.pharmacist_allow_non_med.signal().map(clone!(app, page, order, order_time_mutable, flags => move |pharmacist_allow_non_med| {
                (flags.is_needed_before_pharmacist
                    || (flags.can_pharmacist_accept && (pharmacist_allow_non_med || flags.is_pharm_notify || flags.is_med))
                    || (flags.can_pharmacist_check && (pharmacist_allow_non_med || flags.is_med))
                    || (flags.can_pharmacist_done && (pharmacist_allow_non_med || flags.is_med))
                ).then(|| {
                    html!("div", {
                        .class(class::FLEX_FIX_B2)
                        .child(html!("i", {
                            .class(class::FA_CLOCK)
                            .class("p-2")
                            .style("width","30px")
                            .event(clone!(order_time_mutable => move |_:events::Click| {
                                if order_time_mutable.get_cloned().is_empty() {
                                    order_time_mutable.set(js_now().time().js_string());
                                } else {
                                    order_time_mutable.set(String::new());
                                }
                            }))
                        }))
                        .child_signal(order_time_mutable.signal_cloned().map(clone!(order_time_mutable, order, flags => move |order_time| {
                            (!order_time.is_empty()).then(|| {
                                let min = if !order.is_confirm() {
                                    None
                                } else if let Some(max) = order.max_datetime(flags.is_pharmacist) {
                                    Some(doms::PickerConfigBuilder::default()
                                        .date_constraints(doms::DateConstraintsBuilder::default()
                                            .min_datetime(max)
                                            .build().unwrap()
                                        ).build().unwrap()
                                    )
                                } else {
                                    None
                                };
                                // can edit time only today
                                doms::time_picker(
                                    order_time_mutable.clone(),
                                    Mutable::new(false), always(false), Some(Mutable::new(js_now().date().to_string())),
                                    |d| d.style("max-width", "120px").style("min-width","95px"),
                                    |d| d.class("form-control-sm"),
                                    |d| d.class("form-control-sm"),
                                    clone!(order, flags => move |s| {
                                        if !order.is_confirm() {
                                            s
                                        } else if let (Some(max_dt), Some(t)) = (order.max_datetime(flags.is_pharmacist), time_8601(&s)) {
                                            if max_dt.date() == js_now().date() && t < max_dt.time() {
                                                (max_dt.time() + Duration::minutes(1)).js_string()
                                            } else {
                                                s
                                            }
                                        } else {
                                            s
                                        }
                                    }),
                                    always(min),
                                )
                            })
                        })))
                    })
                })
            })))
            .children([
                // Confirm/Edit/Delete Buttons
                html!("div", {
                    .apply_if(flags.can_change, |dom| {
                        dom.attr("id",&["order_id_", &order.order_id.to_string(), "_action_row_div", cpn_id].concat())
                        .class(class::BOLD_R)
                        // Confirm
                        .apply_if(flags.can_confirm, clone!(app, page, order, order_time_mutable, due_mutables, flags => move |d| {
                            d.child_signal(page.nurse_order_as_result.signal_ref(|order_as_result| !order_as_result.is_empty()).map(clone!(app, page, order, order_time_mutable, due_mutables, flags => move |has_order_as_result| {
                                (flags.is_by_doctor || has_order_as_result || flags.is_note || flags.is_pharm_notify).then(|| {
                                    html!("span", {
                                        .child_signal(due_mutables.changed.signal().map(clone!(app, page, order, due_mutables => move |is_changed| {
                                            is_changed.then(|| {
                                                html!("button" => HtmlButtonElement, {
                                                    .attr("type", "button")
                                                    .class(class::BTN_SM_RB_GRAY)
                                                    .text("ยกเลิกการพิจารณา")
                                                    .event(clone!(due_mutables => move |_:events::Click| {
                                                        due_mutables.changed.set_neq(false);
                                                        for due in due_mutables.items.iter() {
                                                            due.due_doctor.set(None);
                                                            due.due_doctor_note.set(None);
                                                        }
                                                    }))
                                                })
                                            })
                                        })))
                                        .child(html!("button" => HtmlButtonElement, {
                                            .attr("type", "button")
                                            .class(class::BTN_SM_RB_BLUE)
                                            .text("Confirm")
                                            .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, page, order, order_time_mutable, due_mutables, flags => move || {
                                                let patch = if flags.is_nurse && !flags.is_by_doctor && !flags.is_note && !flags.is_pharm_notify {OrderPatchAction::ConfirmAs} else {OrderPatchAction::Confirm};
                                                Self::patch_order(patch, order.clone(), time_8601(&order_time_mutable.lock_ref()), Some(due_mutables.clone()), is_oneday, page.clone(), app.clone());
                                            }),
                                            due_mutables.is_doctor_invalid_signal(),
                                            app.state()))
                                        }))
                                    })
                                })
                            })))
                        }))
                        // Edit
                        .apply_if(
                            flags.allow_order_form
                            && flags.allow_order_edit,
                        |d| {
                            d.child(html!("button", {
                                .attr("type", "button")
                                .class(class::BTN_SM_RB_GOLD)
                                .text("Edit")
                                .event(clone!(app, page, order => move |_: events::Click| {
                                    page.offs_by_parent.lock_mut().clear();
                                    page.edit_order.set(Some(order.clone()));
                                    if is_oneday {
                                        page.show_oneday_input.set_neq(false);
                                        page.show_oneday_input.set(true);
                                    } else {
                                        page.show_continuous_input.set_neq(false);
                                        page.show_continuous_input.set(true);
                                    }
                                    app.scroll_into_view(&["order_id_", &order.order_id.to_string(), "_div", cpn_id].concat());
                                }))
                            }))
                        })
                        // Delete
                        .apply_if(if page.is_ipd {
                            app.endpoint_is_allow(&Method::DELETE, &EndPoint::IpdOrderOrderId, flags.is_pre_admit)
                        } else {
                            app.endpoint_is_allow(&Method::DELETE, &EndPoint::OpdErOrderOrderId, false)
                        }, |d| {
                            d.child(html!("button" => HtmlButtonElement, {
                                .attr("type", "button")
                                .class(class::BTN_SM_RB_RED)
                                .text("Delete")
                                .apply(mixins::click_with_loader_checked(clone!(app, page, order => move || {
                                    Self::delete_order(order.order_id, is_oneday, page.clone(), app.clone());
                                }), app.state()))
                            }))
                        })
                    })
                }),
                // Doctor Confirm button
                html!("div", {
                    .apply_if(flags.can_doctor_confirm_order_as, |dom| dom
                        .class(class::BOLD_R)
                        .child(html!("button" => HtmlButtonElement, {
                            .attr("type", "button")
                            .class(class::BTN_SM_RB_BLUE)
                            .text("แพทย์ยืนยัน รคส")
                            .apply(mixins::click_with_loader_checked(clone!(app, page, order, order_time_mutable => move || {
                                Self::patch_order(OrderPatchAction::DoctorConfirm, order.clone(), time_8601(&order_time_mutable.lock_ref()), None, is_oneday, page.clone(), app.clone());
                            }), app.state()))
                        }))
                    )
                }),
                // Nurse Accept button
                html!("div", {
                    .apply_if(flags.can_nurse_accept, |dom| dom
                        .class(class::BOLD_R)
                        .child(html!("button" => HtmlButtonElement, {
                            .attr("type", "button")
                            .class(class::BTN_SM_RB_BLUE)
                            .text("พยาบาลรับรายการ")
                            .apply(mixins::click_with_loader_checked(clone!(app, page, order, order_time_mutable => move || {
                                Self::patch_order(OrderPatchAction::NurseAccept, order.clone(), time_8601(&order_time_mutable.lock_ref()), None, is_oneday, page.clone(), app.clone());
                            }), app.state()))
                        }))
                    )
                }),
            ])
            // Pharmacist Accept button
            .child_signal(app.pharmacist_allow_non_med.signal().map(clone!(app, page, order, order_time_mutable, due_mutables, flags => move |pharmacist_allow_non_med| {
                (flags.can_pharmacist_accept && (pharmacist_allow_non_med || flags.is_pharm_notify || flags.is_med)).then(|| {
                    html!("div", {
                        .class(class::BOLD_R)
                        .child_signal(due_mutables.changed.signal().map(clone!(app, page, order, due_mutables => move |is_changed| {
                            is_changed.then(|| {
                                html!("button" => HtmlButtonElement, {
                                    .attr("type", "button")
                                    .class(class::BTN_SM_RB_GRAY)
                                    .text("ยกเลิกการพิจารณา")
                                    .event(clone!(due_mutables => move |_:events::Click| {
                                        due_mutables.changed.set_neq(false);
                                        for due in due_mutables.items.iter() {
                                            due.due_pharm.set(None);
                                            due.due_pharm_note.set(None);
                                        }
                                    }))
                                })
                            })
                        })))
                        .child_signal(due_mutables.is_pharm_just_noted_signal().map(clone!(app, page, order, due_mutables => move |is_pharm_noted| {
                            is_pharm_noted.then(|| {
                                html!("button" => HtmlButtonElement, {
                                    .attr("type", "button")
                                    .class(class::BTN_SM_RB_CYAN)
                                    .text("บันทึกเฉพาะความเห็น")
                                    .apply(mixins::click_with_loader_checked(clone!(app, page, order, due_mutables => move || {
                                        Self::patch_due_pharm(due_mutables.clone(), order.clone(), page.clone(), app.clone());
                                    }), app.state()))
                                })
                            })
                        })))
                        .child(html!("button" => HtmlButtonElement, {
                            .attr("type", "button")
                            .class(class::BTN_SM_RB_BLUE)
                            .text("ห้องยารับรายการ")
                            .apply(|d| {
                                if page.is_ipd && !flags.is_pre_admit && order.need_medplan()
                                    && app.endpoint_is_allow(&Method::GET, &EndPoint::HisMedPlanIpdAn, false)
                                { d
                                    .attr("data-bs-toggle", "modal")
                                    .attr("data-bs-target", &["#medPlanFormModal", cpn_id].concat())
                                    .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, page, order, order_time_mutable, due_mutables => move || {
                                        // send DUE patch
                                        Self::patch_due_pharm(due_mutables.clone(), order.clone(), page.clone(), app.clone());
                                        // prepare to open MedPlanForm
                                        let an = order.visit_type.vnan();
                                        let order_doctor = order.order_doctor.clone();
                                        let mut off_icodes = Vec::new();
                                        let mut medplans = Vec::new();
                                        let hosxp_medrec_icode = app.hosxp_medrec_icode();
                                        for oit in order.order_item_types.iter() {
                                            let (need_medplan, is_off) = oit.order_item_type.need_medplan_and_off();
                                            if is_off {
                                                off_icodes.extend(oit.order_items.iter().filter_map(|oi| OffOrderItem::from_order_item(oi, &hosxp_medrec_icode)));
                                            }
                                            if need_medplan {
                                                medplans.extend(oit.order_items.iter().filter_map(|oi| {
                                                    oi.icode.clone().map(|icode| {
                                                        MedPlanMutable::new(MedPlanItem {
                                                            order_item_id: oi.order_item_id,
                                                            an: an.clone(),
                                                            order_date: oi.order_date,
                                                            order_time: time_8601(&order_time_mutable.lock_ref()).or(oi.order_time),
                                                            order_type: oi.order_type.clone(),
                                                            order_item_type: oi.order_item_type.clone(),
                                                            med_name: oi.med_name.clone(),
                                                            order_item_detail: oi.order_item_detail.clone(),
                                                            icode: Some(icode),
                                                            order_doctor: order_doctor.clone(),
                                                            stat: oi.stat.clone(),
                                                            med_reconciliation_item_id: oi.med_reconciliation_item_id,
                                                            first_qty: oi.first_qty,
                                                            qty: oi.qty,
                                                        })
                                                    })
                                                }).collect::<Vec<Rc<MedPlanMutable>>>())
                                            }
                                        }
                                        page.off_icodes.set(off_icodes);
                                        {
                                            let mut mps_lock = page.medplans.lock_mut();
                                            mps_lock.replace_cloned(medplans);
                                        }
                                        page.medplan_form_modal.set(Some(MedPlanForm::new(
                                            order.clone(),
                                            page.clone(),
                                            order_time_mutable.clone(),
                                            is_oneday,
                                        )));
                                    }),
                                    due_mutables.is_pharm_invalid_signal(),
                                    app.state()))
                                } else { d
                                    .apply(mixins::click_with_loader_checked(clone!(app, page, order, order_time_mutable => move || {
                                        OrderCpn::patch_order(OrderPatchAction::PharmacistAccept, order.clone(), time_8601(&order_time_mutable.lock_ref()), None, is_oneday, page.clone(), app.clone());
                                    }), app.state()))
                                }
                            })
                        }))
                    })
                })
            })))
            // Pharmacist Check button
            .child_signal(app.pharmacist_allow_non_med.signal().map(clone!(app, page, order, order_time_mutable, flags => move |pharmacist_allow_non_med| {
                (flags.can_pharmacist_check && (pharmacist_allow_non_med || flags.is_med)).then(|| {
                    html!("div", {
                        .class(class::BOLD_R)
                        .child(html!("button" => HtmlButtonElement, {
                            .attr("type", "button")
                            .class(class::BTN_SM_RB_BLUE)
                            .text("ห้องยาตรวจสอบ")
                            .apply(mixins::click_with_loader_checked(clone!(app, page, order, order_time_mutable => move || {
                                Self::patch_order(OrderPatchAction::PharmacistCheck, order.clone(), time_8601(&order_time_mutable.lock_ref()), None, is_oneday, page.clone(), app.clone());
                            }), app.state()))
                        }))
                    })
                })
            })))
            // Pharmacist Done button
            .child_signal(app.pharmacist_allow_non_med.signal().map(clone!(app, page, order, order_time_mutable, flags => move |pharmacist_allow_non_med| {
                (flags.can_pharmacist_done && (pharmacist_allow_non_med || flags.is_med)).then(|| {
                    html!("div", {
                        .class(class::BOLD_R)
                        .child(html!("button" => HtmlButtonElement, {
                            .attr("type", "button")
                            .class(class::BTN_SM_RB_BLUE)
                            .text("ห้องยาจ่ายยา")
                            .apply(mixins::click_with_loader_checked(clone!(app, page, order, order_time_mutable => move || {
                                Self::patch_order(OrderPatchAction::PharmacistDone, order.clone(), time_8601(&order_time_mutable.lock_ref()), None, is_oneday, page.clone(), app.clone());
                            }), app.state()))
                        }))
                    })
                })
            })))
            .child(html!("div", {
                .class("modal")
                .attr("id", &["medPlanFormModal", cpn_id].concat())
                .attr("role", "dialog")
                .attr("tabindex", "-1")
                .child_signal(page.medplan_form_modal.signal_cloned().map(clone!(app => move |opt| {
                    opt.as_ref().map(clone!(app => move |modal| MedPlanForm::render(modal.clone(), app))).or(Some(blank_modal()))
                })))
            }))
        })
    }

    fn render_order_item(cpn_id: &'static str, order_item: Rc<OrderItem>, order: Rc<Order>, is_oneday: bool, due_mutables: Rc<DueMutables>, flags: Rc<OrderFlags>, page: Rc<Self>, app: Rc<App>) -> Dom {
        let is_due = order_item.due_status.as_ref().map(|due_status| due_status == "Y").unwrap_or_default();
        let has_info = order_item.info_status.as_ref().map(|info_status| info_status == "Y").unwrap_or_default();
        let will_blue = if is_oneday {
            vec!["med", "home-medication", "injection", "ivfluid"]
        } else {
            vec!["med", "injection", "ivfluid"]
        };

        html!("li", {
            .class("clearfix")
            .apply(|dom| {
                // OFF
                if order_item.order_item_type == Some(String::from("off")) { dom
                    .child(html!("span", {.class(class::BADGE_GOLD_L).style("cursor","default").text("OFF")}))
                    .child(html!("span", {
                        .child(html!("span", {
                            .class(class::BOLD_BLUE_EM_L)
                            .text(&order_item.off_med_name.clone().unwrap_or_default())
                            .apply_if(flags.is_pharmacist, |d| { d
                                .style("cursor","copy")
                                .event(clone!(app, order_item => move |_:events::Click| {
                                    spawn_local(clone!(app, order_item => async move {
                                        app.set_clipboard(&order_item.off_med_name.clone().unwrap_or_default()).await;
                                    }));
                                }))
                            })
                        }))
                        .apply_if(order_item.off_med_name.is_some(), |d| d.child(html!("br")))
                        .text(&order_item.off_order_item_detail.clone().unwrap_or_default())
                    }))
                // NOT OFF
                } else { dom
                    // OFF BUTTON
                    .apply_if(
                        (flags.is_doctor || flags.is_nurse)
                        && flags.is_confirm
                        && !flags.is_readonly
                        && order_item.off_by_datetime.is_none()
                        && flags.allow_order_form
                        && flags.allow_order_edit
                        && if page.is_ipd {
                            flags.is_today && (app.has_permission(Permission::IpdOrderOff) || flags.is_pre_admit)
                        } else {
                            app.has_permission(Permission::OpdErOrderOff)
                        },
                    |dom| dom.child(html!("button", {
                        .attr("type", "button")
                        .class(class::BTN_SM_FR_RT)
                        .class_signal("btn-outline-primary", page.offs_by_parent.signal_vec_cloned().filter(clone!(order_item => move |off| {
                            off.off_order_item_id.get().map(|id| id == order_item.order_item_id).unwrap_or_default()
                        })).is_empty())
                        .class_signal("btn-warning", not(page.offs_by_parent.signal_vec_cloned().filter(clone!(order_item => move |off| {
                            off.off_order_item_id.get().map(|id| id == order_item.order_item_id).unwrap_or_default()
                        })).is_empty()))
                        .text("Off")
                        .event(clone!(page, order_item => move |_: events::Click| {
                            if is_oneday {
                                page.show_oneday_input.set_neq(true);
                                if page.show_continuous_input.get() {
                                    page.offs_by_parent.lock_mut().clear();
                                    page.show_continuous_input.set(false);
                                }
                            } else {
                                page.show_continuous_input.set_neq(true);
                                if page.show_oneday_input.get() {
                                    page.offs_by_parent.lock_mut().clear();
                                    page.show_oneday_input.set(false);
                                }
                            }
                            let is_offed = page.offs_by_parent.lock_ref().iter().any(|off| {
                                off.off_order_item_id.get().map(|id| id == order_item.order_item_id).unwrap_or_default()
                            });
                            if is_offed {
                                page.offs_by_parent.lock_mut().retain(|off| off.off_order_item_id.get().map(|id| id != order_item.order_item_id).unwrap_or_default());
                            } else {
                                let med_name_opt = &order_item.med_name;
                                let new_line = if med_name_opt.is_some() {"\n"} else {""};
                                let detail = order_item.order_item_detail.clone().unwrap_or_default();
                                let order_item_mut = OrderItemMutable::new("off", None);
                                order_item_mut.order_item_detail.set([&med_name_opt.clone().unwrap_or_default(), new_line, &detail].concat());
                                order_item_mut.off_order_item_id.set(Some(order_item.order_item_id));
                                page.offs_by_parent.lock_mut().push_cloned(order_item_mut);
                            }
                            page.edit_order.set(None);
                        }))
                    })))
                    // DUE button
                    .apply(|dom| {
                        if is_due && (flags.is_doctor || flags.is_nurse || flags.is_pharmacist) {
                            if let Some(due) = due_mutables.items.iter().find(|dm| dm.order_item_id == order_item.order_item_id) {
                                dom.child_signal(due.due_doctor.signal_cloned().map(clone!(app, due => move |opt| {
                                    opt.is_some().then(|| {
                                        let show_modal = Mutable::new(false);
                                        html!("button", {
                                            .attr("type","button")
                                            .class(class::BTN_SM_FR_RT)
                                            .class(due.btn_color())
                                            .text("DUE")
                                            .event(clone!(show_modal => move |_:events::Click| {
                                                show_modal.set(true);
                                            }))
                                            .future(show_modal.signal().for_each(clone!(app, due, show_modal => move |is_show| {
                                                clone!(app, due, show_modal => async move {
                                                    if is_show {
                                                        let content = html!("div", {
                                                            .class("p-2")
                                                            .style("white-space","pre-wrap")
                                                            .children(doms::square_bracket_to_span(&due.due_usage.clone().unwrap_or_default()))
                                                            .children([
                                                                html!("hr", {.class("my-2")}),
                                                                html!("div", {
                                                                    .children([
                                                                        html!("span", {
                                                                            .class(class::BOLD_RED)
                                                                            .text("ความเห็นแพทย์ : ")
                                                                        }),
                                                                        html!("span", {
                                                                            .class("fw-bold")
                                                                            .apply(|d| {
                                                                                if due.due_doctor.lock_ref().as_ref().map(|s| s == "Y").unwrap_or_default() {
                                                                                    d.class("text-success").text("ตรงตามเกณฑ์")
                                                                                } else {
                                                                                    d.class("text-danger").text(&due.due_doctor_note.get_cloned().unwrap_or_default())
                                                                                }
                                                                            })
                                                                        }),
                                                                    ])
                                                                }),
                                                                html!("div", {
                                                                    .children([
                                                                        html!("span", {
                                                                            .class(class::BOLD_RED)
                                                                            .text("ความเห็นเภสัชกร : ")
                                                                        }),
                                                                        html!("span", {
                                                                            .class("fw-bold")
                                                                            .apply(|d| {
                                                                                match due.due_pharm.lock_ref().as_ref() {
                                                                                    Some(due_pharm) => {
                                                                                        if due_pharm.as_str() == "Y" {
                                                                                            d.class("text-success").text("ตรงตามเกณฑ์")
                                                                                        } else {
                                                                                            d.class("text-danger").text(&due.due_pharm_note.get_cloned().unwrap_or_default())
                                                                                        }
                                                                                    }
                                                                                    None => {
                                                                                        d.class("text-info").text("รอเภสัชกรให้ความเห็น")
                                                                                    }
                                                                                }
                                                                            })
                                                                        }),
                                                                    ])
                                                                }),
                                                            ])
                                                        });
                                                        app.dom_with_close("Drug Utilization Evaluation : DUE", content, false).await;
                                                        show_modal.set(false);
                                                    }
                                                })
                                            })))
                                        })
                                    })
                                })))
                            } else {
                                dom
                            }
                        } else {
                            dom
                        }
                    })
                    // ADDICT PDF BUTTON
                    .apply_if(order_item.addict_type_id.map(|id| id == 2).unwrap_or_default(), |dom| dom
                        .child(html!("div", {
                            .class(class::FLOAT_RB1)
                            .child_signal(page.patient.signal_cloned().map(clone!(app, order, order_item => move |opt| {
                                opt.map(|patient| {
                                    static_pdf_btn_with_modal(
                                        "ย.ส.2",
                                        "ใบสั่งจ่ายยาเสพติดให้โทษในประเภท 2",
                                        include_str!("../../../volume/pwa/templates/statics/addict-habit-forming-order.typ"),
                                        serde_json::json!({
                                            "is_addict": true,
                                            "patient": patient,
                                            "order_doctor_name": order.order_doctor_name,
                                            "order_doctor_licenseno": order.order_doctor_licenseno,
                                            "order_item": order_item,
                                        }).to_string(),
                                        app.clone(),
                                    )
                                })
                            })))
                        }))
                    )
                    // HABIT-FORMING PDF BUTTON
                    .apply_if(order_item.habit_forming_type.map(|id| id == 2).unwrap_or_default(), |dom| dom
                        .child(html!("div", {
                            .class(class::FLOAT_RB1)
                            .child_signal(page.patient.signal_cloned().map(clone!(app, order, order_item => move |opt| {
                                opt.map(|patient| {
                                    static_pdf_btn_with_modal(
                                        "ว.จ.2",
                                        "ใบสั่งจ่ายวัตถุออกฤทธิ์ในประเภท 2",
                                        include_str!("../../../volume/pwa/templates/statics/addict-habit-forming-order.typ"),
                                        serde_json::json!({
                                            "is_addict": false,
                                            "patient": patient,
                                            "order_doctor_name": order.order_doctor_name,
                                            "order_doctor_licenseno": order.order_doctor_licenseno,
                                            "order_item": order_item,
                                        }).to_string(),
                                        app.clone(),
                                    )
                                })
                            })))       
                        }))
                    )
                    // +PLAN BUTTON
                    .apply_if(
                        !flags.is_readonly
                        && flags.is_nurse
                        && flags.is_confirm
                        && (flags.is_nurse_accepted || !flags.is_by_doctor)
                        // && order_item.off_by_datetime.is_none()
                        && if page.is_ipd {
                            app.endpoint_is_allow(&Method::GET, &EndPoint::IpdOrderItem, flags.is_pre_admit)
                        } else {
                            app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErOrderItem, false)
                        },
                    clone!(page, order, order_item => move |dom| dom.child(html!("button", {
                        .attr("type", "button")
                        .class(class::BTN_SM_FR_T_BLUEO)
                        .attr("data-bs-toggle", "modal")
                        .attr("data-bs-target", &["#indexPlanActionFormModal", cpn_id].concat())
                        .text("+Plan")
                        .event(clone!(page, order_item => move |_: events::Click| {
                            page.index_plan_action_modal.set(Some(IndexPlanActionForm::new(
                                order_item.order_item_id,
                                None,
                                None,
                                page.patient.clone(),
                                OrderType::new_from_str(&order.as_ref().order_type),
                                FormType::Plan,
                                page.view_by.clone(),
                            )));
                        }))
                        // ipd-nurse-index-plan-action-form.php::onclickAddIndexPlanOrderItem(event, order_item.order_item_id, order_item.order_item_detail);
                    }))))
                    // MED NAME
                    .child(html!("span", {
                        .child(html!("span", {
                            .apply_if(order_item.order_item_type.as_ref().map(|ty| will_blue.contains(&ty.as_str())).unwrap_or_default(), |d| d.class(class::BOLD_BLUE_EM))
                            .apply_if(order_item.med_name.as_ref().map(|s| !s.is_empty()).unwrap_or_default(), |d| d.class("me-1"))
                            .text(&order_item.med_name.clone().unwrap_or_default())
                            .apply_if(order_item.order_item_type == Some(String::from("home-medication")), |d| d.text(" #").text(&order_item.first_qty.unwrap_or_default().to_string()))
                            .apply_if(flags.is_pharmacist, |d| { d
                                .style("cursor","copy")
                                .event(clone!(app, order_item => move |_:events::Click| {
                                    spawn_local(clone!(app, order_item => async move {
                                        app.set_clipboard(&order_item.med_name.clone().unwrap_or_default()).await;
                                    }));
                                }))
                            })
                        }))
                        // DRUG ALLERGY BADGE
                        .apply_if(order_item.allergy_agent_symptom.is_some(), |d| { d
                            .child(html!("span", {
                                .class(class::BADGE_WRAP_R_RED)
                                .style("cursor","help")
                                .attr("title", &order_item.allergy_agent_symptom.clone().unwrap_or(String::from("ไม่ระบุอาการ")))
                                .text("แพ้ยา/เฝ้าระวัง")
                            }))
                        })
                        // HAD/LASA BADGE
                        .children(app.drug_alert_badge(order_item.displaycolor))
                        // MED RECONCILE BADGE
                        .apply_if(order_item.med_reconciliation_item_id.is_some(), |d| d.child(html!("span", {
                            .style("cursor","help")
                            .apply(|dd| {
                                match &order_item.used {
                                    Some(used) => {
                                        match used.as_str() {
                                            "N" => dd.class(class::BADGE_WRAP_R_GRAY),
                                            "H" => dd.class(class::BADGE_WRAP_R_CYAN),
                                            "Y" => {
                                                if order_item.is_med_rec_change_usage() {
                                                    dd.class(class::BADGE_WRAP_R_GOLD)
                                                } else {
                                                    dd.class(class::BADGE_WRAP_R_GREEN)
                                                }
                                            }
                                            _ => dd,
                                        }
                                    }
                                    None => dd,
                                }

                            })
                            .attr("title", &order_item.med_rec_info())
                            .text("MR")
                        })))
                        .apply_if(order_item.med_name.is_some(), |d| d.child(html!("br")))
                        // ORDER DETAIL
                        .text(&order_item.order_item_detail.clone().unwrap_or_default())
                        .apply_if(order_item.off_by_datetime.is_some(), |d| d.style("text-decoration","line-through"))
                    }))
                    // STAT BADGE
                    .apply_if(order_item.stat == Some(String::from("Y")), |dom| dom.child(html!("span", {
                        .class(class::BADGE_WRAP_R_RED)
                        .style("cursor","default")
                        .text("STAT")
                    })))
                    // OFF DATETIME BADGE
                    .apply_if(order_item.off_by_datetime.is_some(), |dom| dom.child(html!("span", {
                        .class(class::BADGE_WRAP_R_GOLD)
                        .style("cursor","default")
                        .text(&["OFF ", &datetime_th_opt(&order_item.off_by_datetime)].concat())
                    })))
                    // PLAN / ACTION
                    .apply_if(flags.is_doctor || flags.is_nurse || flags.is_pharmacist, |dom| {
                        dom.children(order_item.index_plans.iter().filter_map(clone!(page, order, order_item, flags => move |plan| Self::render_index_plan_badge(
                            cpn_id,
                            plan,
                            &order_item,
                            page.current_date.lock_ref().as_ref().map(|od| od.order_date),
                            OrderType::new_from_str(&order.order_type),
                            flags.is_nurse || flags.is_pharmacist,
                            !flags.is_readonly && flags.is_nurse,
                            page.clone(),
                        ))))
                    })
                    // DUE/Info BOX
                    .apply(|dom| {
                        if is_due && (flags.is_doctor || flags.is_nurse || flags.is_pharmacist) {
                            if let Some(due) = due_mutables.items.iter().find(|dm| dm.order_item_id == order_item.order_item_id) {
                                // When confirm
                                if flags.can_change && flags.can_confirm {
                                    dom.child_signal(map_ref!{
                                        let is_invalid = due.is_doctor_invalid_signal(),
                                        let has_order_as_result = page.nurse_order_as_result.signal_ref(|order_as_result| !order_as_result.is_empty()) =>
                                        *is_invalid && (flags.is_by_doctor || *has_order_as_result)
                                    }.map(clone!(due_mutables, due => move |is_invalid| {
                                        is_invalid.then(|| {
                                            html!("div", {
                                                .class(class::BORDER_SMALL_BG_RED)
                                                .style("white-space","pre-wrap")
                                                .children(doms::square_bracket_to_span(&due.due_usage.clone().unwrap_or_default()))
                                                .child(html!("hr", {.class("my-2")}))
                                                .child_signal(due.due_doctor.signal_cloned().map(clone!(due_mutables, due => move |due_doctor| {
                                                    due_doctor.is_none().then(|| {
                                                        html!("div", {
                                                            .children([
                                                                html!("button", {
                                                                    .attr("type","button")
                                                                    .class(class::BTN_SM_L_BLUE)
                                                                    .text("ตรงตามเกณฑ์")
                                                                    .event(clone!(due_mutables, due => move |_:events::Click| {
                                                                        due.due_doctor.set(Some(String::from("Y")));
                                                                        due_mutables.changed.set(true);
                                                                    }))
                                                                }),
                                                                html!("button", {
                                                                    .attr("type","button")
                                                                    .class(class::BTN_SM_RED)
                                                                    .text("มีความจำเป็น")
                                                                    .event(clone!(due_mutables, due => move |_:events::Click| {
                                                                        due.due_doctor.set(Some(String::from("N")));
                                                                        due_mutables.changed.set(true);
                                                                    }))
                                                                }),
                                                            ])
                                                        })
                                                    })
                                                })))
                                                .child_signal(due.due_doctor.signal_cloned().map(clone!(due_mutables, due => move |due_doctor| {
                                                    due_doctor.as_ref().map(|s| s != "Y").unwrap_or_default().then(|| {
                                                        html!("div", {
                                                            .children([
                                                                html!("span", {
                                                                    .class(class::BOLD_RED)
                                                                    .text("โปรดระบุ เหตุผลการสั่งใช้ยา หรือลบรายการ หากไม่สั่งใช้ยา")
                                                                }),
                                                                html!("textarea" => HtmlTextAreaElement, {
                                                                    .class("w-100")
                                                                    .apply(mixins::textarea_value_auto_expand(due.due_doctor_note_temp.clone(), due.due_doctor_note_changed.clone()))
                                                                }),
                                                                html!("div", {
                                                                    .children([
                                                                        html!("button", {
                                                                            .attr("type","button")
                                                                            .class(class::BTN_SM_L_BLUE)
                                                                            .text("บันทึกเหตุผลการสั่งใช้ยา")
                                                                            .visible_signal(due.due_doctor_note_changed.signal())
                                                                            .event(clone!(due_mutables, due => move |_:events::Click| {
                                                                                due.due_doctor_note.set(Some(due.due_doctor_note_temp.get_cloned()));
                                                                                due.due_doctor_note_changed.set(false);
                                                                                due_mutables.changed.set(true);
                                                                            }))
                                                                        }),
                                                                        html!("button", {
                                                                            .attr("type","button")
                                                                            .class(class::BTN_SM_GRAY)
                                                                            .text("ยกเลิก")
                                                                            .event(clone!(due_mutables, due => move |_:events::Click| {
                                                                                due.due_doctor.set(None);
                                                                                due.due_doctor_note_temp.set_neq(String::new());
                                                                                due.due_doctor_note_changed.set_neq(false);
                                                                                due_mutables.changed.set(true);
                                                                            }))
                                                                        }),
                                                                    ])
                                                                })
                                                            ])
                                                        })
                                                    })
                                                })))
                                            })
                                        })
                                    })))
                                // When pharmacist accept
                                } else if flags.can_pharmacist_accept && flags.is_med {
                                    dom.child_signal(due.is_pharm_invalid_signal().map(clone!(due_mutables, due => move |is_invalid| {
                                        is_invalid.then(|| {
                                            html!("div", {
                                                .class(class::BORDER_SMALL_BG_RED)
                                                .style("white-space","pre-wrap")
                                                .children(doms::square_bracket_to_span(&due.due_usage.clone().unwrap_or_default()))
                                                .child(html!("hr", {.class("my-2")}))
                                                .child_signal(due.is_doctor_noted_signal().map(clone!(due_mutables, due => move |is_noted| {
                                                    is_noted.then(|| {
                                                        html!("div", {
                                                            .children([
                                                                html!("span", {
                                                                    .class(class::BOLD_RED)
                                                                    .text("ความเห็นแพทย์ : ")
                                                                }),
                                                                html!("span", {
                                                                    .class("fw-bold")
                                                                    .text_signal(due.due_doctor_note.signal_cloned().map(|s| s.unwrap_or_default()))
                                                                }),
                                                                html!("hr", {.class("my-2")})
                                                            ])
                                                        })
                                                    })
                                                })))
                                                .child_signal(due.due_pharm.signal_cloned().map(clone!(due_mutables, due => move |due_pharm| {
                                                    due_pharm.is_none().then(|| {
                                                        html!("div", {
                                                            .children([
                                                                html!("button", {
                                                                    .attr("type","button")
                                                                    .class(class::BTN_SM_L_BLUE)
                                                                    .text("ตรงตามเกณฑ์ / ยอมรับเหตุผล")
                                                                    .event(clone!(due_mutables, due => move |_:events::Click| {
                                                                        due.due_pharm.set(Some(String::from("Y")));
                                                                        due_mutables.changed.set(true);
                                                                    }))
                                                                }),
                                                                html!("button", {
                                                                    .attr("type","button")
                                                                    .class(class::BTN_SM_RED)
                                                                    .text("มีข้อเสนอแนะเพื่อพิจารณา")
                                                                    .event(clone!(due_mutables, due => move |_:events::Click| {
                                                                        due.due_pharm.set(Some(String::from("N")));
                                                                        due_mutables.changed.set(true);
                                                                    }))
                                                                }),
                                                            ])
                                                        })
                                                    })
                                                })))
                                                .child_signal(due.due_pharm.signal_cloned().map(clone!(due_mutables, due => move |due_pharm| {
                                                    due_pharm.as_ref().map(|s| s != "Y").unwrap_or_default().then(|| {
                                                        html!("div", {
                                                            .children([
                                                                html!("span", {
                                                                    .class(class::BOLD_RED)
                                                                    .text("ข้อความที่ต้องการแจ้งแพทย์เพื่อพิจารณา")
                                                                }),
                                                                html!("textarea" => HtmlTextAreaElement, {
                                                                    .class("w-100")
                                                                    .apply(mixins::textarea_value_auto_expand(due.due_pharm_note_temp.clone(), due.due_pharm_note_changed.clone()))
                                                                }),
                                                                html!("div", {
                                                                    .children([
                                                                        html!("button", {
                                                                            .attr("type","button")
                                                                            .class(class::BTN_SM_L_BLUE)
                                                                            .text("บันทึกข้อความ")
                                                                            .visible_signal(due.due_pharm_note_changed.signal())
                                                                            .event(clone!(due_mutables, due => move |_:events::Click| {
                                                                                due.due_pharm_note.set(Some(due.due_pharm_note_temp.get_cloned()));
                                                                                due.due_pharm_note_changed.set(false);
                                                                                due_mutables.changed.set(true);
                                                                            }))
                                                                        }),
                                                                        html!("button", {
                                                                            .attr("type","button")
                                                                            .class(class::BTN_SM_GRAY)
                                                                            .text("ยกเลิก")
                                                                            .event(clone!(due_mutables, due => move |_:events::Click| {
                                                                                due.due_pharm.set(None);
                                                                                due.due_pharm_note_temp.set_neq(String::new());
                                                                                due.due_pharm_note_changed.set_neq(false);
                                                                                due_mutables.changed.set(true);
                                                                            }))
                                                                        }),
                                                                    ])
                                                                })
                                                            ])
                                                        })
                                                    })
                                                })))
                                            })
                                        })
                                    })))
                                } else {
                                    dom
                                }
                            } else {
                                dom
                            }
                        } else if has_info && (
                            ((flags.is_doctor || flags.is_nurse) && flags.can_change && flags.can_confirm)
                            || (flags.is_nurse && flags.can_nurse_accept)
                            || (flags.is_pharmacist && (flags.can_pharmacist_accept || flags.can_pharmacist_check || flags.can_pharmacist_done))
                        ) {
                            dom.child(html!("div", {
                                .class(if flags.is_pharmacist && flags.can_pharmacist_done {class::BORDER_SMALL_BG_GOLD} else {class::BORDER_SMALL_BG_CYAN})
                                .style("white-space","pre-wrap")
                                .children(doms::square_bracket_to_span(&order_item.info.clone().unwrap_or_default()))
                            }))
                        } else {
                            dom
                        }
                    })
                }
            })
        })
    }

    fn render_previous_order(cpn_id: &'static str, order_item: &OrderItem, order_type: OrderType, page: Rc<Self>, app: Rc<App>) -> Dom {
        let is_readonly = page.is_readonly();
        let is_pre_admit = page.patient.lock_ref().as_ref().map(|pt| pt.visit_type.is_pre_admit()).unwrap_or_default();
        let is_today = page.is_today();
        let is_oneday = order_item.order_type.clone().unwrap_or_default().as_str() == "oneday";
        let will_blue = if is_oneday {
            vec!["med", "home-medication", "injection", "ivfluid"]
        } else {
            vec!["med", "injection", "ivfluid"]
        };

        let view_by = page.view_by.get_cloned();
        let is_doctor = view_by.as_str() == "doctor" && app.has_permission(Permission::DataTypeDoctorUse);
        let is_nurse = view_by.as_str() == "nurse" && app.has_permission(Permission::DataTypeNurseUse);
        let is_pharmacist = view_by.as_str() == "pharmacist" && app.has_permission(Permission::DataTypePharmacyUse);

        let allow_order_form = if page.is_ipd {
            app.endpoint_is_allow(&Method::POST, &EndPoint::IpdOrderOrder, is_pre_admit)
        } else {
            app.endpoint_is_allow(&Method::POST, &EndPoint::OpdErOrderOrder, false)
        };
        let allow_order_edit = if page.is_ipd {
            app.has_permission(Permission::IpdOrderEdit)
        } else {
            app.has_permission(Permission::OpdErOrderEdit)
        };

        html!("li", {
            .class("clearfix")
            .apply_if(
                !is_readonly
                && (is_doctor || is_nurse)
                && is_today
                && order_item.off_by_datetime.is_none()
                && allow_order_form
                && allow_order_edit
                && (app.has_permission(Permission::IpdOrderOff) || is_pre_admit),
            clone!(page, order_item, order_type => move |dom| dom.child(html!("button", {
                .attr("type", "button")
                .class(class::BTN_SM_FR_RT)
                .class_signal("btn-outline-primary", page.offs_by_parent.signal_vec_cloned().filter(clone!(order_item => move |off| {
                    off.off_order_item_id.get().map(|id| id == order_item.order_item_id).unwrap_or_default()
                })).is_empty())
                .class_signal("btn-warning", not(page.offs_by_parent.signal_vec_cloned().filter(clone!(order_item => move |off| {
                    off.off_order_item_id.get().map(|id| id == order_item.order_item_id).unwrap_or_default()
                })).is_empty()))
                .text("Off")
                .event(clone!(page, order_item, order_type => move |_: events::Click| {
                    if matches!(order_type, OrderType::OneDay) {
                        page.show_oneday_input.set_neq(true);
                        if page.show_continuous_input.get() {
                            page.offs_by_parent.lock_mut().clear();
                            page.show_continuous_input.set(false);
                        }
                    } else {
                        page.show_continuous_input.set_neq(true);
                        if page.show_oneday_input.get() {
                            page.offs_by_parent.lock_mut().clear();
                            page.show_oneday_input.set(false);
                        }
                    }
                    let is_offed = page.offs_by_parent.lock_ref().iter().any(|off| {
                        off.off_order_item_id.get().map(|id| id == order_item.order_item_id).unwrap_or_default()
                    });
                    if is_offed {
                        page.offs_by_parent.lock_mut().retain(|off| off.off_order_item_id.get().map(|id| id != order_item.order_item_id).unwrap_or_default());
                    } else {
                        let med_name_opt = &order_item.med_name;
                        let new_line = if med_name_opt.is_some() {"\n"} else {""};
                        let detail = order_item.order_item_detail.clone().unwrap_or_default();
                        let order_item_mut = OrderItemMutable::new("off", None);
                        order_item_mut.order_item_detail.set([&med_name_opt.clone().unwrap_or_default(), new_line, &detail].concat());
                        order_item_mut.off_order_item_id.set(Some(order_item.order_item_id));
                        page.offs_by_parent.lock_mut().push_cloned(order_item_mut);
                    }
                    page.edit_order.set(None);
                    // onclickOffContinuousOrderItem(event, order_item.order_item_id, (order_item.icode == null ? '' : (order_item.med_name + (order_item.order_item_detail != '' ? '\n' : ''))) + order_item.order_item_detail);
                }))
            }))))
            .apply_if(
                !is_readonly
                && is_nurse
                // && order_item.off_by_datetime.is_none()
                // && ["doctor","nurse"].contains(&order_item.order_owner_type.clone().unwrap_or_default().as_str())
                && app.endpoint_is_allow(&Method::GET, &EndPoint::IpdOrderItem, is_pre_admit),
            clone!(page, order_item, order_type => move |dom| dom.child(html!("button", {
                .attr("type", "button")
                .class(class::BTN_SM_FR_T_BLUEO)
                .attr("data-bs-toggle", "modal")
                .attr("data-bs-target", &["#indexPlanActionFormModal", cpn_id].concat())
                .text("+Plan")
                .event(clone!(page, order_item, order_type => move |_: events::Click| {
                    page.index_plan_action_modal.set(Some(IndexPlanActionForm::new(
                        order_item.order_item_id,
                        None,
                        None,
                        page.patient.clone(),
                        order_type.clone(),
                        FormType::Plan,
                        page.view_by.clone(),
                    )));
                }))
                // ipd-nurse-index-plan-action-form.php::onclickAddIndexPlanOrderItem(event, order_item.order_item_id, order_item.order_item_detail);
            }))))
            .child(html!("span", {
                .child(html!("span", {
                    .apply_if(order_item.order_item_type.as_ref().map(|ty| will_blue.contains(&ty.as_str())).unwrap_or_default(), |d| d.class(class::BOLD_BLUE_EM))
                    .apply_if(order_item.med_name.as_ref().map(|s| !s.is_empty()).unwrap_or_default(), |d| d.class("me-1"))
                    .text(&order_item.med_name.clone().unwrap_or_default())
                    .apply_if(is_pharmacist, |d| { d
                        .style("cursor","copy")
                        .event(clone!(app, order_item => move |_:events::Click| {
                            spawn_local(clone!(app, order_item => async move {
                                app.set_clipboard(&order_item.med_name.clone().unwrap_or_default()).await;
                            }));
                        }))
                    })
                }))
                // duration color + clock
                .apply_if(["med", "ivfluid", "injection", "home-medication"].contains(&order_item.order_item_type.clone().unwrap_or_default().as_ref()), |dom| {
                    // let default_class = ["fw-bold","text-primary-emphasis"];
                    if let Some(du) = order_item.order_duration {
                        let title = [&du.to_string()," วัน (นับจากการสั่งครั้งล่าสุด: ", &date_th_opt(&order_item.order_date),")"].concat();
                        let dom = dom.child(html!("i", {.class(class::FA_CLOCK).attr("title",&title)}));
                        if order_item.duration3.map(|du3| du > (du3 as i32)).unwrap_or_default() {
                            if let Some(c) = &order_item.exceed_duration3_color {
                                dom.style("color",c)
                            } else {
                                dom //.class(default_class)
                            }
                        } else if order_item.duration2.map(|du2| du > (du2 as i32)).unwrap_or_default() {
                            if let Some(c) = &order_item.exceed_duration2_color {
                                dom.style("color",c)
                            } else {
                                dom //.class(default_class)
                            }
                        } else if order_item.duration1.map(|du1| du > (du1 as i32)).unwrap_or_default() {
                            if let Some(c) = &order_item.exceed_duration1_color {
                                dom.style("color",c)
                            } else {
                                dom //.class(default_class)
                            }
                        } else {
                            dom //.class(default_class)
                        }
                    } else {
                        dom //.class(default_class)
                    }
                })
                // Drug allergy badge
                .apply_if(order_item.allergy_agent_symptom.is_some(), |d| d.child(html!("span", {
                    .class(class::BADGE_WRAP_R_RED)
                    .style("cursor","help")
                    .attr("title", &order_item.allergy_agent_symptom.clone().unwrap_or(String::from("ไม่ระบุอาการ")))
                    .text("แพ้ยา/เฝ้าระวัง")
                })))
                // HAD/LASA badge
                .children(app.drug_alert_badge(order_item.displaycolor))
                // Med Reconcile badge
                .apply_if(order_item.med_reconciliation_item_id.is_some(), |d| d.child(html!("span", {
                    .style("cursor","help")
                    .apply(|dd| {
                        match &order_item.used {
                            Some(used) => {
                                match used.as_str() {
                                    "N" => dd.class(class::BADGE_WRAP_R_GRAY),
                                    "H" => dd.class(class::BADGE_WRAP_R_CYAN),
                                    "Y" => {
                                        if order_item.is_med_rec_change_usage() {
                                            dd.class(class::BADGE_WRAP_R_GOLD)
                                        } else {
                                            dd.class(class::BADGE_WRAP_R_GREEN)
                                        }
                                    }
                                    _ => dd,
                                }
                            }
                            None => dd,
                        }

                    })
                    .attr("title", &order_item.med_rec_info())
                    .text("MR")
                })))
                .apply_if(order_item.med_name.is_some(), |dom| dom.child(html!("br")))
                .text(&order_item.order_item_detail.clone().unwrap_or_default())
                .apply_if(order_item.off_by_datetime.is_some(), |dom| dom.style("text-decoration","line-through"))
            }))
            // .apply_if(order_item.stat.clone().unwrap_or_default() == *"Y", |dom| dom.child(html!("span", {
            //     .class(class::BADGE_WRAP_R_RED)
            //     .text("STAT")
            // })))
            .apply_if(order_item.off_by_datetime.is_some(), |dom| dom.child(html!("span", {
                .class(class::BADGE_WRAP_R_GOLD)
                .style("cursor","default")
                .text(&["OFF ", &datetime_th_opt(&order_item.off_by_datetime)].concat())
            })))
            .apply_if(
                is_doctor
                || is_nurse
                || is_pharmacist,
            |dom| {
                dom.children(order_item.index_plans.iter().filter_map(clone!(page => move |plan| Self::render_index_plan_badge(
                    cpn_id,
                    plan,
                    order_item,
                    page.current_date.lock_ref().as_ref().map(|od| od.order_date),
                    if is_oneday { OrderType::OneDay } else { OrderType::Continuous },
                    is_nurse || is_pharmacist,
                    !is_readonly && is_nurse,
                    page.clone(),
                ))))
            })

        })
    }

    // now render only held and offed
    fn render_med_rec(med_rec_item: &Rc<MedReconciliationItem>, app: Rc<App>) -> Dom {
        let is_med_rec_icode = if let (Some(item_icode), Some(app_icode)) = (&med_rec_item.icode, &app.hosxp_medrec_icode()) {
            item_icode == app_icode
        } else {
            false
        };
        let med_name = if is_med_rec_icode {
            med_rec_item.custom_med_name.clone().unwrap_or_default()
        } else {
            med_rec_item.med_name.clone().unwrap_or_default()
        };
        html!("li", {
            .class("clearfix")
            .child(html!("span", {
                .style("text-decoration","line-through")
                .children([
                    html!("span", {
                        .class(class::BOLD_BLUE_EM)
                        .text(&med_name)
                    }),
                    html!("span", {
                        .apply(|dom| {
                            match &med_rec_item.used {
                                Some(used) => {
                                    match used.as_str() {
                                        "H" => dom.class(class::BADGE_WRAP_R_CYAN),
                                        "N" => dom.class(class::BADGE_WRAP_R_GRAY),
                                        "Y" => {
                                            if med_rec_item.is_med_rec_change_usage() {
                                                dom.class(class::BADGE_WRAP_R_GOLD)
                                            } else {
                                                dom.class(class::BADGE_WRAP_R_GREEN)
                                            }
                                        }
                                        _ => dom,
                                    }
                                }
                                None => dom,
                            }
                        })
                        .style("cursor","help")
                        .attr("title", &med_rec_item.med_rec_info())
                        .text("MR")
                    })
                ])
                // Drug allergy badge
                .apply_if(med_rec_item.allergy_agent_symptom.is_some(), |d| d.child(html!("span", {
                    .class(class::BADGE_WRAP_R_RED)
                    .style("cursor","help")
                    .attr("title", &med_rec_item.allergy_agent_symptom.clone().unwrap_or(String::from("ไม่ระบุอาการ")))
                    .text("แพ้ยา/เฝ้าระวัง")
                })))
                .apply_if(med_rec_item.changed_drugusage.is_some() || med_rec_item.old_drugusage.is_some(), |dom| dom.child(html!("br")))
                .text(&med_rec_item.changed_drugusage.clone().or(med_rec_item.old_drugusage.clone()).unwrap_or_default())
            }))
        })
    }

    // let readonly = page.is_readonly();
    // function progress_note_data_to_text(progress_note){
    pub fn render_progress_note(cpn_id: &'static str, progress_note: Rc<ProgressNote>, page: Rc<Self>, app: Rc<App>) -> Dom {
        let has_audit_use = app.has_permission(Permission::DataTypeAuditorUse);

        let is_ipd = page.is_ipd;
        let is_pre_admit = page.patient.lock_ref().as_ref().map(|pt| pt.visit_type.is_pre_admit()).unwrap_or_default();
        let allow_progress_form = if is_ipd {
            app.endpoint_is_allow(&Method::POST, &EndPoint::IpdOrderProgressNote, is_pre_admit)
        } else {
            app.endpoint_is_allow(&Method::POST, &EndPoint::OpdErOrderProgressNote, false)
        };
        let allow_progress_edit = if is_ipd {
            app.has_permission(Permission::ProgressNoteEdit)
        } else {
            app.has_permission(Permission::OpdErProgressNoteEdit)
        };

        let is_my_progress = app.doctor_code().as_ref().map(|code| code == &progress_note.progress_note_doctor).unwrap_or_default();

        // auditor type, only auditor to see
        let is_by_auditor = progress_note.progress_note_owner_type == *"auditor";
        if is_by_auditor && !has_audit_use {
            return html!("div");
        }

        let (display_datetime, display_date) = if is_by_auditor {
            (
                datetime_th_opt(&progress_note.progress_note_enter_datetime),
                progress_note.progress_note_enter_datetime.map(|dt| dt.date()),
            )
        } else {
            (
                [date_th(&progress_note.progress_note_date), time_hm(&progress_note.progress_note_time)].join(" "),
                Some(progress_note.progress_note_date),
            )
        };

        let is_discharged = match (page.patient.lock_ref().as_ref().and_then(|pt| pt.lastdate()), display_date) {
            (Some(dch), Some(display)) => display > dch,
            _ => true,
        };
        let is_ipd = page.is_ipd;
        let is_pre_admit = page.patient.lock_ref().as_ref().map(|pt| pt.visit_type.is_pre_admit()).unwrap_or_default();
        let is_today = progress_note.progress_note_date == js_now().date();
        let is_readonly = page.is_readonly();
        let (owner_type, owner_class) = match progress_note.progress_note_owner_type.as_str() {
            "doctor" => ("Doctor", "bg-primary"),
            "nurse" => ("Nurse", "text-bg-warning"),
            "pharmacist" => ("Pharmacist", "text-bg-info"),
            "other" => ("Other", "text-bg-success"),
            "auditor" => ("Auditor", "text-bg-danger"),
            _ => ("???", "text-bg-danger"),
        };

        html!("div", {
            .visible_signal(not(page.edit_progress_note.signal_cloned().map(clone!(progress_note => move |opt| opt.as_ref().map(|edit| edit.progress_note_id == progress_note.progress_note_id).unwrap_or_default()))))
            .attr("id", &["progress_note_id_", &progress_note.progress_note_id.to_string(), "_div", cpn_id].concat())
            .class(class::BOX_ROUND_T)
            .child(html!("span", {.text(&display_datetime)}))
            .child(html!("span", {
                .class(class::BADGE_R)
                .class(owner_class)
                .style("cursor","default")
                .text(owner_type)
            }))
            .apply_if(progress_note.pre_order_progress_note_date.is_some(), |dom| {
                dom.child(html!("span", {
                    .class(class::BADGE_WRAP_R_GREEN)
                    .style("cursor","help")
                    .attr("title", &[
                        "บันทึกไว้เมื่อ: ",
                        &date_th_opt(&progress_note.pre_order_progress_note_date), " ",
                        &time_hm_opt(&progress_note.pre_order_progress_note_time)
                    ].concat())
                    .text("ล่วงหน้า")
                }))
            })
            .child(html!("div", {
                .attr("id", &["progress_note_id_", &progress_note.progress_note_id.to_string(), "_inner_div", cpn_id].concat())
                .class(class::BORDER_T_Y)
                .children(progress_note.progress_note_item_types.iter().map(|item_type| {
                    html!("div", {
                        .children([
                            html!("span", {
                                .class("fw-bold")
                                .text(item_type.progress_note_item_type.string())
                            }),
                            html!("ul", {
                                .class("dash")
                                .style("white-space","pre-wrap")
                                .children(item_type.progress_note_items.iter().filter_map(|note| {
                                    note.progress_note_item_detail.as_ref().map(|detail| html!("li", {
                                        .child(html!("span", {.text(detail)}))
                                        .apply_if(note.progress_note_item_detail_2.is_some(),|dom| {
                                            dom.children([
                                                html!("br"),
                                                html!("span", {.text(&note.progress_note_item_detail_2.clone().unwrap_or_default())}),
                                            ])
                                        })
                                    }))
                                }))
                            })
                        ])
                    })
                }))
            }))
            .child(html!("div", {
                .class(class::SMALL_R)
                .apply_if(progress_note.order_doctor_is_intern.unwrap_or_default(), |dom| dom.child(html!("span", {.text("(Intern) ")})))
                .children([
                    html!("span", {
                        .text(&[&progress_note.order_doctor_name.clone().unwrap_or_default(), ", "].concat())
                    }),
                    html!("span", {
                        .text(&[&progress_note.entryposition.clone().unwrap_or_default(), ", "].concat())
                    }),
                    html!("span", {
                        .class("text-nowrap")
                        .text(&display_datetime)
                    })
                ])
            }))
            .apply_if(
                is_my_progress
                && !is_readonly
                && (progress_note.progress_note_owner_type.as_str() == page.view_by.lock_ref().as_str()
                    || (is_by_auditor && has_audit_use)),
            |dom| dom
                .child(html!("div", {
                    .attr("id", &["progress_note_id_", &progress_note.progress_note_id.to_string(), "_action_row_div", cpn_id].concat())
                    .class(class::BOLD_R)
                    .apply_if(if is_ipd {is_today || is_discharged} else {true}
                        && allow_progress_form && allow_progress_edit,
                    |d| d
                        .child(html!("button", {
                            .attr("type", "button")
                            .class(class::BTN_SM_RB_GOLD)
                            .text("Edit")
                            .event(clone!(app, page, progress_note => move |_: events::Click| {
                                // DATA_TYPE_AUDITOR_USE
                                page.edit_progress_note.set(Some(progress_note.clone()));
                                if is_by_auditor && has_audit_use {
                                    page.show_progress_note_auditor_input.set(false);
                                    page.show_progress_note_auditor_input.set(true);
                                } else {
                                    page.show_progress_note_input.set(false);
                                    page.show_progress_note_input.set(true);
                                }
                                app.scroll_into_view(&["progress_note_id_", &progress_note.progress_note_id.to_string(), "_div", cpn_id].concat());
                            }))
                        }))
                    )
                    // PROGRESS_NOTE_REMOVE
                    .apply_if(if is_ipd {
                        app.endpoint_is_allow(&Method::DELETE, &EndPoint::IpdOrderProgressNoteId, is_pre_admit)
                    } else {
                        app.endpoint_is_allow(&Method::DELETE, &EndPoint::OpdErOrderProgressNoteId, false)
                    }, |d| d
                        .child(html!("button" => HtmlButtonElement, {
                            .attr("type", "button")
                            .class(class::BTN_SM_RB_RED)
                            .text("Delete")
                            .apply(mixins::click_with_loader_checked(clone!(app, page, progress_note => move || {
                                Self::delete_progress_note(progress_note.progress_note_id, page.clone(), app.clone());
                            }), app.state()))
                        }))
                    )
                }))
            )
            .child(html!("div", {
                .class("mt-1")
                .apply(|dom| {
                    if is_ipd {
                        dom.child(ImageCpn::render("170px", ImageCpn::new_with_key(
                            ImageUsage::IpdProgressNote,
                            progress_note.progress_note_id,
                            is_my_progress && !page.is_readonly(),
                            page.patient.clone(),
                            page.patient.lock_ref().as_ref().map(|pt| pt.visit_type.vnan().to_owned()),
                            "", // will use ImageUsage internally, so we add nothing here
                        ), app))
                    } else {
                        dom.child(ImageCpn::render("170px", ImageCpn::new_with_key(
                            ImageUsage::OpdErProgressNote,
                            progress_note.progress_note_id,
                            is_my_progress && !page.is_readonly(),
                            page.patient.clone(),
                            page.patient.lock_ref().as_ref().map(|pt| pt.visit_type.vnan().to_owned()),
                            "", // will use ImageUsage internally, so we add nothing here
                        ), app))
                    }
                })
            }))
        })
    }

    // SAME AS OPD-ER
    fn render_index_plan_badge(
        cpn_id: &'static str,
        plan: &IndexPlan,
        order_item: &OrderItem,
        current_date: Option<Date>,
        order_type: OrderType,
        show_actions: bool,
        editable: bool,
        page: Rc<Self>,
    ) -> Option<Dom> {
        plan.is_plan_visible(current_date).then(|| {
            // let plan_id = plan.plan_id;
            let order_item_id = plan.order_item_id;
            let plan_id = zero_none(plan.plan_id);
            let plan_date = plan.plan_date;
            let plan_time = plan.plan_time;
            let is_plan_today = current_date == plan_date;
            let is_stat = plan.plan_sch_type == Some(String::from("stat"));
            let plan_type = plan.index_plan_type();
            let actions_all = plan.actions_with_datetime();
            // only visible action
            let actions = if current_date.is_some() { plan.actions_visible(current_date) } else { actions_all.clone() };
            let actions_title = actions_all
                .iter()
                .rev()
                .filter_map(|action| {
                    if let (Some(action_date), Some(action_time)) = (action.action_date, action.action_time) {
                        Some(
                            [
                                action.had_monitor_status(order_item, false),
                                " ",
                                &date_th(&action_date),
                                " ",
                                &time_hm(&action_time),
                                &action.action_result.as_ref().map(|result| [": ", result].concat()).unwrap_or_default(),
                            ]
                            .concat(),
                        )
                    } else {
                        None
                    }
                })
                .collect::<Vec<String>>()
                .join("\n");

            html!("span", {
                .class(class::BADGE_WRAP_RT_GRAY)
                .apply_if(editable || !show_actions, |dom| dom
                    .style("cursor","pointer")
                )
                // DATETIME
                .child(doms::span_with_tooltip(
                    |d| d.text(&{
                        match plan_type {
                            IndexPlanType::SameTime => plan_time.map(|t| time_hm(&t)).unwrap_or(String::from("ไม่ระบุเวลา")),
                            IndexPlanType::EveryDay => String::from("PRN"),
                            IndexPlanType::WholeDay => ["PRN ถึง ", &date_th_opt(&plan_date.and_then(|d| d.next_day())), " ", &time_hm_opt(&plan_time)].concat(),
                            IndexPlanType::SingleTime
                            | IndexPlanType::Undefined => {
                                if is_stat {
                                    ["STAT ", &time_hm_opt(&plan_time)].concat()
                                } else if is_plan_today {
                                    plan_time.map(|t| time_hm(&t)).unwrap_or(String::from("ไม่ระบุเวลา"))
                                } else {
                                    [date_th_opt(&plan_date), time_hm_opt(&plan_time)].join(" ")
                                }
                            }
                        }
                    })
                    .attr("title", &actions_title)
                    .apply_if(editable, |dom| { dom
                        .attr("data-bs-toggle", "modal")
                        .attr("data-bs-target", &["#indexPlanActionFormModal", cpn_id].concat())
                        .event(clone!(page, order_type => move |_: events::Click| {
                            page.index_plan_action_modal.set(Some(IndexPlanActionForm::new(
                                order_item_id.unwrap_or_default(),
                                plan_id,
                                None,
                                page.patient.clone(),
                                order_type.clone(),
                                FormType::Plan,
                                page.view_by.clone(),
                            )));
                        }))
                    }),
                    plan.plan_detail.as_ref(),
                    |d| d,
                ))
                // ACTIONS
                .apply(|dom| {
                    // WITH ACTIONS TEXT
                    if show_actions { dom
                        .children(actions.into_iter().map(clone!(page => move |action| {
                            let label = if action.action_date == current_date {
                                ["(", &action.action_time.map(|t| time_hm(&t)).unwrap_or(String::from("ไม่ระบุเวลา")), ")"].concat()
                            } else {
                                ["(", &action.action_date.map(|d| date_th(&d)).unwrap_or_default(), " ", &action.action_time.map(|t| time_hm(&t)).unwrap_or(String::from("ไม่ระบุเวลา")), ")"].concat()
                            };
                            let monitors_title = action.monitors.iter().filter(|monitor| monitor.monitor_datetime.is_some() && monitor.monitor_abnormal.is_some()).map(|monitor| {
                                let checkmark = if monitor.monitor_abnormal.as_ref().map(|monitor_abnormal| monitor_abnormal == "Y").unwrap_or_default() {"\u{2717} "} else {"\u{2713} "};
                                let monitor_datetime = monitor.monitor_datetime.map(|dt| datetime_th_relative(&dt)).unwrap_or_default();
                                let monitor_user = monitor.monitor_doctor_name.clone().unwrap_or_default();
                                [checkmark, &monitor_datetime, " โดย ", &monitor_user].concat()
                            }).collect::<Vec<String>>().join("\n");
                            doms::span_with_tooltip(
                                clone!(page, order_type, action => move |d| { d
                                    .child(html!("span", {
                                        .class("ms-1")
                                        .style("color","red")
                                        .apply(|dom| {
                                            if action.check_person.is_some() && action.action_person_1.is_none() {
                                                dom.text("\u{2717} ")
                                            } else {
                                                dom.child(doms::had_monitor_status(&action, order_item, true)).text(" ")
                                            }
                                        })
                                        .text(&label)
                                        .attr("title", &monitors_title)
                                    }))
                                    .apply_if(editable, |dd| dd
                                        .attr("data-bs-toggle", "modal")
                                        .attr("data-bs-target", &["#indexPlanActionFormModal", cpn_id].concat())
                                        .event(clone!(page, order_type => move |_: events::Click| {
                                            page.index_plan_action_modal.set(Some(IndexPlanActionForm::new(
                                                order_item_id.unwrap_or_default(),
                                                plan_id,
                                                action.action_id,
                                                page.patient.clone(),
                                                order_type.clone(),
                                                FormType::Action,
                                                page.view_by.clone(),
                                            )));
                                        }))
                                    )
                                }),
                                action.action_result.as_ref(),
                                clone!(action => move |dom| { dom
                                    .apply_if(action.vs_id.is_some(), |d| {
                                        d.child(html!("i", {.class(class::FA_LINK_GREEN)}))
                                    })
                                })
                            )
                        })))
                    // WITH ACTIONS COUNT
                    } else if !actions_all.is_empty() { dom
                        .child(html!("span", {
                            .class("ms-1")
                            .text(&["(x", &actions_all.len().to_string(),")"].concat())
                        }))
                        .apply(|d| {
                            // show full result only the result of current date
                            if let Some(action) = actions_all.iter().find(|action| action.action_date.is_some() && action.action_date == current_date) {
                                let msg = action.action_result.clone().unwrap_or_default();
                                if msg.len() > 9 {
                                    d.children([
                                        html!("i", {.class(class::FA_INFO)}),
                                        html!("span", {
                                            .class("app-tooltip-message")
                                            .text(&msg)
                                        }),
                                    ])
                                } else {
                                    d.class("text-primary").text(&msg)
                                }
                            } else {
                                d
                            }
                        })
                    } else {
                        dom
                    }
                })
            })
        })
    }

    pub fn render_opd_med(i: usize, opd_med: Rc<OpdMed>) -> Dom {
        html!("tr", {
            // .attr("data-hos-guid", &opd_med.hos_guid)
            // .attr("data-icode", &opd_med.icode.clone().unwrap_or_default())
            // .attr("data-item-name", &opd_med.item_name.clone().unwrap_or_default())
            // .attr("data-rxdate", &opd_med.rxdate.map(|d| d.to_string()).unwrap_or_default())
            // .attr("data-usage", &opd_med.usage.clone().unwrap_or_default())
            .children([
                html!("td", {.text(&(i + 1).to_string())}),
                html!("td", {.text(&opd_med.item_name.clone().unwrap_or_default())}),
                html!("td", {.text(&opd_med.usage.as_ref().map(|usage| sanity_dot_space(usage)).unwrap_or_default())}),
                html!("td", {.class("text-end").text(&opd_med.qty.map(|i| i.to_string()).unwrap_or_default())}),
                html!("td", {.text(&[date_th_opt(&opd_med.rxdate), time_hm_opt(&opd_med.rxtime)].join(" "))}),
            ])
        })
    }
}

fn send_sse_by_patch(action: OrderPatchAction, patient: Option<Rc<PatientInfo>>, order: Rc<Order>, nurse_order_as: Option<String>, app: Rc<App>) {
    let ward_opt = patient.as_ref().and_then(|pt| pt.ward.clone());
    let an_opt = patient.as_ref().and_then(|pt| pt.an.clone());
    let ward_name = patient.as_ref().and_then(|pt| pt.ward_name.as_ref().map(|ward| [ward, " "].concat())).unwrap_or_default();
    let bed = patient.as_ref().and_then(|pt| pt.bedno.as_ref().map(|bedno| ["เตียง ", bedno, " "].concat())).unwrap_or_default();
    let hn = patient.as_ref().and_then(|pt| pt.hn.as_ref().map(|hn| ["HN ", hn, " "].concat())).unwrap_or_default();

    let pharmacist_allow_non_med = app.pharmacist_allow_non_med.get();
    let is_med = order.order_item_types.iter().any(|oit| oit.is_med());
    let is_pharm_notify = order.order_item_types.iter().all(|oit| oit.is_pharm_notify());

    if let (Some(ward), Some(an)) = (ward_opt, an_opt) {
        if let Some(message) = match action {
            OrderPatchAction::Confirm => Some(SsePostMessage {
                message: [&ward_name, &bed, &hn, "มีคำสั่งใหม่ รอพยาบาลรับคำสั่ง"].concat(),
                ward: Some(ward), // แจ้ง ward
                route: Some(Route::IpdMain {
                    view_by: String::from("nurse"),
                    an,
                    tab: String::from("order"),
                    sub: order.order_date.to_string(),
                    id: order.order_id,
                }),
                ..Default::default()
            }),
            OrderPatchAction::ConfirmAs => Some(SsePostMessage {
                message: [&ward_name, &bed, &hn, "มีคำสั่งใหม่โดยพยาบาล รอแพทย์ยืนยัน"].concat(),
                person: nurse_order_as, // รายงาน แพทย์ ผู้ถูก รคส
                // ward: Some(ward),
                route: Some(Route::IpdMain {
                    view_by: String::from("doctor"),
                    an,
                    tab: String::from("order"),
                    sub: order.order_date.to_string(),
                    id: order.order_id,
                }),
                ..Default::default()
            }),
            OrderPatchAction::EditAs => Some(SsePostMessage {
                message: [&ward_name, &bed, &hn, "เปลี่ยนแปลงการ รคส. รอแพทย์ยืนยัน"].concat(),
                person: nurse_order_as, // รายงาน แพทย์ ผู้ถูก รคส
                // ward: Some(ward),
                route: Some(Route::IpdMain {
                    view_by: String::from("doctor"),
                    an,
                    tab: String::from("order"),
                    sub: order.order_date.to_string(),
                    id: order.order_id,
                }),
                ..Default::default()
            }),
            OrderPatchAction::DoctorConfirm => (is_med || is_pharm_notify).then(|| SsePostMessage {
                message: [&ward_name, &bed, &hn, "แพทย์ยืนยันคำสั่งแล้ว รอเภสัชกรดำเนินการ"].concat(),
                person: str_some(order.order_doctor.clone()), // แจ้งกลับ พยาบาลผู้ order
                spclty_id: Some(0),                           // แจ้งเภสัช
                route: Some(Route::IpdMain {
                    view_by: String::from("pharmacist"),
                    an,
                    tab: String::from("order"),
                    sub: order.order_date.to_string(),
                    id: order.order_id,
                }),
                ..Default::default()
            }),
            OrderPatchAction::NurseAccept => (is_med || is_pharm_notify).then(|| SsePostMessage {
                message: [&ward_name, &bed, &hn, "พยาบาลรับคำสั่งแล้ว รอเภสัชกรดำเนินการ"].concat(),
                spclty_id: Some(0), // แจ้งเภสัช
                route: Some(Route::IpdMain {
                    view_by: String::from("pharmacist"),
                    an,
                    tab: String::from("order"),
                    sub: order.order_date.to_string(),
                    id: order.order_id,
                }),
                ..Default::default()
            }),
            OrderPatchAction::PharmacistAccept => (pharmacist_allow_non_med || is_med).then(|| SsePostMessage {
                message: [&ward_name, &bed, &hn, "เภสัชกรดำเนินการแล้ว รอการตรวจสอบ"].concat(),
                spclty_id: Some(0), // แจ้งเภสัช
                route: Some(Route::IpdMain {
                    view_by: String::from("pharmacist"),
                    an,
                    tab: String::from("order"),
                    sub: order.order_date.to_string(),
                    id: order.order_id,
                }),
                ..Default::default()
            }),
            OrderPatchAction::PharmacistCheck => (pharmacist_allow_non_med || is_med).then(|| SsePostMessage {
                message: [&ward_name, &bed, &hn, "เภสัชกรดำเนินการแล้ว รอการจ่ายยา"].concat(),
                spclty_id: Some(0), // แจ้งเภสัช
                route: Some(Route::IpdMain {
                    view_by: String::from("pharmacist"),
                    an,
                    tab: String::from("order"),
                    sub: order.order_date.to_string(),
                    id: order.order_id,
                }),
                ..Default::default()
            }),
            OrderPatchAction::PharmacistDone => None,
        } {
            app.send_sse(message);
        }
    }
}

#[derive(Default)]
pub struct OrderItemMutable {
    pub id: u32,
    pub pre_order_master_id: Mutable<Option<u32>>,
    pub order_item_type: Mutable<String>,
    pub order_item_detail: Mutable<String>,
    pub order_item_detail_2: Mutable<String>,
    pub stat: Mutable<String>,
    pub off_order_item_id: Mutable<Option<u32>>,
    pub icode: Mutable<Option<String>>,
    pub med_name: Mutable<Option<String>>,
    pub generic_name: Mutable<Option<String>>,
    pub dosageform: Mutable<Option<String>>,
    pub first_qty: Mutable<String>,
    pub qty: Mutable<String>,
    pub due_usage: Mutable<Option<String>>,
    pub due_status: Mutable<Option<String>>,
    pub info: Mutable<Option<String>>,
    pub info_status: Mutable<Option<String>>,

    pub med_reconciliation_item_id: Mutable<Option<u32>>,
    pub old_drugusage: Mutable<Option<String>>,
    pub receive_from: Mutable<Option<String>>,
    pub receive_date: Mutable<Option<Date>>,
    pub receive_qty: Mutable<Option<i32>>,
    pub last_dose_taken_time: Mutable<Option<PrimitiveDateTime>>,
    pub last_dose_taken_remark: Mutable<Option<String>>,
    pub used: Mutable<Option<String>>,
}

impl OrderItemMutable {
    pub fn new(item_type: &str, pre_order_master_id: Option<u32>) -> Rc<Self> {
        Rc::new(Self {
            id: NEXT_ID.fetch_add(1, Ordering::SeqCst),
            pre_order_master_id: Mutable::new(pre_order_master_id),
            order_item_type: Mutable::new(item_type.to_owned()),
            stat: Mutable::new(String::from("N")),
            ..Default::default()
        })
    }

    pub fn save(order_item: &Rc<Self>) -> Option<OrderItemSave> {
        let order_item_type = str_some(order_item.order_item_type.get_cloned());
        let is_med = order_item_type.as_ref().map(|oit| ["med", "injection", "home-medication"].contains(&oit.as_str())).unwrap_or_default();
        let order_item_detail = str_some(order_item.order_item_detail.get_cloned()).map(|s| if is_med { sanity_dot_space(&s) } else { s });
        (order_item_type.is_some() && order_item_detail.is_some()).then(|| OrderItemSave {
            order_id: None,
            order_item_type,
            order_item_detail,
            stat: str_some(order_item.stat.get_cloned()),
            off_order_item_id: order_item.off_order_item_id.get(),
            icode: order_item.icode.get_cloned(),
            med_reconciliation_item_id: order_item.med_reconciliation_item_id.get(),
            first_qty: order_item.first_qty.lock_ref().parse::<i32>().ok(),
            qty: order_item.qty.lock_ref().parse::<i32>().ok(),
        })
    }

    pub fn save_as_progress_note(order_item: &Rc<Self>) -> Option<ProgressNoteItemSave> {
        let progress_note_item_type = str_some(order_item.order_item_type.get_cloned());
        let progress_note_item_detail = str_some(order_item.order_item_detail.get_cloned());
        (progress_note_item_type.is_some() && progress_note_item_detail.is_some()).then(|| ProgressNoteItemSave {
            progress_note_item_type,
            progress_note_item_detail,
            progress_note_item_detail_2: str_some(order_item.order_item_detail_2.get_cloned()),
        })
    }

    pub fn med_rec_info(&self) -> String {
        let (old_usage, now_title) = if self.is_med_rec_change_usage() {
            (
                self.old_drugusage.lock_ref().as_ref().map(|s| ["วิธีใช้เดิม: ", s, "\n"].concat()).unwrap_or_default(),
                String::from("วิธีใช้ใหม่: "),
            )
        } else {
            (String::new(), String::from("วิธีใช้: "))
        };
        [
            old_usage,
            now_title,
            self.order_item_detail.get_cloned(),
            self.receive_from.lock_ref().as_ref().map(|s| ["\nได้รับจาก: ", s].concat()).unwrap_or_default(),
            self.receive_qty.get().map(|i| ["\nปริมาณ: ", &i.to_string()].concat()).unwrap_or_default(),
            self.receive_date.get().map(|d| ["\nเมื่อวันที่: ", &date_th(&d)].concat()).unwrap_or_default(),
            self.last_dose_taken_time.get().map(|dt| ["\nรับประทานครั้งล่าสุด: ", &datetime_th(&dt)].concat()).unwrap_or_default(),
            self.last_dose_taken_remark.lock_ref().as_ref().map(|s| ["\nหมายเหตุ: ", s].concat()).unwrap_or_default(),
        ]
        .concat()
    }

    pub fn is_med_rec_change_usage(&self) -> bool {
        if let Some(old) = self.old_drugusage.lock_ref().as_ref() {
            old != self.order_item_detail.lock_ref().as_str()
        } else {
            false
        }
    }
}

impl From<OrderItem> for OrderItemMutable {
    fn from(item: OrderItem) -> Self {
        let first_qty = item
            .first_qty
            .map(|i| i.to_string())
            .or(item.med_reconciliation_item_id.is_some().then(|| String::from("0")))
            .unwrap_or_default();
        OrderItemMutable {
            id: NEXT_ID.fetch_add(1, Ordering::SeqCst),
            order_item_type: Mutable::new(item.order_item_type.unwrap_or_default()),
            order_item_detail: Mutable::new(item.order_item_detail.unwrap_or_default()),
            stat: Mutable::new(item.stat.unwrap_or(String::from("N"))),
            off_order_item_id: Mutable::new(item.off_order_item_id),
            icode: Mutable::new(item.icode),
            med_name: Mutable::new(item.med_name),
            generic_name: Mutable::new(item.generic_name),
            dosageform: Mutable::new(item.dosageform),
            first_qty: Mutable::new(first_qty),
            due_usage: Mutable::new(item.due_usage),
            due_status: Mutable::new(item.due_status),
            info: Mutable::new(item.info),
            info_status: Mutable::new(item.info_status),

            med_reconciliation_item_id: Mutable::new(item.med_reconciliation_item_id),
            old_drugusage: Mutable::new(item.old_drugusage),
            receive_from: Mutable::new(item.receive_from),
            receive_date: Mutable::new(item.receive_date),
            receive_qty: Mutable::new(item.receive_qty),
            last_dose_taken_time: Mutable::new(item.last_dose_taken_time),
            last_dose_taken_remark: Mutable::new(item.last_dose_taken_remark),
            used: Mutable::new(item.used),
            ..Default::default()
        }
    }
}

impl From<MedOrderItem> for OrderItemMutable {
    fn from(item: MedOrderItem) -> Self {
        let first_qty = item.med_reconciliation_item_id.is_some().then(|| String::from("0")).unwrap_or_default();
        OrderItemMutable {
            id: NEXT_ID.fetch_add(1, Ordering::SeqCst),
            order_item_type: Mutable::new(item.order_item_type.unwrap_or_default()),
            order_item_detail: Mutable::new(item.order_item_detail.unwrap_or_default()),
            stat: Mutable::new(String::from("N")),
            icode: Mutable::new(item.icode),
            med_name: Mutable::new(item.med_name),
            generic_name: Mutable::new(item.generic_name),
            dosageform: Mutable::new(item.dosageform),
            first_qty: Mutable::new(first_qty),
            due_usage: Mutable::new(item.due_usage),
            due_status: Mutable::new(item.due_status),
            info: Mutable::new(item.info),
            info_status: Mutable::new(item.info_status),

            med_reconciliation_item_id: Mutable::new(item.med_reconciliation_item_id),
            old_drugusage: Mutable::new(item.old_drugusage),
            receive_from: Mutable::new(item.receive_from),
            receive_date: Mutable::new(item.receive_date),
            receive_qty: Mutable::new(item.receive_qty),
            last_dose_taken_time: Mutable::new(item.last_dose_taken_time),
            last_dose_taken_remark: Mutable::new(item.last_dose_taken_remark),
            used: Mutable::new(item.used),
            ..Default::default()
        }
    }
}

impl From<MedReconciliationItem> for OrderItemMutable {
    fn from(item: MedReconciliationItem) -> Self {
        OrderItemMutable {
            id: NEXT_ID.fetch_add(1, Ordering::SeqCst),
            order_item_type: Mutable::new(String::from("home-medication")),
            order_item_detail: Mutable::new(item.changed_drugusage.or(item.old_drugusage.clone()).unwrap_or_default()),
            stat: Mutable::new(String::from("N")),
            icode: Mutable::new(item.icode),
            med_name: Mutable::new(item.custom_med_name.or(item.med_name)),
            generic_name: Mutable::new(item.generic_name),
            dosageform: Mutable::new(item.dosageform),
            first_qty: Mutable::new(String::from("0")),
            due_usage: Mutable::new(item.due_usage),
            due_status: Mutable::new(item.due_status),
            info: Mutable::new(item.info),
            info_status: Mutable::new(item.info_status),

            med_reconciliation_item_id: Mutable::new(Some(item.med_reconciliation_item_id)),
            old_drugusage: Mutable::new(item.old_drugusage),
            receive_from: Mutable::new(item.receive_from),
            receive_date: Mutable::new(item.receive_date),
            receive_qty: Mutable::new(item.receive_qty),
            last_dose_taken_time: Mutable::new(item.last_dose_taken_time),
            last_dose_taken_remark: Mutable::new(item.last_dose_taken_remark),
            used: Mutable::new(item.used),
            ..Default::default()
        }
    }
}

impl From<PreOrderItem> for OrderItemMutable {
    fn from(item: PreOrderItem) -> Self {
        OrderItemMutable {
            id: NEXT_ID.fetch_add(1, Ordering::SeqCst),
            pre_order_master_id: Mutable::new(zero_none(item.pre_order_master_id)),
            order_item_type: Mutable::new(item.order_item_type.unwrap_or_default()),
            order_item_detail: Mutable::new(item.order_item_detail.unwrap_or_default()),
            stat: Mutable::new(item.stat.unwrap_or(String::from("N"))),
            off_order_item_id: Mutable::new(item.off_order_item_id),
            icode: Mutable::new(item.icode),
            med_name: Mutable::new(item.med_name),
            generic_name: Mutable::new(item.generic_name),
            dosageform: Mutable::new(item.dosageform),
            ..Default::default()
        }
    }
}

impl From<ProgressNoteItem> for OrderItemMutable {
    fn from(item: ProgressNoteItem) -> Self {
        OrderItemMutable {
            id: NEXT_ID.fetch_add(1, Ordering::SeqCst),
            pre_order_master_id: Mutable::new(None),
            order_item_type: Mutable::new(item.progress_note_item_type.unwrap_or_default()),
            order_item_detail: Mutable::new(item.progress_note_item_detail.unwrap_or_default()),
            order_item_detail_2: Mutable::new(item.progress_note_item_detail_2.unwrap_or_default()),
            ..Default::default()
        }
    }
}

impl From<PreProgressNoteItem> for OrderItemMutable {
    fn from(item: PreProgressNoteItem) -> Self {
        OrderItemMutable {
            id: NEXT_ID.fetch_add(1, Ordering::SeqCst),
            pre_order_master_id: Mutable::new(item.pre_order_master_id),
            order_item_type: Mutable::new(item.progress_note_item_type.unwrap_or_default()),
            order_item_detail: Mutable::new(item.progress_note_item_detail.unwrap_or_default()),
            order_item_detail_2: Mutable::new(item.progress_note_item_detail_2.unwrap_or_default()),
            ..Default::default()
        }
    }
}

impl PartialEq<OrderItemMutable> for OrderItemMutable {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

pub struct InsertTextAreaButton {
    pub is_new: bool,
    pub word_type: String,
    pub separator: String,
    pub word: String,
    pub minus_from_end: u32,
    pub id: Option<String>,
}

impl InsertTextAreaButton {
    pub fn from_button(word_type: &str, button: Button) -> Self {
        Self {
            is_new: button.is_new,
            word_type: word_type.to_owned(),
            separator: button.separator,
            word: button.word,
            minus_from_end: button.minus_from_end,
            id: button.id.to_owned(),
        }
    }

    pub fn render(button: Rc<Self>, textareas: MutableVec<Rc<OrderItemMutable>>, focus_id_mutable: Mutable<Option<u32>>, changed_mutable: Mutable<bool>, app: Rc<App>) -> Dom {
        let btn_class = if button.is_new { "btn-info" } else { "btn-secondary" };
        html!("button", {
            .attr("type", "button")
            .class(class::BTN_SM_RT)
            .class(btn_class)
            .text(&button.word)
            .event(move |_: events::Click| {
                Self::action(button.clone(), textareas.clone(), focus_id_mutable.clone(), app.clone());
                changed_mutable.set_neq(true);
                // insert_textarea_id_position(textareas, focus_id_mutable, &button.word_type, &button.separator, &button.word, button.minus_from_end);
            })
        })
    }

    pub fn render_maybe_adder(
        button: Rc<Self>,
        textareas: MutableVec<Rc<OrderItemMutable>>,
        order_items_mutable: MutableVec<Rc<OrderItemMutable>>,
        focus_id_mutable: Mutable<Option<u32>>,
        changed_mutable: Mutable<bool>,
        pre_order_master_id: Option<u32>,
        app: Rc<App>,
    ) -> Dom {
        let btn_class = if button.id.is_some() {
            "btn-warning"
        } else if button.is_new {
            "btn-info"
        } else {
            "btn-secondary"
        };
        html!("button", {
            .attr("type", "button")
            .class(class::BTN_SM_RT)
            .class(btn_class)
            .text(&button.word)
            .event(move |_: events::Click| {
                if button.id.is_some() {
                    Self::add_to_items(button.clone(), order_items_mutable.clone(), pre_order_master_id, focus_id_mutable.clone(), changed_mutable.clone());
                } else {
                    Self::action(button.clone(), textareas.clone(), focus_id_mutable.clone(), app.clone());
                }
                changed_mutable.set_neq(true);
                // insert_textarea_id_position(textareas, focus_id_mutable, &button.word_type, &button.separator, &button.word, button.minus_from_end);
            })
        })
    }

    /// - id MUST be Some
    /// - now support only `ivfluid`
    fn add_to_items(button: Rc<Self>, order_items_mutable: MutableVec<Rc<OrderItemMutable>>, pre_order_master_id: Option<u32>, focus_id_mutable: Mutable<Option<u32>>, changed_mutable: Mutable<bool>) {
        let mut lock = order_items_mutable.lock_mut();
        let order_item = OrderItemMutable::new(&button.word_type, pre_order_master_id);
        focus_id_mutable.set(Some(order_item.id));
        order_item.icode.set(button.id.clone());
        order_item.med_name.set(Some(button.word.clone()));
        lock.push_cloned(order_item);
        changed_mutable.set_neq(true);
    }

    fn action(button: Rc<Self>, textareas: MutableVec<Rc<OrderItemMutable>>, focus_id_mutable: Mutable<Option<u32>>, app: Rc<App>) {
        let mut texts = textareas.lock_mut();
        let texts_len = texts.len();
        let (pos, id) = if button.is_new || texts.is_empty() {
            let item = OrderItemMutable::new(&button.word_type, None);
            let item_id = item.id;
            texts.push_cloned(item);
            focus_id_mutable.set_neq(Some(item_id));
            (texts_len, item_id)
        } else {
            match focus_id_mutable.get().and_then(|focus| texts.iter().position(|item| item.id == focus)) {
                Some(pos) => (pos, texts[pos].id),
                None => {
                    let pos = texts.len() - 1;
                    focus_id_mutable.set_neq(Some(texts[pos].id));
                    (pos, texts[pos].id)
                }
            }
        };

        let update = Timeout::new(
            0,
            clone!(textareas => move || {
                let texts = textareas.lock_mut();
                let detail = texts[pos].order_item_detail.get_cloned();

                let detail_char_count = detail.chars().count();
                let word_char_count = button.word.chars().count();
                let separator_char_count = button.separator.chars().count();

                if let Some(textarea) = app.get_id(&["textarea-", &id.to_string()].concat()).and_then(|elm| {
                    elm.dyn_into::<HtmlTextAreaElement>().ok()
                }) {
                    let text_pos = textarea.selection_start().ok().unwrap_or_default().unwrap_or_default() as usize;
                    let inserted_len;
                    if detail.is_empty() {
                        texts[pos].order_item_detail.set(button.word.clone());
                        inserted_len = word_char_count;
                    } else if detail.chars().last().map(|c| c == ' ').unwrap_or_default() {
                        texts[pos].order_item_detail.set([detail, button.word.clone()].concat());
                        inserted_len = word_char_count;
                    } else if text_pos == 0 {
                        texts[pos].order_item_detail.set([detail, button.separator.clone(), button.word.clone()].concat());
                        inserted_len = word_char_count + separator_char_count;
                    } else {
                        let clamp_pos = if text_pos > detail_char_count {detail_char_count} else {text_pos};
                        let str_index = detail.char_indices().nth(clamp_pos).map(|(i,_)|i).unwrap_or(detail.len());
                        texts[pos].order_item_detail.set([&detail[..str_index], &button.separator, &button.word, &detail[str_index..]].concat());
                        inserted_len = word_char_count + separator_char_count;
                    }
                    let new_pos = (text_pos as u32 + inserted_len as u32).saturating_sub(button.minus_from_end);
                    let focus = Timeout::new(0, move || {
                        if let Err(e) = textarea.set_selection_range(new_pos, new_pos) {
                            app.show_jsvalue_error_message(&e);
                        }
                        if let Err(e) = textarea.focus() {
                            app.show_jsvalue_error_message(&e);
                        }
                    });
                    focus.forget();
                }
            }),
        );
        update.forget();
    }
}

// order_item with `out-of-drug-list` item use the same `icode`
// hos.medplan_ipd has only `medplan_number`, `icode`, `order_date`
// when match KPHIS's `order_item` and `medplan_ipd`, only icode not enough to detect
// now we use `icode` + sanitity_dot_space(`drug_usage`)
pub struct OffOrderItem {
    icode: String,
    drug_usage: String,
}

impl OffOrderItem {
    pub fn from_order_item(item: &OrderItem, hosxp_medrec_icode: &Option<String>) -> Option<Rc<Self>> {
        item.off_icode.as_ref().map(|icode| {
            let drug_usage = if let Some(medrec_icode) = hosxp_medrec_icode
                && icode == medrec_icode
            {
                [&item.off_med_name.clone().unwrap_or_default(), " ", &item.off_order_item_detail.clone().unwrap_or_default()].concat()
            } else {
                item.off_order_item_detail.clone().unwrap_or_default()
            };
            Rc::new(Self {
                icode: icode.to_owned(),
                drug_usage: sanity_dot_space(&sanity_tis620(&drug_usage)),
            })
        })
    }

    pub fn from_medplan_ipd(item: &HisMedPlanIpd) -> Option<Rc<Self>> {
        item.icode.as_ref().map(|icode| {
            Rc::new(Self {
                icode: icode.to_owned(),
                drug_usage: sanity_dot_space(&item.drug_usage.to_owned().unwrap_or_default()),
            })
        })
    }
}

impl PartialEq for OffOrderItem {
    fn eq(&self, other: &Self) -> bool {
        self.icode == other.icode && self.drug_usage == other.drug_usage
    }
}

#[derive(Clone)]
pub struct DueMutables {
    items: Vec<DueMutable>,
    changed: Mutable<bool>,
}

impl DueMutables {
    fn new(order: Rc<Order>) -> Rc<Self> {
        let items = order
            .order_item_types
            .iter()
            .flat_map(|order_item_type| {
                order_item_type.order_items.iter().filter_map(|order_item| {
                    let is_due = order_item.due_status.as_ref().map(|due_status| due_status == "Y").unwrap_or_default();
                    is_due.then(|| DueMutable::from(order_item))
                })
            })
            .collect::<Vec<DueMutable>>();

        Rc::new(Self { items, changed: Mutable::new(false) })
    }

    fn is_doctor_invalid_signal(&self) -> impl Signal<Item = bool> + use<> {
        let items = self.items.clone();
        self.changed.signal().map(move |_| items.iter().any(|item| item.is_doctor_invalid()))
    }

    fn is_pharm_invalid_signal(&self) -> impl Signal<Item = bool> + use<> {
        let items = self.items.clone();
        self.changed.signal().map(move |_| items.iter().any(|item| item.is_pharm_invalid()))
    }

    fn is_pharm_just_noted_signal(&self) -> impl Signal<Item = bool> + use<> {
        let items = self.items.clone();
        self.changed.signal().map(move |changed| {
            changed
                && items
                    .iter()
                    .any(|item| item.due_pharm.lock_ref().as_ref().map(|s| s != "Y").unwrap_or_default() && item.due_pharm_note.lock_ref().is_some())
        })
    }
}

#[derive(Clone)]
struct DueMutable {
    order_item_id: u32,
    due_usage: Option<String>,

    due_doctor: Mutable<Option<String>>,
    due_doctor_note: Mutable<Option<String>>,
    due_pharm: Mutable<Option<String>>,
    due_pharm_note: Mutable<Option<String>>,

    due_doctor_note_temp: Mutable<String>,
    due_pharm_note_temp: Mutable<String>,
    due_doctor_note_changed: Mutable<bool>,
    due_pharm_note_changed: Mutable<bool>,
}

impl From<&OrderItem> for DueMutable {
    fn from(item: &OrderItem) -> Self {
        Self {
            order_item_id: item.order_item_id,
            due_usage: item.due_usage.clone(),

            due_doctor: Mutable::new(item.due_doctor.clone()),
            due_doctor_note: Mutable::new(item.due_doctor_note.clone()),
            due_pharm: Mutable::new(item.due_pharm.clone()),
            due_pharm_note: Mutable::new(item.due_pharm_note.clone()),

            due_doctor_note_temp: Mutable::new(String::new()),
            due_pharm_note_temp: Mutable::new(String::new()),
            due_doctor_note_changed: Mutable::new(false),
            due_pharm_note_changed: Mutable::new(false),
        }
    }
}

impl DueMutable {
    fn btn_color(&self) -> &'static str {
        let doctor_not_ok = self.due_doctor.lock_ref().as_ref().map(|due| due != "Y").unwrap_or_default();
        let pharm_not_ok = self.due_pharm.lock_ref().as_ref().map(|due| due != "Y").unwrap_or_default();
        match (doctor_not_ok, pharm_not_ok) {
            (true, true) => "btn-danger",
            (true, false) => "btn-info",
            (false, true) => "btn-warning",
            (false, false) => "btn-success",
        }
    }

    fn is_doctor_invalid(&self) -> bool {
        let is_ok_opt = self.due_doctor.lock_ref().as_ref().map(|s| s == "Y");
        match is_ok_opt {
            Some(true) => false,
            Some(false) => self.due_doctor_note.lock_ref().is_none(),
            None => true,
        }
    }

    fn is_pharm_invalid(&self) -> bool {
        let is_ok_opt = self.due_pharm.lock_ref().as_ref().map(|s| s == "Y");
        match is_ok_opt {
            Some(true) => false,
            Some(false) => self.due_pharm_note.lock_ref().is_none(),
            None => true,
        }
    }

    // fn is_complete_signal(&self) -> impl Signal<Item = bool> + use<> {
    //     map_ref!{
    //         let is_doctor_invalid = self.is_doctor_invalid_signal(),
    //         let is_pharm_invalid = self.is_pharm_invalid_signal() =>
    //         !is_doctor_invalid && !is_pharm_invalid
    //     }
    // }

    fn is_doctor_invalid_signal(&self) -> impl Signal<Item = bool> + use<> {
        map_ref! {
            let is_ok_opt = self.due_doctor.signal_ref(|opt| opt.as_ref().map(|due| due == "Y")),
            let not_has_note = self.due_doctor_note.signal_ref(|opt| opt.is_none()) =>
            match is_ok_opt {
                Some(true) => false,
                Some(false) => *not_has_note,
                None => true,
            }
        }
    }

    fn is_pharm_invalid_signal(&self) -> impl Signal<Item = bool> + use<> {
        map_ref! {
            let is_ok_opt = self.due_pharm.signal_ref(|opt| opt.as_ref().map(|due| due == "Y")),
            let not_has_note = self.due_pharm_note.signal_ref(|opt| opt.is_none()) =>
            match is_ok_opt {
                Some(true) => false,
                Some(false) => *not_has_note,
                None => true,
            }
        }
    }

    fn is_doctor_noted_signal(&self) -> impl Signal<Item = bool> + use<> {
        map_ref! {
            let not_ok = self.due_doctor.signal_ref(|opt| opt.as_ref().map(|due| due != "Y").unwrap_or_default()),
            let has_note = self.due_doctor_note.signal_ref(|opt| opt.is_some()) =>
            *not_ok && *has_note
        }
    }

    // fn is_pharm_noted_signal(&self) -> impl Signal<Item = bool> + use<> {
    //     map_ref!{
    //         let not_ok = self.due_pharm.signal_ref(|opt| opt.as_ref().map(|due| due != "Y").unwrap_or_default()),
    //         let has_note = self.due_pharm_note.signal_ref(|opt| opt.is_some()) =>
    //         *not_ok && *has_note
    //     }
    // }
}

struct OrderFlags {
    // my_code_opt: Option<String>,
    view_by: String,

    is_today: bool,
    // is_ipd: bool,
    is_pre_admit: bool,
    is_med: bool,
    is_readonly: bool,

    is_confirm: bool,
    is_by_doctor: bool,

    is_doctor: bool,
    is_nurse: bool,
    is_pharmacist: bool,
    // is_my_order: bool,
    is_note: bool,
    is_pharm_notify: bool,

    // is_nurse_order_as: bool,
    // is_doctor_can_confirm: bool,
    is_doctor_confirm: bool,
    is_nurse_accepted: bool,
    is_pharm_accepted: bool,
    is_pharm_checked: bool,
    // is_pharm_can_done: bool,
    is_pharm_done: bool,

    allow_order_form: bool,
    allow_order_edit: bool,
    // allow_order_patch: bool,

    // has_ipd_order_accept: bool,
    // has_opd_er_order_accept: bool,
    can_change: bool,
    can_nurse_accept: bool,
    can_nurse_edit_as: bool,
    can_doctor_confirm_order_as: bool,
    can_pharmacist_accept: bool,
    can_pharmacist_check: bool,
    can_pharmacist_done: bool,
    can_confirm: bool,

    is_order_as_wait_for_doctor: bool,
    is_needed_before_pharmacist: bool,
}

impl OrderFlags {
    fn from_order(order: &Rc<Order>, page: &Rc<OrderCpn>, app: &Rc<App>) -> Rc<Self> {
        let my_code_opt = app.doctor_code();
        let view_by = page.view_by.get_cloned();

        let is_today = page.is_today();
        let is_pre_admit = page.patient.lock_ref().as_ref().map(|pt| pt.visit_type.is_pre_admit()).unwrap_or_default();
        let is_med = order.order_item_types.iter().any(|oit| oit.is_med());
        let is_readonly = page.is_readonly();

        let is_confirm = order.is_confirm();
        let is_by_doctor = order.is_by_doctor();

        let is_doctor = view_by.as_str() == "doctor" && app.has_permission(Permission::DataTypeDoctorUse);
        let is_nurse = view_by.as_str() == "nurse" && app.has_permission(Permission::DataTypeNurseUse);
        let is_pharmacist = view_by.as_str() == "pharmacist" && app.has_permission(Permission::DataTypePharmacyUse);
        let is_my_order = order.is_my_order(&my_code_opt);
        let is_note = order.order_item_types.iter().all(|oit| oit.is_note());
        let is_pharm_notify = order.order_item_types.iter().all(|oit| oit.is_pharm_notify());

        let is_nurse_order_as = order.is_nurse_order_as();
        let is_doctor_can_confirm = order.is_doctor_can_confirm(&my_code_opt);
        let is_doctor_confirm = order.is_doctor_confirm();
        let is_nurse_accepted = order.is_nurse_accepted();
        let is_pharm_accepted = order.is_pharm_accepted();
        let is_pharm_checked = order.is_pharm_checked();
        let is_pharm_can_done = order.is_pharm_can_done(&my_code_opt, app.is_checked_pharmacist_can_done());
        let is_pharm_done = order.is_pharm_done();

        let allow_order_form = if page.is_ipd {
            app.endpoint_is_allow(&Method::POST, &EndPoint::IpdOrderOrder, is_pre_admit)
        } else {
            app.endpoint_is_allow(&Method::POST, &EndPoint::OpdErOrderOrder, false)
        };
        let allow_order_edit = !is_readonly
            && if page.is_ipd {
                app.has_permission(Permission::IpdOrderEdit)
            } else {
                app.has_permission(Permission::OpdErOrderEdit)
            };
        let allow_order_patch = !is_readonly
            && if page.is_ipd {
                app.endpoint_is_allow(&Method::PATCH, &EndPoint::IpdOrderOrder, is_pre_admit)
            } else {
                app.endpoint_is_allow(&Method::PATCH, &EndPoint::OpdErOrderOrder, false)
            };

        let has_ipd_order_accept = app.has_permission(Permission::IpdOrderAccept) || is_pre_admit;
        let has_opd_er_order_accept = app.has_permission(Permission::OpdErOrderAccept);

        let can_change = !is_confirm && is_my_order && order.order_owner_type == view_by && !is_readonly;
        let can_nurse_accept = is_nurse && is_by_doctor && is_confirm && !is_nurse_accepted && allow_order_patch && if page.is_ipd { has_ipd_order_accept } else { has_opd_er_order_accept };
        let can_nurse_edit_as = is_nurse && is_my_order && !is_note && !is_pharm_notify && !is_by_doctor && !is_doctor_confirm && allow_order_edit && allow_order_patch;
        let can_doctor_confirm_order_as = is_doctor && !is_by_doctor && is_confirm && is_doctor_can_confirm && !is_doctor_confirm && allow_order_patch;
        let can_pharmacist_accept = is_pharmacist && is_confirm && !is_pharm_accepted && allow_order_patch;
        let can_pharmacist_check = is_pharmacist && is_confirm && is_pharm_accepted && !is_pharm_checked && allow_order_patch;
        let can_pharmacist_done = is_pharmacist
            && is_confirm
            && is_pharm_checked
            && !is_pharm_done
            && is_pharm_can_done
            && allow_order_patch
            && if page.is_ipd {
                app.has_permission(Permission::IpdOrderDone) || is_pre_admit
            } else {
                app.has_permission(Permission::OpdErOrderDone)
            };
        let can_confirm = allow_order_patch
            && if page.is_ipd {
                app.has_permission(Permission::IpdOrderConfirm) || is_pre_admit
            } else {
                app.has_permission(Permission::OpdErOrderConfirm)
            };

        let is_order_as_wait_for_doctor = !is_readonly && is_confirm && !is_by_doctor && is_nurse_order_as && !is_doctor_confirm;
        let is_needed_before_pharmacist = !is_readonly && ((can_change && (can_confirm || allow_order_edit)) || can_nurse_accept || can_doctor_confirm_order_as);

        Rc::new(Self {
            // my_code_opt,
            view_by,

            is_today,
            is_pre_admit,
            is_med,
            is_readonly,

            is_confirm,
            is_by_doctor,

            is_doctor,
            is_nurse,
            is_pharmacist,
            // is_my_order,
            is_note,
            is_pharm_notify,

            // is_nurse_order_as,
            // is_doctor_can_confirm,
            is_doctor_confirm,
            is_nurse_accepted,
            is_pharm_accepted,
            is_pharm_checked,
            // is_pharm_can_done,
            is_pharm_done,

            allow_order_form,
            allow_order_edit,
            // allow_order_patch,

            // has_ipd_order_accept,
            // has_opd_er_order_accept,
            can_change,
            can_nurse_accept,
            can_nurse_edit_as,
            can_doctor_confirm_order_as,
            can_pharmacist_accept,
            can_pharmacist_check,
            can_pharmacist_done,
            can_confirm,

            is_order_as_wait_for_doctor,
            is_needed_before_pharmacist,
        })
    }
}
