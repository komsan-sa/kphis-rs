#![allow(dead_code)]

use axum::{
    Json,
    body::Bytes,
    extract::{ConnectInfo, Multipart, Path, Query, State},
    http::HeaderMap,
    http::Response,
};
use http_body_util::Full;
use std::{net::SocketAddr, rc::Rc};
use tower_cookies::Cookies;
use web_sys::Blob;

use kphis_api_core::state::{ApiState, RequestState};
use kphis_model::{
    app::{self, AppState},
    avatar, dc_plan, drug_use_duration, emr,
    endpoint::{EndPoint, QueryString},
    fetch, focus_list, focus_note, image, index_action, index_monitor, index_plan, ipd, lab, med_reconcile, opd_er, order, pacs, patient_info, post_admit, pre_admit, pre_order, prescription,
    progress_note, refer_note, refer_out, report, search, sse, user, vital_sign, xray,
};
use kphis_util::error::AppError;

// Generic function that enforces return type checked
fn check_rj<H, F, HFut, FFut, T>(_handler_fn: H, _fetch_fn: F)
where
    H: Fn(RequestState) -> HFut,
    F: Fn(Rc<AppState>) -> FFut,
    HFut: Future<Output = Result<Json<T>, AppError>>,
    FFut: Future<Output = Result<T, AppError>>,
{
}

//=====   =====//
// query alone //
//=====   =====//
fn check_q_rj<'a, H, F, HFut, FFut, T, Q>(_handler_fn: H, _fetch_fn: F)
where
    H: Fn(Query<Q>, RequestState) -> HFut,
    F: Fn(&'a Q, Rc<AppState>) -> FFut,
    Q: QueryString + 'a,
    HFut: Future<Output = Result<Json<T>, AppError>>,
    FFut: Future<Output = Result<T, AppError>>,
{
}
fn check_q_add_rj<'a, H, F, HFut, FFut, T, Q, A>(_handler_fn: H, _fetch_fn: F)
where
    H: Fn(Query<Q>, RequestState) -> HFut,
    F: Fn(A, &'a Q, Rc<AppState>) -> FFut,
    Q: QueryString + 'a,
    HFut: Future<Output = Result<Json<T>, AppError>>,
    FFut: Future<Output = Result<T, AppError>>,
{
}

//=====  =====//
// path alone //
//=====  =====//
fn check_pi_rj<H, F, HFut, FFut, T, I>(_handler_fn: H, _fetch_fn: F)
where
    H: Fn(Path<I>, RequestState) -> HFut,
    F: Fn(I, Rc<AppState>) -> FFut,
    HFut: Future<Output = Result<Json<T>, AppError>>,
    FFut: Future<Output = Result<T, AppError>>,
{
}
fn check_pi_add_rj<H, F, HFut, FFut, T, I, A>(_handler_fn: H, _fetch_fn: F)
where
    H: Fn(Path<I>, RequestState) -> HFut,
    F: Fn(A, I, Rc<AppState>) -> FFut,
    HFut: Future<Output = Result<Json<T>, AppError>>,
    FFut: Future<Output = Result<T, AppError>>,
{
}
fn check_ps_rj<'a, H, F, HFut, FFut, T>(_handler_fn: H, _fetch_fn: F)
where
    H: Fn(Path<String>, RequestState) -> HFut,
    F: Fn(&'a str, Rc<AppState>) -> FFut,
    HFut: Future<Output = Result<Json<T>, AppError>>,
    FFut: Future<Output = Result<T, AppError>>,
{
}
fn check_pss_rj<'a, H, F, HFut, FFut, T>(_handler_fn: H, _fetch_fn: F)
where
    H: Fn(Path<(String, String)>, RequestState) -> HFut,
    F: Fn(&'a str, &'a str, Rc<AppState>) -> FFut,
    HFut: Future<Output = Result<Json<T>, AppError>>,
    FFut: Future<Output = Result<T, AppError>>,
{
}
fn check_psss_rj<'a, H, F, HFut, FFut, T>(_handler_fn: H, _fetch_fn: F)
where
    H: Fn(Path<(String, String, String)>, RequestState) -> HFut,
    F: Fn(&'a str, &'a str, &'a str, Rc<AppState>) -> FFut,
    HFut: Future<Output = Result<Json<T>, AppError>>,
    FFut: Future<Output = Result<T, AppError>>,
{
}
fn check_psi_rj<'a, H, F, HFut, FFut, T, I>(_handler_fn: H, _fetch_fn: F)
where
    H: Fn(Path<(String, I)>, RequestState) -> HFut,
    F: Fn(&'a str, I, Rc<AppState>) -> FFut,
    HFut: Future<Output = Result<Json<T>, AppError>>,
    FFut: Future<Output = Result<T, AppError>>,
{
}
fn check_pxi_rj<'a, H, F, HFut, FFut, T, X: 'a, I>(_handler_fn: H, _fetch_fn: F)
where
    H: Fn(Path<(X, I)>, RequestState) -> HFut,
    F: Fn(&'a X, I, Rc<AppState>) -> FFut,
    HFut: Future<Output = Result<Json<T>, AppError>>,
    FFut: Future<Output = Result<T, AppError>>,
{
}

//===== ===== =====//
// multipart alone //
//===== ===== =====//
fn check_multipart_rj<H, F, HFut, FFut, T, X>(_handler_fn: H, _fetch_fn: F)
where
    H: Fn(RequestState, Multipart) -> HFut,
    F: Fn(X, Rc<AppState>) -> FFut,
    HFut: Future<Output = Result<Json<T>, AppError>>,
    FFut: Future<Output = Result<T, AppError>>,
{
}

//===== === =====//
// payload alone //
//===== === =====//
fn check_payload_rj<'a, H, F, HFut, FFut, T, X: 'a>(_handler_fn: H, _fetch_fn: F)
where
    H: Fn(RequestState, Json<X>) -> HFut,
    F: Fn(&'a X, Rc<AppState>) -> FFut,
    HFut: Future<Output = Result<Json<T>, AppError>>,
    FFut: Future<Output = Result<T, AppError>>,
{
}
fn check_payload_add_rj<'a, H, F, HFut, FFut, T, X: 'a, A>(_handler_fn: H, _fetch_fn: F)
where
    H: Fn(RequestState, Json<X>) -> HFut,
    F: Fn(&'a X, A, Rc<AppState>) -> FFut,
    HFut: Future<Output = Result<Json<T>, AppError>>,
    FFut: Future<Output = Result<T, AppError>>,
{
}
fn check_payloads_rj<'a, H, F, HFut, FFut, T, X: 'a>(_handler_fn: H, _fetch_fn: F)
where
    H: Fn(RequestState, Json<Vec<X>>) -> HFut,
    F: Fn(&'a [X], Rc<AppState>) -> FFut,
    HFut: Future<Output = Result<Json<T>, AppError>>,
    FFut: Future<Output = Result<T, AppError>>,
{
}
fn check_payloads_add_rj<'a, H, F, HFut, FFut, T, X: 'a, A>(_handler_fn: H, _fetch_fn: F)
where
    H: Fn(RequestState, Json<Vec<X>>) -> HFut,
    F: Fn(&'a [X], A, Rc<AppState>) -> FFut,
    HFut: Future<Output = Result<Json<T>, AppError>>,
    FFut: Future<Output = Result<T, AppError>>,
{
}

// =====  ===== //
// path + query //
// =====  ===== //
fn check_ps_q_rj<'a, H, F, HFut, FFut, T, Q>(_handler_fn: H, _fetch_fn: F)
where
    H: Fn(Path<String>, Query<Q>, RequestState) -> HFut,
    F: Fn(&'a str, &'a Q, Rc<AppState>) -> FFut,
    Q: QueryString + 'a,
    HFut: Future<Output = Result<Json<T>, AppError>>,
    FFut: Future<Output = Result<T, AppError>>,
{
}
fn check_pi_q_rj<'a, H, F, HFut, FFut, T, Q, I>(_handler_fn: H, _fetch_fn: F)
where
    H: Fn(Path<I>, Query<Q>, RequestState) -> HFut,
    F: Fn(I, &'a Q, Rc<AppState>) -> FFut,
    Q: QueryString + 'a,
    HFut: Future<Output = Result<Json<T>, AppError>>,
    FFut: Future<Output = Result<T, AppError>>,
{
}

//===== ==== =====//
// path + payload //
//===== ==== =====//
fn check_ps_payload_rj<'a, H, F, HFut, FFut, T, X: 'a>(_handler_fn: H, _fetch_fn: F)
where
    H: Fn(Path<String>, RequestState, Json<X>) -> HFut,
    F: Fn(&'a X, &'a str, Rc<AppState>) -> FFut,
    HFut: Future<Output = Result<Json<T>, AppError>>,
    FFut: Future<Output = Result<T, AppError>>,
{
}
fn check_pi_payload_rj<'a, H, F, HFut, FFut, T, I, X: 'a>(_handler_fn: H, _fetch_fn: F)
where
    H: Fn(Path<I>, RequestState, Json<X>) -> HFut,
    F: Fn(&'a X, I, Rc<AppState>) -> FFut,
    HFut: Future<Output = Result<Json<T>, AppError>>,
    FFut: Future<Output = Result<T, AppError>>,
{
}
fn check_pi_payload_add_rj<'a, H, F, HFut, FFut, T, I, X: 'a, A>(_handler_fn: H, _fetch_fn: F)
where
    H: Fn(Path<I>, RequestState, Json<X>) -> HFut,
    F: Fn(&'a X, A, I, Rc<AppState>) -> FFut,
    HFut: Future<Output = Result<Json<T>, AppError>>,
    FFut: Future<Output = Result<T, AppError>>,
{
}

// ===== ==== ===== //
// query + payload  //
// ===== ==== ===== //
fn check_q_payload_rj<'a, H, F, HFut, FFut, T, Q, X: 'a>(_handler_fn: H, _fetch_fn: F)
where
    H: Fn(Query<Q>, RequestState, Json<X>) -> HFut,
    F: Fn(&'a X, &'a Q, Rc<AppState>) -> FFut,
    Q: QueryString + 'a,
    HFut: Future<Output = Result<Json<T>, AppError>>,
    FFut: Future<Output = Result<T, AppError>>,
{
}
fn check_q_payload_add2_rj<'a, H, F, HFut, FFut, T, Q, X: 'a, A, B>(_handler_fn: H, _fetch_fn: F)
where
    H: Fn(Query<Q>, RequestState, Json<X>) -> HFut,
    F: Fn(&'a X, A, B, &'a Q, Rc<AppState>) -> FFut,
    Q: QueryString + 'a,
    HFut: Future<Output = Result<Json<T>, AppError>>,
    FFut: Future<Output = Result<T, AppError>>,
{
}
fn check_q_payloads_add_rj<'a, H, F, HFut, FFut, T, Q, X: 'a, A>(_handler_fn: H, _fetch_fn: F)
where
    H: Fn(Query<Q>, RequestState, Json<Vec<X>>) -> HFut,
    F: Fn(A, &'a [X], &'a Q, Rc<AppState>) -> FFut,
    Q: QueryString + 'a,
    HFut: Future<Output = Result<Json<T>, AppError>>,
    FFut: Future<Output = Result<T, AppError>>,
{
}
// ===== ===== ===== ===== //
// path + query + payload  //
// ===== ===== ===== ===== //
fn check_ps_q_payload_rj<'a, H, F, HFut, FFut, T, Q, X: 'a>(_handler_fn: H, _fetch_fn: F)
where
    H: Fn(Path<String>, Query<Q>, RequestState, Json<X>) -> HFut,
    F: Fn(&'a X, &'a str, &'a Q, Rc<AppState>) -> FFut,
    Q: QueryString + 'a,
    HFut: Future<Output = Result<Json<T>, AppError>>,
    FFut: Future<Output = Result<T, AppError>>,
{
}

//===== ===== =====//
// login specific  //
//===== ===== =====//
fn check_login_rj<H, F, HFut, FFut, T>(_handler_fn: H, _fetch_fn: F)
where
    H: Fn(ConnectInfo<SocketAddr>, HeaderMap, State<ApiState>, Cookies) -> HFut,
    F: Fn(Rc<AppState>) -> FFut,
    HFut: Future<Output = Result<Json<T>, AppError>>,
    FFut: Future<Output = Result<T, AppError>>,
{
}
fn check_login_payload_rj<'a, H, F, HFut, FFut, T, X>(_handler_fn: H, _fetch_fn: F)
where
    H: Fn(ConnectInfo<SocketAddr>, HeaderMap, State<ApiState>, Cookies, Json<X>) -> HFut,
    F: Fn(&'a str, &'a str, Rc<AppState>) -> FFut,
    HFut: Future<Output = Result<Json<T>, AppError>>,
    FFut: Future<Output = Result<T, AppError>>,
{
}
fn check_login_payload_add_rj<'a, H, F, HFut, FFut, T, X, A>(_handler_fn: H, _fetch_fn: F)
where
    H: Fn(ConnectInfo<SocketAddr>, HeaderMap, State<ApiState>, Cookies, Json<X>) -> HFut,
    F: Fn(A, &'a str, &'a str, Rc<AppState>) -> FFut,
    HFut: Future<Output = Result<Json<T>, AppError>>,
    FFut: Future<Output = Result<T, AppError>>,
{
}

//===== =====//
// with Blob //
//===== =====//
fn check_psss_rb<'a, H, F, HFut, FFut>(_handler_fn: H, _fetch_fn: F)
where
    H: Fn(Path<(String, String, String)>, RequestState) -> HFut,
    F: Fn(&'a str, &'a str, &'a str, Rc<AppState>) -> FFut,
    HFut: Future<Output = Result<Response<Full<Bytes>>, AppError>>,
    FFut: Future<Output = Result<Blob, AppError>>,
{
}

//===== ===== ===== ===== ===== ===== ===== =====//
// Do not run, just for compile-time type check  //
//===== ===== ===== ===== ===== ===== ===== =====//
#[rustfmt::skip]
fn test_handler_fetch_match(endpoint: EndPoint) {
    match endpoint {
        EndPoint::AvatarOpdEr => {
            check_rj(kphis_api_handler::avatar::get_avatar_opd_er, avatar::AvatarOpdEr::call_api_get);
        }
        EndPoint::AvatarIpd => {
            check_q_rj(kphis_api_handler::avatar::get_avatar_ipd, avatar::AvatarWard::call_api_get);
        }
        EndPoint::DrugUseDuration => {
            check_q_rj(kphis_api_handler::drug_use_duration::get_drug_use_duration, drug_use_duration::DrugUseDuration::call_api_get);
            check_payload_rj(kphis_api_handler::drug_use_duration::post_drug_use_duration, drug_use_duration::DrugUseDuration::call_api_post);
        }
        EndPoint::ExistsKeyId => {
            check_pss_rj(kphis_api_handler::app::get_exists, fetch::call_api_get_exists_key_id);
        }
        EndPoint::EmrDateHn => {
            check_ps_rj(kphis_api_handler::emr::get_emr_date, emr::EmrDate::call_api_get);
        }
        EndPoint::EmrVisitVn => {
            check_ps_rj(kphis_api_handler::emr::get_emr_visit, emr::EmrVisit::call_api_get);
        }
        EndPoint::HisIptDiagAn => {
            check_ps_rj(kphis_api_handler::ipd::his::get_his_ipt_diag, ipd::his::HisIptDiag::call_api_get);
        }
        EndPoint::HisIptOprtAn => {
            check_ps_rj(kphis_api_handler::ipd::his::get_his_ipt_oprt, ipd::his::HisIptOprt::call_api_get);
        }
        EndPoint::HisMedPlanIpdAn => {
            check_ps_rj(kphis_api_handler::ipd::his::get_med_plan_ipd_remains, ipd::his::HisMedPlanIpd::call_api_get);
        }
        EndPoint::HisOperationAdmitAn => {
            check_ps_rj(kphis_api_handler::ipd::his::get_ipd_his_opertion_admit, ipd::his::HisOperationAdmit::call_api_get);
        }
        EndPoint::HisReferOutVnan => {
            check_ps_rj(kphis_api_handler::refer_out::get_his_referout_data, refer_out::HisReferOutData::call_api_get);
            check_ps_payload_rj(kphis_api_handler::refer_out::post_his_referout, refer_out::HisReferOutSave::call_api_post);
        }
        EndPoint::Image => {
            check_multipart_rj(kphis_api_handler::image::file_path::post_image_file, image::file_path::ImagePath::call_api_post_files_returning);
            check_payload_rj(kphis_api_handler::image::file_path::patch_image_path, image::file_path::ImagePath::call_api_patch);
            // check_payloads_rj(kphis_api_handler::image::file_path::delete_image_file, Unimplemented);
        }
        EndPoint::ImageUsage => {
            check_payloads_rj(kphis_api_handler::image::file_path::post_image_usage, image::file_path::ImagePath::call_api_post);
            check_payloads_rj(kphis_api_handler::image::file_path::delete_image_usage, image::file_path::ImagePath::call_api_delete);
        }
        EndPoint::ImageUsageId => {
            check_pxi_rj(kphis_api_handler::image::file_path::get_image_usage_id, image::file_path::ImagePath::call_api_get);
        }
        EndPoint::IpdAdmissionNoteDrAn => {
            check_ps_rj(kphis_api_handler::ipd::admission_note_dr::get_ipd_admission_note_dr, ipd::admission_note_dr::IpdAdmissionNoteDrRaw::call_api_get);
        }
        EndPoint::IpdAdmissionNoteDrPharmCheckAn => {
            check_ps_rj(kphis_api_handler::ipd::admission_note_dr::patch_ipd_pharmacy_check, patient_info::PatientInfo::call_api_patch);
        }
        EndPoint::IpdAdmissionNoteDr => {
            // 2 handler | 1 fetcher
            check_payload_add_rj(kphis_api_handler::ipd::admission_note_dr::post_ipd_admission_note_dr, ipd::admission_note_dr::IpdAdmissionNoteDrSave::call_api_save);
            check_payload_add_rj(kphis_api_handler::ipd::admission_note_dr::put_ipd_admission_note_dr, ipd::admission_note_dr::IpdAdmissionNoteDrSave::call_api_save);
        }
        EndPoint::IpdAdmissionNoteNurseAn => {
            check_ps_rj(kphis_api_handler::ipd::admission_note_nurse::get_ipd_admission_note_nurse, ipd::admission_note_nurse::IpdNurseAdmissionNote::call_api_get);
        }
        EndPoint::IpdAdmissionNoteNurse => {
            // 2 handler | 1 fetcher
            check_payload_add_rj(kphis_api_handler::ipd::admission_note_nurse::post_ipd_admission_note_nurse, ipd::admission_note_nurse::IpdNurseAdmissionNote::call_api_save);
            check_payload_add_rj(kphis_api_handler::ipd::admission_note_nurse::put_ipd_admission_note_nurse, ipd::admission_note_nurse::IpdNurseAdmissionNote::call_api_save);
        }
        EndPoint::IpdConsult => {
            check_q_rj(kphis_api_handler::ipd::consult::get_ipd_consult_list, ipd::consult::IpdConsultList::call_api_get);
            check_payload_rj(kphis_api_handler::ipd::consult::post_ipd_consult, ipd::consult::ConsultSave::call_api_post);
            check_q_rj(kphis_api_handler::ipd::consult::delete_ipd_consult_by_id, ipd::consult::Consult::call_api_delete);
        }
        EndPoint::IpdConsultAn => {
            check_ps_rj(kphis_api_handler::ipd::consult::get_ipd_consult_by_an, ipd::consult::ConsultWithName::call_api_get);
        }
        EndPoint::IpdConsultId => {
            check_pi_rj(kphis_api_handler::ipd::consult::get_ipd_consult_by_id, ipd::consult::Consult::call_api_get);
        }
        EndPoint::IpdDcPlanAn => {
            check_ps_rj(kphis_api_handler::ipd::dc_plan::get_ipd_dc_plan, dc_plan::DischargePlan::call_api_get_ipd);
            check_ps_payload_rj(kphis_api_handler::ipd::dc_plan::post_ipd_dc_plan, dc_plan::DischargePlanSave::call_api_post_ipd);
            check_ps_q_rj(kphis_api_handler::ipd::dc_plan::delete_ipd_dc_plan, dc_plan::DischargePlan::call_api_delete_ipd);
        }
        EndPoint::IpdDcPlanTmpDx => {
            check_q_rj(kphis_api_handler::ipd::dc_plan_tmp::get_ipd_dc_plan_tmp_dx, ipd::dc_plan_tmp::DcPlanTmpDx::call_api_get);
            check_payload_rj(kphis_api_handler::ipd::dc_plan_tmp::post_ipd_dc_plan_tmp_dx, ipd::dc_plan_tmp::DcPlanTmpDx::call_api_post);
            check_q_rj(kphis_api_handler::ipd::dc_plan_tmp::delete_ipd_dc_plan_tmp_dx, ipd::dc_plan_tmp::DcPlanTmpDx::call_api_delete);
        }
        EndPoint::IpdDcPlanTmpMed => {
            check_q_rj(kphis_api_handler::ipd::dc_plan_tmp::get_ipd_dc_plan_tmp_med, ipd::dc_plan_tmp::DcPlanTmpMed::call_api_get);
            check_payload_rj(kphis_api_handler::ipd::dc_plan_tmp::post_ipd_dc_plan_tmp_med, ipd::dc_plan_tmp::DcPlanTmpMed::call_api_post);
            check_q_rj(kphis_api_handler::ipd::dc_plan_tmp::delete_ipd_dc_plan_tmp_med, ipd::dc_plan_tmp::DcPlanTmpMed::call_api_delete);
        }
        EndPoint::IpdDcPlanTmpEnv => {
            check_q_rj(kphis_api_handler::ipd::dc_plan_tmp::get_ipd_dc_plan_tmp_env, ipd::dc_plan_tmp::DcPlanTmpEnv::call_api_get);
            check_payload_rj(kphis_api_handler::ipd::dc_plan_tmp::post_ipd_dc_plan_tmp_env, ipd::dc_plan_tmp::DcPlanTmpEnv::call_api_post);
            check_q_rj(kphis_api_handler::ipd::dc_plan_tmp::delete_ipd_dc_plan_tmp_env, ipd::dc_plan_tmp::DcPlanTmpEnv::call_api_delete);
        }
        EndPoint::IpdDcPlanTmpTx => {
            check_q_rj(kphis_api_handler::ipd::dc_plan_tmp::get_ipd_dc_plan_tmp_tx, ipd::dc_plan_tmp::DcPlanTmpTx::call_api_get);
            check_payload_rj(kphis_api_handler::ipd::dc_plan_tmp::post_ipd_dc_plan_tmp_tx, ipd::dc_plan_tmp::DcPlanTmpTx::call_api_post);
            check_q_rj(kphis_api_handler::ipd::dc_plan_tmp::delete_ipd_dc_plan_tmp_tx, ipd::dc_plan_tmp::DcPlanTmpTx::call_api_delete);
        }
        EndPoint::IpdDcPlanTmpDiet => {
            check_q_rj(kphis_api_handler::ipd::dc_plan_tmp::get_ipd_dc_plan_tmp_diet, ipd::dc_plan_tmp::DcPlanTmpDiet::call_api_get);
            check_payload_rj(kphis_api_handler::ipd::dc_plan_tmp::post_ipd_dc_plan_tmp_diet, ipd::dc_plan_tmp::DcPlanTmpDiet::call_api_post);
            check_q_rj(kphis_api_handler::ipd::dc_plan_tmp::delete_ipd_dc_plan_tmp_diet, ipd::dc_plan_tmp::DcPlanTmpDiet::call_api_delete);
        }
        EndPoint::IpdDoctorInCharge => {
            check_q_rj(kphis_api_handler::ipd::doctor_in_charge::get_ipd_doctor_in_charge, ipd::doctor_in_charge::IpdDoctorInCharge::call_api_get);
            check_payload_rj(kphis_api_handler::ipd::doctor_in_charge::post_ipd_doctor_in_charge, ipd::doctor_in_charge::IpdDoctorInCharge::call_api_post);
            check_q_rj(kphis_api_handler::ipd::doctor_in_charge::delete_ipd_doctor_in_charge, ipd::doctor_in_charge::IpdDoctorInCharge::call_api_delete);
        }
        EndPoint::IpdDocumentDatetimeAn => {
            check_ps_rj(kphis_api_handler::ipd::document::get_ipd_document_datetime, ipd::document::IpdDocumentDatetime::call_api_get);
        }
        EndPoint::IpdDocumentListVnAn => {
            check_pss_rj(kphis_api_handler::ipd::document::get_ipd_document_list, ipd::document::IpdDocumentExists::call_api_get);
        }
        EndPoint::IpdDocumentScanAn => {
            check_ps_rj(kphis_api_handler::ipd::document::get_ipd_document_types, ipd::document::DocumentScan::call_api_get_ipd);
            check_ps_payload_rj(kphis_api_handler::ipd::document::post_ipd_document_type, image::file_path::DocumentType::call_api_post_ipd);
            check_ps_payload_rj(kphis_api_handler::ipd::document::delete_ipd_document_type, image::file_path::DocumentType::call_api_delete_ipd);
        }
        EndPoint::IpdFocusListAn => {
            check_ps_q_rj(kphis_api_handler::ipd::focus_list::get_ipd_focus_list, focus_list::FocusList::call_api_get_ipd);
            check_ps_q_payload_rj(kphis_api_handler::ipd::focus_list::post_ipd_focus_list, focus_list::FocusListSave::call_api_post_ipd);
            check_ps_q_rj(kphis_api_handler::ipd::focus_list::delete_ipd_focus_list, focus_list::FocusList::call_api_delete_ipd);
        }
        EndPoint::IpdFocusNoteAn => {
            check_ps_q_rj(kphis_api_handler::ipd::focus_note::get_ipd_focus_note, focus_note::FocusNote::call_api_get_ipd);
            check_ps_q_payload_rj(kphis_api_handler::ipd::focus_note::post_ipd_focus_note, focus_note::FocusNoteSave::call_api_post_ipd);
            check_ps_q_rj(kphis_api_handler::ipd::focus_note::delete_ipd_focus_note, focus_note::FocusNote::call_api_delete_ipd);
        }
        EndPoint::IpdIndexActionId => {
            check_pi_add_rj(kphis_api_handler::ipd::index_action::delete_ipd_index_action, index_action::IndexAction::call_api_delete);
        }
        EndPoint::IpdIndexAction => {
            check_payload_rj(kphis_api_handler::ipd::index_action::post_ipd_index_action, index_action::IndexAction::call_api_post);
        }
        EndPoint::IpdIndexMedPayAn => {
            // kphis_api_handler::ipd::index_plan::get_ipd_index_med_pay, REPORT ONLY
        }
        EndPoint::IpdIndexMonitorId => {
            check_pi_add_rj(kphis_api_handler::ipd::index_monitor::delete_ipd_index_monitor, index_monitor::IndexMonitor::call_api_delete);
        }
        EndPoint::IpdIndexMonitor => {
            check_payload_rj(kphis_api_handler::ipd::index_monitor::post_ipd_index_monitor, index_monitor::IndexMonitor::call_api_post);
        }
        EndPoint::IpdIndexNoteId => {
            check_pi_rj(kphis_api_handler::ipd::index_note::delete_ipd_index_note, ipd::index_note::IndexNote::call_api_delete);
        }
        EndPoint::IpdIndexNote => {
            check_q_rj(kphis_api_handler::ipd::index_note::get_ipd_index_note, ipd::index_note::IndexNote::call_api_get);
            check_payload_rj(kphis_api_handler::ipd::index_note::post_ipd_index_note, ipd::index_note::IndexNote::call_api_post);
        }
        EndPoint::IpdIndexPlanDateAn => {
            check_ps_rj(kphis_api_handler::ipd::index_plan::get_index_plan_date, index_plan::IndexPlanDate::call_api_get);
        }
        EndPoint::IpdIndexPlanId => {
            check_pi_add_rj(kphis_api_handler::ipd::index_plan::delete_ipd_index_plan, index_plan::IndexPlan::call_api_delete);
        }
        EndPoint::IpdIndexPlan => {
            check_payload_rj(kphis_api_handler::ipd::index_plan::post_ipd_index_plan, index_plan::IndexPlanSave::call_api_post);
        }
        EndPoint::IpdIo => {
            check_q_rj(kphis_api_handler::ipd::io::get_ipd_io_shift, ipd::io::IoShift::call_api_get_ipd);
            check_payload_rj(kphis_api_handler::ipd::io::post_ipd_io_shift, ipd::io::IoShift::call_api_post);
            check_q_add_rj(kphis_api_handler::ipd::io::delete_ipd_io_shift, ipd::io::IoShift::call_api_delete);
        }
        EndPoint::IpdIoDateAn => {
            check_ps_rj(kphis_api_handler::ipd::io::get_ipd_io_date, ipd::io::IoDate::call_api_get_ipd);
        }
        EndPoint::IpdMedReconcile => {
            check_q_add_rj(kphis_api_handler::ipd::med_reconcile::get_ipd_med_reconcile, med_reconcile::MedReconciliation::call_api_get);
            check_q_payloads_add_rj(kphis_api_handler::ipd::med_reconcile::post_ipd_med_reconcile, med_reconcile::MedReconciliation::call_api_post);
            check_q_payloads_add_rj(kphis_api_handler::ipd::med_reconcile::patch_ipd_med_reconcile, med_reconcile::MedReconciliation::call_api_patch);
            check_q_add_rj(kphis_api_handler::ipd::med_reconcile::delete_ipd_med_reconcile, med_reconcile::MedReconciliation::call_api_delete);
        }
        EndPoint::IpdMedReconcileHosxpAn => {
            check_ps_rj(kphis_api_handler::ipd::med_reconcile::get_ipd_med_reconcile_hosxp, med_reconcile::MedReconciliationDetail::call_api_get_ipd);
        }
        EndPoint::IpdMedReconcileLastDoseAn => {
            check_ps_rj(kphis_api_handler::ipd::med_reconcile::get_ipd_med_reconcile_last_dose, med_reconcile::AdmissionNoteLastDose::call_api_get);
        }
        EndPoint::IpdMedReconcileNoteId => {
            check_pi_rj(kphis_api_handler::ipd::med_reconcile::get_ipd_med_reconcile_note, med_reconcile::MedReconciliationNote::call_api_get_ipd);
            check_pi_payload_rj(kphis_api_handler::ipd::med_reconcile::post_ipd_med_reconcile_note, med_reconcile::MedReconciliationNote::call_api_post_ipd);
        }
        EndPoint::IpdMedReconcileRemedVisitHn => {
            check_ps_rj(kphis_api_handler::ipd::med_reconcile::get_ipd_med_reconcile_remed_visit, med_reconcile::ReMedVisit::call_api_get);
        }
        EndPoint::IpdMedReconcileRemedMed => {
            check_q_rj(kphis_api_handler::ipd::med_reconcile::get_ipd_med_reconcile_remed_med, med_reconcile::ReMedMedication::call_api_get);
        }
        EndPoint::IpdMra => {
            check_q_rj(kphis_api_handler::ipd::mra::get_ipd_mra, ipd::mra::IpdMra::call_api_get);
            // 2 handlers / 1 fetcher
            check_payload_rj(kphis_api_handler::ipd::mra::post_ipd_mra, ipd::mra::IpdMra::call_api_save);
            check_payload_rj(kphis_api_handler::ipd::mra::put_ipd_mra, ipd::mra::IpdMra::call_api_save);
            check_q_rj(kphis_api_handler::ipd::mra::delete_ipd_mra, ipd::mra::IpdMra::call_api_delete);
        }
        EndPoint::IpdOrderItem => {
            check_q_rj(kphis_api_handler::ipd::order::get_ipd_order_item, order::OrderItem::call_api_get_ipd);
            check_payload_add_rj(kphis_api_handler::ipd::order::patch_ipd_order_item, order::OrderItemPatch::call_api_patch);
        }
        EndPoint::IpdOrderPrevious => {
            check_q_rj(kphis_api_handler::ipd::order::get_ipd_order_previous, order::OrderItem::call_api_get_ipd_previous);
        }
        EndPoint::IpdOrderOnedayPreviousAn => {
            check_ps_rj(kphis_api_handler::ipd::order::get_ipd_order_one_day_previous, order::MedOrderItem::call_api_get_ipd_oneday_previous);
        }
        EndPoint::IpdOrderProgressPrevious => {
            check_q_rj(kphis_api_handler::ipd::progress_note::get_ipd_progress_previous, progress_note::ProgressNoteItem::call_api_get_previous);
        }
        EndPoint::IpdOrderToHomeMedAn => {
            check_ps_rj(kphis_api_handler::ipd::order::get_ipd_home_med_from_cont, order::MedOrderItem::call_api_get_ipd_cont_to_home_med);
        }
        EndPoint::IpdOrderOrderDateAn => {
            check_ps_rj(kphis_api_handler::ipd::order::get_ipd_order_date, order::OrderDate::call_api_get);
        }
        EndPoint::IpdOrderOrderId => {
            check_pi_add_rj(kphis_api_handler::ipd::order::delete_ipd_order, order::Order::call_api_delete);
        }
        EndPoint::IpdOrderOrder => {
            check_q_rj(kphis_api_handler::ipd::order::get_ipd_order, order::Order::call_api_get_ipd);
            check_payload_rj(kphis_api_handler::ipd::order::post_ipd_order, order::OrderSave::call_api_post);
            check_payload_add_rj(kphis_api_handler::ipd::order::patch_ipd_order, order::OrderPatch::call_api_patch);
        }
        EndPoint::IpdOrderProgressNoteId => {
            check_pi_add_rj(kphis_api_handler::ipd::progress_note::delete_ipd_progress_note, progress_note::ProgressNote::call_api_delete);
        }
        EndPoint::IpdOrderProgressNote => {
            check_q_rj(kphis_api_handler::ipd::progress_note::get_ipd_progress_note, progress_note::ProgressNote::call_api_get_ipd);
            check_payload_rj(kphis_api_handler::ipd::progress_note::post_ipd_progress_note, progress_note::ProgressNoteSave::call_api_post);
        }
        EndPoint::IpdOrderPharmacy => {
            check_q_rj(kphis_api_handler::ipd::order::get_ipd_order_pharmacy, ipd::pharmacy_monitor::IpdOrderPharmacyMonitor::call_api_get);
        }
        EndPoint::IpdPasscode => {
            check_rj(kphis_api_handler::ipd::passcode::get_ipd_ward_passcode, ipd::passcode::ConfigIpdWardPasscode::call_api_get);
            check_payload_rj(kphis_api_handler::ipd::passcode::post_ipd_ward_passcode, ipd::passcode::PasscodeGenRequest::call_api_post);
        }
        EndPoint::IpdPostAdmitCount => {
            check_rj(kphis_api_handler::post_admit::get_ipd_post_admit_count, post_admit::get_post_admit_count);
        }
        EndPoint::IpdPostAdmitList => {
            check_q_rj(kphis_api_handler::post_admit::get_ipd_post_admit_list, post_admit::PostAdmitList::call_api_get);
        }
        EndPoint::IpdPreAdmit => {
            check_q_rj(kphis_api_handler::pre_admit::get_ipd_pre_admit_list, pre_admit::PreAdmitList::call_api_get);
            check_payload_rj(kphis_api_handler::pre_admit::post_ipd_pre_admit, pre_admit::PreAdmitSave::call_api_post);
            check_payload_rj(kphis_api_handler::pre_admit::patch_ipd_pre_admit, pre_admit::PreAdmitPatch::call_api_patch);
        }
        EndPoint::IpdPreOrderMasterId => {
            check_pi_rj(kphis_api_handler::pre_order::delete_ipd_pre_order_master, pre_order::master::PreOrderMaster::call_api_delete);
        }
        EndPoint::IpdPreOrderMaster => {
            check_q_rj(kphis_api_handler::pre_order::get_ipd_pre_order_list, pre_order::master::PreOrderMaster::call_api_get);
            check_payload_rj(kphis_api_handler::pre_order::post_ipd_pre_order_master, pre_order::master::PreOrderMasterSave::call_api_post);
        }
        EndPoint::IpdPreOrderInto => {
            check_payload_rj(kphis_api_handler::pre_order::post_ipd_pre_order_into, pre_order::order::PreOrderIntoCommand::call_api_post);
        }
        EndPoint::IpdPreOrderOrderId => {
            check_pi_rj(kphis_api_handler::pre_order::delete_ipd_pre_order, pre_order::order::PreOrder::call_api_delete);
        }
        EndPoint::IpdPreOrderOrder => {
            check_q_rj(kphis_api_handler::pre_order::get_ipd_pre_order, pre_order::order::PreOrder::call_api_get);
            check_payload_rj(kphis_api_handler::pre_order::post_ipd_pre_order, pre_order::order::PreOrderSave::call_api_post);
        }
        EndPoint::IpdPreOrderProgressNoteId => {
            check_pi_rj(kphis_api_handler::pre_order::delete_ipd_pre_progress_note, pre_order::progress_note::PreProgressNote::call_api_delete);
        }
        EndPoint::IpdPreOrderProgressNote => {
            check_q_rj(kphis_api_handler::pre_order::get_ipd_pre_progress_note, pre_order::progress_note::PreProgressNote::call_api_get);
            check_payload_rj(kphis_api_handler::pre_order::post_ipd_pre_progress_note, pre_order::progress_note::PreProgressNoteSave::call_api_post);
        }
        EndPoint::IpdShowPatientMainAn => {
            check_ps_rj(kphis_api_handler::ipd::show_patient_main::get_ipd_show_patient_main, patient_info::PatientInfo::call_api_get_an);
        }
        EndPoint::IpdSummary => {
            check_q_rj(kphis_api_handler::ipd::summary::get_ipd_summary, ipd::summary::Summary::call_api_get);
            check_payload_rj(kphis_api_handler::ipd::summary::post_ipd_summary, ipd::summary::SummarySave::call_api_post);
            check_payload_rj(kphis_api_handler::ipd::summary::patch_ipd_summary, ipd::summary::SummaryCodeSave::call_api_patch);
        }
        EndPoint::IpdSummaryAudit => {
            check_q_rj(kphis_api_handler::ipd::summary_audit::get_ipd_summary_audit, ipd::summary_audit::SummaryAudit::call_api_get);
            check_payload_rj(kphis_api_handler::ipd::summary_audit::post_ipd_summary_audit, ipd::summary_audit::SummaryAudit::call_api_save);
            check_q_rj(kphis_api_handler::ipd::summary_audit::delete_ipd_summary_audit, ipd::summary_audit::SummaryAudit::call_api_delete);
        }
        EndPoint::IpdSummaryNoteId => {
            check_pi_rj(kphis_api_handler::ipd::summary::get_ipd_summary_note, ipd::summary::SummaryNote::call_api_get);
            // 3 hanbdlers / 1 fetcher
            check_pi_payload_add_rj(kphis_api_handler::ipd::summary::post_ipd_summary_note, ipd::summary::SummaryNoteSave::call_api_save);
            check_pi_payload_add_rj(kphis_api_handler::ipd::summary::patch_ipd_summary_note, ipd::summary::SummaryNoteSave::call_api_save);
            check_pi_payload_add_rj(kphis_api_handler::ipd::summary::delete_ipd_summary_note, ipd::summary::SummaryNoteSave::call_api_save);
        }
        EndPoint::IpdSummaryStatusId => {
            check_pi_rj(kphis_api_handler::ipd::summary::get_ipd_summary_status, ipd::summary::SummaryStatus::call_api_get);
            check_pi_payload_rj(kphis_api_handler::ipd::summary::put_ipd_summary_status, ipd::summary::SummaryStatus::call_api_put);
        }
        EndPoint::IpdTmpGroup => {
            check_q_rj(kphis_api_handler::ipd::tmp::get_ipd_tmp_group, ipd::tmp::TmpGroup::call_api_get);
            check_payload_rj(kphis_api_handler::ipd::tmp::post_ipd_tmp_group, ipd::tmp::TmpGroup::call_api_post);
            check_q_rj(kphis_api_handler::ipd::tmp::delete_ipd_tmp_group, ipd::tmp::TmpGroup::call_api_delete);
        }
        EndPoint::IpdTmpSubgroup => {
            check_q_rj(kphis_api_handler::ipd::tmp::get_ipd_subgroup, ipd::tmp::TmpSubGroup::call_api_get);
            check_payload_rj(kphis_api_handler::ipd::tmp::post_ipd_subgroup, ipd::tmp::TmpSubGroup::call_api_post);
            check_q_rj(kphis_api_handler::ipd::tmp::delete_ipd_subgroup, ipd::tmp::TmpSubGroup::call_api_delete);
        }
        EndPoint::IpdTmpFocus => {
            check_q_rj(kphis_api_handler::ipd::tmp::get_ipd_focus, ipd::tmp::TmpFocus::call_api_get);
            check_payload_rj(kphis_api_handler::ipd::tmp::post_ipd_focus, ipd::tmp::TmpFocus::call_api_post);
            check_q_rj(kphis_api_handler::ipd::tmp::delete_ipd_focus, ipd::tmp::TmpFocus::call_api_delete);
        }
        EndPoint::IpdTmpGoal => {
            check_q_rj(kphis_api_handler::ipd::tmp::get_ipd_goal, ipd::tmp::TmpGoal::call_api_get);
            check_payload_rj(kphis_api_handler::ipd::tmp::post_ipd_goal, ipd::tmp::TmpGoal::call_api_post);
            check_q_rj(kphis_api_handler::ipd::tmp::delete_ipd_goal, ipd::tmp::TmpGoal::call_api_delete);
        }
        EndPoint::IpdTmpIntvt => {
            check_q_rj(kphis_api_handler::ipd::tmp::get_ipd_intvt, ipd::tmp::TmpIntvt::call_api_get);
            check_payload_rj(kphis_api_handler::ipd::tmp::post_ipd_intvt, ipd::tmp::TmpIntvt::call_api_post);
            check_q_rj(kphis_api_handler::ipd::tmp::delete_ipd_intvt, ipd::tmp::TmpIntvt::call_api_delete);
        }
        EndPoint::IpdTmpDlc => {
            check_q_rj(kphis_api_handler::ipd::tmp::get_ipd_dlc, ipd::tmp::TmpDlc::call_api_get);
            check_payload_rj(kphis_api_handler::ipd::tmp::post_ipd_dlc, ipd::tmp::TmpDlc::call_api_post);
            check_q_rj(kphis_api_handler::ipd::tmp::delete_ipd_dlc, ipd::tmp::TmpDlc::call_api_delete);
        }
        EndPoint::IpdVitalSignId => {
            check_pi_add_rj(kphis_api_handler::ipd::vital_sign::delete_ipd_vital_sign, vital_sign::VitalSign::call_api_delete);
        }
        EndPoint::IpdVitalSign => {
            check_q_add_rj(kphis_api_handler::ipd::vital_sign::get_ipd_vital_sign, vital_sign::VitalSign::call_api_get);
            // 2 handlers / 1 fetcher
            check_q_payload_add2_rj(kphis_api_handler::ipd::vital_sign::post_ipd_vital_sign, vital_sign::VitalSignSave::call_api_save);
            check_q_payload_add2_rj(kphis_api_handler::ipd::vital_sign::put_ipd_vital_sign, vital_sign::VitalSignSave::call_api_save);
        }
        EndPoint::LabHead => {
            check_q_rj(kphis_api_handler::lab::get_lab_head, lab::LabHead::call_api_get);
        }
        EndPoint::LabItem => {
            check_q_rj(kphis_api_handler::lab::get_lab_item, lab::LabItem::call_api_get);
        }
        EndPoint::LabReadId => {
            check_pi_rj(kphis_api_handler::lab::post_lab_read, lab::LabHead::call_api_post_readed);
            check_pi_rj(kphis_api_handler::lab::delete_lab_read, lab::LabHead::call_api_delete_readed);
        }
        EndPoint::LabWbcKeyValue => {
            check_pss_rj(kphis_api_handler::lab::get_wbc_band, lab::LabWbcBand::call_api_get);
        }
        EndPoint::MedReconcileHn => {
            check_ps_rj(kphis_api_handler::med_reconciliation::get_med_reconciliation_head, med_reconcile::MedReconciliationHeader::call_api_get);
        }
        EndPoint::OpdErDcPlanId => {
            check_pi_rj(kphis_api_handler::opd_er::dc_plan::get_opd_er_dc_plan, dc_plan::DischargePlan::call_api_get_opd_er);
            check_pi_payload_rj(kphis_api_handler::opd_er::dc_plan::post_opd_er_dc_plan, dc_plan::DischargePlanSave::call_api_post_opd_er);
            check_pi_q_rj(kphis_api_handler::opd_er::dc_plan::delete_opd_er_dc_plan, dc_plan::DischargePlan::call_api_delete_opd_er);
        }
        EndPoint::OpdErDocumentListVnId => {
            check_psi_rj(kphis_api_handler::opd_er::document::get_opd_er_document_list, opd_er::document::OpdErDocumentExists::call_api_get);
        }
        EndPoint::OpdErDocumentScanId => {
            check_pi_rj(kphis_api_handler::opd_er::document::get_opd_er_document_types, ipd::document::DocumentScan::call_api_get_opd_er);
            check_pi_payload_rj(kphis_api_handler::opd_er::document::post_opd_er_document_type, image::file_path::DocumentType::call_api_post_opd_er);
            check_pi_payload_rj(kphis_api_handler::opd_er::document::delete_opd_er_document_type, image::file_path::DocumentType::call_api_delete_opd_er);
        }
        EndPoint::OpdErFocusListId => {
            check_pi_q_rj(kphis_api_handler::opd_er::focus_list::get_opd_er_focus_list, focus_list::FocusList::call_api_get_opd_er);
            check_pi_payload_rj(kphis_api_handler::opd_er::focus_list::post_opd_er_focus_list, focus_list::FocusListSave::call_api_post_opd_er);
            check_pi_q_rj(kphis_api_handler::opd_er::focus_list::delete_opd_er_focus_list, focus_list::FocusList::call_api_delete_opd_er);
        }
        EndPoint::OpdErFocusNoteId => {
            check_pi_q_rj(kphis_api_handler::opd_er::focus_note::get_opd_er_focus_note, focus_note::FocusNote::call_api_get_opd_er);
            check_pi_payload_rj(kphis_api_handler::opd_er::focus_note::post_opd_er_focus_note, focus_note::FocusNoteSave::call_api_post_opd_er);
            check_pi_q_rj(kphis_api_handler::opd_er::focus_note::delete_opd_er_focus_note, focus_note::FocusNote::call_api_delete_opd_er);
        }
        EndPoint::OpdErHisMedVn => {
            check_ps_rj(kphis_api_handler::opd_er::hosxp_med::get_opd_med, opd_er::hosxp_med::OpdMed::call_api_get);
        }
        EndPoint::OpdErIndexActionId => {
            check_pi_add_rj(kphis_api_handler::opd_er::index_action::delete_opd_er_index_action, index_action::IndexAction::call_api_delete);
        }
        EndPoint::OpdErIndexAction => {
            check_payload_rj(kphis_api_handler::opd_er::index_action::post_opd_er_index_action, index_action::IndexAction::call_api_post);
        }
        EndPoint::OpdErIndexMonitorId => {
            check_pi_add_rj(kphis_api_handler::opd_er::index_monitor::delete_opd_er_index_monitor, index_monitor::IndexMonitor::call_api_delete);
        }
        EndPoint::OpdErIndexMonitor => {
            check_payload_rj(kphis_api_handler::opd_er::index_monitor::post_opd_er_index_monitor, index_monitor::IndexMonitor::call_api_post);
        }
        EndPoint::OpdErIndexPlanId => {
            check_pi_add_rj(kphis_api_handler::opd_er::index_plan::delete_opd_er_index_plan, index_plan::IndexPlan::call_api_delete);
        }
        EndPoint::OpdErIndexPlan => {
            check_payload_rj(kphis_api_handler::opd_er::index_plan::post_opd_er_index_plan, index_plan::IndexPlanSave::call_api_post);
        }
        EndPoint::OpdErIo => {
            check_q_rj(kphis_api_handler::opd_er::io::get_opd_er_io_shift, ipd::io::IoShift::call_api_get_opd_er);
            check_payload_rj(kphis_api_handler::opd_er::io::post_opd_er_io_shift, ipd::io::IoShift::call_api_post);
            check_q_add_rj(kphis_api_handler::opd_er::io::delete_opd_er_io_shift, ipd::io::IoShift::call_api_delete);
        }
        EndPoint::OpdErIoDateId => {
            check_pi_rj(kphis_api_handler::opd_er::io::get_opd_er_io_date, ipd::io::IoDate::call_api_get_opd_er);
        }
        EndPoint::OpdErMedicalHistory => {
            check_q_rj(kphis_api_handler::opd_er::medical_history::get_opd_er_medical_history, opd_er::medical_history::OpdErMedicalHistory::call_api_get);
        }
        EndPoint::OpdErMedicalHistoryTrauma => {
            check_q_rj(kphis_api_handler::opd_er::medical_history::get_opd_er_trauma_history, opd_er::medical_history::TraumaHistory::call_api_get);
            check_payload_rj(kphis_api_handler::opd_er::medical_history::post_opd_er_trauma_history, opd_er::medical_history::TraumaHistory::call_api_post);
        }
        EndPoint::OpdErMedicalHistoryAllergy => {
            check_q_rj(kphis_api_handler::opd_er::medical_history::get_opd_er_allergy_history, opd_er::medical_history::AllergyHistory::call_api_get);
            check_payloads_rj(kphis_api_handler::opd_er::medical_history::post_opd_er_allergy_history, opd_er::medical_history::AllergyHistory::call_api_post);
        }
        EndPoint::OpdErMedicalHistoryScreen => {
            check_q_rj(kphis_api_handler::opd_er::medical_history::get_opd_er_screen_history, opd_er::medical_history::NurseScreeningHistory::call_api_get);
            check_q_payload_rj(kphis_api_handler::opd_er::medical_history::post_opd_er_screen_history, opd_er::medical_history::NurseScreeningHistory::call_api_post);
        }
        EndPoint::OpdErMedicalHistoryConsult => {
            check_q_rj(kphis_api_handler::opd_er::medical_history::get_opd_er_consult_history, opd_er::medical_history::ConsultHistory::call_api_get);
            check_payloads_rj(kphis_api_handler::opd_er::medical_history::post_opd_er_consult_history, opd_er::medical_history::ConsultHistory::call_api_post);
        }
        EndPoint::OpdErMedicalHistoryScan => {
            check_q_rj(kphis_api_handler::opd_er::medical_history::get_opd_er_scan_history, opd_er::medical_history::ScanHistory::call_api_get);
            check_payload_rj(kphis_api_handler::opd_er::medical_history::post_opd_er_scan_history, opd_er::medical_history::ScanHistory::call_api_post);
        }
        EndPoint::OpdErMedicalHistoryFt => {
            check_q_rj(kphis_api_handler::opd_er::medical_history::get_opd_er_ft_history, opd_er::medical_history::SetFtHistory::call_api_get);
            check_payload_rj(kphis_api_handler::opd_er::medical_history::post_opd_er_ft_history, opd_er::medical_history::SetFtHistory::call_api_post);
        }
        EndPoint::OpdErMedReconcile => {
            check_q_add_rj(kphis_api_handler::opd_er::med_reconcile::get_opd_er_med_reconcile, med_reconcile::MedReconciliation::call_api_get);
            check_q_payloads_add_rj(kphis_api_handler::opd_er::med_reconcile::post_opd_er_med_reconcile, med_reconcile::MedReconciliation::call_api_post);
            check_q_payloads_add_rj(kphis_api_handler::opd_er::med_reconcile::patch_opd_er_med_reconcile, med_reconcile::MedReconciliation::call_api_patch);
            check_q_add_rj(kphis_api_handler::opd_er::med_reconcile::delete_opd_er_med_reconcile, med_reconcile::MedReconciliation::call_api_delete);
        }
        EndPoint::OpdErMedReconcileNoteId => {
            check_pi_rj(kphis_api_handler::opd_er::med_reconcile::get_opd_er_med_reconcile_note, med_reconcile::MedReconciliationNote::call_api_get_opd_er);
            check_pi_payload_rj(kphis_api_handler::opd_er::med_reconcile::post_opd_er_med_reconcile_note, med_reconcile::MedReconciliationNote::call_api_post_opd_er);
        }
        EndPoint::OpdErOrderMasterCheckVn => {
            check_ps_rj(kphis_api_handler::opd_er::order_master::get_opd_er_order_master_check, opd_er::order_master::OpdErOrderMasterCheck::call_api_get);
        }
        EndPoint::OpdErOrderMasterId => {
            check_pi_rj(kphis_api_handler::opd_er::order_master::get_opd_er_order_master, opd_er::order_master::OpdErOrderMaster::call_api_get);
        }
        EndPoint::OpdErOrderMaster => {
            check_q_rj(kphis_api_handler::opd_er::order_master::get_opd_er_order_master_list, opd_er::order_master::OpdErOrderMasterList::call_api_get);
            check_payload_rj(kphis_api_handler::opd_er::order_master::post_opd_er_order_master, opd_er::order_master::OpdErOrderMasterSave::call_api_post);
        }
        EndPoint::OpdErOrderItem => {
            check_q_rj(kphis_api_handler::opd_er::order::get_opd_er_order_item, order::OrderItem::call_api_get_opd_er);
            check_payload_add_rj(kphis_api_handler::opd_er::order::patch_opd_er_order_item, order::OrderItemPatch::call_api_patch);
        }
        EndPoint::OpdErOrderOrderId => {
            check_pi_add_rj(kphis_api_handler::opd_er::order::delete_opd_er_order, order::Order::call_api_delete);
        }
        EndPoint::OpdErOrderOrder => {
            check_q_rj(kphis_api_handler::opd_er::order::get_opd_er_order, order::Order::call_api_get_opd_er);
            check_payload_rj(kphis_api_handler::opd_er::order::post_opd_er_order, order::OrderSave::call_api_post);
            check_payload_add_rj(kphis_api_handler::opd_er::order::patch_opd_er_order, order::OrderPatch::call_api_patch);
        }
        EndPoint::OpdErOrderProgressNoteId => {
            check_pi_add_rj(kphis_api_handler::opd_er::progress_note::delete_opd_er_progress_note, progress_note::ProgressNote::call_api_delete);
        }
        EndPoint::OpdErOrderProgressNote => {
            check_q_rj(kphis_api_handler::opd_er::progress_note::get_opd_er_progress_note, progress_note::ProgressNote::call_api_get_opd_er);
            check_payload_rj(kphis_api_handler::opd_er::progress_note::post_opd_er_progress_note, progress_note::ProgressNoteSave::call_api_post);
        }
        EndPoint::OpdErOrderPharmacy => {
            check_q_rj(kphis_api_handler::opd_er::order::get_opd_er_order_pharmacy, opd_er::pharmacy_monitor::OpdErOrderPharmacyMonitor::call_api_get);
        }
        EndPoint::OpdErShowPatientMainId => {
            check_pi_rj(kphis_api_handler::opd_er::show_patient_main::get_opd_er_show_patient_main_id, patient_info::PatientInfo::call_api_get_id);
        }
        EndPoint::OpdErShowPatientMainVn => {
            check_ps_rj(kphis_api_handler::opd_er::show_patient_main::get_opd_er_show_patient_main_vn, patient_info::PatientInfo::call_api_get_vn);
        }
        EndPoint::OpdErVitalSignId => {
            check_pi_add_rj(kphis_api_handler::opd_er::vital_sign::delete_opd_er_vital_sign, vital_sign::VitalSign::call_api_delete);
        }
        EndPoint::OpdErVitalSign => {
            check_q_add_rj(kphis_api_handler::opd_er::vital_sign::get_opd_er_vital_sign, vital_sign::VitalSign::call_api_get);
            // 2 handlers / 1 fetcher
            check_q_payload_add2_rj(kphis_api_handler::opd_er::vital_sign::post_opd_er_vital_sign, vital_sign::VitalSignSave::call_api_save);
            check_q_payload_add2_rj(kphis_api_handler::opd_er::vital_sign::put_opd_er_vital_sign, vital_sign::VitalSignSave::call_api_save);
        }
        EndPoint::PrescrptionScreen => {
            check_q_rj(kphis_api_handler::prescription::get_prescription_screen, prescription::PrescriptionScreen::call_api_get);
            check_q_rj(kphis_api_handler::prescription::post_prescription_screen, prescription::PrescriptionScreen::call_api_post);
            check_q_payload_rj(kphis_api_handler::prescription::patch_prescription_screen, prescription::PrescriptionScreenPatch::call_api_patch);
        }
        EndPoint::ReferNoteVnan => {
            check_ps_rj(kphis_api_handler::refer_note::get_refernote, refer_note::ReferNote::call_api_get);
            check_ps_payload_rj(kphis_api_handler::refer_note::post_refernote, refer_note::ReferNoteSave::call_api_post);
        }
        EndPoint::ReportCustom => {
            check_q_rj(kphis_api_handler::report::get_custom_report, report::CustomReport::call_api_get);
            check_payload_rj(kphis_api_handler::report::post_custom_report, report::CustomReport::call_api_post);
            check_q_rj(kphis_api_handler::report::delete_custom_report, report::CustomReport::call_api_delete);
        }
        EndPoint::ReportRawQuery => {
            check_payload_rj(kphis_api_handler::report::post_query_to_json_string, report::ReportQuery::call_api_post);
        }
        EndPoint::ReportRawTemplateTypeId => {
            check_psss_rj(kphis_api_pdf::handler::get_raw_single_template, report::TypstRaw::call_api_get);
        }
        EndPoint::ReportTemplateTypeId => {
            check_psss_rb(kphis_api_pdf::handler::get_single_pdf, app::AppState::call_api_get_pdf_report);
        }
        EndPoint::ScanHisImage => {
            check_q_rj(kphis_api_handler::image::scan_his::get_scan_his_image, image::scan_his::ScanImage::call_api_get);
        }
        EndPoint::SearchBoxHospText => {
            check_ps_rj(kphis_api_handler::search::searchbox::get_hosp_searchbox, search::searchbox::HospSearchBox::call_api_get);
        }
        EndPoint::SearchBoxMedDuplicate => {
            check_q_rj(kphis_api_handler::search::searchbox::get_drug_duplication_check, search::searchbox::DrugDuplicateCheck::call_api_get);
        }
        EndPoint::SearchBoxMedInteraction => {
            check_q_rj(kphis_api_handler::search::searchbox::get_drug_interaction_check, search::searchbox::DrugInteractionCheck::call_api_get);
        }
        EndPoint::SearchBoxMedHnText => {
            check_pss_rj(kphis_api_handler::search::searchbox::get_med_searchbox, search::searchbox::MedSearchbox::call_api_get);
        }
        EndPoint::SearchBoxOpdVisitModeText => {
            check_pss_rj(kphis_api_handler::search::searchbox::get_opd_visit_searchbox, search::searchbox::OpdVisitSearchbox::call_api_get);
        }
        EndPoint::SearchBoxIvfluidText => {
            check_ps_rj(kphis_api_handler::search::searchbox::get_ivfluid_searchbox, search::searchbox::IvfluidSearchbox::call_api_get);
        }
        EndPoint::SearchBoxLabText => {
            check_ps_rj(kphis_api_handler::search::searchbox::get_lab_searchbox, search::searchbox::LabSearchbox::call_api_get);
        }
        EndPoint::SearchBoxPatientText => {
            check_ps_rj(kphis_api_handler::search::searchbox::get_patient_searchbox, search::searchbox::PatientSearchbox::call_api_get);
        }
        EndPoint::SearchBoxXrayText => {
            check_ps_rj(kphis_api_handler::search::searchbox::get_xray_searchbox, search::searchbox::XraySearchbox::call_api_get);
        }
        EndPoint::SearchDr => {
            check_q_rj(kphis_api_handler::search::ipd_search_patient_dr::get_ipd_dr_search_patient, search::ipd_search_patient_dr::IpdSearchPatientDrResponse::call_api_get);
        }
        EndPoint::SearchNurse => {
            check_q_rj(kphis_api_handler::search::ipd_search_patient_nurse::get_ipd_nurse_search_patient, search::ipd_search_patient_nurse::IpdSearchPatientNurseResponse::call_api_get);
        }
        EndPoint::SearchPharmacist => {
            check_q_rj(kphis_api_handler::search::ipd_search_patient_pharmacist::get_ipd_pharmacist_search_patient, search::ipd_search_patient_pharmacist::IpdSearchPatientPharmacistResponse::call_api_get);
        }
        EndPoint::SearchOther => {
            check_q_rj(kphis_api_handler::search::ipd_search_patient_other::get_ipd_other_search_patient, search::ipd_search_patient_other::IpdSearchPatientOtherResponse::call_api_get);
        }
        EndPoint::Sse => {
            // kphis_api_handler::sse::get_sse, NEW EventSource
            check_rj(kphis_api_handler::sse::logout, app::AppState::call_api_delete_sse_end);
        }
        EndPoint::SseGroup => {
            check_payload_rj(kphis_api_handler::sse::post_sse_group, sse::SseGroup::call_api_post);
        }
        EndPoint::SseMessage => {
            check_q_rj(kphis_api_handler::sse::get_sse_message, sse::SseMessage::call_api_get);
            check_payload_rj(kphis_api_handler::sse::post_sse_message, sse::SsePostMessage::call_api_post);
            check_payloads_rj(kphis_api_handler::sse::patch_sse_messages, sse::SseMessage::call_api_patch);
        }
        EndPoint::User => {
            // GET
            check_login_rj(kphis_api_handler::user::his::refresh_token, user::his::LoginResponse::call_api_get_access_renew);
            // POST
            check_login_payload_rj(kphis_api_handler::user::his::check_login, user::his::LoginResponse::call_api_post_access);
            // PUT
            check_login_payload_rj(kphis_api_handler::user::his::refresh_cookie, user::his::LoginResponse::call_api_put_refresh_renew);
            // PATCH
            check_login_payload_add_rj(kphis_api_handler::user::his::check_totp, user::his::LoginResponse::call_api_patch_access_2fa);
        }
        EndPoint::UserConfig => {
            check_payload_rj(kphis_api_handler::user::config::post_user_config, user::config::UserConfigResponse::call_api_post);
            check_payload_rj(kphis_api_handler::user::config::patch_user_config, user::config::UserConfigCommand::call_api_patch);
        }
        EndPoint::UserRolePrelude => {
            check_rj(kphis_api_handler::user::role::get_user_role_prelude, user::role::UserRoleOptions::call_api_get);
        }
        EndPoint::UserRoleRole => {
            check_q_rj(kphis_api_handler::user::role::get_role_permission_list, user::role::RolePermissionList::call_api_get);
            check_payload_rj(kphis_api_handler::user::role::post_role_permission, user::role::RolePermissionSave::call_api_post);
            check_q_rj(kphis_api_handler::user::role::delete_role_permission, user::role::RolePermissionSave::call_api_delete);
        }
        EndPoint::UserRoleUser => {
            check_q_rj(kphis_api_handler::user::role::get_user_role_list, user::role::UserRoleList::call_api_get);
            check_payload_rj(kphis_api_handler::user::role::post_user_role, user::role::UserRoleSave::call_api_post);
        }
        EndPoint::XrayReportHn => {
            check_ps_rj(kphis_api_handler::xray::get_xray_report, xray::XrayReport::call_api_get);
        }
        EndPoint::XrayReadId => {
            check_pi_rj(kphis_api_handler::xray::post_xray_read, pacs::PacsXnData::call_api_post_readed);
            check_pi_rj(kphis_api_handler::xray::delete_xray_read, pacs::PacsXnData::call_api_delete_readed);
        }
        EndPoint::XrayPacsXn => {
            check_pi_rj(kphis_api_handler::pacs::get_pacs_xn, pacs::PacsXnData::call_api_get);
        }
        EndPoint::Unknown => {}
    }
}
