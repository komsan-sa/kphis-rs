// ipd-summary-2.php

// NOW Coder and Auditor use the same Permission::DataTypeAuditorUse
// IDEA about chart audit-review cycle
//   1. NO DATA : hos.ipt.an WITHOUT kphis.ipd_summary_2.an
//   2. NOT SUMMARY : kphis.ipd_summary_2.summary_id WITHOUT ipd_summary_attending_doctor.summary_id
//   3. CODE :   kphis.ipd_summary_2.status == NULL
//   4. REVIEW : kphis.ipd_summary_2.status == REVIEW
//   5. CODE :   kphis.ipd_summary_2.status == CODE
//   6. AUDIT :  kphis.ipd_summary_2.status == AUDIT
//   7. CLAIM :   kphis.ipd_summary_2.status == CLAIM
//   8. DONE :   kphis.ipd_summary_2.status == DONE
//
// [1,2]: Doctor fill Summary data [บันทึก] => set status=CODE
// [ 4 ]: Doctor fix  Summary data [บันทึก] => set status=CODE
// [3,5]: Coder fill Codes data [บันทึก + ส่ง Audit] => set status=AUDIT
// [ 6 ]: Auditor [บันทึก + ส่ง Review] => set status=REVIEW
// [ 6 ]: Auditor [บันทึก + พร้อม Claim] => set status=CLAIM
// [ 7 ]: Auditor [บันทึก + สำเร็จ] => set status=DONE

use dominator::{Dom, EventOptions, clone, events, html, window_size};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, not},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::{collections::HashSet, ops::Deref, rc::Rc, str::FromStr};
use time::{PrimitiveDateTime, Time};
use web_sys::{HtmlButtonElement, HtmlInputElement, HtmlTextAreaElement};

use kphis_drg_worker::drg::model::{GrouperInput, GrouperOutput};
use kphis_model::{
    LEFT_PANEL_MIN_WIDTH, SCREEN_WIDTH_EXTRA,
    endpoint::EndPoint,
    fetch::Method,
    ipd::{
        his::HisOperationAdmit,
        summary::{AuditStatus, DchData, DoctorData, DxData, LabAlertData, Summary, SummaryCodeSave, SummaryData, SummaryDataSave, SummaryParams, SummarySave, SummaryStatus, XRayData},
    },
    report::{SystemReport, TypstReport},
    route::Route,
    search::searchbox::{HospSearchBox, Icd10},
    sse::SsePostMessage,
    tab::Tab,
    user::permission::Permission,
};
use kphis_ui_app::{App, DaggerAsteriskState};
use kphis_ui_component::{
    gadget::{
        aside_resizer::AsideResizerCpn,
        pdf_button::PdfButtons,
        searchbox::{dx::DxSearchboxCpn, hosp::HospSearchboxCpn},
    },
    modal::{blank_modal, lab_history::LabHistory},
    show_patient_main::ShowPatientMainCpn,
    summary_note::{SummaryNoteCpn, render_lab_alert, render_problem_list},
};
use kphis_ui_core::{
    class, doms,
    draggable::{DRAGGABLE_STYLE, DragState, Dragable, Group, SelectedDragable, drag_end, drag_move},
    mixins,
};
use kphis_util::{
    datetime::{date_th_opt, datetime_from_opt, datetime_th_opt, js_now, time_hm_opt},
    util::{f32_rescale, icd9_dot, icd10_dot, los_f32_to_u32, str_some, zero_none},
};

pub const SUMMARY_STYLE: &str = r#"
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
}"#;

/// - GET `EndPoint::IpdSummary`
/// - GET `EndPoint::IpdSummaryNoteId` (SummaryNoteCpn)
/// - GET `EndPoint::IpdShowPatientMainAn` (ShowPatientMainCpn)
/// - GET `EndPoint::OpdErShowPatientMainVn` (ShowPatientMainCpn)
/// - GET `EndPoint::OpdErShowPatientMainId` (ShowPatientMainCpn)
/// - GET `EndPoint::SearchBoxHospText` (HospSearchboxCpn)
/// - POST `EndPoint::IpdSummary` (guarded, remove 'บันทึก' btn)
/// - PATCH `EndPoint::IpdSummary` (guarded, remove 'บันทึก' btn)
/// - POST `EndPoint::IpdSummaryNoteId `(SummaryNoteCpn, guarded, remove note-edit div)
/// - PATCH `EndPoint::IpdSummaryNoteId` (SummaryNoteCpn, guarded, remove 'บันทึก' btn)
/// - DELETE `EndPoint::IpdSummaryNoteId` (SummaryNoteCpn, guarded, remove 'ลบ' btn)
/// - GET `EndPoint::LabItem` (LabHistory, guarded, disable open lab-modal)
/// - PATCH `EndPoint::IpdSummaryStatusId`
#[derive(Default)]
pub struct SummaryPage {
    loaded: Mutable<bool>,

    view_by: Mutable<String>,
    an: Mutable<String>,
    // patient main
    patient: Mutable<Rc<ShowPatientMainCpn>>,

    // Dragable
    drag_state: Mutable<Option<DragState<Rc<Icd10>>>>,
    selected_draggables: Mutable<Vec<Rc<SelectedDragable<Rc<Icd10>>>>>,

    // form
    changed: Mutable<bool>,
    code_changed: Mutable<bool>,
    summary_id: Mutable<u32>,
    // summary_plan_date: Mutable<String>,
    // summary_plan_time: Mutable<String>,
    dagger_asterisk_state: Rc<DaggerAsteriskState>,

    // draggable dxs
    principal_diagnosis_group: Mutable<Rc<Group<Rc<Icd10>>>>,
    pre_admission_comorbidity_group: Mutable<Rc<Group<Rc<Icd10>>>>,
    post_admission_comorbidity_group: Mutable<Rc<Group<Rc<Icd10>>>>,
    other_diagnosis_group: Mutable<Rc<Group<Rc<Icd10>>>>,
    external_cause_group: Mutable<Rc<Group<Rc<Icd10>>>>,

    operating_room: Mutable<String>,
    tracheostomy: Mutable<String>,
    packed_redcells: Mutable<String>,
    fresh_frozen_plasma: Mutable<String>,
    platelets: Mutable<String>,
    cryoprecipitate: Mutable<String>,
    whole_blood: Mutable<String>,
    chemotherapy: Mutable<String>,
    hemodialysis: Mutable<String>,
    mechanical_ventilation: Mutable<String>,
    computer_tomography: Mutable<String>,
    computer_tomography_text: Mutable<String>,
    mri: Mutable<String>,
    mri_text: Mutable<String>,
    non_or_other: Mutable<String>,
    non_or_other_text: Mutable<String>,
    special_other: Mutable<String>,
    special_other_text: Mutable<String>,
    discharge_status: Mutable<String>,
    discharge_type: Mutable<String>,
    hospital_refer: Mutable<Option<Rc<HospSearchBox>>>,

    coder_name: Mutable<Option<String>>,
    principal_diagnosis_code: Mutable<Option<String>>,
    pre_admission_comorbidity_codes: Mutable<Option<String>>,
    post_admission_comorbidity_codes: Mutable<Option<String>>,
    other_diagnosis_codes: Mutable<Option<String>>,
    external_cause_codes: Mutable<Option<String>>,
    main_procedure_code: Mutable<Option<String>>,
    other_procedure_codes: Mutable<Option<String>>,
    status: Mutable<AuditStatus>,

    attending_doctors: MutableVec<Rc<DoctorData>>,
    attending_doctor_sign: Mutable<bool>,
    approve_doctors: MutableVec<Rc<DoctorData>>,
    approve_doctor_sign: Mutable<bool>,

    hosxp_operating_room_procedure: Mutable<Vec<Rc<HisOperationAdmit>>>,
    hosxp_ct: Mutable<Vec<Rc<XRayData>>>,
    hosxp_mri: Mutable<Vec<Rc<XRayData>>>,
    dch: Mutable<Option<Rc<DchData>>>,
    lab_alerts: MutableVec<Rc<LabAlertData>>,
    lab_history_modal: Mutable<Option<Rc<LabHistory>>>,
    problem_lists: MutableVec<String>,

    status_changed: Mutable<bool>,
}

impl SummaryPage {
    pub fn new(view_by: String, an: String) -> Rc<Self> {
        Rc::new(Self {
            view_by: Mutable::new(view_by),
            an: Mutable::new(an),
            // dragging: Dragging::new(),
            principal_diagnosis_group: Mutable::new(Rc::new(Group::new_single())),
            pre_admission_comorbidity_group: Mutable::new(Rc::new(Group::new())),
            post_admission_comorbidity_group: Mutable::new(Rc::new(Group::new())),
            other_diagnosis_group: Mutable::new(Rc::new(Group::new())),
            external_cause_group: Mutable::new(Rc::new(Group::new())),
            ..Default::default()
        })
    }

    fn is_valid_summary_id_signal(&self) -> impl Signal<Item = bool> + use<> {
        self.summary_id.signal_cloned().map(|summary_id| summary_id > 0)
    }

    fn hosxp_operating_room_procedure_signal(&self) -> impl Signal<Item = String> + use<> {
        self.hosxp_operating_room_procedure.signal_cloned().map(|op| {
            op.iter()
                .map(|or| {
                    let icd9 = or.icd9.as_ref().map(|icd9| [" [ICD9 : ", icd9, "]"].concat()).unwrap_or_default();
                    let doctor = or.doctor_name.as_ref().map(|name| [" [", name, "]"].concat()).unwrap_or_default();
                    [
                        &or.name.clone().unwrap_or_default(),
                        &icd9,
                        &doctor,
                        " (",
                        &datetime_th_opt(&or.begin_datetime),
                        " - ",
                        &datetime_th_opt(&or.end_datetime),
                        ")",
                    ]
                    .concat()
                })
                .collect::<Vec<String>>()
                .join("\n")
        })
    }

    fn hosxp_ct_signal(&self) -> impl Signal<Item = String> + use<> {
        self.hosxp_ct.signal_cloned().map(|hosxp_ct| {
            hosxp_ct
                .iter()
                .map(|ct| {
                    [
                        &ct.xray_items_name.clone().unwrap_or_default(),
                        " (",
                        &date_th_opt(&ct.examined_date),
                        " ",
                        &time_hm_opt(&ct.examined_time),
                        ")",
                    ]
                    .concat()
                })
                .collect::<Vec<String>>()
                .join("\n")
        })
    }

    fn hosxp_mri_signal(&self) -> impl Signal<Item = String> + use<> {
        self.hosxp_mri.signal_cloned().map(|hosxp_mri| {
            hosxp_mri
                .iter()
                .map(|mri| {
                    [
                        &mri.xray_items_name.clone().unwrap_or_default(),
                        " (",
                        &date_th_opt(&mri.examined_date),
                        " ",
                        &time_hm_opt(&mri.examined_time),
                        ")",
                    ]
                    .concat()
                })
                .collect::<Vec<String>>()
                .join("\n")
        })
    }

    // ipd-summary-2-data.php
    fn load(page: Rc<Self>, app: Rc<App>) {
        let an = str_some(page.an.get_cloned());
        if an.is_some() {
            let params = SummaryParams { an, ..Default::default() };
            app.async_load(
                true,
                clone!(app => async move {
                    // GET `EndPoint::IpdSummary`
                    match Summary::call_api_get(&params, app.state()).await {
                        Ok(response) => {
                            let hosxp_discharge_status = response.dch_data.as_ref().and_then(|dch| dch.dchstts.clone()).unwrap_or_default();
                            let hosxp_discharge_type = response.dch_data.as_ref().and_then(|dch| dch.dchtype.clone()).unwrap_or_default();
                            page.dch.set(response.dch_data.map(Rc::new));
                            if let Some(summary) = response.summary {
                                page.summary_id.set_neq(summary.summary_id);

                                let pdx = Dragable::new(Icd10::new(&summary.principal_diagnosis_icd10, &summary.principal_diagnosis));
                                page.principal_diagnosis_group.lock_ref().draggables.lock_mut().replace_cloned(vec![pdx]);
                                if let Some(code) = summary.principal_diagnosis_icd10.as_ref() {
                                    page.dagger_asterisk_state.insert_code(code);
                                }

                                page.operating_room.set_neq(summary.operating_room.clone().unwrap_or_default());
                                page.tracheostomy.set_neq(summary.tracheostomy.clone().unwrap_or_default());
                                page.mechanical_ventilation.set_neq(summary.mechanical_ventilation.clone().unwrap_or_default());
                                page.packed_redcells.set_neq(summary.packed_redcells.clone().unwrap_or_default());
                                page.fresh_frozen_plasma.set_neq(summary.fresh_frozen_plasma.clone().unwrap_or_default());
                                page.platelets.set_neq(summary.platelets.clone().unwrap_or_default());
                                page.cryoprecipitate.set_neq(summary.cryoprecipitate.clone().unwrap_or_default());
                                page.whole_blood.set_neq(summary.whole_blood.clone().unwrap_or_default());
                                page.computer_tomography.set_neq(summary.computer_tomography.clone().unwrap_or_default());
                                page.computer_tomography_text.set_neq(summary.computer_tomography_text.clone().unwrap_or_default());
                                page.chemotherapy.set_neq(summary.chemotherapy.clone().unwrap_or_default());
                                page.mri.set_neq(summary.mri.clone().unwrap_or_default());
                                page.mri_text.set_neq(summary.mri_text.clone().unwrap_or_default());
                                page.hemodialysis.set_neq(summary.hemodialysis.clone().unwrap_or_default());
                                page.non_or_other.set_neq(summary.non_or_other.clone().unwrap_or_default());
                                page.non_or_other_text.set_neq(summary.non_or_other_text.clone().unwrap_or_default());
                                page.special_other.set_neq(summary.special_other.clone().unwrap_or_default());
                                page.special_other_text.set_neq(summary.special_other_text.clone().unwrap_or_default());
                                page.discharge_status.set_neq(summary.discharge_status.clone().unwrap_or(hosxp_discharge_status));
                                page.discharge_type.set_neq(summary.discharge_type.clone().unwrap_or(hosxp_discharge_type));
                                page.hospital_refer.set(HospSearchBox::new(&summary.hospital_refer, &summary.hosptype, &summary.hospname));

                                page.coder_name.set_neq(summary.coder_name.clone());
                                page.principal_diagnosis_code.set_neq(summary.principal_diagnosis_code.clone());
                                page.pre_admission_comorbidity_codes.set_neq(summary.pre_admission_comorbidity_codes.clone());
                                page.post_admission_comorbidity_codes.set_neq(summary.post_admission_comorbidity_codes.clone());
                                page.other_diagnosis_codes.set_neq(summary.other_diagnosis_codes.clone());
                                page.external_cause_codes.set_neq(summary.external_cause_codes.clone());
                                page.main_procedure_code.set_neq(summary.main_procedure_code.clone());
                                page.other_procedure_codes.set_neq(summary.other_procedure_codes.clone());

                                let status = if let Some(s) = summary.status.as_ref() {
                                    AuditStatus::from_str(s).unwrap_or_default()
                                } else {
                                    let is_attended = response.doctor_data.iter().any(|dr| dr.ty == 1);
                                    let is_approved = response.doctor_data.iter().any(|dr| dr.ty == 2);
                                    match (is_attended, is_approved) {
                                        (true, true) => AuditStatus::Code,
                                        (true, false) => AuditStatus::Approve,
                                        (false, _) => AuditStatus::Null,
                                    }
                                };
                                page.status.set(status);

                            } else {
                                page.discharge_status.set_neq(hosxp_discharge_status);
                                page.discharge_type.set_neq(hosxp_discharge_type);
                                page.changed.set_neq(true);
                            }

                            let hosxp_ct = response.xray_data.iter().filter(|x| x.xray_items_group == Some(3)).cloned().map(Rc::new).collect::<Vec<Rc<XRayData>>>();
                            // if !hosxp_ct.is_empty() {page.computer_tomography.set_neq(String::from("Y"))};
                            page.hosxp_ct.set(hosxp_ct);

                            let hosxp_mri = response.xray_data.iter().filter(|x| x.xray_items_group == Some(4)).cloned().map(Rc::new).collect::<Vec<Rc<XRayData>>>();
                            // if !hosxp_mri.is_empty() {page.mri.set_neq(String::from("Y"))};
                            page.hosxp_mri.set(hosxp_mri);

                            page.hosxp_operating_room_procedure.lock_mut().extend(response.or_data.into_iter().map(Rc::new));

                            let dx2s = response.dx_data.iter().filter(|dx| dx.ty == 2).map(Icd10::from).map(Rc::new).map(|d| Dragable::new(Some(d)));
                            page.pre_admission_comorbidity_group.lock_ref().draggables.lock_mut().extend(dx2s);
                            let dx3s = response.dx_data.iter().filter(|dx| dx.ty == 3).map(Icd10::from).map(Rc::new).map(|d| Dragable::new(Some(d)));
                            page.post_admission_comorbidity_group.lock_ref().draggables.lock_mut().extend(dx3s);
                            let dx4s = response.dx_data.iter().filter(|dx| dx.ty == 4).map(Icd10::from).map(Rc::new).map(|d| Dragable::new(Some(d)));
                            page.other_diagnosis_group.lock_ref().draggables.lock_mut().extend(dx4s);
                            let dx5s = response.dx_data.iter().filter(|dx| dx.ty == 5).map(Icd10::from).map(Rc::new).map(|d| Dragable::new(Some(d)));
                            page.external_cause_group.lock_ref().draggables.lock_mut().extend(dx5s);
                            for dx in response.dx_data.iter() {
                                if let Some(code) = dx.icd.as_ref() {
                                    page.dagger_asterisk_state.insert_code(code);
                                }
                            }
                            page.dagger_asterisk_state.start_parsing();

                            page.lab_alerts.lock_mut().extend(response.lab_alert_data.into_iter().map(Rc::new));
                            page.problem_lists.lock_mut().replace_cloned(response.problem_list_data);

                            page.attending_doctors.lock_mut().extend(response.doctor_data.iter().filter(|dr| dr.ty == 1).cloned().map(Rc::new));
                            page.approve_doctors.lock_mut().extend(response.doctor_data.iter().filter(|dr| dr.ty == 2).cloned().map(Rc::new));
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }),
            )
        }
    }

    fn create_procs(&self) -> Vec<String> {
        // Format: `Operation (ICD-9) [Oparator] (Start - End)`
        let mut procs = self
            .operating_room
            .lock_ref()
            .as_str()
            .split('\n')
            .map(|line| {
                line.split(' ')
                    .find(|word| word.starts_with('(') && word.ends_with(')') && word.contains('.'))
                    .map(|code| code.trim_start_matches('(').trim_end_matches(')').replace('.', ""))
            })
            .flatten()
            .collect::<Vec<String>>();
        if self.tracheostomy.lock_ref().as_str() == "Y" {
            procs.push("311".to_owned());
        }
        if self.packed_redcells.lock_ref().as_str() == "Y" {
            procs.push("9904".to_owned());
        }
        if self.fresh_frozen_plasma.lock_ref().as_str() == "Y" {
            procs.push("9907".to_owned());
        }
        if self.platelets.lock_ref().as_str() == "Y" {
            procs.push("9905".to_owned());
        }
        if self.cryoprecipitate.lock_ref().as_str() == "Y" {
            procs.push("9906".to_owned());
        }
        if self.whole_blood.lock_ref().as_str() == "Y" {
            procs.push("9903".to_owned());
        }
        if self.chemotherapy.lock_ref().as_str() == "Y" {
            procs.push("9925".to_owned());
        }
        if self.hemodialysis.lock_ref().as_str() == "Y" {
            procs.push("3995".to_owned());
        }
        match self.mechanical_ventilation.lock_ref().as_str() {
            "1" => {
                procs.push("9672".to_owned());
            }
            "2" => {
                procs.push("9671".to_owned());
            }
            "3" => {
                procs.push("9390".to_owned());
            }
            _ => {}
        }

        procs
    }

    fn check_drg(is_coder: bool, page: Rc<Self>, app: Rc<App>) {
        if let Some(patient) = page.patient.lock_ref().patient.get_cloned() {
            app.async_load(
                true,
                clone!(app, page => async move {
                    let gender = patient.sex.to_owned();
                    let dob = patient.birthday.map(|d| PrimitiveDateTime::new(d, Time::MIDNIGHT));
                    let adm_wt = patient.bw.map(|bw| bw as u16);
                    let adm_date = datetime_from_opt(patient.regdate, patient.regtime);
                    let dch_date = datetime_from_opt(patient.dchdate, patient.dchtime).or(Some(js_now()));
                    let dch_type = patient.dchtype.to_owned().unwrap_or(String::from("01"));
                    let leave_day = patient.leave_home_day.map(|d| d as u32).unwrap_or_default();

                    let input_build = if is_coder {
                        let pdx = page.principal_diagnosis_code.lock_ref().as_ref().map(|s| s.trim().replace('.',"")).unwrap_or_default();
                        let mut sdxs = Vec::new();
                        sdxs.extend(page.pre_admission_comorbidity_codes.lock_ref().as_ref().map(|ss| ss.split(",").map(|s| s.trim().replace('.',"").to_owned()).collect::<Vec<String>>()).unwrap_or_default());
                        sdxs.extend(page.post_admission_comorbidity_codes.lock_ref().as_ref().map(|ss| ss.split(",").map(|s| s.trim().replace('.',"").to_owned()).collect::<Vec<String>>()).unwrap_or_default());
                        let mut procs = Vec::new();
                        let proc_main = page.main_procedure_code.lock_ref().as_ref().map(|s| s.trim().replace('.',"")).unwrap_or_default();
                        if !proc_main.is_empty() {
                            procs.push(proc_main);
                        }
                        procs.extend(page.other_procedure_codes.lock_ref().as_ref().map(|ss| ss.split(",").map(|s| s.trim().replace('.',"").to_owned()).collect::<Vec<String>>()).unwrap_or_default());

                        GrouperInput::new(&pdx, &sdxs, &procs, &gender, dob, adm_wt, adm_date, dch_date, &dch_type, leave_day)
                    } else {
                        let pdx = page
                            .principal_diagnosis_group
                            .lock_ref()
                            .draggables
                            .lock_ref()
                            .first()
                            .and_then(|d| d.state.lock_ref().as_ref().map(|dx| dx.icd10.clone()))
                            .unwrap_or_default();
                        let mut sdxs = Vec::new();
                        sdxs.extend(group_to_vec_code(page.pre_admission_comorbidity_group.get_cloned()));
                        sdxs.extend(group_to_vec_code(page.post_admission_comorbidity_group.get_cloned()));
                        let procs = page.create_procs();

                        GrouperInput::new(&pdx, &sdxs, &procs, &gender, dob, adm_wt, adm_date, dch_date, &dch_type, leave_day)
                    };

                    match input_build {
                        Ok(input) => {
                            if let Ok(input_json) = serde_json::to_string(&input) {
                                let output_json = app.drg_worker().await.run(input_json).await;
                                if let Ok(output) = serde_json::from_str::<GrouperOutput>(&output_json) {
                                    if output.errors.is_empty() {
                                        let drgs_report = output.drg.iter().map(|drg| {
                                            let adj_rw = f32_rescale(drg.adj_rw(), 5);
                                            let drg_data = &drg.drg;
                                            let input_data = &drg.source;
                                            let los_min = if drg_data.wtlos > 3.0 {f32_rescale(drg_data.wtlos / 3.0, 2)} else {1.0};
                                            let sdxs = input_data.sdxs.clone().into_iter().collect::<Vec<String>>().join(", ");
                                            let procs = input_data.procs.clone().into_iter().collect::<Vec<String>>().join(", ");
                                            [
                                                "    - AdjRW: ", &adj_rw.to_string(),
                                                "\n    - DRG: ", &drg_data.drg, " ", &drg_data.detail,
                                                "\n    - RW: ", &drg_data.rw.to_string(),
                                                "\n    - WtLOS: ", &drg_data.wtlos.to_string(),
                                                "\n    - OT: ", &drg_data.ot.to_string(),
                                                "\n    - PDx: ", &input_data.pdx,
                                                "\n    - SDx: ", &if sdxs.is_empty() {String::from("ไม่มี")} else {sdxs},
                                                "\n    - Proc: ", &if procs.is_empty() {String::from("ไม่มี")} else {procs},
                                                "\n    - LOS: ", &input_data.los.to_string(), " วัน",
                                                "\n    - วันนอนขั้นต่ำ: ", &los_min.to_string(), " วัน (AdjRw ", &f32_rescale(drg_data.adj_rw(los_f32_to_u32(los_min)), 5).to_string(), ")",
                                                "\n    - วันนอนขั้นสูง 1: ", &drg_data.ot.to_string(), " วัน (AdjRw ", &f32_rescale(drg_data.adj_rw(los_f32_to_u32(drg_data.ot)), 5).to_string(), ")",
                                                "\n    - วันนอนขั้นสูง 2: ", &(drg_data.ot * 2.0).to_string(), " วัน (AdjRw ", &f32_rescale(drg_data.adj_rw(los_f32_to_u32(drg_data.ot * 2.0)), 5).to_string(),")",
                                                "\n    - วันนอนขั้นสูง 3: ", &(drg_data.ot * 3.0).to_string(), " วัน (AdjRw ", &f32_rescale(drg_data.adj_rw(los_f32_to_u32(drg_data.ot * 3.0)), 5).to_string(), ")",
                                            ].concat()
                                        }).collect::<Vec<String>>().join("\n");
                                        app.alert_with_close("ผลการตรวจสอบ", &drgs_report, false).await;
                                    } else {
                                        app.alert_with_close("ผลการตรวจสอบ", &output.errors.iter().map(|e| e.string()).collect::<Vec<String>>().join("\n"), true).await;
                                    }
                                }
                            }
                        }
                        Err(errors) => {
                            app.alert_with_close("ผลการตรวจสอบ", &errors.iter().map(|e| e.string()).collect::<Vec<String>>().join("\n"), true).await;
                        }
                    }
                }),
            )
        }
    }

    async fn validate(page: Rc<Self>, app: Rc<App>) -> bool {
        let mut messages = Vec::with_capacity(3);
        if let Some(hosxp_dch) = page.dch.lock_ref().as_ref() {
            if hosxp_dch.dchdate.is_some() {
                if hosxp_dch.dchstts.as_ref().map(|stts| stts != &page.discharge_status.lock_ref().as_str()).unwrap_or_default() {
                    messages.push("Discharge Status ไม่ตรงกับข้อมูลใน HOSxP");
                }
                if hosxp_dch.dchtype.as_ref().map(|stts| stts != &page.discharge_type.lock_ref().as_str()).unwrap_or_default() {
                    messages.push("Discharge Type ไม่ตรงกับข้อมูลใน HOSxP");
                }
                if page.discharge_type.lock_ref().as_str() == "04" && page.hospital_refer.lock_ref().is_none() {
                    messages.push("ไม่ได้ระบุ ชื่อสถานพยาบาลที่ส่งต่อ")
                }
            } else {
                messages.push("ยังไม่ได้ D/C")
            }
        }

        if !messages.is_empty() {
            app.confirm(&[&messages.join("\n"), "\nยืนยันบันทึกข้อมูลหรือไม่"].concat()).await
        } else {
            true
        }
    }

    fn parse_data(page: &Rc<Self>) -> SummaryData {
        let (principal_diagnosis_icd10, principal_diagnosis) = page
            .principal_diagnosis_group
            .lock_ref()
            .draggables
            .lock_ref()
            .first()
            .and_then(|d| d.state.lock_ref().as_ref().map(|dx| (str_some(dx.icd10.clone()), dx.ename.clone())))
            .unwrap_or_default();
        let hospital_refer = page.hospital_refer.lock_ref();
        SummaryData {
            summary_id: page.summary_id.get(),
            an: page.an.get_cloned(),
            principal_diagnosis,
            principal_diagnosis_icd10,
            operating_room: str_some(page.operating_room.get_cloned()),
            tracheostomy: str_some(page.tracheostomy.get_cloned()),
            mechanical_ventilation: str_some(page.mechanical_ventilation.get_cloned()),
            packed_redcells: str_some(page.packed_redcells.get_cloned()),
            fresh_frozen_plasma: str_some(page.fresh_frozen_plasma.get_cloned()),
            platelets: str_some(page.platelets.get_cloned()),
            cryoprecipitate: str_some(page.cryoprecipitate.get_cloned()),
            whole_blood: str_some(page.whole_blood.get_cloned()),
            computer_tomography: str_some(page.computer_tomography.get_cloned()),
            computer_tomography_text: str_some(page.computer_tomography_text.get_cloned()),
            chemotherapy: str_some(page.chemotherapy.get_cloned()),
            mri: str_some(page.mri.get_cloned()),
            mri_text: str_some(page.mri_text.get_cloned()),
            hemodialysis: str_some(page.hemodialysis.get_cloned()),
            non_or_other: str_some(page.non_or_other.get_cloned()),
            non_or_other_text: str_some(page.non_or_other_text.get_cloned()),
            special_other: str_some(page.special_other.get_cloned()),
            special_other_text: str_some(page.special_other_text.get_cloned()),
            discharge_status: str_some(page.discharge_status.get_cloned()),
            discharge_type: str_some(page.discharge_type.get_cloned()),
            hospital_refer: hospital_refer.as_ref().map(|hosp| hosp.id.clone()),
            hosptype: hospital_refer.as_ref().and_then(|h| h.hosptype.clone()),
            hospname: hospital_refer.as_ref().and_then(|h| h.hospname.clone()),
            coder_name: page.coder_name.get_cloned(),
            principal_diagnosis_code: page.principal_diagnosis_code.get_cloned(),
            pre_admission_comorbidity_codes: page.pre_admission_comorbidity_codes.get_cloned(),
            post_admission_comorbidity_codes: page.post_admission_comorbidity_codes.get_cloned(),
            other_diagnosis_codes: page.other_diagnosis_codes.get_cloned(),
            external_cause_codes: page.external_cause_codes.get_cloned(),
            main_procedure_code: page.main_procedure_code.get_cloned(),
            other_procedure_codes: page.other_procedure_codes.get_cloned(),
            status: page.status.lock_ref().as_data(),
        }
    }

    fn parse_save(page: &Rc<Self>) -> SummaryDataSave {
        let (principal_diagnosis_icd10, principal_diagnosis) = page
            .principal_diagnosis_group
            .lock_ref()
            .draggables
            .lock_ref()
            .first()
            .and_then(|d| d.state.lock_ref().as_ref().map(|dx| (str_some(dx.icd10.clone()), dx.ename.clone())))
            .unwrap_or_default();
        // status CODE, REVIEW, AUDIT, CLAIM, DONE will turn to CODE/APPROVE/NULL
        // read `kphis_model::ipd::summary::sql_where_having` for more information
        let status = match (page.attending_doctors.lock_ref().is_empty(), page.approve_doctors.lock_ref().is_empty()) {
            (false, false) => AuditStatus::Code,
            (false, true) => AuditStatus::Approve,
            (true, _) => AuditStatus::Null,
        };
        SummaryDataSave {
            summary_id: page.summary_id.get(),
            an: page.an.get_cloned(),
            principal_diagnosis,
            principal_diagnosis_icd10,
            operating_room: str_some(page.operating_room.get_cloned()),
            tracheostomy: str_some(page.tracheostomy.get_cloned()),
            mechanical_ventilation: str_some(page.mechanical_ventilation.get_cloned()),
            packed_redcells: str_some(page.packed_redcells.get_cloned()),
            fresh_frozen_plasma: str_some(page.fresh_frozen_plasma.get_cloned()),
            platelets: str_some(page.platelets.get_cloned()),
            cryoprecipitate: str_some(page.cryoprecipitate.get_cloned()),
            whole_blood: str_some(page.whole_blood.get_cloned()),
            computer_tomography: str_some(page.computer_tomography.get_cloned()),
            computer_tomography_text: str_some(page.computer_tomography_text.get_cloned()),
            chemotherapy: str_some(page.chemotherapy.get_cloned()),
            mri: str_some(page.mri.get_cloned()),
            mri_text: str_some(page.mri_text.get_cloned()),
            hemodialysis: str_some(page.hemodialysis.get_cloned()),
            non_or_other: str_some(page.non_or_other.get_cloned()),
            non_or_other_text: str_some(page.non_or_other_text.get_cloned()),
            special_other: str_some(page.special_other.get_cloned()),
            special_other_text: str_some(page.special_other_text.get_cloned()),
            discharge_status: str_some(page.discharge_status.get_cloned()),
            discharge_type: str_some(page.discharge_type.get_cloned()),
            hospital_refer: page.hospital_refer.lock_ref().as_ref().map(|hosp| hosp.id.clone()),
            status: status.as_data(),
        }
    }

    fn finalized(page: Rc<Self>) -> SummarySave {
        SummarySave {
            summary: Self::parse_save(&page),
            dx2_data: group_to_dxdatas(page.pre_admission_comorbidity_group.get_cloned(), 2),
            dx3_data: group_to_dxdatas(page.post_admission_comorbidity_group.get_cloned(), 3),
            dx4_data: group_to_dxdatas(page.other_diagnosis_group.get_cloned(), 4),
            dx5_data: group_to_dxdatas(page.external_cause_group.get_cloned(), 5),
            attending_doctor: page.attending_doctor_sign.get(),
            approve_doctor: page.approve_doctor_sign.get(),
        }
    }

    // ipd-summary-2-save.php
    fn save_summary(page: Rc<Self>, app: Rc<App>) {
        let is_lock = page.status.lock_ref().is_summary_locked();
        if is_lock {
            app.alert_error("รายการนี้ ไม่สามารถแก้ไขได้ !!!", "รายการนี้ อยู่ระหว่างส่งเคลม หรือสิ้นสุดแล้ว ไม่สามารถแก้ไขได้");
        } else {
            app.async_load(
                true,
                clone!(app => async move {
                    let is_validate = Self::validate(page.clone(), app.clone()).await;
                    if is_validate {
                        let saver = Self::finalized(page.clone());
                        let status_opt = saver.summary.status.as_ref().and_then(|s| AuditStatus::from_str(s).ok());
                        // POST `EndPoint::IpdSummary`
                        match saver.call_api_post(app.state()).await {
                            Ok((id, responses)) => {
                                app.alert_execute_responses(&responses, clone!(app, page => async move {
                                    // app.alert("บันทึกข้อมูลสำเร็จ");
                                    page.summary_id.set(id);
                                    page.attending_doctor_sign.set_neq(false);
                                    page.approve_doctor_sign.set_neq(false);
                                    if let Some(status) = status_opt {
                                        page.status.set(status);
                                    }
                                    app.get_post_admit_count().await;
                                    page.changed.set(false);
                                })).await;
                            }
                            Err(e) => {
                                app.alert_app_error(&e).await;
                            }
                        }
                    }
                }),
            )
        }
    }

    fn save_code(page: Rc<Self>, app: Rc<App>) {
        if let (Some(summary_id), Some(an)) = (zero_none(page.summary_id.get()), str_some(page.an.get_cloned())) {
            app.async_load(
                true,
                clone!(app => async move {
                    let status = page.status.get_cloned();
                    let saver = SummaryCodeSave {
                        summary_id,
                        an,
                        coder_name: page.coder_name.get_cloned(),
                        principal_diagnosis_code: page.principal_diagnosis_code.get_cloned(),
                        pre_admission_comorbidity_codes: page.pre_admission_comorbidity_codes.get_cloned(),
                        post_admission_comorbidity_codes: page.post_admission_comorbidity_codes.get_cloned(),
                        other_diagnosis_codes: page.other_diagnosis_codes.get_cloned(),
                        external_cause_codes: page.external_cause_codes.get_cloned(),
                        main_procedure_code: page.main_procedure_code.get_cloned(),
                        other_procedure_codes: page.other_procedure_codes.get_cloned(),
                        status: status.as_data(),
                    };
                    // PATCH `EndPoint::IpdSummary`
                    match saver.call_api_patch(app.state()).await {
                        Ok(response) => {
                            app.alert_execute_response(&response, clone!(app, page => async move {
                                // app.alert("บันทึกข้อมูลสำเร็จ");
                                page.code_changed.set(false);
                                if matches!(status, AuditStatus::Review) {
                                    Self::send_review_sse(page.clone(), app.clone());
                                }
                            })).await;
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }),
            )
        }
    }

    fn send_review_sse(page: Rc<Self>, app: Rc<App>) {
        let an = page.an.get_cloned();
        let mut doctors = HashSet::new();
        doctors.extend(page.attending_doctors.lock_ref().iter().map(|d| d.doctor.clone()));
        doctors.extend(page.approve_doctors.lock_ref().iter().map(|d| d.doctor.clone()));
        for doctor in doctors {
            let message = SsePostMessage {
                message: ["AN: ", &an, " รอแพทย์ทบทวนการสรุปเวชระเบียน"].concat(),
                person: Some(doctor),
                route: Some(Route::Summary {
                    view_by: String::from("doctor"),
                    an: an.clone(),
                }),
                ..Default::default()
            };
            app.send_sse(message);
        }
    }

    fn update_status(page: Rc<Self>, app: Rc<App>) {
        if let Some(summary_id) = zero_none(page.summary_id.get()) {
            let status = page.status.get_cloned();
            app.async_load(
                true,
                clone!(app => async move {
                    let saver = SummaryStatus { status: status.as_data() };
                    // PATCH `EndPoint::IpdSummaryStatusId`
                    match saver.call_api_put(summary_id, app.state()).await {
                        Ok(response) => {
                            app.alert_execute_response(&response, clone!(app => async move {
                                // app.alert("บันทึกข้อมูลสำเร็จ");
                                page.status_changed.set(false);
                            })).await;
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }),
            )
        }
    }

    pub fn render(page: Rc<Self>, app: Rc<App>) -> Dom {
        app.set_title("KPHIS - IPD Summary");

        let show_patient_main = ShowPatientMainCpn::new_with_an(page.an.get_cloned());
        let hn = show_patient_main.hn.clone();
        let patient_main = ShowPatientMainCpn::render(false, show_patient_main.clone(), app.clone());
        page.patient.set(show_patient_main);

        html!("div", {
            .children([
                html!("style", {.text(SUMMARY_STYLE)}),
                // patient panel
                patient_main,
            ])
            .child_signal(window_size().map(|ws| ws.width < SCREEN_WIDTH_EXTRA).dedupe().map(move |is_not_wide| {
                Some(if is_not_wide {
                    Self::render_form(page.clone(), app.clone())
                } else {
                    // aside_resizer
                    let report_selected = SystemReport::new(&app.report_select.lock_ref()).or(Some(SystemReport::IpdOrder));
                    AsideResizerCpn::render(
                        Self::render_form(page.clone(), app.clone()),
                        Some((true, page.patient.lock_ref().patient.clone())),
                        AsideResizerCpn::new(
                            Mutable::new(report_selected), Mutable::new(false),
                            Mutable::new(None), Mutable::new(false),
                            page.an.clone(), hn.clone(), SystemReport::ipd_set(),
                            "summary-main", None, None, app.clone(),
                        ),
                        app.clone(),
                    )
                })
            }))
        })
    }

    pub fn render_form(page: Rc<Self>, app: Rc<App>) -> Dom {
        let is_auditor = app.has_permission(Permission::DataTypeAuditorUse);
        let is_pre_admit = app.is_pre_admit(&page.an.lock_ref());
        let can_save_summary =
            page.view_by.lock_ref().deref() == "doctor" && app.has_permission(Permission::DataTypeDoctorUse) && app.endpoint_is_allow(&Method::POST, &EndPoint::IpdSummary, is_pre_admit);

        // // for test UI
        // let is_auditor = false;
        // let can_save_summary = true;

        html!("div", {
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let loaded = page.loaded.signal() =>
                !busy && !loaded
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load(page.clone(), app.clone());
                    page.loaded.set(true);
                }
                async {}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let is_parsing_dagger_asterisk = page.dagger_asterisk_state.is_parsing_signal() =>
                !busy && *is_parsing_dagger_asterisk
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    DaggerAsteriskState::parse_codes(page.dagger_asterisk_state.clone(), app.clone());
                }
                async {}
            })))
            .apply_if(can_save_summary, |dom| { dom
                // TODO: attach this only when dragging
                .global_event(clone!(page => move |_: events::MouseUp| {
                    drag_end(
                        page.drag_state.clone(),
                        page.selected_draggables.clone(),
                        &[page.principal_diagnosis_group.clone()],
                        &[  page.pre_admission_comorbidity_group.clone(),
                            page.post_admission_comorbidity_group.clone(),
                            page.other_diagnosis_group.clone(),
                            page.external_cause_group.clone(),
                        ],
                        page.changed.clone(),
                    );
                }))
                // According to the spec, a "mousemove" event has no default action but some browser will highlight text
                .global_event_with_options(&EventOptions::preventable(), clone!(page => move |e: events::MouseMove| {
                    e.prevent_default();
                    drag_move(e.mouse_x(), e.mouse_y(), page.drag_state.clone(), page.selected_draggables.clone());
                }))
            })
            //.attr("id", "ipd_summary_form")
            .class(class::CONF_B)
            .attr("id", "summary-main")
            .style("min-width",LEFT_PANEL_MIN_WIDTH)
            .children([
                html!("style", { .text(DRAGGABLE_STYLE)}),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        html!("div", {
                            .class("col-auto")
                            .child(html!("button", {
                                .attr("type", "button")
                                .class(class::BTN_BLUE)
                                .child(html!("i", {.class(class::FA_L_ARROW)}))
                                .text(" กลับ")
                                .event(clone!(app, page => move |_: events::Click| {
                                    if app.go_back_else() {
                                        let route = Route::IpdMain {
                                            view_by: page.view_by.get_cloned(),
                                            an: page.an.get_cloned(),
                                            tab: Tab::Order.str().to_owned(),
                                            sub: String::new(),
                                            id: 0,
                                        };
                                        if route.has_permission(app.state()) {
                                            route.hard_redirect();
                                        } else {
                                            Route::Info.hard_redirect();
                                        }
                                    }
                                }))
                            }))
                        }),
                        html!("div", {
                            .class(class::COLA_BOLD_P2)
                            .text("IN-PATIENT-SUMMARY")
                        }),
                        html!("div", {
                            .class("col-auto")
                            .child_signal(page.status.signal_cloned().map(|status| {
                                Some(html!("span", {
                                    .class(class::INPUT_GROUP_TEXT_BOLD)
                                    .class(status.color_class())
                                    .text(status.status_text())
                                }))
                            }))
                        }),
                    ])
                }),
                html!("hr"),
                // Problem list, Lab alert, (Auditor note: after has summary_id)
                html!("div", {
                    .class(class::ROW_T)
                    .child(html!("div", {
                        .style("column-width","480px")
                        .style("column-gap","8px")
                        // .class(class::ROW_COL_RESP_G2)
                        // .class_signal("row-cols-xl-3", page.is_valid_summary_id_signal())
                        // .class_signal("row-cols-md-2", not(page.is_valid_summary_id_signal()))
                        .children([
                            // Problem List
                            html!("div", {
                                .class("col")
                                .child(render_problem_list(page.problem_lists.clone()))
                            }),
                            // Lab Alert
                            html!("div", {
                                .class("col")
                                .child(render_lab_alert(page.lab_alerts.clone(), page.patient.lock_ref().hn.clone(), page.lab_history_modal.clone(), app.clone()))
                            }),
                        ])
                        // Auditor Note
                        .child_signal(page.summary_id.signal().map(clone!(app, page => move |summary_id| {
                            zero_none(summary_id).map(|id| {
                                SummaryNoteCpn::render(is_pre_admit, SummaryNoteCpn::new(id), app.clone())
                            })
                        })))
                        .apply_if(can_save_summary, |dom| dom
                            .child_signal(map_ref! {
                                let has_summary_id = page.is_valid_summary_id_signal(),
                                let is_changed = page.status_changed.signal(),
                                let is_review = page.status.signal_cloned().map(|s| matches!(s, AuditStatus::Review)) =>
                                *has_summary_id && (*is_review || *is_changed)
                            }.map(clone!(app, page => move |ready| {
                                ready.then(|| {
                                    Self::render_review_status(page.clone(), app.clone())
                                })
                            })))
                        )
                    }))
                }),
                html!("div", {
                    .class(class::ROW_T)
                    .child(html!("div", {
                        .style("column-width","720px")
                        .style("column-gap","8px")
                        // .apply_if(is_auditor, |dom| { dom
                        //     .class_signal("row-cols-xl-2", page.is_valid_summary_id_signal())
                        // })
                        // Diagnosis (1)-(5)
                        .child(Self::render_diagnosis(can_save_summary, page.clone(), app.clone()))
                        // CODER
                        .apply_if(is_auditor, clone!(app, page => move |dom| { dom
                            .child_signal(page.is_valid_summary_id_signal().map(clone!(page => move |is_valid| {
                                is_valid.then(|| {
                                    Self::render_coder(is_pre_admit, page.clone(), app.clone())
                                })
                            })))
                        }))
                    }))
                }),
                // OPERATING ROOM PROCEDURES
                Self::render_or_procs(can_save_summary, page.clone()),
                // NON OPERATING ROOM PROCEDURES
                Self::render_nor_procs(can_save_summary, page.clone()),
                // SPECIAL INVESTIGATIONS
                Self::render_specials(can_save_summary, page.clone()),
                // DISCHARGE STATUS
                Self::render_dch(can_save_summary, page.clone(), app.clone()),
                Self::render_signers(can_save_summary, page.clone(), app.clone()),
                html!("hr"),
                html!("div", {
                    .class("row")
                    .children([
                        html!("div", {
                            .class(class::COL_R)
                            .children([
                                html!("div", {
                                    .class("float-start")
                                    .children(PdfButtons::buttons(
                                        PdfButtons::new(
                                            TypstReport::from_system_with_coercion(SystemReport::IpdSummary, &app.state().report_coercions()),
                                            page.an.clone(),
                                            page.summary_id.clone(),
                                            page.changed.clone(),
                                            clone!(page => move || {
                                                let dx2 = group_to_dxdatas(page.pre_admission_comorbidity_group.get_cloned(), 2);
                                                let dx3 = group_to_dxdatas(page.post_admission_comorbidity_group.get_cloned(), 3);
                                                let dx4 = group_to_dxdatas(page.other_diagnosis_group.get_cloned(), 4);
                                                let dx5 = group_to_dxdatas(page.external_cause_group.get_cloned(), 5);
                                                let dx = [dx2, dx3, dx4, dx5].concat();
                                                let attending = page.attending_doctors.lock_ref().to_vec();
                                                let approve = page.approve_doctors.lock_ref().to_vec();
                                                let doctor = [attending, approve].concat();
                                                serde_json::json!({
                                                    "id": page.an.get_cloned(),
                                                    "patient": page.patient.lock_ref().patient.get_cloned(),
                                                    "summary": Self::parse_data(&page),
                                                    "dx": dx,
                                                    // "op": op,
                                                    // "x34" : x34,
                                                    "doctor": doctor,
                                                }).to_string()
                                            })
                                        ), "Print PDF", None, app.clone()
                                    ))
                                }),
                                html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_L_BLUE)
                                    .child(html!("i", {.class(class::FA_L_ARROW)}))
                                    .text(" กลับ")
                                    .event(clone!(app => move |_: events::Click| {
                                        if app.go_back_else() {
                                            for route in [Route::IpdSearchPatientDr, Route::IpdSearchPatientNurse, Route::IpdSearchPatientPharmacist, Route::IpdSearchPatientOther, Route::Info] {
                                                if route.has_permission(app.state()) {
                                                    route.hard_redirect();
                                                    break;
                                                }
                                            }
                                        }
                                    }))
                                }),
                            ])
                            .apply_if(can_save_summary, |dom| {
                                dom.child(html!("button" => HtmlButtonElement, {
                                    .attr("type", "button")
                                    .class(class::BTN_R)
                                    .class_signal("btn-primary", page.changed.signal())
                                    .class_signal("btn-secondary", not(page.changed.signal()))
                                    //.attr("id", "btn_save_summary")
                                    .child(html!("i", {.class(class::FA_SAVE)}))
                                    .text(" บันทึก")
                                    .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, page => move || {
                                        Self::save_summary(page.clone(), app.clone());
                                    }), not(page.changed.signal()), app.state()))
                                    // .attr("onclick", "summary_save()")
                                }))
                            })
                        }),
                    ])
                }),
                html!("br"),
                html!("div", {
                    .class("modal")
                    .attr("id", "labHistoryModal")
                    .attr("role", "dialog")
                    .attr("tabindex", "-1")
                    .child_signal(page.lab_history_modal.signal_cloned().map(clone!(app => move |opt| {
                        opt.as_ref().map(clone!(app => move |modal| {
                            LabHistory::render(modal.clone(), app, None)
                        })).or(Some(blank_modal()))
                    })))
                }),
            ])
        })
    }

    fn render_diagnosis(can_save_summary: bool, page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .class("col")
            .style("break-inside","avoid")
            .child(html!("div", {
                .class(class::CARD)
                .children([
                    html!("div", {
                        .class("card-header")
                        .child(html!("div", {.class("fw-bold").text("DIAGNOSIS")}))
                    }),
                    html!("div", {
                        .class("card-body")
                        .apply_if(can_save_summary, |dom| dom
                            .children_signal_vec(page.is_valid_summary_id_signal().map(|is_valid| {
                                if is_valid {
                                    vec![
                                        doms::badge_info_center("สรุปเป็นคำวินิจฉัยโรค (clinical term) ไม่สรุปเป็นคำวินิจฉัยตามการให้รหัส ICD"),
                                        doms::badge_info_center("ไม่ใช้ตัวย่อในการสรุป (ยกเว้นตัวย่อตาม WHO ICD 10 และ ICD 9 CM)"),
                                        html!("p"),
                                    ]
                                } else {
                                    vec![
                                        doms::badge_info_center("สรุปเป็นคำวินิจฉัยโรค (clinical term) ไม่สรุปเป็นคำวินิจฉัยตามการให้รหัส ICD และไม่ใช้ตัวย่อในการสรุป (ยกเว้นตัวย่อตาม WHO ICD 10 และ ICD 9 CM)"),
                                        html!("p"),
                                    ]
                                }
                            }).to_signal_vec())
                        )
                        .children([
                            // (1) PRINCIPAL DIAGNOSIS
                            html!("div", {
                                .class("row")
                                .child(html!("div", {
                                    .class("col")
                                    .children([
                                        html!("div", {
                                            .class("row")
                                            .child(html!("div", {
                                                .class("col")
                                                .child(html!("label", {
                                                    .class("fw-bold")
                                                    .style("user-select","none")
                                                    .text("(1) PRINCIPAL DIAGNOSIS")
                                                }))
                                            }))
                                        }),
                                        html!("div", {
                                            .class("row")
                                            .child(html!("div", {
                                                .class("col")
                                                .child(html!("ul", {
                                                    .style("list-style-type","none")
                                                    .children_signal_vec(page.principal_diagnosis_group.lock_ref().draggables.signal_vec_cloned().map(clone!(app, page => move |draggable| {
                                                        Dragable::render(
                                                            page.principal_diagnosis_group.get_cloned(),
                                                            draggable.clone(),
                                                            page.drag_state.clone(),
                                                            DxSearchboxCpn::render(
                                                                DxSearchboxCpn::new(false, true, draggable.state.clone(), page.dagger_asterisk_state.clone()),
                                                                app.clone(),
                                                                None, // make DxSearchboxCpn cannot be removed
                                                                page.changed.clone(),
                                                                can_save_summary,
                                                            ),
                                                            can_save_summary,
                                                        )
                                                    })))
                                                    .apply(Group::group_mixins(page.principal_diagnosis_group.get_cloned(), page.drag_state.clone()))
                                                }))
                                            }))
                                        }),
                                    ])
                                }))
                            }),
                            // (2) PRE ADMISSION COMORBIDITY
                            html!("div", {
                                .class("row")
                                .child(html!("div", {
                                    .class("col")
                                    .children([
                                        render_add_draggable("(2) PRE ADMISSION COMORBIDITY", page.pre_admission_comorbidity_group.get_cloned(), can_save_summary, page.changed.clone()),
                                        html!("div", {
                                            .class("row")
                                            .child(html!("div", {
                                                .class("col")
                                                .child(html!("ol", {
                                                    .children_signal_vec(page.pre_admission_comorbidity_group.lock_ref().draggables.signal_vec_cloned().map(clone!(app, page => move |draggable| {
                                                        let group = page.pre_admission_comorbidity_group.get_cloned();
                                                        Dragable::render(
                                                            group.clone(),
                                                            draggable.clone(),
                                                            page.drag_state.clone(),
                                                            DxSearchboxCpn::render(
                                                                DxSearchboxCpn::new(false, true, draggable.state.clone(), page.dagger_asterisk_state.clone()),
                                                                app.clone(),
                                                                Some((group, draggable.id, draggable.selected.clone())), // make DxSearchboxCpn removable
                                                                page.changed.clone(),
                                                                can_save_summary,
                                                            ),
                                                            can_save_summary,
                                                        )
                                                    })))
                                                    .apply(Group::group_mixins(page.pre_admission_comorbidity_group.get_cloned(), page.drag_state.clone()))
                                                }))
                                            }))
                                        }),
                                    ])
                                }))
                            }),
                            // (3) COMPLICATION (POST ADMISSION COMORBIDITY)
                            html!("div", {
                                .class("row")
                                .child(html!("div", {
                                    .class("col")
                                    .children([
                                        render_add_draggable("(3) COMPLICATION (POST ADMISSION COMORBIDITY)", page.post_admission_comorbidity_group.get_cloned(), can_save_summary, page.changed.clone()),
                                        html!("div", {
                                            .class("row")
                                            .child(html!("div", {
                                                .class("col")
                                                .child(html!("ol", {
                                                    .children_signal_vec(page.post_admission_comorbidity_group.lock_ref().draggables.signal_vec_cloned().map(clone!(app, page => move |draggable| {
                                                        let group = page.post_admission_comorbidity_group.get_cloned();
                                                        Dragable::render(
                                                            group.clone(),
                                                            draggable.clone(),
                                                            page.drag_state.clone(),
                                                            DxSearchboxCpn::render(
                                                                DxSearchboxCpn::new(false, true, draggable.state.clone(), page.dagger_asterisk_state.clone()),
                                                                app.clone(),
                                                                Some((group, draggable.id, draggable.selected.clone())), // make DxSearchboxCpn removable
                                                                page.changed.clone(),
                                                                can_save_summary,
                                                            ),
                                                            can_save_summary,
                                                        )
                                                    })))
                                                    .apply(Group::group_mixins(page.post_admission_comorbidity_group.get_cloned(), page.drag_state.clone()))
                                                }))
                                            }))
                                        }),
                                    ])
                                }))
                            }),
                            // (4) OTHER DIAGNOSIS
                            html!("div", {
                                .class("row")
                                .child(html!("div", {
                                    .class("col")
                                    .children([
                                        render_add_draggable("(4) OTHER DIAGNOSIS", page.other_diagnosis_group.get_cloned(), can_save_summary, page.changed.clone()),
                                        html!("div", {
                                            .class("row")
                                            .child(html!("div", {
                                                .class("col")
                                                .child(html!("ol", {
                                                    .children_signal_vec(page.other_diagnosis_group.lock_ref().draggables.signal_vec_cloned().map(clone!(app, page => move |draggable| {
                                                        let group = page.other_diagnosis_group.get_cloned();
                                                        Dragable::render(
                                                            group.clone(),
                                                            draggable.clone(),
                                                            page.drag_state.clone(),
                                                            DxSearchboxCpn::render(
                                                                DxSearchboxCpn::new(false, true, draggable.state.clone(), page.dagger_asterisk_state.clone()),
                                                                app.clone(),
                                                                Some((group, draggable.id, draggable.selected.clone())), // make DxSearchboxCpn removable
                                                                page.changed.clone(),
                                                                can_save_summary,
                                                            ),
                                                            can_save_summary,
                                                        )
                                                    })))
                                                    .apply(Group::group_mixins(page.other_diagnosis_group.get_cloned(), page.drag_state.clone()))
                                                }))
                                            }))
                                        }),
                                    ])
                                }))
                            }),
                            // (5) EXTERNAL CAUSE OF INJURY
                            html!("div", {
                                .class("row")
                                .child(html!("div", {
                                    .class("col")
                                    .children([
                                        render_add_draggable("(5) EXTERNAL CAUSE OF INJURY", page.external_cause_group.get_cloned(), can_save_summary, page.changed.clone()),
                                        html!("div", {
                                            .class("row")
                                            .child(html!("div", {
                                                .class("col")
                                                .child(html!("ol", {
                                                    .children_signal_vec(page.external_cause_group.lock_ref().draggables.signal_vec_cloned().map(clone!(app, page => move |draggable| {
                                                        let group = page.external_cause_group.get_cloned();
                                                        Dragable::render(
                                                            group.clone(),
                                                            draggable.clone(),
                                                            page.drag_state.clone(),
                                                            DxSearchboxCpn::render(
                                                                DxSearchboxCpn::new(true, true, draggable.state.clone(), page.dagger_asterisk_state.clone()),
                                                                app.clone(),
                                                                Some((group, draggable.id, draggable.selected.clone())), // make DxSearchboxCpn removable
                                                                page.changed.clone(),
                                                                can_save_summary,
                                                            ),
                                                            can_save_summary,
                                                        )
                                                    })))
                                                    .apply(Group::group_mixins(page.external_cause_group.get_cloned(), page.drag_state.clone()))
                                                }))
                                            }))
                                        }),
                                    ])
                                }))
                            }),
                            html!("div", {
                                .class(class::ROW_B)
                                .child(html!("div", {
                                    .class(class::COL_R)
                                    .children([
                                        html!("a", {
                                            .class(class::BTN_BLUEO)
                                            .attr("href", "https://icd.who.int/browse10/2016/en")
                                            .attr("rel","noopener noreferrer")
                                            .attr("target","_blank")
                                            .child(html!("i", {.class(class::FA_EXT_LINK)}))
                                            .text(" ICD-10-WHO")
                                        }),
                                        html!("button", {
                                            .attr("type","button")
                                            .class(class::BTN_R_CYAN)
                                            .text("คำนวน DRG")
                                            // .visible(can_save_summary)
                                            .event(clone!(app, page => move |_:events::Click| {
                                                Self::check_drg(false, page.clone(), app.clone());
                                            }))
                                        }),
                                    ])
                                }))
                            }),
                        ])
                    }),
                ])
            }))
        })
    }

    fn render_coder(is_pre_admit: bool, page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .class("col")
            .style("break-inside","avoid")
            .child(html!("div", {
                .class(class::CARD)
                .children([
                    html!("div", {
                        .class("card-header")
                        .child(html!("div", {.class("fw-bold").text("CODES AND STATUS")}))
                    }),
                    html!("div", {
                        .class("card-body")
                        .children([
                            html!("div", {.class(class::BOLD_T1).text("By Coder")}),
                            html!("div", {
                                .class(class::INPUT_GROUP_SM_T)
                                .children([
                                    doms::span_group_text("Name"),
                                    html!("input" => HtmlInputElement, {
                                        .attr("type", "text")
                                        .attr("maxlength", "50")
                                        .class("form-control")
                                        .apply(mixins::opt_string_value(page.coder_name.clone(), page.code_changed.clone()))
                                    }),
                                ])
                            }),
                            html!("div", {.class(class::BOLD_T1).text("ICD Coding")}),
                            html!("div", {
                                .class(class::INPUT_GROUP_SM_T)
                                .children([
                                    doms::span_group_text("(1) Main"),
                                    html!("input" => HtmlInputElement, {
                                        .attr("type", "text")
                                        .attr("maxlength", "7")
                                        .class("form-control")
                                        .apply(mixins::opt_string_value(page.principal_diagnosis_code.clone(), page.code_changed.clone()))
                                    }),
                                ])
                            }),
                            html!("div", {
                                .class(class::INPUT_GROUP_SM_T)
                                .children([
                                    doms::span_group_text("(2) Comorbidity"),
                                    html!("input" => HtmlInputElement, {
                                        .attr("type", "text")
                                        .class("form-control")
                                        .apply(mixins::opt_string_value(page.pre_admission_comorbidity_codes.clone(), page.code_changed.clone()))
                                    }),
                                ])
                            }),
                            html!("div", {
                                .class(class::INPUT_GROUP_SM_T)
                                .children([
                                    doms::span_group_text("(3) Complication"),
                                    html!("input" => HtmlInputElement, {
                                        .attr("type", "text")
                                        .class("form-control")
                                        .apply(mixins::opt_string_value(page.post_admission_comorbidity_codes.clone(), page.code_changed.clone()))
                                    }),
                                ])
                            }),
                            html!("div", {
                                .class(class::INPUT_GROUP_SM_T)
                                .children([
                                    doms::span_group_text("(4) Other"),
                                    html!("input" => HtmlInputElement, {
                                        .attr("type", "text")
                                        .class("form-control")
                                        .apply(mixins::opt_string_value(page.other_diagnosis_codes.clone(), page.code_changed.clone()))
                                    }),
                                ])
                            }),
                            html!("div", {
                                .class(class::INPUT_GROUP_SM_T)
                                .children([
                                    doms::span_group_text("(5) E-code"),
                                    html!("input" => HtmlInputElement, {
                                        .attr("type", "text")
                                        .class("form-control")
                                        .apply(mixins::opt_string_value(page.external_cause_codes.clone(), page.code_changed.clone()))
                                    }),
                                ])
                            }),
                            html!("div", {.class(class::BOLD_T1).text("Procedure Coding")}),
                            html!("div", {
                                .class(class::INPUT_GROUP_SM_T)
                                .children([
                                    doms::span_group_text("Main"),
                                    html!("input" => HtmlInputElement, {
                                        .attr("type", "text")
                                        .attr("maxlength", "7")
                                        .class("form-control")
                                        .apply(mixins::opt_string_value(page.main_procedure_code.clone(), page.code_changed.clone()))
                                    }),
                                ])
                            }),
                            html!("div", {
                                .class(class::INPUT_GROUP_SM_T)
                                .children([
                                    doms::span_group_text("Other(s)"),
                                    html!("input" => HtmlInputElement, {
                                        .attr("type", "text")
                                        .class("form-control")
                                        .apply(mixins::opt_string_value(page.other_procedure_codes.clone(), page.code_changed.clone()))
                                    }),
                                ])
                            }),
                            html!("div", {.class(class::BOLD_T1).text("Status")}),
                            html!("div", {
                                .class(class::INPUT_GROUP_SM_T)
                                .children([
                                    doms::status_btn(AuditStatus::Review, page.status.clone(), page.code_changed.clone()),
                                    doms::status_btn(AuditStatus::Code, page.status.clone(), page.code_changed.clone()),
                                    doms::status_btn(AuditStatus::Audit, page.status.clone(), page.code_changed.clone()),
                                    doms::status_btn(AuditStatus::Claim, page.status.clone(), page.code_changed.clone()),
                                    doms::status_btn(AuditStatus::Appeal, page.status.clone(), page.code_changed.clone()),
                                    doms::status_btn(AuditStatus::Done, page.status.clone(), page.code_changed.clone()),
                                ])
                            }),
                            html!("br"),
                            html!("div", {
                                .class("row")
                                .child(html!("div", {
                                    .class(class::COL_R)
                                    .children([
                                        html!("a", {
                                            .class(class::BTN_BLUEO)
                                            .attr("href", "https://icd.who.int/browse10/2016/en")
                                            .attr("rel","noopener noreferrer")
                                            .attr("target","_blank")
                                            .child(html!("i", {.class(class::FA_EXT_LINK)}))
                                            .text(" ICD-10-WHO")
                                        }),
                                        html!("button", {
                                            .attr("type","button")
                                            .class(class::BTN_R_CYAN)
                                            .text("คำนวน DRG")
                                            .event(clone!(app, page => move |_:events::Click| {
                                                Self::check_drg(true, page.clone(), app.clone());
                                            }))
                                        }),
                                        html!("button", {
                                            .attr("type", "button")
                                            .class(class::BTN_R_BLUE)
                                            .text("AUTO")
                                            .event(clone!(app, page => move|_:events::Click| {
                                                let mut found = false;
                                                // if let Some(coder_name) = app.user_name().and_then(|n| n.split(" ").next().map(|s| s.to_owned())) {
                                                let coder_name = app.user_name();
                                                if page.coder_name.get_cloned() != coder_name {
                                                    page.coder_name.set(coder_name);
                                                    found = true;
                                                }
                                                let principal_diagnosis_code = str_some(group_to_comma_string(page.principal_diagnosis_group.get_cloned()));
                                                if page.principal_diagnosis_code.get_cloned() != principal_diagnosis_code {
                                                    page.principal_diagnosis_code.set(principal_diagnosis_code);
                                                    found = true;
                                                }
                                                let pre_admission_comorbidity_codes = str_some(group_to_comma_string(page.pre_admission_comorbidity_group.get_cloned()));
                                                if page.pre_admission_comorbidity_codes.get_cloned() != pre_admission_comorbidity_codes {
                                                    page.pre_admission_comorbidity_codes.set(pre_admission_comorbidity_codes);
                                                    found = true;
                                                }
                                                let post_admission_comorbidity_codes = str_some(group_to_comma_string(page.post_admission_comorbidity_group.get_cloned()));
                                                if page.post_admission_comorbidity_codes.get_cloned() != post_admission_comorbidity_codes {
                                                    page.post_admission_comorbidity_codes.set(post_admission_comorbidity_codes);
                                                    found = true;
                                                }
                                                let other_diagnosis_codes = str_some(group_to_comma_string(page.other_diagnosis_group.get_cloned()));
                                                if page.other_diagnosis_codes.get_cloned() != other_diagnosis_codes {
                                                    page.other_diagnosis_codes.set(other_diagnosis_codes);
                                                    found = true;
                                                }
                                                let external_cause_codes = str_some(group_to_comma_string(page.external_cause_group.get_cloned()));
                                                if page.external_cause_codes.get_cloned() != external_cause_codes {
                                                    page.external_cause_codes.set(external_cause_codes);
                                                    found = true;
                                                }
                                                let mut procs = page.create_procs();
                                                let (main_procedure_code, other_procedure_codes) = if procs.len() > 1 {
                                                    (procs.first().map(|c| icd9_dot(c)), Some(procs.split_off(1).iter().map(|c| icd9_dot(c)).collect::<Vec<String>>().join(", ")))
                                                } else {
                                                    (procs.first().map(|c| icd9_dot(c)), None)
                                                };
                                                if page.main_procedure_code.get_cloned() != main_procedure_code {
                                                    page.main_procedure_code.set(main_procedure_code);
                                                    found = true;
                                                }
                                                if page.other_procedure_codes.get_cloned() != other_procedure_codes {
                                                    page.other_procedure_codes.set(other_procedure_codes);
                                                    found = true;
                                                }
                                                if !matches!(page.status.get_cloned(), AuditStatus::Audit) {
                                                    page.status.set_neq(AuditStatus::Audit);
                                                    found = true;
                                                }

                                                page.code_changed.set_neq(found);
                                            }))
                                        }),
                                    ])
                                    .apply_if(app.endpoint_is_allow(&Method::PATCH, &EndPoint::IpdSummary, is_pre_admit), |dom| dom
                                        .child(html!("button" => HtmlButtonElement, {
                                            .attr("type", "button")
                                            .class(class::BTN_R)
                                            .class_signal("btn-primary", page.code_changed.signal())
                                            .class_signal("btn-secondary", not(page.code_changed.signal()))
                                            .child(html!("i", {.class(class::FA_SAVE)}))
                                            .text(" บันทึก")
                                            .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, page => move || {
                                                Self::save_code(page.clone(), app.clone());
                                            }), not(page.code_changed.signal()), app.state()))
                                        }))
                                    )
                                }))
                            }),
                        ])
                    }),
                ])
            }))
        })
    }

    fn render_or_procs(can_save_summary: bool, page: Rc<Self>) -> Dom {
        html!("div", {
            .class("row")
            .child(html!("div", {
                .class("col")
                .child(html!("div", {
                    .class(class::CARD)
                    .children([
                        html!("div", {
                            .class("card-header")
                            .child(html!("span", {
                                .class("fw-bold")
                                .text("OPERATING ROOM PROCEDURES")
                            }))
                        }),
                        html!("div", {
                            .class("card-body")
                            .apply_if(can_save_summary, |dom| { dom
                                .child_signal(page.hosxp_operating_room_procedure_signal().map(clone!(page => move |hosxp_op| {
                                    (!hosxp_op.is_empty()).then(|| {
                                        html!("button", {
                                            .attr("type", "button")
                                            .class(class::BTN_SM_FR_BLUE)
                                            .text("คัดลอกจาก HOSxP")
                                            .event(clone!(page => move |_:events::Click| {
                                                let new = {
                                                    let old = page.operating_room.lock_ref();
                                                    let prefix = if old.is_empty() {""} else {"\n"};
                                                    [old.as_str(), prefix,  &hosxp_op].concat()
                                                };
                                                page.operating_room.set(new);
                                            }))
                                        })
                                    })
                                })))
                            })
                            .children([
                                html!("div", {
                                    .class("mb-2")
                                    .text("กรุณากรอกในรูปแบบ 'ชื่อการผ่าตัด (ICD-9) [ชื่อผู้ผ่าตัด] (วัน/เวลาเริ่มผ่าตัด - วัน/เวลาผ่าตัดเสร็จ)'")
                                }),
                                html!("div", {
                                    .class(class::INPUT_GROUP_T)
                                    .child(html!("textarea" => HtmlTextAreaElement, {
                                        .class("form-control")
                                        .apply_if(!can_save_summary, |d| d.attr("disabled",""))
                                        .apply(mixins::textarea_value_auto_expand(page.operating_room.clone(), page.changed.clone()))
                                    }))

                                }),
                            ])
                            .child_signal(page.hosxp_operating_room_procedure_signal().map(|hosxp_op| {
                                (!hosxp_op.is_empty()).then(|| {
                                    html!("div", {
                                        .class(class::INPUT_GROUP_T)
                                        .children([
                                            html!("small", {
                                                .class("input-group-text")
                                                .text("ข้อมูลใน HOSxP")
                                            }),
                                            html!("textarea", {
                                                .class("form-control")
                                                .attr("disabled", "")
                                                .prop("value", &hosxp_op)
                                            }),
                                        ])
                                    })
                                })
                            }))
                        })
                    ])
                }))
            }))
        })
    }

    fn render_nor_procs(can_save_summary: bool, page: Rc<Self>) -> Dom {
        html!("div", {
            .class("row")
            .child(html!("div", {
                .class("col")
                .child(html!("div", {
                    .class(class::CARD)
                    .children([
                        html!("div", {
                            .class("card-header")
                            .child(html!("span", {
                                .class(class::COL_SM12_BOLD)
                                .text("NON OPERATING ROOM PROCEDURES")
                            }))
                        }),
                        html!("div", {
                            .class("card-body")
                            .children([
                                html!("div", {
                                    .class("row")
                                    .child(html!("div", {
                                        .class("col-auto")
                                        .child(render_checkbox(page.tracheostomy.clone(),page.changed.clone(),"tracheostomy","TRACHEOSTOMY",can_save_summary))
                                    }))
                                }),
                                html!("div", {
                                    .class("row")
                                    .children([
                                        html!("div", {
                                            .class("col-auto")
                                            .child(render_checkbox(page.packed_redcells.clone(),page.changed.clone(),"packed_redcells","PACKED RED CELLS",can_save_summary))
                                        }),
                                        html!("div", {
                                            .class("col-auto")
                                            .child(render_checkbox(page.fresh_frozen_plasma.clone(),page.changed.clone(),"fresh_frozen_plasma","FRESH FROZEN PLASMA",can_save_summary))
                                        }),
                                        html!("div", {
                                            .class("col-auto")
                                            .child(render_checkbox(page.platelets.clone(),page.changed.clone(),"platelets","PLATELETS",can_save_summary))
                                        }),
                                        html!("div", {
                                            .class("col-auto")
                                            .child(render_checkbox(page.cryoprecipitate.clone(),page.changed.clone(),"cryoprecipitate","CRYOPRECIPITATE",can_save_summary))
                                        }),
                                        html!("div", {
                                            .class("col-auto")
                                            .child(render_checkbox(page.whole_blood.clone(),page.changed.clone(),"whole_blood","WHOLE BLOOD",can_save_summary))
                                        }),
                                    ])
                                }),
                                html!("div", {
                                    .class("row")
                                    .child(html!("div", {
                                        .class("col-auto")
                                        .child(render_checkbox(page.chemotherapy.clone(),page.changed.clone(),"chemotherapy","CHEMOTHERAPY",can_save_summary))
                                    }))
                                }),
                                html!("div", {
                                    .class("row")
                                    .child(html!("div", {
                                        .class("col-auto")
                                        .child(render_checkbox(page.hemodialysis.clone(),page.changed.clone(),"hemodialysis","HEMODIALYSIS",can_save_summary))
                                    }))
                                }),
                                // MECHANICAL VENTILATION
                                html!("fieldset", {
                                    .class("row")
                                    .children([
                                        html!("legend", {
                                            .class(class::FORM_COL_LBL_SM3_PT0)
                                            .text("MECHANICAL VENTILATION")
                                        }),
                                        html!("div", {
                                            .class("col")
                                            .children([
                                                html!("div", {
                                                    .class("form-check")
                                                    .children([
                                                        html!("input" => HtmlInputElement, {
                                                            .class("form-check-input")
                                                            .attr("type", "radio")
                                                            .attr("id", "mechanical_ventilation0")
                                                            .apply_if(!can_save_summary, |d| d.attr("disabled",""))
                                                            .apply(mixins::radio_match(page.mechanical_ventilation.clone(), page.changed.clone(), ""))
                                                        }),
                                                        doms::label_check_for("mechanical_ventilation0","ไม่ได้ใช้"),
                                                    ])
                                                }),
                                                html!("div", {
                                                    .class("form-check")
                                                    .children([
                                                        html!("input" => HtmlInputElement, {
                                                            .class("form-check-input")
                                                            .attr("type", "radio")
                                                            .attr("id", "mechanical_ventilation1")
                                                            .apply_if(!can_save_summary, |d| d.attr("disabled",""))
                                                            .apply(mixins::radio_match(page.mechanical_ventilation.clone(), page.changed.clone(), "1"))
                                                        }),
                                                        doms::label_check_for("mechanical_ventilation1","ใช้ INVASIVE (ET-Tube, Tracheostomy) - มากกว่า 96 ชม."),
                                                    ])
                                                }),
                                                html!("div", {
                                                    .class("form-check")
                                                    .children([
                                                        html!("input" => HtmlInputElement, {
                                                            .class("form-check-input")
                                                            .attr("type", "radio")
                                                            .attr("id", "mechanical_ventilation2")
                                                            .apply_if(!can_save_summary, |d| d.attr("disabled",""))
                                                            .apply(mixins::radio_match(page.mechanical_ventilation.clone(), page.changed.clone(), "2"))
                                                        }),
                                                        doms::label_check_for("mechanical_ventilation2","ใช้ INVASIVE (ET-Tube, Tracheostomy) - น้อยกว่า 96 ชม."),
                                                    ])
                                                }),
                                                html!("div", {
                                                    .class("form-check")
                                                    .children([
                                                        html!("input" => HtmlInputElement, {
                                                            .class("form-check-input")
                                                            .attr("type", "radio")
                                                            .attr("id", "mechanical_ventilation3")
                                                            .apply_if(!can_save_summary, |d| d.attr("disabled",""))
                                                            .apply(mixins::radio_match(page.mechanical_ventilation.clone(), page.changed.clone(), "3"))
                                                        }),
                                                        doms::label_check_for("mechanical_ventilation3","ใช้ NON-INVASIVE (NCPAP, NIMV, NPCPAP, NIPPV, HFNC)"),
                                                    ])
                                                }),
                                            ])
                                        }),
                                    ])
                                }),
                                // NON-OR OTHER
                                html!("div", {
                                    .class("row")
                                    .child(html!("div", {
                                        .class("col")
                                        .children([
                                            render_checkbox(page.non_or_other.clone(),page.changed.clone(),"non_or_other","อื่นๆ",can_save_summary),
                                            html!("textarea" => HtmlTextAreaElement, {
                                                .class("form-control")
                                                .attr("rows", "2")
                                                .apply(mixins::textarea_value_auto_expand(page.non_or_other_text.clone(), page.changed.clone()))
                                                .apply(mixins::other_not_match_disable_or_forced(page.non_or_other.clone(), "Y", !can_save_summary))
                                                .future(page.non_or_other.signal_cloned().for_each(clone!(page => move |yes| {
                                                    if yes.is_empty() {
                                                        page.non_or_other_text.set_neq(String::new());
                                                    }
                                                    async {}
                                                })))
                                                // .attr("onkeyup", "autoGrowTextArea(this)")
                                            }),
                                        ])
                                    }))
                                }),
                            ])
                        }),
                    ])
                }))
            }))
        })
    }

    fn render_specials(can_save_summary: bool, page: Rc<Self>) -> Dom {
        html!("div", {
            .class("row")
            .child(html!("div", {
                .class("col")
                .child(html!("div", {
                    .class(class::CARD)
                    .children([
                        html!("div", {
                            .class("card-header")
                            .child(html!("span", {
                                .class(class::COL_SM12_BOLD)
                                .text("SPECIAL INVESTIGATIONS")
                            }))
                        }),
                        html!("div", {
                            .class("card-body")
                            .children([
                                // COMPUTER TOMOGRAPHY
                                html!("div", {
                                    .class("row")
                                    .child(html!("div", {
                                        .class("col")
                                        .children([
                                            render_checkbox(page.computer_tomography.clone(),page.changed.clone(),"computer_tomography","COMPUTER TOMOGRAPHY",can_save_summary),
                                            html!("div", {
                                                .class(class::INPUT_GROUP_T)
                                                .child(html!("textarea" => HtmlTextAreaElement, {
                                                    .class("form-control")
                                                    .attr("rows", "2")
                                                    .apply(mixins::textarea_value_auto_expand(page.computer_tomography_text.clone(), page.changed.clone()))
                                                    .apply(mixins::other_not_match_disable_or_forced(page.computer_tomography.clone(), "Y", !can_save_summary))
                                                    .future(page.computer_tomography.signal_cloned().for_each(clone!(page => move |yes| {
                                                        if yes.is_empty() {
                                                            page.computer_tomography_text.set_neq(String::new());
                                                        }
                                                        async {}
                                                    })))
                                                }))
                                                .apply_if(can_save_summary, |dom| { dom
                                                    .child_signal(page.hosxp_ct_signal().map(clone!(page => move |hosxp_ct| {
                                                        (!hosxp_ct.is_empty()).then(|| {
                                                            html!("button", {
                                                                .attr("type", "button")
                                                                .class(class::BTN_BLUEO)
                                                                .text("คัดลอกจาก HOSxP")
                                                                .event(clone!(page => move |_:events::Click| {
                                                                    page.computer_tomography_text.set([page.computer_tomography_text.lock_ref().as_str(), &hosxp_ct].concat())
                                                                }))
                                                            })
                                                        })
                                                    })))
                                                })
                                            }),
                                        ])
                                        .child_signal(page.hosxp_ct_signal().map(|hosxp_ct| {
                                            (!hosxp_ct.is_empty()).then(|| {
                                                html!("div", {
                                                    .class(class::INPUT_GROUP_T)
                                                    .children([
                                                        html!("small", {
                                                            .class("input-group-text")
                                                            .text("ข้อมูลใน HOSxP")
                                                        }),
                                                        html!("textarea", {
                                                            .class("form-control")
                                                            .attr("disabled", "")
                                                            .prop("value", &hosxp_ct)
                                                        }),
                                                    ])
                                                })
                                            })
                                        }))
                                    }))
                                }),
                                // MRI
                                html!("div", {
                                    .class("row")
                                    .child(html!("div", {
                                        .class("col")
                                        .children([
                                            render_checkbox(page.mri.clone(),page.changed.clone(),"mri","MRI",can_save_summary),
                                            html!("div", {
                                                .class(class::INPUT_GROUP_T)
                                                .child(html!("textarea" => HtmlTextAreaElement, {
                                                    .class("form-control")
                                                    .attr("rows", "2")
                                                    .apply(mixins::textarea_value_auto_expand(page.mri_text.clone(), page.changed.clone()))
                                                    .apply(mixins::other_not_match_disable_or_forced(page.mri.clone(), "Y", !can_save_summary))
                                                    .future(page.mri.signal_cloned().for_each(clone!(page => move |yes| {
                                                        if yes.is_empty() {
                                                            page.mri_text.set_neq(String::new());
                                                        }
                                                        async {}
                                                    })))
                                                }))
                                                .apply_if(can_save_summary, |dom| { dom
                                                    .child_signal(page.hosxp_mri_signal().map(clone!(page => move |hosxp_mri| {
                                                        (!hosxp_mri.is_empty()).then(|| {
                                                            html!("button", {
                                                                .attr("type", "button")
                                                                .class(class::BTN_BLUEO)
                                                                .text("คัดลอกจาก HOSxP")
                                                                .event(clone!(page => move |_:events::Click| {
                                                                    page.mri_text.set([page.mri_text.lock_ref().as_str(), &hosxp_mri].concat())
                                                                }))
                                                            })
                                                        })
                                                    })))
                                                })
                                            }),
                                        ])
                                        .child_signal(page.hosxp_mri_signal().map(|hosxp_mri| {
                                            (!hosxp_mri.is_empty()).then(|| {
                                                html!("div", {
                                                    .class(class::INPUT_GROUP_T)
                                                    .children([
                                                        html!("small", {
                                                            .class("input-group-text")
                                                            .text("ข้อมูลใน HOSxP")
                                                        }),
                                                        html!("textarea", {
                                                            .class("form-control")
                                                            .attr("disabled", "")
                                                            .prop("value", &hosxp_mri)
                                                        }),
                                                    ])
                                                })
                                            })
                                        }))
                                    }))
                                }),
                                // SPECIAL INVESTIGATION OTHER
                                html!("div", {
                                    .class("row")
                                    .child(html!("div", {
                                        .class("col")
                                        .children([
                                            render_checkbox(page.special_other.clone(),page.changed.clone(),"special_other","อื่นๆ",can_save_summary),
                                            html!("textarea" => HtmlTextAreaElement, {
                                                .class("form-control")
                                                .attr("rows", "2")
                                                .apply(mixins::textarea_value_auto_expand(page.special_other_text.clone(), page.changed.clone()))
                                                .apply(mixins::other_not_match_disable_or_forced(page.special_other.clone(), "Y", !can_save_summary))
                                                .future(page.special_other.signal_cloned().for_each(clone!(page => move |yes| {
                                                    if yes.is_empty() {
                                                        page.special_other_text.set_neq(String::new());
                                                    }
                                                    async {}
                                                })))
                                                // .attr("onkeyup", "autoGrowTextArea(this)")
                                            }),
                                        ])
                                    }))
                                }),
                            ])
                        }),
                    ])
                }))
            }))
        })
    }

    fn render_dch(can_save_summary: bool, page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .class(class::ROW_T)
            .child(html!("div", {
                .style("column-width","480px")
                .style("column-gap","8px")
                .children([
                    html!("div", {
                        .class("col")
                        .style("break-inside","avoid")
                        .child(html!("div", {
                            .class(class::CARD)
                            .children([
                                html!("div", {
                                    .class("card-header")
                                    .child(html!("span", {
                                        .class("fw-bold")
                                        .text("DISCHARGE STATUS")
                                    }))
                                }),
                                html!("div", {
                                    .class("card-body")
                                    .children(render_radios(page.discharge_status.clone(), page.changed.clone(), "discharge_status", &[
                                        ("01","COMPLETE RECOVERED"),
                                        ("02","IMPROVED"),
                                        ("03","NOT IMPROVED"),
                                        ("04","DELIVERED"),
                                        ("05","UNDELIVERED"),
                                        ("06","NORMAL CHILD DISCHARGE WITH MOTHER"),
                                        ("07","NORMAL CHILD DISCHARGE SEPARATELY"),
                                        ("09","DEAD"),
                                    ], can_save_summary))
                                }),
                            ])
                        }))
                    }),
                    html!("div", {
                        .class("col")
                        .style("break-inside","avoid")
                        .child(html!("div", {
                            .class(class::CARD)
                            .children([
                                html!("div", {
                                    .class("card-header")
                                    .child(html!("span", {
                                        .class("fw-bold")
                                        .text("DISCHARGE TYPE")
                                    }))
                                }),
                                html!("div", {
                                    .class("card-body")
                                    .children(render_radios(page.discharge_type.clone(), page.changed.clone(), "discharge_type", &[
                                        ("01","WITH APPROVAL"),
                                        ("02","AGAINST ADVICE"),
                                        ("03","ESCAPE"),
                                        ("04","BY TRANSFER"),
                                    ], can_save_summary))
                                    .child_signal(page.discharge_type.signal_cloned().map(clone!(app, page => move |discharge_type| {
                                        (discharge_type == "04").then(|| {
                                            html!("div", {
                                                .class("row")
                                                .child(html!("div", {
                                                    .class("col")
                                                    .children([
                                                        html!("label", {
                                                            .class("col-form-label")
                                                            //.attr("for", "hospital_refer")
                                                            .text("ชื่อสถานพยาบาลที่ส่งต่อ")
                                                        }),
                                                        HospSearchboxCpn::render(HospSearchboxCpn::new(page.hospital_refer.clone()), app.clone(), page.changed.clone(), can_save_summary, true),
                                                    ])
                                                }))
                                            })
                                        })
                                    })))
                                    .future(page.discharge_type.signal_cloned().for_each(clone!(page => move |discharge_type| {
                                        if discharge_type != "04" {
                                            page.hospital_refer.set(None);
                                        }
                                        async {}
                                    })))
                                    .children(render_radios(page.discharge_type.clone(), page.changed.clone(), "discharge_type", &[
                                        ("05","OTHER"),
                                        ("08","DEAD, AUTOPSY"),
                                        ("09","DEAD, NO AUTOPSY"),
                                    ], can_save_summary))
                                }),
                            ])
                        }))
                    }),
                    html!("div", {
                        .class("col")
                        .style("break-inside","avoid")
                        .child(html!("div", {
                            .class(class::CARD)
                            .children([
                                html!("div", {
                                    .class("card-header")
                                    .child(html!("span", {
                                        .class("fw-bold")
                                        .text("HOSXP DISCHARGE DATA")
                                    }))
                                }),
                                html!("div", {
                                    .class("card-body")
                                    .child_signal(page.dch.signal_cloned().map(|opt| {
                                        opt.as_ref().map(|dch| {
                                            if dch.dchdate.is_some() {
                                                html!("div", {
                                                    .class("hosxp-dch-data-div")
                                                    .children([
                                                        html!("div", {
                                                            .children([
                                                                html!("span", {.text("D/C Status: ").class("fw-bold")}),
                                                                html!("span", {.text(&dch.dchstts_name.clone().unwrap_or_default())}),
                                                            ])
                                                        }),
                                                        html!("div", {
                                                            .children([
                                                                html!("span", {.text("D/C Type: ").class("fw-bold")}),
                                                                html!("span", {.text(&dch.dchtype_name.clone().unwrap_or_default())}),
                                                            ])
                                                        }),
                                                        html!("div", {
                                                            .children([
                                                                html!("span", {.text("D/C เมื่อ: ").class("fw-bold")}),
                                                                html!("span", {.text(&[date_th_opt(&dch.dchdate), time_hm_opt(&dch.dchtime)].join(" "))}),
                                                            ])
                                                        }),
                                                    ])
                                                })
                                            } else {
                                                html!("div", {
                                                    .class("hosxp-dch-data-div")
                                                    .child(html!("div", {.text("ยังไม่ได้ D/C")}))
                                                })
                                            }
                                        })
                                    }))
                                }),
                            ])
                        }))
                    }),
                ])
            }))
        })
    }

    fn render_signers(can_save_summary: bool, page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .class(class::ROW_T)
            .child(html!("div", {
                .style("column-width","480px")
                .style("column-gap","8px")
                .children([
                    html!("div", {
                        .class("col")
                        .style("break-inside","avoid")
                        .child(html!("div", {
                            .class(class::CARD)
                            .children([
                                html!("div", {
                                    .class("card-header")
                                    .child(html!("span", {
                                        .class("fw-bold")
                                        .text("ATTENDING PHYSICIAN")
                                    }))
                                }),
                                html!("div", {
                                    .class("card-body")
                                    .children_signal_vec(page.attending_doctors.signal_vec_cloned().map(move |dr| {
                                        html!("div", {
                                            .class("mb-1")
                                            .child(html!("input", {
                                                .attr("type", "text")
                                                .class("form-control")
                                                .attr("readonly","")
                                                .apply_if(!can_save_summary, |d| d.attr("disabled",""))
                                                .attr("value", &dr.doctor_name.clone().unwrap_or_default())
                                            }))
                                        })
                                    }))
                                    .apply_if(can_save_summary, |dom| {dom
                                        .child(html!("div", {
                                            .class("row")
                                            .child(html!("div", {
                                                .class(class::COL_R)
                                                .child_signal(page.attending_doctors.signal_vec_cloned().to_signal_cloned().map(clone!(app, page => move |drs| {
                                                    app.doctor_code().and_then(clone!(app, page => move |doctor| {
                                                        (!drs.iter().any(|dr| dr.doctor == doctor)).then(|| {
                                                            html!("button", {
                                                                .attr("type", "button")
                                                                .class(class::BTN_BLUE)
                                                                .text("ลงชื่อแพทย์")
                                                                .apply_if(!can_save_summary, |d| d.attr("disabled",""))
                                                                .event(clone!(app, page, doctor => move |_:events::Click| {
                                                                    let doctor_data = DoctorData {
                                                                        ty: 1,
                                                                        doctor: doctor.clone(),
                                                                        doctor_name: app.doctor_name(),
                                                                        licenseno: app.doctor_licenseno(),
                                                                    };
                                                                    page.attending_doctors.lock_mut().push_cloned(Rc::new(doctor_data));
                                                                    page.attending_doctor_sign.set(true);
                                                                    page.changed.set_neq(true);
                                                                }))
                                                            })
                                                        })
                                                    }))
                                                })))
                                                .child_signal(page.attending_doctor_sign.signal_cloned().map(clone!(app, page => move |sign| {
                                                    sign.then(|| {
                                                        html!("button", {
                                                            .attr("type", "button")
                                                            .class(class::BTN_GRAY)
                                                            .text("ยกเลิกการลงชื่อ")
                                                            .event(clone!(app, page => move |_:events::Click| {
                                                                page.attending_doctors.lock_mut().retain(|dr| dr.doctor != app.doctor_code().unwrap_or_default());
                                                                page.attending_doctor_sign.set(false);
                                                            }))
                                                        })
                                                    })
                                                })))
                                            }))
                                        }))
                                    })
                                }),
                            ])
                        }))
                    }),
                    html!("div", {
                        .class("col")
                        .style("break-inside","avoid")
                        .child(html!("div", {
                            .class(class::CARD)
                            .children([
                                html!("div", {
                                    .class("card-header")
                                    .child(html!("span", {
                                        .class("fw-bold")
                                        .text("APPROVED BY")
                                    }))
                                }),
                                html!("div", {
                                    .class("card-body")
                                    .children_signal_vec(page.approve_doctors.signal_vec_cloned().map(move |dr| {
                                        html!("div", {
                                            .class("mb-1")
                                            .child(html!("input", {
                                                .attr("type", "text")
                                                .class("form-control")
                                                .attr("readonly","")
                                                .apply_if(!can_save_summary, |d| d.attr("disabled",""))
                                                .attr("value", &dr.doctor_name.clone().unwrap_or_default())
                                            }))
                                        })
                                    }))
                                    .apply_if(can_save_summary, |dom| {dom
                                        .child(html!("div", {
                                            .class("row")
                                            .child(html!("div", {
                                                .class(class::COL_R)
                                                .child_signal(page.approve_doctors.signal_vec_cloned().to_signal_cloned().map(clone!(app, page => move |drs| {
                                                    app.doctor_code().and_then(clone!(app, page => move |doctor| {
                                                        (!drs.iter().any(|dr| dr.doctor == doctor)).then(|| {
                                                            html!("button", {
                                                                .attr("type", "button")
                                                                .class(class::BTN_BLUE)
                                                                .text("ลงชื่อแพทย์")
                                                                .apply_if(!can_save_summary, |d| d.attr("disabled",""))
                                                                .event(clone!(app, page, doctor => move |_:events::Click| {
                                                                    let doctor_data = DoctorData {
                                                                        ty: 2,
                                                                        doctor: doctor.clone(),
                                                                        doctor_name: app.doctor_name(),
                                                                        licenseno: app.doctor_licenseno(),
                                                                    };
                                                                    page.approve_doctors.lock_mut().push_cloned(Rc::new(doctor_data));
                                                                    page.approve_doctor_sign.set(true);
                                                                    page.changed.set_neq(true);
                                                                }))
                                                            })
                                                        })
                                                    }))
                                                })))
                                                .child_signal(page.approve_doctor_sign.signal_cloned().map(clone!(app, page => move |sign| {
                                                    sign.then(|| {
                                                        html!("button", {
                                                            .attr("type", "button")
                                                            .class(class::BTN_GRAY)
                                                            .text("ยกเลิกการลงชื่อ")
                                                            .event(clone!(app, page => move |_:events::Click| {
                                                                page.approve_doctors.lock_mut().retain(|dr| dr.doctor != app.doctor_code().unwrap_or_default());
                                                                page.approve_doctor_sign.set(false);
                                                            }))
                                                        })
                                                    })
                                                })))
                                            }))
                                        }))
                                    })
                                }),
                            ])
                        }))
                    }),
                ])
            }))
        })
    }

    fn render_review_status(page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .class(class::CARD)
            .style("break-inside","avoid")
            .children([
                html!("div", {.class(class::CARD_HEAD_BDARKS_LIGHTS).class("fw-bold").text("SUMMARY STATUS")}),
                html!("div", {
                    .class("card-body")
                    .child(html!("div", {
                        .class(class::INPUT_GROUP_SM_T)
                        .children([
                            doms::status_btn(AuditStatus::Review, page.status.clone(), page.status_changed.clone()),
                            doms::status_btn(AuditStatus::Code, page.status.clone(), page.status_changed.clone()),
                            doms::status_btn(AuditStatus::Audit, page.status.clone(), page.status_changed.clone()),
                        ])
                    }))
                    .apply_if(app.endpoint_is_allow(&Method::PATCH, &EndPoint::IpdSummaryNoteId, false), |dom| dom
                        .child(html!("div", {
                            .class("text-end")
                            .child(html!("button" => HtmlButtonElement, {
                                .attr("type","button")
                                .class(class::BTN_SM_BLUE)
                                .child(html!("i", {.class(class::FA_SAVE)}))
                                .text(" บันทึก")
                                .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, page => move || {
                                    Self::update_status(page.clone(), app.clone());
                                }), not(page.status_changed.signal()), app.state()))
                            }))
                        }))
                    )
                }),
            ])
        })
    }
}

fn render_add_draggable(label: &str, group: Rc<Group<Rc<Icd10>>>, can_save_summary: bool, changed_mutable: Mutable<bool>) -> Dom {
    html!("div", {
        .class("row")
        .child(html!("div", {
            .class("col")
            .child(html!("span", {
                .class("fw-bold")
                .style("user-select","none")
                .text(label)
            }))
            .apply_if(can_save_summary, |dom| { dom
                .child_signal(group.has_empty_draggable().map(clone!(group, changed_mutable => move |has_empty| {
                    (!has_empty).then(|| {
                        html!("button", {
                            .attr("type", "button")
                            .class(class::BTN_SM_R_GRAY)
                            .child(html!("i", {.class(class::FA_PLUS)}))
                            .apply_if(!can_save_summary, |d| d.attr("disabled",""))
                            .event(clone!(group, changed_mutable => move |_:events::Click| {
                                group.add_new_draggable();
                                changed_mutable.set_neq(true);
                            }))
                        })
                    })
                })))
            })
        }))
    })
}

fn render_checkbox(mutable: Mutable<String>, changed: Mutable<bool>, id: &str, label: &str, can_save_summary: bool) -> Dom {
    html!("div", {
        .class("form-check")
        .children([
            html!("input" => HtmlInputElement, {
                .attr("type", "checkbox")
                .class("form-check-input")
                .attr("id", id)
                .apply_if(!can_save_summary, |d| d.attr("disabled",""))
                .apply(mixins::checkbox_toggle(mutable, changed, "Y", ""))
            }),
            doms::label_check_for(id, label),
        ])
    })
}

fn render_radios(mutable: Mutable<String>, changed: Mutable<bool>, name: &str, items: &[(&'static str, &'static str)], can_save_summary: bool) -> impl Iterator<Item = Dom> {
    items.iter().map(clone!(mutable, changed => move |(value, label)| {
        let id = &[name, value].concat();
        html!("div", {
            .class("form-check")
            .children([
                html!("input" => HtmlInputElement, {
                    .attr("type", "radio")
                    .class("form-check-input")
                    .attr("id", &id)
                    .apply_if(!can_save_summary, |d| d.attr("disabled",""))
                    .apply(mixins::radio_match(mutable.clone(), changed.clone(), value))
                }),
                doms::label_check_for(&id, label),
            ])
        })
    }))
}

fn group_to_dxdatas(group: Rc<Group<Rc<Icd10>>>, ty: i32) -> Vec<DxData> {
    group
        .draggables
        .lock_ref()
        .iter()
        .filter_map(|d| {
            d.state.lock_ref().as_ref().map(|s| DxData {
                ty,
                detail: s.ename.clone(),
                icd: str_some(s.icd10.clone()),
            })
        })
        .collect()
}

fn group_to_vec_code(group: Rc<Group<Rc<Icd10>>>) -> Vec<String> {
    group
        .draggables
        .lock_ref()
        .iter()
        .filter_map(|d| d.state.lock_ref().as_ref().map(|s| s.icd10.to_owned()))
        .collect::<Vec<String>>()
}

fn group_to_comma_string(group: Rc<Group<Rc<Icd10>>>) -> String {
    group
        .draggables
        .lock_ref()
        .iter()
        .filter_map(|d| d.state.lock_ref().as_ref().map(|s| icd10_dot(&s.icd10)))
        .collect::<Vec<String>>()
        .join(", ")
}
