use sqlx::{AssertSqlSafe, FromRow, MySql, Pool, Row};
use std::cmp::Ordering;
use time::Date;

use kphis_model::{
    fetch::ExecuteResponse,
    prescription::{
        DrugInteraction, Lab, Medicine, NextAppointment, PostalPatch, PrescriptionInfo, PrescriptionScreen, PrescriptionScreenParams, PrescriptionScreenPatch, PrescriptionVn, PtNote, TelemedPatch,
        VisitDate,
    },
};
use kphis_sql::prescription;
use kphis_util::{
    error::{AppError, Source},
    util::str_some,
};

use super::{query1_all, query1_opt, query2_all};

// pharmacy-prescription-screen-data-select.php
#[allow(clippy::too_many_arguments)]
pub async fn get_prescription_screen(
    params: PrescriptionScreenParams,
    hlen: usize,
    vlen: usize,
    egfr_codes: &[u64],
    scr_codes: &[u64],
    lab_codes: &[(String, Vec<u64>)],
    message_icodes: &[(String, Vec<String>)],
    message_egfr_icodes: &[(String, u64, Vec<String>)],
    message_crcl_icodes: &[(String, u64, Vec<String>)],
    pool: &Pool<MySql>,
    hosxp: &str,
    kphis_extra: &str,
) -> Result<PrescriptionScreen, AppError> {
    let patient_opt = params.search.and_then(str_some);
    let info = if patient_opt.is_some() {
        let mut info_opt = select_info_qn(&patient_opt, hlen, vlen, pool, hosxp).await?;
        if let Some(mut info) = info_opt.take() {
            if let Some(hn) = info.hn.as_ref() {
                info.dates = select_info_date(hn, pool, hosxp).await?;
                info.notes = select_note_from_hosxp(hn, pool, hosxp).await?;
                info.drug_allergies = select_drug_allergy(hn, pool, hosxp).await?;

                let last_egfr = select_egfr(egfr_codes, true, &None, hn, pool, hosxp).await?.unwrap_or(Lab::new("eGFR"));
                info.last_labs.push(last_egfr);

                let last_crcl = select_crcl(scr_codes, true, &None, hn, pool, hosxp).await?.unwrap_or(Lab::new("CrCl"));
                info.last_labs.push(last_crcl);

                if !lab_codes.is_empty() {
                    let labs = select_labs(lab_codes, true, hn, pool, hosxp).await?;
                    for (lab_name, _) in lab_codes.iter() {
                        if let Some(item) = labs.iter().find(|lab| lab.lab_name == *lab_name) {
                            info.last_labs.push(item.clone());
                        } else {
                            info.last_labs.push(Lab::new(lab_name));
                        }
                    }
                }
            }
            info_opt = Some(info);
        }
        info_opt
    } else {
        None
    };

    let vn = info.as_ref().and_then(|info| info.vn.clone()).or(params.vn.and_then(str_some));
    let visit = match vn {
        Some(vn) => {
            let mut info_vn_opt = select_info_vn(&vn, pool, hosxp, kphis_extra).await?;
            if let Some(mut info_vn) = info_vn_opt.take() {
                let an_opt = info_vn.an.clone().and_then(str_some);
                info_vn.medicines = select_info_medicine(&vn, &an_opt, pool, hosxp).await?;
                info_vn.next_app = select_next_app(&vn, pool, hosxp).await?;
                info_vn.drug_interactions = select_info_drug_interaction(&vn, pool, hosxp).await?;
                let egfr = select_egfr(egfr_codes, false, &None, &vn, pool, hosxp).await?.unwrap_or(Lab::new("eGFR"));
                info_vn.labs.push(egfr);
                let crcl = select_crcl(scr_codes, false, &None, &vn, pool, hosxp).await?.unwrap_or(Lab::new("CrCl"));
                info_vn.labs.push(crcl);
                if !lab_codes.is_empty() {
                    let labs = select_labs(lab_codes, false, &vn, pool, hosxp).await?;
                    for (lab_name, _) in lab_codes {
                        if let Some(item) = labs.iter().find(|lab| lab.lab_name == *lab_name) {
                            info_vn.labs.push(item.clone());
                        } else {
                            info_vn.labs.push(Lab::new(lab_name));
                        }
                    }
                }

                if !message_icodes.is_empty() {
                    info_vn.mess_vn.extend(select_info_message(message_icodes, &vn, pool, hosxp).await?);
                }

                if let Some(hn) = &info_vn.hn
                    && !hn.is_empty()
                {
                    if !message_egfr_icodes.is_empty() {
                        let egfr_opt = select_egfr(egfr_codes, true, &info_vn.vstdate, &hn, pool, hosxp).await?;
                        if let Some(egfr) = egfr_opt.and_then(|lab| lab.lab_order_result).and_then(|s| s.parse::<f64>().ok()) {
                            let messages_egfr = select_info_ckd_message(message_egfr_icodes, "eGFR", &egfr.to_string(), &vn, pool, hosxp).await?;
                            info_vn.mess_vn.extend(messages_egfr);
                        }
                    }

                    if !message_crcl_icodes.is_empty() {
                        let crcl_opt = select_crcl(scr_codes, true, &info_vn.vstdate, &hn, pool, hosxp).await?;
                        if let Some(crcl) = crcl_opt.and_then(|lab| lab.lab_order_result).and_then(|s| s.parse::<f64>().ok()) {
                            let messages_crcl = select_info_ckd_message(message_crcl_icodes, "CrCl", &crcl.to_string(), &vn, pool, hosxp).await?;
                            info_vn.mess_vn.extend(messages_crcl);
                        }
                    }
                }
                info_vn_opt = Some(info_vn)
            }
            info_vn_opt
        }
        None => None,
    };

    Ok(PrescriptionScreen { info, visit })
}

// patient is qn/hn/vn/cid/fullname
async fn select_info_qn(patient_opt: &Option<String>, hlen: usize, vlen: usize, pool: &Pool<MySql>, hosxp: &str) -> Result<Option<PrescriptionInfo>, AppError> {
    let info_sql = prescription::select_info_qn(patient_opt, hlen, vlen, hosxp);
    let mut info_query = sqlx::query(AssertSqlSafe(info_sql));
    if let Some(patient) = patient_opt {
        let wildcard = ["%", patient.trim(), "%"].concat();
        let pt_len = patient.len();
        if let Some(id) = patient.parse::<u64>().ok() {
            if pt_len == 13 {
                info_query = info_query.bind(patient);
            } else if pt_len < 5 {
                info_query = info_query.bind(id);
            } else {
                match hlen.cmp(&vlen) {
                    Ordering::Equal => {
                        info_query = info_query.bind(wildcard.clone()).bind(wildcard.clone());
                    }
                    Ordering::Greater | Ordering::Less => {
                        info_query = info_query.bind(wildcard.clone());
                    }
                }
            }
        } else {
            info_query = info_query.bind(wildcard.clone());
        }
    }

    info_query
        .fetch_optional(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select Info"))?
        .as_ref()
        .map(PrescriptionInfo::from_row)
        .transpose()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select Info"))
}

async fn select_info_date(hn: &str, pool: &Pool<MySql>, hosxp: &str) -> Result<Vec<VisitDate>, AppError> {
    let date_sql = prescription::select_info_date(hosxp);
    query1_all(hn, &date_sql, pool, "Select Info Date")
        .await?
        .iter()
        .map(VisitDate::from_row)
        .collect::<sqlx::Result<Vec<VisitDate>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select Info Date"))
}

async fn select_note_from_hosxp(hn: &str, pool: &Pool<MySql>, hosxp: &str) -> Result<Vec<PtNote>, AppError> {
    let note_sql = prescription::select_note_from_hosxp(hosxp);
    query1_all(hn, &note_sql, pool, "Select Info Note")
        .await?
        .iter()
        .map(PtNote::from_row)
        .collect::<sqlx::Result<Vec<PtNote>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select Info Note"))
}

async fn select_drug_allergy(hn: &str, pool: &Pool<MySql>, hosxp: &str) -> Result<Vec<String>, AppError> {
    let drug_allergies_sql = prescription::select_drug_allergy(hosxp);
    query1_all(hn, &drug_allergies_sql, pool, "Select Info Allergies")
        .await?
        .iter()
        .filter_map(|row| row.try_get("drugallergy").transpose())
        .collect::<sqlx::Result<Vec<String>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select Info Allergies"))
}

/// is_last using hn, else using vn
async fn select_egfr(egfr_codes: &[u64], is_last: bool, not_after: &Option<Date>, vnhn: &str, pool: &Pool<MySql>, hosxp: &str) -> Result<Option<Lab>, AppError> {
    let egfr_sql = prescription::select_egfr(egfr_codes, is_last, not_after, hosxp);
    query1_opt(vnhn, &egfr_sql, pool, "Select Info eGFR")
        .await?
        .as_ref()
        .map(Lab::from_row)
        .transpose()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select Info eGFR"))
}

/// is_last using hn, else using vn.<br>
/// *NOTE*: due to CrCl formular need body wheight and sex from opd data (opdscreen and vn_stat).<br>
/// Result will not include IPD lab.
async fn select_crcl(scr_codes: &[u64], is_last: bool, not_after: &Option<Date>, vnhn: &str, pool: &Pool<MySql>, hosxp: &str) -> Result<Option<Lab>, AppError> {
    let crcl_sql = prescription::select_crcl(scr_codes, is_last, not_after, hosxp);
    query1_opt(vnhn, &crcl_sql, pool, "Select Info CrCl")
        .await?
        .as_ref()
        .map(Lab::from_row)
        .transpose()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select Info CrCl"))
}

/// is_last using hn, else using vn
async fn select_labs(lab_codes: &[(String, Vec<u64>)], is_last: bool, vnhn: &str, pool: &Pool<MySql>, hosxp: &str) -> Result<Vec<Lab>, AppError> {
    let lab_last_sql = prescription::select_labs(lab_codes, is_last, hosxp);
    let mut lab_last_query = sqlx::query(AssertSqlSafe(lab_last_sql));
    for _ in 0..lab_codes.len() {
        lab_last_query = lab_last_query.bind(vnhn);
    }
    lab_last_query
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select Lab Last"))?
        .iter()
        .map(Lab::from_row)
        .collect::<sqlx::Result<Vec<Lab>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select Lab Last"))
}

async fn select_info_vn(vn: &str, pool: &Pool<MySql>, hosxp: &str, kphis_extra: &str) -> Result<Option<PrescriptionVn>, AppError> {
    let info_vn_sql = prescription::select_info_vn(hosxp, kphis_extra);
    query1_opt(vn, &info_vn_sql, pool, "Select VN")
        .await?
        .as_ref()
        .map(PrescriptionVn::from_row)
        .transpose()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select VN"))
}

async fn select_info_medicine(vn: &str, an_opt: &Option<String>, pool: &Pool<MySql>, hosxp: &str) -> Result<Vec<Medicine>, AppError> {
    let info_medicine_sql = prescription::select_info_medicine(an_opt.is_some(), hosxp);
    let mut info_medicine_query = sqlx::query(AssertSqlSafe(info_medicine_sql));
    if let Some(an) = an_opt {
        info_medicine_query = info_medicine_query.bind(an);
    } else {
        info_medicine_query = info_medicine_query.bind(vn);
    }
    info_medicine_query
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select VN Medicine"))?
        .iter()
        .map(Medicine::from_row)
        .collect::<sqlx::Result<Vec<Medicine>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select VN Medicine"))
}

async fn select_info_drug_interaction(vn: &str, pool: &Pool<MySql>, hosxp: &str) -> Result<Vec<DrugInteraction>, AppError> {
    let drug_interaction_sql = prescription::select_info_drug_interaction(hosxp);
    query2_all(vn, vn, &drug_interaction_sql, pool, "Select VN Drug Interaction")
        .await?
        .iter()
        .map(DrugInteraction::from_row)
        .collect::<sqlx::Result<Vec<DrugInteraction>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select VN Drug Interaction"))
}

async fn select_info_message(message_icodes: &[(String, Vec<String>)], vn: &str, pool: &Pool<MySql>, hosxp: &str) -> Result<Vec<String>, AppError> {
    let message_sql = prescription::select_info_message(message_icodes, hosxp);
    let mut message_query = sqlx::query(AssertSqlSafe(message_sql));
    for _ in 0..message_icodes.len() {
        message_query = message_query.bind(vn);
    }
    message_query
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select Message"))?
        .iter()
        .filter_map(|row| row.try_get("message").transpose())
        .collect::<sqlx::Result<Vec<String>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select Message"))
}

pub async fn select_next_app(vn: &str, pool: &Pool<MySql>, hosxp: &str) -> Result<Vec<NextAppointment>, AppError> {
    let next_app_sql = prescription::select_next_app(hosxp);
    query1_all(vn, &next_app_sql, pool, "Select Next Appointment")
        .await?
        .iter()
        .map(NextAppointment::from_row)
        .collect::<sqlx::Result<Vec<NextAppointment>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select Next Appointment"))
}

async fn select_info_ckd_message(message_egfr_icodes: &[(String, u64, Vec<String>)], lab: &str, pt_value: &str, vn: &str, pool: &Pool<MySql>, hosxp: &str) -> Result<Vec<String>, AppError> {
    let message_egfr_sql = prescription::select_info_ckd_message(message_egfr_icodes, lab, pt_value, hosxp);
    let mut message_egfr_query = sqlx::query(AssertSqlSafe(message_egfr_sql));
    for _ in 0..message_egfr_icodes.len() {
        message_egfr_query = message_egfr_query.bind(vn);
    }
    message_egfr_query
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select Message eGFR"))?
        .iter()
        .filter_map(|row| row.try_get("message").transpose())
        .collect::<sqlx::Result<Vec<String>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select Message eGFR"))
}

pub async fn post_prescription_screen(vn: &str, doctorcode: &str, user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<ExecuteResponse, AppError> {
    let sql = prescription::insert_duplicate_update_accept_prescription_screen(kphis_extra);
    let result = sqlx::query(AssertSqlSafe(sql))
        .bind(vn)
        .bind(doctorcode)
        .bind(user)
        .bind(user)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert PrescriptionScreen"))?;

    Ok(ExecuteResponse::from_query_result(result, "Insert PrescriptionScreen"))
}

pub async fn patch_prescription_screen(
    vn: &str,
    action: &str,
    payload: &PrescriptionScreenPatch,
    doctorcode: &str,
    user: &str,
    pool: &Pool<MySql>,
    kphis_extra: &str,
) -> Result<ExecuteResponse, AppError> {
    match action {
        "check" => update_check_prescription_screen(vn, doctorcode, user, pool, kphis_extra).await,
        "done" => update_done_prescription_screen(vn, doctorcode, user, pool, kphis_extra).await,
        "postal" => {
            if let Some(postal) = &payload.postal {
                update_postal_prescription_screen(postal, vn, doctorcode, user, pool, kphis_extra).await
            } else {
                Err(AppError::app_400("Post PrescriptionScreen"))
            }
        }
        "telemed" => {
            if let Some(telemed) = &payload.telemed {
                update_telemed_prescription_screen(telemed, vn, doctorcode, user, pool, kphis_extra).await
            } else {
                Err(AppError::app_400("Post PrescriptionScreen"))
            }
        }
        "pharmacy-care" => update_pharmacy_care_prescription_screen(&payload.pharmacy_care, vn, doctorcode, user, pool, kphis_extra).await,
        _ => Err(AppError::app_400("Post PrescriptionScreen")),
    }
}

async fn update_check_prescription_screen(vn: &str, doctorcode: &str, user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<ExecuteResponse, AppError> {
    let sql = prescription::update_check_prescription_screen(kphis_extra);
    let result = sqlx::query(AssertSqlSafe(sql))
        .bind(doctorcode)
        .bind(user)
        .bind(vn)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Check PrescriptionScreen"))?;

    Ok(ExecuteResponse::from_query_result(result, "Check PrescriptionScreen"))
}

async fn update_done_prescription_screen(vn: &str, doctorcode: &str, user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<ExecuteResponse, AppError> {
    let sql = prescription::update_done_prescription_screen(kphis_extra);
    let result = sqlx::query(AssertSqlSafe(sql))
        .bind(doctorcode)
        .bind(user)
        .bind(vn)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Done PrescriptionScreen"))?;

    Ok(ExecuteResponse::from_query_result(result, "Done PrescriptionScreen"))
}

async fn update_postal_prescription_screen(postal: &PostalPatch, vn: &str, doctorcode: &str, user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<ExecuteResponse, AppError> {
    let sql = prescription::update_postal_prescription_screen(kphis_extra);
    let result = sqlx::query(AssertSqlSafe(sql))
        .bind(&postal.postal_status)
        .bind(doctorcode)
        .bind(user)
        .bind(vn)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Postal PrescriptionScreen"))?;

    Ok(ExecuteResponse::from_query_result(result, "Postal PrescriptionScreen"))
}

async fn update_telemed_prescription_screen(telemed: &TelemedPatch, vn: &str, doctorcode: &str, user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<ExecuteResponse, AppError> {
    let sql = prescription::insert_duplicate_update_telemed_prescription_screen(kphis_extra);
    let result = sqlx::query(AssertSqlSafe(sql))
        .bind(vn)
        .bind(&telemed.telemed_add)
        .bind(&telemed.telemed_dose_up)
        .bind(&telemed.telemed_dose_down)
        .bind(&telemed.telemed_off)
        .bind(&telemed.telemed_other)
        .bind(doctorcode)
        .bind(user)
        .bind(user)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Telemed PrescriptionScreen"))?;

    Ok(ExecuteResponse::from_query_result(result, "Telemed PrescriptionScreen"))
}

async fn update_pharmacy_care_prescription_screen(pharmacy_care: &Option<String>, vn: &str, doctorcode: &str, user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<ExecuteResponse, AppError> {
    let sql = prescription::update_pharmacy_care_prescription_screen(kphis_extra);
    let result = sqlx::query(AssertSqlSafe(sql))
        .bind(pharmacy_care)
        .bind(doctorcode)
        .bind(user)
        .bind(vn)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "PharmacyCare PrescriptionScreen"))?;

    Ok(ExecuteResponse::from_query_result(result, "PharmacyCare PrescriptionScreen"))
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;
    use kphis_sqlx_tester::MySqlTester;
    use kphis_util::datetime::date_8601;

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_info_qn() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/ovst.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/patient.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/tambol.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/vn_stat.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/sex.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/ovst.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/patient.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/tambol.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/vn_stat.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/sex.sql")).execute(&tester.db_pool).await.unwrap();

        let none = select_info_qn(&None, 7, 12, &tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(none.is_none());
        let find_queue = select_info_qn(&Some(String::from("1")),7,12,&tester.db_pool,&tester.hosxp).await.unwrap();
        assert_eq!(find_queue.and_then(|info| info.vn).unwrap_or_default(),String::from("671111111111"));
        let find_hn_short = select_info_qn(&Some(String::from("0123")),7,12,&tester.db_pool,&tester.hosxp).await.unwrap();
        assert!(find_hn_short.is_none());
        let find_hn = select_info_qn(&Some(String::from("00123")),7,12,&tester.db_pool,&tester.hosxp).await.unwrap();
        assert_eq!(find_hn.and_then(|info| info.vn).unwrap_or_default(),String::from("671111111111"));
        let find_vn = select_info_qn(&Some(String::from("6123123595")),7,12,&tester.db_pool,&tester.hosxp).await.unwrap();
        assert_eq!(find_vn.and_then(|info| info.vn).unwrap_or_default(),String::from("661231235959"));
        let find_cid = select_info_qn(&Some(String::from("1111111111111")),7,12,&tester.db_pool,&tester.hosxp).await.unwrap();
        assert_eq!(find_cid.and_then(|info| info.vn).unwrap_or_default(),String::from("671111111111"));
        let find_name = select_info_qn(&Some(String::from("สม")),7,12,&tester.db_pool,&tester.hosxp).await.unwrap();
        assert_eq!(find_name.and_then(|info| info.vn).unwrap_or_default(),String::from("671111111111"));
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_info_date() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/ovst.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/ovst.sql")).execute(&tester.db_pool).await.unwrap();

        let found = select_info_date("0001234", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(found.len(), 4);
        let not_found = select_info_date("0006666", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_note_from_hosxp() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/ptnote.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/ptnote.sql")).execute(&tester.db_pool).await.unwrap();

        let found = select_note_from_hosxp("0001234", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(found.len(), 1);
        let not_found = select_note_from_hosxp("0006666", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_drug_allergy() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/opd_allergy.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/opd_allergy.sql")).execute(&tester.db_pool).await.unwrap();

        let found = select_drug_allergy("0001234", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(found.len(), 1);
        let not_found = select_drug_allergy("0006666", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_egfr() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/lab_head.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/lab_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/lab_items.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/lab_head.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/lab_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/lab_items.sql")).execute(&tester.db_pool).await.unwrap();

        // this function can get both OPD and IPD
        let hn_all = select_egfr(&[364],true,&None,"0001234",&tester.db_pool,&tester.hosxp).await.unwrap();
        assert_eq!(hn_all.and_then(|lab| lab.vn).unwrap_or_default(),String::from("660001234"));
        let hn_not_after = select_egfr(&[364],true,&date_8601("2023-01-11"),"0001234",&tester.db_pool,&tester.hosxp).await.unwrap();
        assert_eq!(hn_not_after.and_then(|lab| lab.vn).unwrap_or_default(),String::from("650001234"));
        let vn_all = select_egfr(&[364],false,&None,"661231235959",&tester.db_pool,&tester.hosxp).await.unwrap();
        assert_eq!(vn_all.and_then(|lab| lab.vn).unwrap_or_default(),String::from("661231235959"));
        let vn_before_not_after = select_egfr(&[364],false,&date_8601("2023-01-11"),"651231235959",&tester.db_pool,&tester.hosxp).await.unwrap();
        assert_eq!(vn_before_not_after.and_then(|lab| lab.vn).unwrap_or_default(),String::from("651231235959"));
        let vn_after_not_after = select_egfr(&[364],false,&date_8601("2023-01-11"),"661231235959",&tester.db_pool,&tester.hosxp).await.unwrap();
        assert!(vn_after_not_after.is_none());
        let not_found = select_egfr(&[364],true,&None,"0006666",&tester.db_pool,&tester.hosxp).await.unwrap();
        assert!(not_found.is_none());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_crcl() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/lab_head.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/lab_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/lab_items.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/vn_stat.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/opdscreen.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/lab_head.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/lab_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/lab_items.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/vn_stat.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/opdscreen.sql")).execute(&tester.db_pool).await.unwrap();

        // this function can get only OPD
        let hn_all = select_crcl(&[78],true,&None,"0001234",&tester.db_pool,&tester.hosxp).await.unwrap();
        assert_eq!(hn_all.and_then(|lab| lab.vn).unwrap_or_default(),String::from("661231235959"));
        let hn_not_after = select_crcl(&[78],true,&date_8601("2023-01-11"),"0001234",&tester.db_pool,&tester.hosxp).await.unwrap();
        assert_eq!(hn_not_after.and_then(|lab| lab.vn).unwrap_or_default(),String::from("651231235959"));
        let vn_all = select_crcl(&[78],false,&None,"661231235959",&tester.db_pool,&tester.hosxp).await.unwrap();
        assert_eq!(vn_all.and_then(|lab| lab.vn).unwrap_or_default(),String::from("661231235959"));
        let vn_before_not_after = select_crcl(&[78],false,&date_8601("2023-01-11"),"651231235959",&tester.db_pool,&tester.hosxp).await.unwrap();
        assert_eq!(vn_before_not_after.and_then(|lab| lab.vn).unwrap_or_default(),String::from("651231235959"));
        let vn_after_not_after = select_crcl(&[78],false,&date_8601("2023-01-11"),"661231235959",&tester.db_pool,&tester.hosxp).await.unwrap();
        assert!(vn_after_not_after.is_none());
        let not_found = select_crcl(&[78],true,&None,"0006666",&tester.db_pool,&tester.hosxp).await.unwrap();
        assert!(not_found.is_none());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_labs() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/lab_head.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/lab_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/lab_items.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/lab_head.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/lab_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/lab_items.sql")).execute(&tester.db_pool).await.unwrap();

        let lab_codes_2 = vec![(String::from("eGFR"), vec![364]),(String::from("CrCl"), vec![78])];
        let lab_codes_3 = vec![(String::from("eGFR"), vec![364]),(String::from("CrCl"), vec![78]),(String::from("Unknown"), vec![666])];
        // this function can get both OPD and IPD
        let hn_all = select_labs(&lab_codes_2,true,"0001234",&tester.db_pool,&tester.hosxp).await.unwrap();
        assert_eq!(hn_all.len(), 2);
        let hn_some = select_labs(&lab_codes_3,true,"0001234",&tester.db_pool,&tester.hosxp).await.unwrap();
        assert_eq!(hn_some.len(), 2);
        let vn_all = select_labs(&lab_codes_2,false,"661231235959",&tester.db_pool,&tester.hosxp).await.unwrap();
        assert_eq!(vn_all.len(), 2);
        let vn_some = select_labs(&lab_codes_3,false,"661231235959",&tester.db_pool,&tester.hosxp).await.unwrap();
        assert_eq!(vn_some.len(), 2);
        let not_found = select_labs(&lab_codes_2,true,"0006666",&tester.db_pool,&tester.hosxp).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_info_vn() {
        let tester = MySqlTester::new_hosxp_and_kphis_extra().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/ovst.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/vn_stat.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/pttype.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/opdscreen.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/ovstdiag.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/icd101.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis_extra/prescription_screen.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/ovst.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/vn_stat.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/pttype.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/opdscreen.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/ovstdiag.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/icd101.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/prescription_screen.sql")).execute(&tester.db_pool).await.unwrap();

        let found = select_info_vn("661231235959", &tester.db_pool, &tester.hosxp, &tester.kphis_extra).await.unwrap();
        assert!(found.is_some());
        let found_with_pharms = select_info_vn("670101111111", &tester.db_pool, &tester.hosxp, &tester.kphis_extra).await.unwrap();
        assert!(found_with_pharms.is_some());
        let not_found = select_info_vn("666666666666", &tester.db_pool, &tester.hosxp, &tester.kphis_extra).await.unwrap();
        assert!(not_found.is_none());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_info_medicine() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/opitemrece.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/drugitems.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/drugusage.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/sp_use.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/opitemrece.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/drugitems.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/drugusage.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/sp_use.sql")).execute(&tester.db_pool).await.unwrap();

        let found_vn = select_info_medicine("661231235959", &None, &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(found_vn.len(), 6);
        // vn is not used in this function, home-med only
        let found_an = select_info_medicine("",&Some(String::from("660001234")),&tester.db_pool,&tester.hosxp).await.unwrap();
        assert_eq!(found_an.len(), 1);
        let not_found = select_info_medicine("666666666666", &None, &tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_info_drug_interaction() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/drug_interaction.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/opitemrece.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/drugitems.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/drug_interaction.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/opitemrece.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/drugitems.sql")).execute(&tester.db_pool).await.unwrap();

        let found = select_info_drug_interaction("661231235959", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(found.len(), 1);
        let no_interaction = select_info_drug_interaction("670111111111", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(no_interaction.is_empty());
        let not_found = select_info_drug_interaction("666666666666", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_info_message() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/opitemrece.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/drugitems.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/opitemrece.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/drugitems.sql")).execute(&tester.db_pool).await.unwrap();

        let message_icodes = vec![(String::from("Effect"),vec![String::from("1000222"), String::from("1900333")])];
        let found = select_info_message(&message_icodes,"661231235959",&tester.db_pool,&tester.hosxp).await.unwrap();
        assert_eq!(found.len(), 1);
        let no_effect = select_info_message(&message_icodes,"670111111111",&tester.db_pool,&tester.hosxp).await.unwrap();
        assert!(no_effect.is_empty());
        let not_found = select_info_message(&message_icodes,"666666666666",&tester.db_pool,&tester.hosxp).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_next_app() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/oapp.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/clinic.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/oapp.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/clinic.sql")).execute(&tester.db_pool).await.unwrap();

        let found = select_next_app("661231235959", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(found.len(), 2);
        let not_found = select_next_app("666666666666", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_info_ckd_message() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/opitemrece.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/opitemrece.sql")).execute(&tester.db_pool).await.unwrap();

        let message_egfr_icodes = vec![(String::from("Message"), 30, vec![String::from("1900333")])];
        let below = select_info_ckd_message(&message_egfr_icodes,"eGFR",&20.to_string(),"661231235959",&tester.db_pool,&tester.hosxp).await.unwrap();
        assert_eq!(below.len(), 1);
        let equal = select_info_ckd_message(&message_egfr_icodes,"eGFR",&30.to_string(),"661231235959",&tester.db_pool,&tester.hosxp).await.unwrap();
        assert!(equal.is_empty());
        let no_message = select_info_ckd_message(&message_egfr_icodes,"eGFR",&20.to_string(),"670111111111",&tester.db_pool,&tester.hosxp).await.unwrap();
        assert!(no_message.is_empty());
        let not_found = select_info_ckd_message(&message_egfr_icodes,"eGFR",&20.to_string(),"666666666666",&tester.db_pool,&tester.hosxp).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_post_prescription_screen() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis_extra/prescription_screen.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/prescription_screen.sql")).execute(&tester.db_pool).await.unwrap();

        let success = post_prescription_screen("661231235959","009","user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected, 1); // insert
        let again_success = post_prescription_screen("661231235959","009","user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(again_success.rows_affected, 2); // update
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_check_prescription_screen() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis_extra/prescription_screen.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/prescription_screen.sql")).execute(&tester.db_pool).await.unwrap();

        let not_accepted = update_check_prescription_screen("671111111122","009","user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(not_accepted.rows_affected, 0);
        let success = update_check_prescription_screen("671111111111","009","user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected, 1);
        let again_success = update_check_prescription_screen("671111111111","009","user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(again_success.rows_affected, 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_done_prescription_screen() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis_extra/prescription_screen.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/prescription_screen.sql")).execute(&tester.db_pool).await.unwrap();

        let not_accepted = update_done_prescription_screen("671111111122","009","user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(not_accepted.rows_affected, 0);
        let not_checked = update_done_prescription_screen("671111111111","009","user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(not_checked.rows_affected, 0);
        let success = update_done_prescription_screen("670111111111","009","user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected, 1);
        let again_success = update_done_prescription_screen("670111111111","009","user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(again_success.rows_affected, 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_postal_prescription_screen() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis_extra/prescription_screen.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/prescription_screen.sql")).execute(&tester.db_pool).await.unwrap();

        let postal = PostalPatch::demo();
        let no_vn = update_postal_prescription_screen(&postal, "991111111111","009","user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(no_vn.rows_affected, 0);
        let success = update_postal_prescription_screen(&postal, "670111111111","009","user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected, 1);
        let again_success = update_postal_prescription_screen(&postal, "670111111111","009","user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(again_success.rows_affected, 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_telemed_prescription_screen() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis_extra/prescription_screen.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/prescription_screen.sql")).execute(&tester.db_pool).await.unwrap();

        let telemed = TelemedPatch::demo();
        let no_vn = update_telemed_prescription_screen(&telemed, "991111111111","009","user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(no_vn.rows_affected, 1); // insert
        let success = update_telemed_prescription_screen(&telemed, "670111111111","009","user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected, 2); // update
        let again_success = update_telemed_prescription_screen(&telemed, "670111111111","009","user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(again_success.rows_affected, 2); // update
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_pharmacy_care_prescription_screen() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis_extra/prescription_screen.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/prescription_screen.sql")).execute(&tester.db_pool).await.unwrap();

        let no_vn = update_pharmacy_care_prescription_screen(&None, "991111111111","009","user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(no_vn.rows_affected, 0); // insert
        let success = update_pharmacy_care_prescription_screen(&None, "670111111111","009","user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected, 1); // update
        let again_success = update_pharmacy_care_prescription_screen(&Some(String::from("Critical drug")), "670111111111","009","user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(again_success.rows_affected, 1); // update
    }
}
