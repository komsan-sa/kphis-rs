use std::collections::HashMap;

use kphis_model::{
    order::{OrderParams, OrderItemSave}, ipd::pharmacy_monitor::IpdOrderPharmacyParams,
    pre_order::order::PreOrderSave,
};

use crate::{
    TABLE_CREATE_COLUMNS, TABLE_CREATE_PREPARED, TABLE_UPDATE_SET,
    ipd::and_ipt_patient,
};

// // ipd-dr-order-date.php
// SELECT DISTINCT order_date, (order_date = DATE(NOW())) AS is_today FROM kphis.ipd_order
// WHERE an=?
// UNION
// SELECT DISTINCT progress_note_date AS order_date, (progress_note_date = date(now())) AS is_today FROM kphis.ipd_progress_note WHERE an=?
// order by order_date ASC;
pub fn select_ipd_order_date(is_asc: bool, kphis: &str) -> String {
    let order_by = if is_asc { "ASC" } else { "DESC" };
    [
        "SELECT DISTINCT order_date, (order_date = DATE(NOW())) AS is_today FROM ",kphis,".ipd_order WHERE an=? UNION \
        SELECT DISTINCT progress_note_date AS order_date, (progress_note_date = DATE(NOW())) AS is_today FROM ",kphis,".ipd_progress_note WHERE an=? \
        ORDER BY order_date ",order_by,";"
    ].concat()
}

// // ipd-dr-order-one-day-data.php
// SELECT i.*,d.name AS order_doctor_name,s.name AS nurse_order_as_name,
//     IF(i.order_owner_type = 'doctor',d.licenseno,s.licenseno) AS doctor_licenseno,
//     n.name AS nurse_accept_name,pa.name AS pharmacist_accept_name,pc.name AS pharmacist_check_name,pd.name AS pharmacist_done_name
// FROM kphis.ipd_order i
//     LEFT JOIN hos.doctor d ON d.code = i.order_doctor
//     LEFT JOIN hos.doctor s ON s.code = i.nurse_order_as
//     LEFT JOIN hos.doctor n ON n.code = i.nurse_accept
//     LEFT JOIN hos.doctor pa ON pa.code = i.pharmacist_accept
//     LEFT JOIN hos.doctor pc ON pc.code = i.pharmacist_check
//     LEFT JOIN hos.doctor pd ON pd.code = i.pharmacist_done
// WHERE 1=1
// ORDER BY i.order_date,i.order_time,i.order_id;
/// (order_id), (an), (order_date), (order_confirm), (doctor_not_confirm_as='Y' => doctorcode)
pub fn select_order(params: &OrderParams, intern_roles: &[String], hosxp: &str, kphis: &str) -> String {
    let order_id = if params.order_id.is_some() {" AND i.order_id=? "} else {""};
    let an = if params.an.is_some() {" AND i.an=? "} else {""};
    let order_type = params.order_type.as_ref().map(|t| {
        if ["oneday", "continuous"].contains(&t.as_str()) {
                [" AND i.order_type ='", t, "' "].concat()
            } else {
                String::new()
            }
        }
    ).unwrap_or_default();
    let order_date = if params.current_date.is_some() {" AND i.order_date=? "} else {""};
    let order_confirm = if params.order_confirm.is_some() {" AND i.order_confirm=? "} else {""};
    let order_owner_types = params.order_owner_types.as_ref().map(|types| {
        let in_c = types.split(',').map(|s| s.trim())
            .filter(|t| ["doctor", "nurse", "pharmacist"].contains(t))
            .collect::<Vec<&str>>().join("','");
            // let in_c = types.into_iter()
        [" AND i.order_owner_type IN ('", &in_c, "') "].concat()
    }).unwrap_or_default();
    let view_by = if params.doctor_not_confirm_as.as_ref().map(|s| s.as_str() == "Y").unwrap_or_default() {
        " AND i.order_owner_type = 'nurse' AND i.order_confirm = 'Y' AND i.nurse_order_as=? AND i.doctor_confirm_time IS NULL "
    } else {
        params.view_by.as_ref().map(|v| match v.as_str() {
            // "doctor" => " AND ((i.order_owner_type = 'nurse' AND i.order_confirm = 'Y') OR (i.order_owner_type = 'doctor')) ",
            // "nurse" => " AND ((i.order_owner_type = 'doctor' AND i.order_confirm = 'Y') OR (i.order_owner_type = 'nurse')) ",
            "doctor"|"nurse" => " AND i.order_owner_type IN ('doctor','nurse') ",
            "pharmacist"|"other" => " AND (i.order_owner_type IN ('doctor','nurse') AND i.order_confirm = 'Y') ",
            _ => " AND 0=1 ",
        }).unwrap_or_default()
    };

    [
        "SELECT i.*,ipt.hn,CONCAT(p.pname,p.fname,' ',p.lname) AS fullname,w.name AS ward_name,iptadm.bedno,\
            d1.`name` AS order_doctor_name,d1.licenseno AS order_doctor_licenseno,\
            (SELECT entryposition FROM ",hosxp,".opduser WHERE doctorcode=i.order_doctor AND (account_disable IS NULL OR account_disable='N') LIMIT 1) AS order_doctor_entryposition,\
            (SELECT EXISTS(SELECT * FROM ",hosxp,".opduser ou LEFT JOIN ",kphis,".system_ac_role_user ru ON ou.loginname=ru.loginname \
                WHERE ou.doctorcode=i.order_doctor AND (ou.account_disable IS NULL OR ou.account_disable='N') AND ru.role IN ('",&intern_roles.join("','"),"'))) AS order_doctor_is_intern,\
            d2.`name` AS nurse_order_as_name,d2.licenseno AS nurse_order_as_licenseno,\
            (SELECT entryposition FROM ",hosxp,".opduser WHERE doctorcode=i.nurse_order_as AND (account_disable IS NULL OR account_disable='N') LIMIT 1) AS nurse_order_as_entryposition,\
            (SELECT EXISTS(SELECT * FROM ",hosxp,".opduser ou LEFT JOIN ",kphis,".system_ac_role_user ru ON ou.loginname=ru.loginname \
                WHERE ou.doctorcode=i.nurse_order_as AND (ou.account_disable IS NULL OR ou.account_disable='N') AND ru.role IN ('",&intern_roles.join("','"),"'))) AS nurse_order_as_is_intern,\
            d3.`name` AS nurse_accept_name,d3.licenseno AS nurse_accept_licenseno,\
            (SELECT entryposition FROM ",hosxp,".opduser WHERE doctorcode=i.nurse_accept AND (account_disable IS NULL OR account_disable='N') LIMIT 1) AS nurse_accept_entryposition,\
            d4.`name` AS pharmacist_accept_name,d4.licenseno AS pharmacist_accept_licenseno,\
            (SELECT entryposition FROM ",hosxp,".opduser WHERE doctorcode=i.pharmacist_accept AND (account_disable IS NULL OR account_disable='N') LIMIT 1) AS pharmacist_accept_entryposition,\
            d5.`name` AS pharmacist_check_name,d5.licenseno AS pharmacist_check_licenseno,\
            (SELECT entryposition FROM ",hosxp,".opduser WHERE doctorcode=i.pharmacist_check AND (account_disable IS NULL OR account_disable='N') LIMIT 1) AS pharmacist_check_entryposition,\
            d6.`name` AS pharmacist_done_name,d6.licenseno AS pharmacist_done_licenseno,\
            (SELECT entryposition FROM ",hosxp,".opduser WHERE doctorcode=i.pharmacist_done AND (account_disable IS NULL OR account_disable='N') LIMIT 1) AS pharmacist_done_entryposition \
        FROM ",kphis,".ipd_order i \
            LEFT JOIN ",hosxp,".ipt ON ipt.an=i.an \
            LEFT JOIN ",hosxp,".patient p ON p.hn=ipt.hn \
            LEFT JOIN ",hosxp,".ward w ON w.ward=ipt.ward \
            LEFT JOIN ",hosxp,".iptadm ON iptadm.an=ipt.an \
            LEFT JOIN ",hosxp,".doctor d1 ON d1.`code`=i.order_doctor \
            LEFT JOIN ",hosxp,".doctor d2 ON d2.`code`=i.nurse_order_as \
            LEFT JOIN ",hosxp,".doctor d3 ON d3.`code`=i.nurse_accept \
            LEFT JOIN ",hosxp,".doctor d4 ON d4.`code`=i.pharmacist_accept \
            LEFT JOIN ",hosxp,".doctor d5 ON d5.`code`=i.pharmacist_check \
            LEFT JOIN ",hosxp,".doctor d6 ON d6.`code`=i.pharmacist_done \
        WHERE 1=1 ",&order_type, order_id, an, order_date, order_confirm, &order_owner_types, view_by,
        "ORDER BY i.order_date,i.order_time,i.order_id;"
    ].concat()
}

/// an
pub fn select_order_only(kphis: &str) -> String {
    [
        "SELECT * FROM ",kphis,".ipd_order WHERE an=? ORDER BY order_date,order_time,order_id;"
    ].concat()
}

// // ipd-dr-order-one-day-data.php
// SELECT distinct oi.order_item_type, oi.order_id
// FROM kphis.ipd_order_item oi
// JOIN kphis.ipd_order o ON o.order_id = oi.order_id AND o.an = oi.an
// LEFT JOIN kphis.ipd_order_item_type oit ON oi.order_item_type = oit.order_item_type AND o.order_type=oit.order_type
// WHERE o.order_type='oneday' AND oi.order_id IN (?)
// ORDER BY oit.display_order, oi.order_item_id;
/// (order_type)
pub fn select_order_types(ids: &[u32], has_order_type: bool, kphis: &str) -> String {
    let in_c = ids.iter().map(|id| id.to_string()).collect::<Vec<String>>().join(",");
    let order_type = if has_order_type {" AND o.order_type=? "} else {""};
    [
        "SELECT DISTINCT oi.order_item_type, oi.order_id \
        FROM ",kphis,".ipd_order_item oi \
            JOIN ",kphis,".ipd_order o ON o.order_id = oi.order_id AND o.an = oi.an \
            LEFT JOIN ",kphis,".ipd_order_item_type oit ON oi.order_item_type = oit.order_item_type AND o.order_type=oit.order_type \
        WHERE oi.order_id IN (",&in_c,") ", order_type,
        "ORDER BY oit.display_order, oi.order_item_id;"
    ].concat()
}

// SELECT oi.*,o.order_date,o.order_time,o.order_type,o.order_owner_type,ooi.order_item_detail as off_order_item_detail,
// (SELECT TIMESTAMP(off_by_order.order_date,off_by_order.order_time) FROM kphis.ipd_order_item ioi
// JOIN kphis.ipd_order off_by_order ON ioi.order_id = off_by_order.order_id AND off_by_order.an = ioi.an AND off_by_order.order_date = ? AND ((off_by_order.order_date = date(now())
// AND off_by_order.order_confirm = 'Y') OR (off_by_order.order_date < date(now()) AND off_by_order.order_confirm = 'Y'))
// WHERE ioi.off_order_item_id = oi.order_item_id AND off_by_order.an = oi.an LIMIT 1) AS off_by_datetime,
// IF(mr.custom_med_name IS NOT NULL,mr.custom_med_name,CONCAT(di.`name`, ' ', di.strength, ' ',di.units)) AS med_name, di.displaycolor, di.generic_name, ooi.icode as off_icode, CONCAT(off_di.`name`, ' ', off_di.strength, ' ',off_di.units) AS off_med_name,
// off_di.displaycolor AS off_displaycolor,
// GROUP_CONCAT(DISTINCT(CONCAT(allergy.agent,'=',IFNULL(allergy.symptom,''))) ORDER BY allergy.agent) AS allergy_agent_symptom
// FROM kphis.ipd_order_item oi
// JOIN kphis.ipd_order o ON o.order_id = oi.order_id
// LEFT JOIN kphis.ipd_order_item ooi ON ooi.order_item_id = oi.off_order_item_id
// LEFT JOIN kphis.ipd_med_reconciliation_item mr ON mr.med_reconciliation_item_id=oi.med_reconciliation_item_id
// LEFT JOIN hos.drugitems di ON di.icode = oi.icode
// LEFT JOIN hos.drugitems off_di ON off_di.icode = ooi.icode
// LEFT JOIN hos.ipt ON o.an = ipt.an
// LEFT JOIN hos.opd_allergy allergy ON ((allergy.agent LIKE CONCAT('%',di.generic_name,'%') AND allergy.hn=ipt.hn AND di.generic_name IS NOT NULL AND TRIM(di.generic_name) <> '') OR (di.generic_name LIKE CONCAT('%',allergy.agent,'%') AND allergy.hn=ipt.hn AND allergy.agent IS NOT NULL AND TRIM(allergy.agent) <> ''))
// WHERE oi.order_id=? AND oi.order_item_type=? AND o.order_type='oneday' GROUP BY oi.order_item_id ORDER BY oi.order_item_id;
/// (an), (order_date), (order_id), (order_item_id), (order_item_type), (order_type)<br>
/// NOTE: order_id can use outside PARAMS
pub fn select_order_item(
    params: &OrderParams,
    has_order_id: bool,
    has_order_item_type: bool,
    order_item_ids: &[u32],
    hosxp: &str,
    kphis: &str,
) -> String {
    let (an, limit) = if params.an.is_some() {(" AND o.an=? ","")} else {("", " LIMIT 100")};
    let order_date = if params.current_date.is_some() {" AND o.order_date=? "} else {""};
    let vb = if params.view_by.as_ref().map(|vb| vb.as_str() == "doctor").unwrap_or_default()
        {""} else {" AND off_by_order.order_confirm = 'Y' "};
    let order_id = if has_order_id {" AND o.order_id=? "} else {""};
    let order_item_id = if params.order_item_id.is_some() {" AND oi.order_item_id=? "} else if !order_item_ids.is_empty() {
        &[" AND oi.order_item_id IN (", &order_item_ids.iter().map(|u| u.to_string()).collect::<Vec<String>>().join(","), ")"].concat()
    } else {""};
    let order_item_type = if has_order_item_type {" AND oi.order_item_type=? "} else {""};
    let order_type = if params.order_type.is_some() {" AND o.order_type=? "} else {""};
    // removed from end of line 3 : AND off_by_order.an=ioi.an AND off_by_order.order_date=? \
    [
        "SELECT oi.*,o.order_date,o.order_time,o.order_type,o.order_owner_type,ooi.order_item_detail AS off_order_item_detail,d.`name` AS order_doctor_name,d.`licenseno` AS order_doctor_licenseno,\
            dud.`usage` AS due_usage,dud.status AS due_status,dud.monitor,dud.monitor_count,dud.monitor_duration,dud.monitor_status,dud.info,dud.info_status,\
            (SELECT TIMESTAMP(off_by_order.order_date,off_by_order.order_time) FROM ",kphis,".ipd_order_item ioi \
                JOIN ",kphis,".ipd_order off_by_order ON ioi.order_id=off_by_order.order_id \
                AND ((off_by_order.order_date=DATE(NOW()) ", vb,") OR (off_by_order.order_date < DATE(NOW()) AND off_by_order.order_confirm='Y')) \
                WHERE ioi.off_order_item_id=oi.order_item_id AND off_by_order.an=oi.an LIMIT 1) AS off_by_datetime,\
            IF(mr.custom_med_name IS NULL OR mr.custom_med_name='',CONCAT(di.`name`,' ',di.strength,' ',di.units),mr.custom_med_name) AS med_name,\
            di.displaycolor,di.generic_name,di.dosageform,di.addict_type_id,di.habit_forming_type,ooi.icode AS off_icode,\
            IF(omr.custom_med_name IS NULL OR omr.custom_med_name='',CONCAT(off_di.`name`,' ',off_di.strength,' ',off_di.units),omr.custom_med_name) AS off_med_name,off_di.displaycolor AS off_displaycolor,\
            mr.old_drugusage,mr.receive_from,mr.receive_date,mr.receive_qty,mr.last_dose_taken_time,mr.last_dose_taken_remark,mr.`use` AS used,\
            GROUP_CONCAT(DISTINCT(CONCAT(allergy.agent,'=',IFNULL(allergy.symptom,''))) ORDER BY allergy.agent) AS allergy_agent_symptom \
        FROM ",kphis,".ipd_order_item oi \
            JOIN ",kphis,".ipd_order o ON o.order_id=oi.order_id \
            LEFT JOIN ",kphis,".kphis_drug_use_duration dud ON dud.icode=oi.icode \
            LEFT JOIN ",kphis,".ipd_order_item ooi ON ooi.order_item_id=oi.off_order_item_id \
            LEFT JOIN ",kphis,".ipd_med_reconciliation_item mr ON mr.med_reconciliation_item_id=oi.med_reconciliation_item_id \
            LEFT JOIN ",kphis,".ipd_med_reconciliation_item omr ON omr.med_reconciliation_item_id=ooi.med_reconciliation_item_id \
            LEFT JOIN ",hosxp,".drugitems di ON di.icode=oi.icode \
            LEFT JOIN ",hosxp,".drugitems off_di ON off_di.icode=ooi.icode \
            LEFT JOIN ",hosxp,".ipt ON o.an=ipt.an \
            LEFT JOIN ",hosxp,".doctor d ON d.`code`=IF(o.nurse_order_as IS NULL,o.order_doctor,o.nurse_order_as) \
            LEFT JOIN ",hosxp,".opd_allergy allergy ON (\
                (allergy.agent LIKE CONCAT('%',di.generic_name,'%') AND allergy.hn=ipt.hn AND di.generic_name IS NOT NULL AND TRIM(di.generic_name) <> '') \
                OR (di.generic_name LIKE CONCAT('%',allergy.agent,'%') AND allergy.hn=ipt.hn AND allergy.agent IS NOT NULL AND TRIM(allergy.agent) <> '')) \
        WHERE 1=1 ", an, order_date, order_id, order_item_id, order_item_type, order_type, " GROUP BY oi.order_item_id ORDER BY oi.order_item_id", limit,";"
    ].concat()
}

// SELECT DISTINCT order_item_id FROM kphis.ipd_nurse_index_plan WHERE an='660001509' AND plan_date='2026-02-11';
/// an, plan_date
pub fn select_order_item_ids_by_an_and_plan_date(kphis: &str) -> String {
    [
        "SELECT DISTINCT order_item_id FROM ",kphis,".ipd_nurse_index_plan WHERE an=? AND plan_date=?;"
    ].concat()
}

/// order_id
pub fn select_order_item_only(kphis: &str) -> String {
    [
        "SELECT * FROM ",kphis,".ipd_order_item WHERE order_id=? ORDER BY order_item_id;"
    ].concat()
}

// // SELECT ia.action_id,ia.plan_id,ia.an,ia.action_result,ia.action_remark,ia.action_date,ia.action_time,ia.action_report_back,ia.action_blood_had,ia.action_person_1,ia.action_person_2,ip.order_item_id,ip.plan_detail,ip.plan_date,ip.plan_time,ip.plan_sch_type
// // FROM kphis.ipd_nurse_index_action AS ia INNER JOIN kphis.ipd_nurse_index_plan AS ip on ia.plan_id = ip.plan_id
// // WHERE ip.order_item_id=:order_item_id ORDER BY ip.plan_date, ip.plan_time, ia.action_date, ia.action_time;
// /// (order_date)
// pub fn select_order_action(id: u32, with_plan_date: bool, kphis: &str) -> String {
//     let order_date = if with_plan_date {" AND ip.plan_date = ? "} else {""};
//     [
//         "SELECT ia.action_id,ia.plan_id,ia.an,ia.action_result,ia.action_remark,ia.action_date,ia.action_time,ia.action_report_back,ia.action_blood_had,ia.action_person_1,ia.action_person_2,ip.order_item_id,ip.plan_detail,ip.plan_date,ip.plan_time,ip.plan_sch_type \
//         FROM ",kphis,".ipd_nurse_index_action AS ia INNER JOIN ",kphis,".ipd_nurse_index_plan AS ip on ia.plan_id = ip.plan_id \
//         WHERE ip.order_item_id=",&id.to_string(),order_date," ORDER BY ip.plan_date, ip.plan_time, ia.action_date, ia.action_time;"
//     ].concat()
// }

// INSERT INTO kphis.ipd_order (an,order_date,order_time,order_doctor,order_type,order_owner_type,order_confirm,create_user,create_datetime,update_user,update_datetime,version)
// VALUES (?,DATE(NOW()),TIME(NOW()),?,?,?,'N',?,NOW(),?,NOW(),1);
/// an, order_doctor, order_type, order_owner_type, loginname, loginname
pub fn insert_order(kphis: &str) -> String {
    [
        "INSERT INTO ",kphis,".ipd_order (an,order_date,order_time,order_doctor,order_type,order_owner_type,order_confirm",TABLE_CREATE_COLUMNS,") \
        VALUES (?,DATE(NOW()),TIME(NOW()),?,?,?,'N'",TABLE_CREATE_PREPARED,");"
    ].concat()
}

// SELECT order_confirm FROM kphis.ipd_order WHERE order_id=?;
/// order_id
pub fn get_order_confirm(kphis: &str) -> String {
    [
        "SELECT order_confirm FROM ",kphis,".ipd_order WHERE order_id=?;"
    ].concat()
}

// UPDATE kphis.ipd_order SET order_date=date(now()), order_time=time(now()), order_doctor=?, update_user=?, update_datetime=NOW(), version=(version+1) WHERE order_id=?;
/// order_doctor, loginname, order_id
pub fn update_order(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".ipd_order SET order_date=NOW(), order_time=NOW(), order_doctor=?",TABLE_UPDATE_SET," WHERE order_id=?;"
    ].concat()
}

// UPDATE kphis.ipd_order_item SET nurse_assign=?, update_user=?, update_datetime=NOW(), version=(version+1) WHERE order_item_id=?;
/// nurse_assing, loginname, order_item_id
pub fn update_order_item_nurse_assign(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".ipd_order_item SET nurse_assign=?",TABLE_UPDATE_SET," WHERE order_item_id=?;"
    ].concat()
}

// UPDATE kphis.ipd_order_item SET order_item_type=?, update_user=?, update_datetime=NOW(), version=(version+1) WHERE order_item_id=?;
/// order_item_type, loginname, order_item_id
pub fn update_order_item_type(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".ipd_order_item SET order_item_type=?",TABLE_UPDATE_SET," WHERE order_item_id=?;"
    ].concat()
}

// UPDATE kphis.ipd_order_item SET due_doctor=?, due_doctor_note=?, update_user=?, update_datetime=NOW(), version=(version+1) WHERE order_item_id=?;
/// due_doctor, due_doctor_note, loginname, order_item_id
pub fn update_order_item_due_doctor(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".ipd_order_item SET due_doctor=?,due_doctor_note=?",TABLE_UPDATE_SET," WHERE order_item_id=?;"
    ].concat()
}

// UPDATE kphis.ipd_order_item SET due_pharm=?, due_pharm_note=?, update_user=?, update_datetime=NOW(), version=(version+1) WHERE order_item_id=?;
/// due_pharm, due_pharm_note, loginname, order_item_id
pub fn update_order_item_due_pharm(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".ipd_order_item SET due_pharm=?,due_pharm_note=?",TABLE_UPDATE_SET," WHERE order_item_id=?;"
    ].concat()
}

// UPDATE kphis.ipd_order_item SET first_qty=?, qty=?, update_user=?, update_datetime=NOW(), version=(version+1) WHERE order_item_id=?;
/// first_qty, qty, loginname, order_item_id
pub fn update_order_item_qty(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".ipd_order_item SET first_qty=?,qty=?",TABLE_UPDATE_SET," WHERE order_item_id=?;"
    ].concat()
}

// DELETE FROM kphis.ipd_order_item WHERE order_id=?;
/// order_id
pub fn delete_order_item(kphis: &str) -> String {
    [
        "DELETE FROM ", kphis, ".ipd_order_item WHERE order_id=?;"
    ].concat()
}

// INSERT INTO kphis.ipd_order_item (order_id,an,order_item_type,order_item_detail,stat,off_order_item_id,icode,med_reconciliation_item_id,first_qty,qty,create_user,create_datetime,update_user,update_datetime,version)
// VALUES (?,?,?,?,?,?,?,?,?,?,?,now(),?,now(),1)
/// (order_id, an, order_item_type, order_item_detail, stat, off_order_item_id, icode, med_reconciliation_item_id, first_qty, qty, loginname, loginname) x item_len
pub fn insert_order_items(item_len: usize, kphis: &str) -> String {
    let values = vec!["(?,?,?,?,?,?,?,?,?,?,?,NOW(),?,NOW(),1)";item_len].join(",");
    [
        "INSERT INTO ",kphis,".ipd_order_item (order_id,an,order_item_type,order_item_detail,stat,off_order_item_id,icode,med_reconciliation_item_id,first_qty,qty",TABLE_CREATE_COLUMNS,") \
        VALUES ", &values,";"
    ].concat()
}

// pub fn insert_order_items_only(order_id: u32, an: &str, order_item_onlys: &[OrderItemOnly], kphis: &str) -> String {
//     let values = order_item_onlys.iter().map(|item| {
//         [
//             "(",&order_id.to_owned(),",'",
//             an,"',",
//             &item.order_item_type.as_ref().map(|s| ["'", s, "'"].concat()).unwrap_or(String::from("NULL")),",",
//             &item.order_item_detail.as_ref().map(|s| ["'", s, "'"].concat()).unwrap_or(String::from("NULL")),",",
//             &item.stat.as_ref().map(|s| ["'", s, "'"].concat()).unwrap_or(String::from("NULL")),",",
//             &item.off_order_item_id.map(|s| s.to_owned()).unwrap_or(String::from("NULL")),",",
//             &item.icode.as_ref().map(|s| ["'", s, "'"].concat()).unwrap_or(String::from("NULL")),",",
//             &item.med_reconciliation_item_id.map(|s| s.to_owned()).unwrap_or(String::from("NULL")),",'",
//             &item.create_user,"','",
//             &item.create_datetime.to_owned(),"','",
//             &item.update_user,"','",
//             &item.update_datetime.to_owned(),"',",
//             &item.version.to_owned(),")"
//         ].concat()
//     }).collect::<Vec<String>>().join(",");
//     [
//         "INSERT INTO ",kphis,".ipd_order_item (order_id,an,order_item_type,order_item_detail,stat,off_order_item_id,icode,med_reconciliation_item_id,create_user,create_datetime,update_user,update_datetime,version) \
//         VALUES ", &values,";"
//     ].concat()
// }

// SELECT oi.icode,oi.med_reconciliation_item_id,oi.order_item_detail,dud.`usage` AS due_usage,dud.info,dud.info_status,
// (SELECT TIMESTAMP(oo.order_date,oo.order_time)
//     FROM kphis.ipd_order_item ooi
//     JOIN kphis.ipd_order oo ON ooi.order_id = oo.order_id AND oo.an = ooi.an AND oo.order_confirm = 'Y'
//     WHERE ooi.off_order_item_id = oi.order_item_id AND ooi.order_item_type = 'off' AND oo.an = oi.an LIMIT 1
// ) AS off_by_datetime,
// IF(mr.custom_med_name IS NOT NULL,mr.custom_med_name,CONCAT(di.`name`,' ',di.strength,' ',di.units)) AS med_name, di.generic_name, di.displaycolor
// FROM kphis.ipd_order_item oi
//     JOIN kphis.ipd_order o ON o.order_id = oi.order_id
//     LEFT JOIN kphis.kphis_drug_use_duration dud ON dud.icode=oi.icode AND dud.status='Y'
//     LEFT JOIN hos.drugitems di ON di.icode = oi.icode
//     LEFT JOIN ",kphis,".ipd_med_reconciliation_item mr ON mr.med_reconciliation_item_id=oi.med_reconciliation_item_id
// WHERE oi.an = ? AND oi.order_item_type IN ('med','injection') AND o.order_type='continuous' AND o.order_confirm = 'Y'
// HAVING off_by_datetime IS NULL ORDER BY oi.order_item_id;
/// an
pub fn get_home_med_from_cont(hosxp: &str, kphis: &str) -> String {
    [
        "SELECT oi.icode,oi.med_reconciliation_item_id,oi.order_item_detail,oi.order_item_type,dud.`usage` AS due_usage,dud.`status` AS due_status,dud.info,dud.info_status,\
            (SELECT TIMESTAMP(oo.order_date,oo.order_time) \
                FROM ",kphis,".ipd_order_item ooi \
                JOIN ",kphis,".ipd_order oo ON ooi.order_id=oo.order_id AND oo.an=ooi.an AND oo.order_confirm='Y' \
                WHERE ooi.off_order_item_id=oi.order_item_id AND ooi.order_item_type='off' AND oo.an=oi.an LIMIT 1 \
            ) AS off_by_datetime,\
            IF(mr.custom_med_name IS NULL OR mr.custom_med_name='',CONCAT(di.`name`,' ',di.strength,' ',di.units),mr.custom_med_name) AS med_name,\
            di.generic_name,di.dosageform,di.displaycolor,di.addict_type_id,di.habit_forming_type,\
            mr.old_drugusage,mr.receive_from,mr.receive_date,mr.receive_qty,mr.last_dose_taken_time,mr.last_dose_taken_remark,mr.`use` AS used \
        FROM ",kphis,".ipd_order_item oi \
            JOIN ",kphis,".ipd_order o ON o.order_id=oi.order_id \
            LEFT JOIN ",kphis,".kphis_drug_use_duration dud ON dud.icode=oi.icode \
            LEFT JOIN ",kphis,".ipd_med_reconciliation_item mr ON mr.med_reconciliation_item_id=oi.med_reconciliation_item_id \
            LEFT JOIN ",hosxp,".drugitems di ON di.icode=oi.icode \
        WHERE oi.an=? AND oi.order_item_type IN ('med','injection') AND o.order_type='continuous' AND o.order_confirm='Y' \
        HAVING off_by_datetime IS NULL ORDER BY oi.order_item_id;"
    ].concat()
}

// // ipd-dr-order-continuous-previous-data.php
// SELECT oi.order_item_id,oi.order_id,oi.an,oi.order_item_type,oi.order_item_detail,oi.stat,oi.off_order_item_id,oi.icode,oi.med_reconciliation_item_id,o.order_type,o.order_owner_type,
//     (SELECT TIMESTAMP(off.order_date,off.order_time) FROM kphis.ipd_order_item ofi JOIN kphis.ipd_order off ON ofi.order_id = off.order_id AND ofi.an = off.an AND off.order_date = ?
//     AND ((off.order_date = DATE(NOW()) AND off.order_confirm = 'Y'
//     ) OR (off.order_date < DATE(NOW()) AND off.order_confirm = 'Y'))
//     WHERE ofi.off_order_item_id = oi.order_item_id AND off.an = oi.an LIMIT 1) AS off_by_datetime,
//     IF(mr.custom_med_name IS NOT NULL,mr.custom_med_name,CONCAT(di.`name`,' ',di.strength,' ',di.units)) AS med_name,di.generic_name,di.displaycolor,
//     GROUP_CONCAT(DISTINCT(CONCAT(allergy.agent,'=',IFNULL(allergy.symptom,''))) ORDER BY allergy.agent) AS allergy_agent_symptom,
//     o.order_date,DATEDIFF(DATE(?), o.order_date) AS order_duration,
//     dud.duration1,dud.exceed_duration1_color,dud.duration2,dud.exceed_duration2_color,dud.duration3,dud.exceed_duration3_color
// FROM kphis.ipd_order_item oi JOIN kphis.ipd_order o ON o.order_id = oi.order_id LEFT JOIN hos.drugitems di ON di.icode = oi.icode
//     LEFT JOIN kphis.ipd_med_reconciliation_item mr ON mr.med_reconciliation_item_id=oi.med_reconciliation_item_id
//     LEFT JOIN hos.ipt ON o.an = ipt.an LEFT JOIN hos.opd_allergy allergy ON (
//     (allergy.agent LIKE CONCAT('%',di.generic_name,'%') AND allergy.hn = ipt.hn AND di.generic_name IS NOT NULL AND TRIM(di.generic_name) <> '')
//     OR (di.generic_name LIKE CONCAT('%',allergy.agent,'%') AND allergy.hn = ipt.hn AND allergy.agent IS NOT NULL AND TRIM(allergy.agent) <> ''))
//     LEFT JOIN kphis.kphis_drug_use_duration dud ON dud.icode=oi.icode AND dud.status='Y'
// WHERE o.order_date < ? AND o.order_type = 'continuous' AND o.an = ? AND o.order_confirm = 'Y' AND oi.off_order_item_id IS NULL AND oi.order_item_id NOT IN (
//     SELECT ofi2.off_order_item_id FROM kphis.ipd_order_item ofi2 JOIN kphis.ipd_order off2 ON ofi2.order_id = off2.order_id AND off2.order_date < ? AND off2.order_confirm = 'Y'
//     WHERE off2.an = o.an AND ofi2.off_order_item_id = oi.order_item_id)
//     AND (o.order_owner_type = 'doctor' OR o.order_owner_type = 'nurse')
// GROUP BY oi.order_item_id ORDER BY o.order_date, o.order_time, oi.order_item_id;
/// order_date, order_date, an, order_date,
// /// order_date, order_date, order_date, an, order_date,
/// order_date, order_date, order_date, an, (order_type), !with_offed(order_date),
pub fn select_previous(params: &OrderParams, hosxp: &str, kphis: &str) -> String {
    let (view_by1, view_by2) = if let Some(vb) = &params.view_by {
        match vb.as_str() {
            "doctor" => ("", " AND (o.order_owner_type='doctor' OR o.order_owner_type='nurse') "),
            "nurse" | "pharmacist" | "other" => (
                " AND off.order_confirm='Y' ",
                " AND (o.order_owner_type='doctor' OR o.order_owner_type='nurse') ",
            ),
            _ => ("", " AND 1=0 "),
        }
    } else {
        (" AND off.order_confirm='Y' ", "")
    };
    let order_type = if params.order_type.is_some() {" AND o.order_type=? "} else {""};
    // let order_item_type = if params.order_item_types.is_some() {" AND oi.order_item_type=? "} else {""};
    let order_item_types = if let Some(ts) = params.order_item_types.as_ref() {
        [
            " AND oi.order_item_type IN ('",
            &ts.split(',').map(|s| s.trim()).collect::<Vec<&str>>().join("','"),
            "') "
        ].concat()
    } else {
        String::new()
    };
    let with_offed = if params.with_offed.is_none() {
        " AND oi.order_item_id NOT IN (\
        SELECT ofi2.off_order_item_id FROM kphis.ipd_order_item ofi2 JOIN kphis.ipd_order off2 ON ofi2.order_id=off2.order_id AND off2.order_date < ? AND off2.order_confirm='Y' \
        WHERE off2.an=o.an AND ofi2.off_order_item_id=oi.order_item_id) "
    } else {
        ""
    };
    // removed from end of line 3 :  AND ofi.an=off.an AND off.order_date=? \
    [
        "SELECT oi.order_item_id,oi.order_id,oi.an,oi.order_item_type,oi.nurse_assign,oi.order_item_detail,oi.stat,oi.off_order_item_id,oi.icode,oi.med_reconciliation_item_id,oi.first_qty,oi.qty,\
            oi.due_doctor,oi.due_doctor_note,oi.due_pharm,oi.due_pharm_note,o.order_type,o.order_owner_type,d.`name` AS order_doctor_name,d.`licenseno` AS order_doctor_licenseno,\
            (SELECT TIMESTAMP(off.order_date,off.order_time) FROM ",kphis,".ipd_order_item ofi JOIN ",kphis,".ipd_order off ON ofi.order_id=off.order_id \
                AND ((off.order_date = DATE(NOW()) ",view_by1,") OR (off.order_date < DATE(NOW()) AND off.order_confirm='Y')) WHERE ofi.off_order_item_id=oi.order_item_id AND off.an=oi.an LIMIT 1) AS off_by_datetime,\
            IF(mr.custom_med_name IS NULL OR mr.custom_med_name='',CONCAT(di.`name`,' ',di.strength,' ',di.units),mr.custom_med_name) AS med_name,\
            di.generic_name,di.dosageform,di.displaycolor,di.addict_type_id,di.habit_forming_type,\
            mr.old_drugusage,mr.receive_from,mr.receive_date,mr.receive_qty,mr.last_dose_taken_time,mr.last_dose_taken_remark,mr.`use` AS used,\
            GROUP_CONCAT(DISTINCT(CONCAT(allergy.agent,'=',IFNULL(allergy.symptom,''))) ORDER BY allergy.agent) AS allergy_agent_symptom,\
            o.order_date,o.order_time,DATEDIFF(DATE(?), o.order_date) AS order_duration,\
            dud.`usage` AS due_usage,dud.status AS due_status,dud.monitor,dud.monitor_count,dud.monitor_duration,dud.monitor_status,dud.info,dud.info_status,\
            dud.duration1,dud.exceed_duration1_color,dud.duration2,dud.exceed_duration2_color,dud.duration3,dud.exceed_duration3_color \
        FROM ",kphis,".ipd_order_item oi \
            JOIN ",kphis,".ipd_order o ON o.order_id=oi.order_id \
            LEFT JOIN ",kphis,".ipd_med_reconciliation_item mr ON mr.med_reconciliation_item_id=oi.med_reconciliation_item_id \
            LEFT JOIN ",hosxp,".drugitems di ON di.icode=oi.icode \
            LEFT JOIN ",hosxp,".ipt ON o.an=ipt.an \
            LEFT JOIN ",hosxp,".doctor d ON d.`code`=IF(o.nurse_order_as IS NULL,o.order_doctor,o.nurse_order_as) \
            LEFT JOIN ",hosxp,".opd_allergy allergy ON (\
                (allergy.agent LIKE CONCAT('%',di.generic_name,'%') AND allergy.hn=ipt.hn AND di.generic_name IS NOT NULL AND TRIM(di.generic_name) <> '') OR \
                (di.generic_name LIKE CONCAT('%',allergy.agent,'%') AND allergy.hn=ipt.hn AND allergy.agent IS NOT NULL AND TRIM(allergy.agent) <> '')) \
            LEFT JOIN ",kphis,".kphis_drug_use_duration dud ON dud.icode=oi.icode \
        WHERE o.order_date < ? AND o.an=? ",order_type,&order_item_types," AND o.order_confirm='Y' AND oi.off_order_item_id IS NULL ",with_offed,view_by2," GROUP BY oi.order_item_id ORDER BY o.order_date,o.order_time,oi.order_item_id;"
    ].concat()
}
// pub fn select_continuous_previous(
//     params: &OrderParams,
//     hosxp: &str,
//     kphis: &str,
// ) -> String {
//     let (view_by1, view_by2) = match params.view_by.clone().unwrap_or_default().as_ref() {
//         "doctor" => ("", " AND o.order_owner_type='doctor' "),
//         "nurse" | "pharmacist" | "other" => (" AND off.order_confirm='Y' ", " AND (o.order_owner_type='doctor' OR o.order_owner_type='nurse') "),
//         _ => (" AND off.order_confirm='Y' ", ""),
//     };
//     [
//         "SELECT oi.order_item_id,oi.order_id,oi.an,oi.order_item_type,oi.order_item_detail,oi.stat,oi.off_order_item_id,oi.icode,oi.med_reconciliation_item_id,o.order_owner_type,\
//             (SELECT ofi.order_item_id FROM ",kphis,".ipd_order_item ofi JOIN ",kphis,".ipd_order off ON ofi.order_id=off.order_id AND ofi.an=off.an AND off.order_date=? \
//                 AND ((off.order_date = DATE(NOW()) ",view_by1,") OR (off.order_date < DATE(NOW()) AND off.order_confirm='Y')) WHERE ofi.off_order_item_id=oi.order_item_id AND off.an=oi.an LIMIT 1) AS off_by_order_item_id,\
//             CONCAT(di.`name`,' ',di.strength,' ',di.units) AS med_name,di.displaycolor,\
//             GROUP_CONCAT(DISTINCT(CONCAT(allergy.agent,'=',IFNULL(allergy.symptom,''))) ORDER BY allergy.agent) AS allergy_agent_symptom,\
//             o.order_date,DATEDIFF(DATE(?), o.order_date) AS order_duration,\
//             dud.duration1,dud.exceed_duration1_color,dud.duration2,dud.exceed_duration2_color,dud.duration3,dud.exceed_duration3_color \
//         FROM ",kphis,".ipd_order_item oi JOIN ",kphis,".ipd_order o ON o.order_id=oi.order_id LEFT JOIN ",hosxp,".drugitems di ON di.icode=oi.icode \
//             LEFT JOIN ",hosxp,".ipt ON o.an=ipt.an LEFT JOIN ",hosxp,".opd_allergy allergy ON (\
//             (allergy.agent LIKE CONCAT('%',di.generic_name,'%') AND allergy.hn=ipt.hn AND di.generic_name IS NOT NULL AND TRIM(di.generic_name) <> '') \
//             OR (di.generic_name LIKE CONCAT('%',allergy.agent,'%') AND allergy.hn=ipt.hn AND allergy.agent IS NOT NULL AND TRIM(allergy.agent) <> '')) \
//             LEFT JOIN ",kphis,".kphis_drug_use_duration dud ON dud.icode=oi.icode \
//         WHERE o.order_date < ? AND o.order_type='continuous' AND o.an=? AND o.order_confirm='Y' AND oi.off_order_item_id IS NULL AND oi.order_item_id NOT IN (\
//             SELECT ofi2.off_order_item_id FROM kphis.ipd_order_item ofi2 JOIN kphis.ipd_order off2 ON ofi2.order_id=off2.order_id AND off2.order_date < ? AND off2.order_confirm='Y' \
//             WHERE off2.an=o.an AND ofi2.off_order_item_id=oi.order_item_id) ",view_by2,"GROUP BY oi.order_item_id ORDER BY o.order_date,o.order_time,oi.order_item_id;"
//     )
// }

// SELECT oi.icode,oi.med_reconciliation_item_id,oi.order_item_detail,oi.order_item_type,dud.`usage` AS due_usage,dud.info,dud.info_status,
//     (SELECT TIMESTAMP(ofo.order_date,ofo.order_time) FROM kphis.ipd_order_item ofoi JOIN kphis.ipd_order ofo ON ofoi.order_id=ofo.order_id AND ofo.an=ofoi.an AND ofo.order_confirm='Y'
//         WHERE ofoi.off_order_item_id=oi.order_item_id AND ofoi.order_item_type='off' AND ofo.an=oi.an LIMIT 1) AS off_by_datetime,
//     CONCAT(di.`name`,' ',di.strength,' ',di.units) AS med_name,di.generic_name,
//     mr.old_drugusage,mr.receive_from,mr.receive_date,mr.receive_qty,mr.last_dose_taken_time,mr.last_dose_taken_remark \
// FROM kphis.ipd_order_item oi
//     JOIN kphis.ipd_order o ON o.order_id=oi.order_id
//     LEFT JOIN kphis.kphis_drug_use_duration dud ON dud.icode=oi.icode AND dud.status='Y'
//     LEFT JOIN kphis.ipd_med_reconciliation_item mr ON mr.med_reconciliation_item_id=oi.med_reconciliation_item_id
//     LEFT JOIN hos.drugitems di ON di.icode=oi.icode
// WHERE oi.an=? AND oi.order_item_type <> 'off' AND o.order_type='oneday' AND o.order_confirm='Y' AND o.order_date=DATE_ADD(DATE(NOW()),INTERVAL -1 DAY)
// HAVING off_by_datetime IS NULL ORDER BY oi.order_item_id;
/// an
pub fn select_one_day_previous(hosxp: &str, kphis: &str) -> String {
    [
        "SELECT oi.icode,oi.med_reconciliation_item_id,oi.order_item_detail,oi.order_item_type,dud.`usage` AS due_usage,dud.`status` AS due_status,dud.info,dud.info_status,\
            (SELECT TIMESTAMP(ofo.order_date,ofo.order_time) FROM ",kphis,".ipd_order_item ofoi JOIN ",kphis,".ipd_order ofo ON ofoi.order_id=ofo.order_id AND ofo.an=ofoi.an AND ofo.order_confirm='Y' \
                WHERE ofoi.off_order_item_id=oi.order_item_id AND ofoi.order_item_type='off' AND ofo.an=oi.an LIMIT 1) AS off_by_datetime,\
            CONCAT(di.`name`,' ',di.strength,' ',di.units) AS med_name,di.generic_name,di.dosageform,di.displaycolor,di.addict_type_id,di.habit_forming_type,\
            mr.old_drugusage,mr.receive_from,mr.receive_date,mr.receive_qty,mr.last_dose_taken_time,mr.last_dose_taken_remark,mr.`use` AS used \
        FROM ",kphis,".ipd_order_item oi \
            JOIN ",kphis,".ipd_order o ON o.order_id=oi.order_id \
            LEFT JOIN ",kphis,".kphis_drug_use_duration dud ON dud.icode=oi.icode \
            LEFT JOIN ",kphis,".ipd_med_reconciliation_item mr ON mr.med_reconciliation_item_id=oi.med_reconciliation_item_id \
            LEFT JOIN ",hosxp,".drugitems di ON di.icode=oi.icode \
        WHERE oi.an=? AND oi.order_item_type <> 'off' AND o.order_type='oneday' AND o.order_confirm='Y' AND o.order_date=DATE_ADD(DATE(NOW()),INTERVAL -1 DAY) \
        HAVING off_by_datetime IS NULL ORDER BY oi.order_item_id;"
    ].concat()
}

// // ipd-dr-order-one-day-confirm.php, ipd-dr-order-continuous-confirm.php
// --1 SELECT order_confirm, IF(order_date = DATE(NOW()), 'Y', 'N') AS order_date_is_today FROM kphis.ipd_order WHERE order_id = ?;
// --2 UPDATE kphis.ipd_order SET order_confirm ='Y', order_date = NOW(), order_time = NOW(), order_doctor = ?, update_user = ?, update_datetime = NOW(), version = (version+1) WHERE order_id = ? AND order_confirm != 'Y' AND order_date = DATE(NOW());
// --we change two sqls into one
// UPDATE kphis.ipd_order SET order_confirm ='Y', order_date = NOW(), order_time = NOW(), order_doctor = ?, update_user = ?, update_datetime = NOW(), version = (version+1) WHERE order_id = ? AND order_confirm != 'Y';
/// (order_time), doctor_code, loginname, order_id
pub fn update_confirm_order(is_fixed_time: bool, kphis: &str) -> String {
    let order_time = if is_fixed_time {"?"} else {"NOW()"};
    [
        "UPDATE ",kphis,".ipd_order SET order_confirm ='Y',order_date=NOW(),order_time=",order_time,",order_doctor=?",TABLE_UPDATE_SET," WHERE order_id=? AND order_confirm !='Y';"
    ].concat()
}

// UPDATE kphis.ipd_order SET nurse_order_as = ?, order_confirm ='Y', order_date = NOW(), order_time = NOW(), order_doctor = ?, update_user = ?, update_datetime = NOW(), version = (version+1) WHERE order_id = ? AND order_confirm != 'Y';
/// (order_time), nurse_order_as, doctor_code, loginname, order_id
pub fn update_confirm_order_as(is_fixed_time: bool, kphis: &str) -> String {
    let order_time = if is_fixed_time {"?"} else {"NOW()"};
    [
        "UPDATE ",kphis,".ipd_order SET order_date=NOW(), order_time=",order_time,", nurse_order_as=?, order_confirm='Y', order_doctor=?",TABLE_UPDATE_SET," WHERE order_id=? AND order_confirm != 'Y';"
    ].concat()
}

// UPDATE kphis.ipd_order SET nurse_order_as=?, update_user=?, update_datetime=NOW(), version=(version+1) WHERE order_id=? AND doctor_confirm_time IS NULL;
/// nurse_order_as, loginname, order_id
pub fn update_edit_order_as(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".ipd_order SET nurse_order_as=?",TABLE_UPDATE_SET," WHERE order_id=? AND doctor_confirm_time IS NULL;"
    ].concat()
}

// UPDATE kphis.ipd_order SET doctor_confirm_time=NOW(), update_user=?, update_datetime=NOW(), version=(version+1) WHERE doctor_confirm_time IS NULL AND nurse_order_as=? AND order_id=?;
/// (order_time), loginname, doctor_code, (order_id)
pub fn update_doctor_confirm_order(has_order_id: bool, is_fixed_time: bool, kphis: &str) -> String {
    let order_id = if has_order_id {" AND order_id=?"} else {""};
    let doctor_confirm_time = if is_fixed_time {"TIMESTAMP(CURDATE(),?)"} else {"NOW()"};
    [
        "UPDATE ",kphis,".ipd_order SET doctor_confirm_time=",doctor_confirm_time,TABLE_UPDATE_SET,
        " WHERE doctor_confirm_time IS NULL AND nurse_order_as=?", order_id, ";"
    ].concat()
}

// // ipd-dr-order-one-day-nurse-accept.php, ipd-dr-order-continuous-nurse-accept.php
// --1 SELECT nurse_accept, nurse_accept_time FROM kphis.ipd_order WHERE nurse_accept_time IS NULL AND order_id=?;
// --2 UPDATE kphis.ipd_order SET nurse_accept_time=NOW(), nurse_accept=?, update_user=?, update_datetime=NOW(), version=(version+1) WHERE order_id=?;
// --we change two sqls into one
// UPDATE kphis.ipd_order SET nurse_accept_time=NOW(), nurse_accept=?, update_user=?, update_datetime=NOW(), version=(version+1) WHERE nurse_accept_time IS NULL AND order_id=?;
/// (order_time), doctor_code, loginname, order_id
pub fn update_nurse_accept_order(is_fixed_time: bool, kphis: &str) -> String {
    let nurse_accept_time = if is_fixed_time {"TIMESTAMP(CURDATE(),?)"} else {"NOW()"};
    [
        "UPDATE ",kphis,".ipd_order SET nurse_accept_time=",nurse_accept_time,", nurse_accept=?",TABLE_UPDATE_SET," WHERE nurse_accept_time IS NULL AND order_id=?;"
    ].concat()
}

// // ipd-dr-order-one-day-pharmacist-accept.php, ipd-dr-order-continuous-pharmacist-accept.php
// --1 SELECT pharmacist_accept, pharmacist_accept_time FROM kphis.ipd_order WHERE pharmacist_accept_time IS NULL AND order_id=?;
// --2 UPDATE kphis.ipd_order SET pharmacist_accept_time=NOW(), pharmacist_accept=?, pharmacist_order_status='accepted', update_user=?, update_datetime=NOW(), version=(version+1) WHERE order_id=?;
// --we change two sqls into one
// UPDATE kphis.ipd_order SET pharmacist_accept_time=NOW(), pharmacist_accept=?, pharmacist_order_status='accepted', update_user=?, update_datetime=NOW(), version=(version+1) WHERE pharmacist_accept_time IS NULL AND order_id=?;
/// (order_time), doctor_code, loginname, order_id
pub fn update_pharmacist_accept_order(is_fixed_time: bool, kphis: &str) -> String {
    let pharmacist_accept_time = if is_fixed_time {"TIMESTAMP(CURDATE(),?)"} else {"NOW()"};
    [
        "UPDATE ",kphis,".ipd_order SET pharmacist_accept_time=",pharmacist_accept_time,", pharmacist_accept=?, pharmacist_order_status='accepted'",TABLE_UPDATE_SET," WHERE pharmacist_accept_time IS NULL AND order_id=?;"
    ].concat()
}

// UPDATE kphis.ipd_order SET pharmacist_check_time=NOW(), pharmacist_check=?, pharmacist_order_status='check', update_user=?, update_datetime=NOW(), version=(version+1) WHERE pharmacist_check_time IS NULL AND order_id=?;
/// (order_time), doctor_code, loginname, order_id
pub fn update_pharmacist_check_order(is_fixed_time: bool, kphis: &str) -> String {
    let pharmacist_check_time = if is_fixed_time {"TIMESTAMP(CURDATE(),?)"} else {"NOW()"};
    [
        "UPDATE ",kphis,".ipd_order SET pharmacist_check_time=",pharmacist_check_time,", pharmacist_check=?, pharmacist_order_status='checked'",TABLE_UPDATE_SET," WHERE pharmacist_check_time IS NULL AND order_id=?;"
    ].concat()
}

// // ipd-dr-order-one-day-pharmacist-done.php, ipd-dr-order-continuous-pharmacist-done.php
// --1 SELECT pharmacist_done, pharmacist_done_time FROM kphis.ipd_order WHERE pharmacist_done_time IS NULL AND order_id=?;
// --2 UPDATE kphis.ipd_order SET pharmacist_done_time=NOW(), pharmacist_done=?, pharmacist_order_status='done', update_user=?, update_datetime=NOW(), version=(version+1) WHERE order_id=?;
// --we change two sqls into one
// UPDATE kphis.ipd_order SET pharmacist_done_time=NOW(), pharmacist_done=?, pharmacist_order_status='done', update_user=?, update_datetime=NOW(), version=(version+1) WHERE pharmacist_done_time IS NULL AND order_id=?;
/// (order_time), doctor_code, loginname, order_id
pub fn update_pharmacist_done_order(is_fixed_time: bool, kphis: &str) -> String {
    let pharmacist_done_time = if is_fixed_time {"TIMESTAMP(CURDATE(),?)"} else {"NOW()"};
    [
        "UPDATE ",kphis,".ipd_order SET pharmacist_done_time=",pharmacist_done_time,", pharmacist_done=?, pharmacist_order_status='done'",TABLE_UPDATE_SET," WHERE pharmacist_done_time IS NULL AND order_id=?;"
    ].concat()
}

// // ipd-dr-order-one-day-delete.php, ipd-dr-order-continuous-delete.php
// DELETE o, i FROM kphis.ipd_order AS o JOIN kphis.ipd_order_item AS i ON i.order_id = o.order_id WHERE o.order_id = ? AND o.order_confirm != 'Y';
// *** cannot use alias in delete `https://bugs.mysql.com/bug.php?id=82189` ***
/// delete `order` and `order_item`
/// order_id
pub fn delete_order(kphis: &str) -> String {
    [
        "DELETE ",kphis,".ipd_order, ",kphis,".ipd_order_item \
        FROM ",kphis,".ipd_order \
            LEFT JOIN ",kphis,".ipd_order_item ON ipd_order_item.order_id=ipd_order.order_id \
        WHERE ipd_order.order_id=? AND ipd_order.order_confirm != 'Y';"
    ].concat()
}

/// delete `order`, `order_item`, `index_plan` and `index_action`
/// an
pub fn delete_order_bundle(kphis: &str) -> String {
    [
        "DELETE ",kphis,".ipd_order,",kphis,".ipd_order_item,",kphis,".ipd_nurse_index_plan,",kphis,".ipd_nurse_index_action \
        FROM ",kphis,".ipd_order \
            LEFT JOIN ",kphis,".ipd_order_item ON ipd_order_item.order_id=ipd_order.order_id \
            LEFT JOIN ",kphis,".ipd_nurse_index_plan ON ipd_nurse_index_plan.order_item_id=ipd_order_item.order_item_id \
            LEFT JOIN ",kphis,".ipd_nurse_index_action ON ipd_nurse_index_action.plan_id=ipd_nurse_index_plan.plan_id \
        WHERE ipd_order.an=?;"
    ].concat()
}

// INSERT INTO kphis.ipd_order (an,order_date,order_time,order_doctor,order_type,order_owner_type,order_confirm,create_user,create_datetime,update_user,update_datetime,version)
// VALUES (?,DATE(NOW()),TIME(NOW()),?,?,?,'N',?,NOW(),?,NOW(),1);
/// an, order_doctor, order_type, order_owner_type, loginname, loginname
pub fn insert_many_orders(
    pre_orders: &[PreOrderSave],
    an: &str,
    loginname: &str,
    doctorcode: &str,
    kphis: &str,
) -> String {
    pre_orders.iter().map(|order| {
        [
            "INSERT INTO ",kphis,".ipd_order (an,order_date,order_time,order_doctor,order_type,order_owner_type,order_confirm",TABLE_CREATE_COLUMNS,") \
                VALUES ('",an,"',DATE(NOW()),TIME(NOW()),'",
                &doctorcode,"','",
                &order.order_type,"','",
                &order.order_owner_type,"','N','",
                loginname, "',NOW(),'",loginname,"',NOW(),1);"
        ].concat()
    }).collect::<Vec<String>>().concat()
}

// INSERT INTO kphis.ipd_order (an,order_date,order_time,pre_order_id,pre_order_date,pre_order_time,order_doctor,order_type,order_owner_type,order_confirm,create_user,create_datetime,update_user,update_datetime,version)
// VALUES (?,DATE(NOW()),TIME(NOW()),?,?,?,?,?,?,'N',?,NOW(),?,NOW(),1);
/// an, pre_order_id, pre_order_date, pre_order_time, order_doctor, order_type, order_owner_type, loginname, loginname
pub fn insert_many_orders_with_pre(
    pre_orders: &[PreOrderSave],
    an: &str,
    loginname: &str,
    kphis: &str,
) -> String {
    pre_orders.iter().map(|order| {
        [
            "INSERT INTO ",kphis,".ipd_order (an,order_date,order_time,pre_order_id,pre_order_date,pre_order_time,order_doctor,order_type,order_owner_type,order_confirm",TABLE_CREATE_COLUMNS,") \
                VALUES ('",an,"',DATE(NOW()),TIME(NOW()),",
                &order.order_id.to_string(),",'",
                &order.order_date.to_string(),"','",
                &order.order_time.to_string(),"','",
                &order.order_doctor,"','",
                &order.order_type,"','",
                &order.order_owner_type,"','N','",
                loginname, "',NOW(),'",loginname,"',NOW(),1);"
        ].concat()
    }).collect::<Vec<String>>().concat()
}

// INSERT INTO kphis.ipd_order_item (order_id,an,order_item_type,order_item_detail,stat,off_order_item_id,icode,create_user,create_datetime,update_user,update_datetime,version)
// VALUES (?,?,?,?,?,?,?,?,now(),?,now(),1)
/// order_id, an, order_item_type, order_item_detail, stat, off_order_item_id, icode, loginname, loginname
pub fn insert_pre_to_order_items(
    order_items: &[OrderItemSave],
    order_id_map: &HashMap<u32, u64>,
    an: &str,
    loginname: &str,
    kphis: &str,
) -> String {
    let values = order_items
        .iter()
        .map(|item| {
            [
                "(",&order_id_map.get(&item.order_id.unwrap_or_default()).map(|id| id.to_string()).unwrap_or(String::from("NULL")),",'",
                an,"',",
                &item.order_item_type.as_ref().map(|s| ["'", s, "'"].concat()).unwrap_or(String::from("NULL")),",",
                &item.order_item_detail.as_ref().map(|s| ["'", s, "'"].concat()).unwrap_or(String::from("NULL")),",",
                &item.stat.as_ref().map(|s| ["'", s, "'"].concat()).unwrap_or(String::from("NULL")),",",
                &item.off_order_item_id.map(|id| id.to_string()).unwrap_or(String::from("NULL")),",",
                &item.icode.as_ref().map(|s| ["'", s, "'"].concat()).unwrap_or(String::from("NULL")),",'",
                loginname,"',NOW(),'",loginname,"',NOW(),1)"
            ].concat()
        }).collect::<Vec<String>>().join(",");
    [
        "INSERT INTO ",kphis,".ipd_order_item (order_id,an,order_item_type,order_item_detail,stat,off_order_item_id,icode",TABLE_CREATE_COLUMNS,") \
        VALUES ",&values,";"
    ].concat()
}

// // ipd-pharmacy-order-monitor-table.php
// // we change
// IF((SUM(IF(ipd_order.pharmacist_order_status IS NULL,1,0)) > 0) AND ((SELECT COUNT(*) FROM kphis.ipd_order_item oi JOIN kphis.ipd_order o ON oi.order_id=o.order_id WHERE o.order_confirm='Y' AND oi.an=ipt.an AND oi.stat='Y' AND o.pharmacist_order_status IS NULL) > 0),'ยังไม่ได้รับ - มี Stat Order',
//     IF((SUM(IF(ipd_order.pharmacist_order_status IS NULL,1,0)) > 0) AND ((SELECT COUNT(*) FROM kphis.ipd_order_item oi JOIN kphis.ipd_order o ON oi.order_id=o.order_id WHERE o.order_confirm='Y' AND oi.an=ipt.an AND oi.order_item_type='home-medication' AND o.pharmacist_order_status IS NULL) > 0),'ยังไม่ได้รับ - มี Home-Med Order',
//     IF(SUM(IF(ipd_order.pharmacist_order_status IS NULL,1,0)) > 0,'ยังไม่ได้รับ',
//     IF((SUM(IF(ipd_order.pharmacist_order_status='accepted',1,0)) > 0) AND ((SELECT COUNT(*) FROM kphis.ipd_order_item oi JOIN kphis.ipd_order o ON oi.order_id=o.order_id WHERE o.order_confirm='Y' AND oi.an=ipt.an AND oi.stat='Y' AND o.pharmacist_order_status='accepted') > 0),'รับแล้ว - มี Stat Order',
//     IF((SUM(IF(ipd_order.pharmacist_order_status='accepted',1,0)) > 0) AND ((SELECT COUNT(*) FROM kphis.ipd_order_item oi JOIN kphis.ipd_order o ON oi.order_id=o.order_id WHERE o.order_confirm='Y' AND oi.an=ipt.an AND oi.order_item_type='home-medication' AND o.pharmacist_order_status='accepted') > 0),'รับแล้ว - มี Home-Med Order',
//     IF(SUM(IF(ipd_order.pharmacist_order_status='accepted',1,0)) > 0,'รับแล้ว',
//     IF((SUM(IF(ipd_order.pharmacist_order_status='done',1,0)) > 0) AND ((SELECT COUNT(*) FROM kphis.ipd_order_item oi JOIN kphis.ipd_order o ON oi.order_id=o.order_id WHERE o.order_confirm='Y' AND oi.an=ipt.an AND oi.stat='Y' AND o.pharmacist_order_status='accepted') > 0),'ตรวจสอบแล้ว - มี Stat Order',
//     IF((SUM(IF(ipd_order.pharmacist_order_status='done',1,0)) > 0) AND ((SELECT COUNT(*) FROM kphis.ipd_order_item oi JOIN kphis.ipd_order o ON oi.order_id=o.order_id WHERE o.order_confirm='Y' AND oi.an=ipt.an AND oi.order_item_type='home-medication' AND o.pharmacist_order_status='accepted') > 0),'ตรวจสอบแล้ว - มี Home-Med Order',
//     IF(SUM(IF(ipd_order.pharmacist_order_status='done',1,0)) > 0,'ตรวจสอบแล้ว',NULL))))))))) AS order_priority_text,
// IF((SUM(IF(ipd_order.pharmacist_order_status IS NULL,1,0)) > 0) AND ((SELECT COUNT(*) FROM kphis.ipd_order_item oi JOIN kphis.ipd_order o ON oi.order_id=o.order_id WHERE o.order_confirm='Y' AND oi.an=ipt.an AND oi.stat='Y' AND o.pharmacist_order_status IS NULL) > 0),1,
//     IF((SUM(IF(ipd_order.pharmacist_order_status IS NULL,1,0)) > 0) AND ((SELECT COUNT(*) FROM kphis.ipd_order_item oi JOIN kphis.ipd_order o ON oi.order_id=o.order_id WHERE o.order_confirm='Y' AND oi.an=ipt.an AND oi.order_item_type='home-medication' AND o.pharmacist_order_status IS NULL) > 0),2,
//     IF(SUM(IF(ipd_order.pharmacist_order_status IS NULL,1,0)) > 0,3,
//     IF((SUM(IF(ipd_order.pharmacist_order_status='accepted',1,0)) > 0) AND ((SELECT COUNT(*) FROM kphis.ipd_order_item oi JOIN kphis.ipd_order o ON oi.order_id=o.order_id WHERE o.order_confirm='Y' AND oi.an=ipt.an AND oi.stat='Y' AND o.pharmacist_order_status='accepted') > 0),4,
//     IF((SUM(IF(ipd_order.pharmacist_order_status='accepted',1,0)) > 0) AND ((SELECT COUNT(*) FROM kphis.ipd_order_item oi JOIN kphis.ipd_order o ON oi.order_id=o.order_id WHERE o.order_confirm='Y' AND oi.an=ipt.an AND oi.order_item_type='home-medication' AND o.pharmacist_order_status='accepted') > 0),5,
//     IF(SUM(IF(ipd_order.pharmacist_order_status='accepted',1,0)) > 0,6,
//     IF((SUM(IF(ipd_order.pharmacist_order_status='done',1,0)) > 0) AND ((SELECT COUNT(*) FROM kphis.ipd_order_item oi JOIN kphis.ipd_order o ON oi.order_id=o.order_id WHERE o.order_confirm='Y' AND oi.an=ipt.an AND oi.stat='Y' AND o.pharmacist_order_status='accepted') > 0),7,
//     IF((SUM(IF(ipd_order.pharmacist_order_status='done',1,0)) > 0) AND ((SELECT COUNT(*) FROM kphis.ipd_order_item oi JOIN kphis.ipd_order o ON oi.order_id=o.order_id WHERE o.order_confirm='Y' AND oi.an=ipt.an AND oi.order_item_type='home-medication' AND o.pharmacist_order_status='accepted') > 0),8,
//     IF(SUM(IF(ipd_order.pharmacist_order_status='done',1,0)) > 0,9,10))))))))) AS order_priority,
// // to count_item_not_accept_stat, count_item_not_accept_homemed, count_item_accept_stat, count_item_accept_homemed for calculate client side
// SELECT IF(SUM(IF((SELECT COUNT(*) FROM kphis.ipd_order_item oi WHERE oi.order_id=ipd_order.order_id AND oi.stat='Y') > 0,1,0)) > 0,'Y','N') AS has_stat,
//     SUM(IF((SELECT COUNT(*) FROM kphis.ipd_order_item oi WHERE oi.order_id=ipd_order.order_id AND oi.stat='Y') > 0,1,0)) AS count_stat,
//     SUM(IF(ipd_order.pharmacist_order_status IS NULL,1,0)) AS count_not_accept,
//     SUM(IF(ipd_order.pharmacist_order_status='accepted',1,0)) AS count_accept,
//     SUM(IF(ipd_order.pharmacist_order_status='done',1,0)) AS count_done,
//     (SELECT COUNT(*) FROM kphis.ipd_order_item oi JOIN kphis.ipd_order o ON oi.order_id=o.order_id WHERE o.order_confirm='Y' AND oi.an=ipt.an AND o.pharmacist_order_status IS NULL AND oi.stat='Y') AS count_item_not_accept_stat,
//     (SELECT COUNT(*) FROM kphis.ipd_order_item oi JOIN kphis.ipd_order o ON oi.order_id=o.order_id WHERE o.order_confirm='Y' AND oi.an=ipt.an AND o.pharmacist_order_status IS NULL AND oi.order_item_type='home-medication') AS count_item_not_accept_homemed,
//     (SELECT COUNT(*) FROM kphis.ipd_order_item oi JOIN kphis.ipd_order o ON oi.order_id=o.order_id WHERE o.order_confirm='Y' AND oi.an=ipt.an AND o.pharmacist_order_status='accepted' AND oi.stat='Y') AS count_item_accept_stat,
//     (SELECT COUNT(*) FROM kphis.ipd_order_item oi JOIN kphis.ipd_order o ON oi.order_id=o.order_id WHERE o.order_confirm='Y' AND oi.an=ipt.an AND o.pharmacist_order_status='accepted' AND oi.order_item_type='home-medication') AS count_item_accept_homemed,
//     ipt.hn,ipt.an,substring(CONCAT(spclty.name,' - ',w.name),1,200) AS sname,w.name AS ward_name,iptadm.bedno,
//     CONCAT(patient.pname,patient.fname,' ',patient.lname) AS pname,aa.income AS income,ptt.pcode AS rtcode,ptt.name AS rtname,
//     dt.name AS admdoctor_name,aa.admdate,aa.age_y,aa.age_m,aa.age_d,dc1.name AS dchtype_name,dc2.name AS dchstts_name,
//     IF(SUM(IF(ipd_order.pharmacist_order_status IS NULL,1,0)) > 0,
//         MIN(IF(ipd_order.pharmacist_order_status IS NULL,CONCAT(ipd_order.order_date,' ',ipd_order.order_time),NULL)),
//         IF(SUM(IF(ipd_order.pharmacist_order_status='accepted',1,0)) > 0,
//             MIN(IF(ipd_order.pharmacist_order_status='accepted',CONCAT(ipd_order.order_date,' ',ipd_order.order_time),NULL)),
//             MIN(CONCAT(ipd_order.order_date,' ',ipd_order.order_time)))) AS min_order_date_time,
//     IF(SUM(IF(ipd_order.pharmacist_order_status IS NULL,1,0)) > 0,
//         MAX(IF(ipd_order.pharmacist_order_status IS NULL,CONCAT(ipd_order.order_date,' ',ipd_order.order_time),NULL)),
//         IF(SUM(IF(ipd_order.pharmacist_order_status='accepted',1,0)) > 0,
//             MAX(IF(ipd_order.pharmacist_order_status='accepted',CONCAT(ipd_order.order_date,' ',ipd_order.order_time),NULL)),
//             MAX(CONCAT(ipd_order.order_date,' ',ipd_order.order_time)))) AS max_order_date_time,
//     IF(SUM(IF((SELECT COUNT(*) FROM kphis.ipd_order_item oi WHERE oi.order_id=ipd_order.order_id
//         AND (ipd_order.pharmacist_order_status IS NULL OR ipd_order.pharmacist_order_status <> 'done') AND (oi.order_item_type IN ('ivfluid','injection','home-medication','med')
//         OR (ipd_order.order_owner_type='nurse' AND oi.order_item_type='other'))) > 0,1,0)) > 0,'Y','N') AS containing_med_order_item
// FROM kphis.ipd_order
//     INNER JOIN hos.ipt ON ipd_order.an=ipt.an
//     LEFT JOIN hos.spclty ON spclty.spclty=ipt.spclty
//     LEFT JOIN hos.iptadm ON iptadm.an=ipt.an
//     LEFT JOIN hos.patient ON patient.hn=ipt.hn
//     LEFT JOIN hos.doctor dt ON dt.code=ipt.admdoctor
//--   LEFT JOIN hos.roomno ON roomno.roomno=iptadm.roomno
//     LEFT JOIN hos.an_stat aa ON aa.an=ipt.an
//     LEFT JOIN hos.ward w ON w.ward=ipt.ward
//     LEFT JOIN hos.dchtype dc1 ON dc1.dchtype=ipt.dchtype
//     LEFT JOIN hos.dchstts dc2 ON dc2.dchstts=ipt.dchstts
//     LEFT JOIN hos.doctor di ON di.code=ipt.incharge_doctor
//     LEFT JOIN hos.pttype ptt ON ptt.pttype=ipt.pttype
//     LEFT JOIN kphis.ipd_doctor_in_charge ipd_dr ON ipd_dr.an=ipt.an AND ipd_dr.activated='on'
//     LEFT JOIN hos.doctor d3 ON d3.`code`=ipd_dr.doctor
//     LEFT JOIN kphis.opd_er_order_master om ON om.vn=ipt.vn
// WHERE ipd_order.order_confirm='Y'
//
// // 2025-09-06: only show 'ivfluid','injection','home-medication','med' and 'pharm'
// SELECT (SELECT COUNT(*) FROM kphis.ipd_order_item oi JOIN kphis.ipd_order o ON oi.order_id=o.order_id WHERE o.order_confirm='Y' AND oi.an=ipt.an AND oi.order_item_type IN ('ivfluid','injection','home-medication','med') AND oi.icode IS NOT NULL AND oi.stat='Y') AS count_stat,
//     (SELECT COUNT(*) FROM kphis.ipd_order_item oi JOIN kphis.ipd_order o ON oi.order_id=o.order_id WHERE o.order_confirm='Y' AND oi.an=ipt.an AND oi.order_item_type IN ('ivfluid','injection','home-medication','med') AND oi.icode IS NOT NULL AND o.pharmacist_order_status IS NULL) AS count_not_accept,
//     (SELECT COUNT(*) FROM kphis.ipd_order_item oi JOIN kphis.ipd_order o ON oi.order_id=o.order_id WHERE o.order_confirm='Y' AND oi.an=ipt.an AND oi.order_item_type IN ('ivfluid','injection','home-medication','med') AND oi.icode IS NOT NULL AND o.pharmacist_order_status='accepted') AS count_accept,
//     (SELECT COUNT(*) FROM kphis.ipd_order_item oi JOIN kphis.ipd_order o ON oi.order_id=o.order_id WHERE o.order_confirm='Y' AND oi.an=ipt.an AND oi.order_item_type IN ('ivfluid','injection','home-medication','med') AND oi.icode IS NOT NULL AND o.pharmacist_order_status='checked') AS count_check,
//     (SELECT COUNT(*) FROM kphis.ipd_order_item oi JOIN kphis.ipd_order o ON oi.order_id=o.order_id WHERE o.order_confirm='Y' AND oi.an=ipt.an AND oi.order_item_type IN ('ivfluid','injection','home-medication','med') AND oi.icode IS NOT NULL AND o.pharmacist_order_status='done') AS count_done,
//     (SELECT COUNT(*) FROM kphis.ipd_order_item oi JOIN kphis.ipd_order o ON oi.order_id=o.order_id WHERE o.order_confirm='Y' AND oi.an=ipt.an AND oi.order_item_type='pharm') AS count_pharm_notify,
//     (SELECT COUNT(*) FROM kphis.ipd_order_item oi JOIN kphis.ipd_order o ON oi.order_id=o.order_id WHERE o.order_confirm='Y' AND oi.an=ipt.an AND oi.order_item_type IN ('ivfluid','injection','home-medication','med') AND oi.icode IS NOT NULL AND o.pharmacist_order_status IS NULL AND oi.stat='Y') AS count_item_not_accept_stat,
//     (SELECT COUNT(*) FROM kphis.ipd_order_item oi JOIN kphis.ipd_order o ON oi.order_id=o.order_id WHERE o.order_confirm='Y' AND oi.an=ipt.an AND oi.order_item_type='home-medication' AND o.pharmacist_order_status IS NULL) AS count_item_not_accept_homemed,
//     (SELECT COUNT(*) FROM kphis.ipd_order_item oi JOIN kphis.ipd_order o ON oi.order_id=o.order_id WHERE o.order_confirm='Y' AND oi.an=ipt.an AND oi.order_item_type IN ('ivfluid','injection','home-medication','med') AND oi.icode IS NOT NULL AND o.pharmacist_order_status='accepted' AND oi.stat='Y') AS count_item_accept_stat,
//     (SELECT COUNT(*) FROM kphis.ipd_order_item oi JOIN kphis.ipd_order o ON oi.order_id=o.order_id WHERE o.order_confirm='Y' AND oi.an=ipt.an AND oi.order_item_type='home-medication' AND o.pharmacist_order_status='accepted') AS count_item_accept_homemed,
//     ipt.hn,ipt.an,substring(CONCAT(spclty.name,' - ',w.name),1,200) AS sname,w.name AS ward_name,iptadm.bedno,
//     CONCAT(patient.pname,patient.fname,' ',patient.lname) AS pname,aa.income AS income,ptt.pcode AS rtcode,ptt.name AS rtname,
//     dt.name AS admdoctor_name,aa.admdate,aa.age_y,aa.age_m,aa.age_d,dc1.name AS dchtype_name,dc2.name AS dchstts_name,
//     (SELECT MIN(CONCAT(o.order_date,' ',o.order_time)) FROM kphis.ipd_order_item oi JOIN kphis.ipd_order o ON oi.order_id=o.order_id WHERE oi.an=ipt.an 
//         AND oi.order_item_type IN ('ivfluid','injection','home-medication','med','pharm') AND oi.icode IS NOT NULL
//         AND o.pharmacist_order_status IS NULL) AS min_order_date_time,\
//     (SELECT MAX(CONCAT(o.order_date,' ',o.order_time)) FROM kphis.ipd_order_item oi JOIN kphis.ipd_order o ON oi.order_id=o.order_id WHERE oi.an=ipt.an 
//         AND oi.order_item_type IN ('ivfluid','injection','home-medication','med','pharm') AND oi.icode IS NOT NULL
//         AND o.pharmacist_order_status IS NULL) AS max_order_date_time 
// FROM hos.ipt 
//     LEFT JOIN kphis.ipd_order ON ipd_order.an=ipt.an 
//     LEFT JOIN hos.spclty ON spclty.spclty=ipt.spclty 
//     LEFT JOIN hos.iptadm ON iptadm.an=ipt.an 
//     LEFT JOIN hos.patient ON patient.hn=ipt.hn 
//     LEFT JOIN hos.doctor dt ON dt.code=ipt.admdoctor 
//     LEFT JOIN hos.an_stat aa ON aa.an=ipt.an 
//     LEFT JOIN hos.ward w ON w.ward=ipt.ward 
//     LEFT JOIN hos.dchtype dc1 ON dc1.dchtype=ipt.dchtype 
//     LEFT JOIN hos.dchstts dc2 ON dc2.dchstts=ipt.dchstts 
//     LEFT JOIN hos.doctor di ON di.code=ipt.incharge_doctor 
//     LEFT JOIN hos.pttype ptt ON ptt.pttype=ipt.pttype 
//     LEFT JOIN kphis.ipd_doctor_in_charge ipd_dr ON ipd_dr.an=ipt.an AND ipd_dr.activated='on' 
//     LEFT JOIN hos.doctor d3 ON d3.`code`=ipd_dr.doctor 
//     LEFT JOIN kphis.opd_er_order_master om ON om.vn=ipt.vn 
// WHERE ipd_order.order_confirm='Y' GROUP BY ipt.an 
// ORDER BY max_order_date_time DESC,ipt.dchdate DESC,ipt.dchtime DESC,ipt.regdate DESC,ipt.regtime DESC,LEFT(iptadm.bedno,3),MID(iptadm.bedno,4,999),iptadm.bedno 
// LIMIT 100;
/// has_patitent(1-2 pt by input length),<br>
/// not_pathent((doctor_in_charge),(order_date_from x1),(order_date_to x2))
pub fn select_pharmacy_order(
    params: &IpdOrderPharmacyParams,
    hlen: usize,
    alen: usize,
    hosxp: &str,
    kphis: &str,
) -> String {
    let patient = and_ipt_patient(&params.patient, hlen, alen, hosxp);
    let not_patient = " AND (om.opd_er_order_master_id IS NULL OR (om.opd_er_order_master_id IS NOT NULL AND om.er_patient_status_id=7)) ";
    let is_inverse_ward_select = params.inverse_ward_select.as_ref().map(|ins| ins == "Y").unwrap_or_default();
    let wards_sanitized = params.wards.as_ref().map(|words| {
        words.split(',').map(|ward| ward.chars().filter(|c| c.is_alphanumeric()).collect()).collect::<Vec<String>>()
    }).unwrap_or_default();
    let wards = if wards_sanitized.is_empty() {
        if is_inverse_ward_select {String::new()} else {String::from(" AND false ")}
    } else {
        [
            " AND ipt.ward",
            if is_inverse_ward_select {" NOT"} else {""},
            " IN ('",&wards_sanitized.join("','"),"') "
        ].concat()
    };
    let doctor_in_charge = if params.doctor_in_charge.is_some() {
        [" AND ipt.an IN (SELECT an FROM ",kphis,".ipd_doctor_in_charge WHERE doctor=?) "].concat()
    } else {
        String::new()
    };
    // let order_date_from = if params.order_date_from.is_some() {" AND ipd_order.order_date >= ? "} else {""};
    // let order_date_to = if params.order_date_to.is_some() {" AND ipd_order.order_date <= ? "} else {""};
    // // ipt.dchstts MUST filled unless still admited
    let order_date_from = if params.order_date_from.is_some() {" AND (ipt.dchdate >= ? OR ipt.dchstts IS NULL) "} else {""};
    let order_date_to = if params.order_date_to.is_some() {" AND (ipt.dchdate <= ? OR ipt.dchstts IS NULL) AND ipt.regdate <= ? "} else {""};
    // we found null dchdate with dchstts, found malformed ipt.dchstts such as "0" or "", so we use dc2.name instead
    let is_discharged = match params.is_discharged.clone().unwrap_or_default().as_str() {
        "Y" => " AND ipt.dchstts IS NOT NULL ",
        "N" => " AND ipt.dchstts IS NULL ",
        _ => "",
    };

    let where_clause = [
        &patient.clone().unwrap_or([not_patient, &wards, &doctor_in_charge].concat()),
        is_discharged,
        order_date_from,
        order_date_to
    ].concat();

    [
        "SELECT (SELECT COUNT(*) FROM ",kphis,".ipd_order_item oi JOIN ",kphis,".ipd_order o ON oi.order_id=o.order_id WHERE o.order_confirm='Y' AND oi.an=ipt.an AND oi.order_item_type IN ('ivfluid','injection','home-medication','med') AND oi.stat='Y') AS count_stat,\
            (SELECT COUNT(*) FROM ",kphis,".ipd_order_item oi JOIN ",kphis,".ipd_order o ON oi.order_id=o.order_id WHERE o.order_confirm='Y' AND oi.an=ipt.an AND oi.order_item_type IN ('ivfluid','injection','home-medication','med') AND oi.icode IS NOT NULL AND o.pharmacist_order_status IS NULL) AS count_not_accept,\
            (SELECT COUNT(*) FROM ",kphis,".ipd_order_item oi JOIN ",kphis,".ipd_order o ON oi.order_id=o.order_id WHERE o.order_confirm='Y' AND oi.an=ipt.an AND oi.order_item_type IN ('ivfluid','injection','home-medication','med') AND oi.icode IS NOT NULL AND o.pharmacist_order_status='accepted') AS count_accept,\
            (SELECT COUNT(*) FROM ",kphis,".ipd_order_item oi JOIN ",kphis,".ipd_order o ON oi.order_id=o.order_id WHERE o.order_confirm='Y' AND oi.an=ipt.an AND oi.order_item_type IN ('ivfluid','injection','home-medication','med') AND oi.icode IS NOT NULL AND o.pharmacist_order_status='checked') AS count_check,\
            (SELECT COUNT(*) FROM ",kphis,".ipd_order_item oi JOIN ",kphis,".ipd_order o ON oi.order_id=o.order_id WHERE o.order_confirm='Y' AND oi.an=ipt.an AND oi.order_item_type IN ('ivfluid','injection','home-medication','med') AND oi.icode IS NOT NULL AND o.pharmacist_order_status='done') AS count_done,\
            (SELECT COUNT(*) FROM ",kphis,".ipd_order_item oi JOIN ",kphis,".ipd_order o ON oi.order_id=o.order_id WHERE o.order_confirm='Y' AND oi.an=ipt.an AND oi.order_item_type='pharm') AS count_pharm_notify,\
            (SELECT COUNT(*) FROM ",kphis,".ipd_order_item oi JOIN ",kphis,".ipd_order o ON oi.order_id=o.order_id WHERE o.order_confirm='Y' AND oi.an=ipt.an AND oi.order_item_type IN ('ivfluid','injection','home-medication','med') AND oi.icode IS NOT NULL AND o.pharmacist_order_status IS NULL AND oi.stat='Y') AS count_item_not_accept_stat,\
            (SELECT COUNT(*) FROM ",kphis,".ipd_order_item oi JOIN ",kphis,".ipd_order o ON oi.order_id=o.order_id WHERE o.order_confirm='Y' AND oi.an=ipt.an AND oi.order_item_type='home-medication' AND o.pharmacist_order_status IS NULL) AS count_item_not_accept_homemed,\
            (SELECT COUNT(*) FROM ",kphis,".ipd_order_item oi JOIN ",kphis,".ipd_order o ON oi.order_id=o.order_id WHERE o.order_confirm='Y' AND oi.an=ipt.an AND oi.order_item_type IN ('ivfluid','injection','home-medication','med') AND oi.icode IS NOT NULL AND o.pharmacist_order_status='accepted' AND oi.stat='Y') AS count_item_accept_stat,\
            (SELECT COUNT(*) FROM ",kphis,".ipd_order_item oi JOIN ",kphis,".ipd_order o ON oi.order_id=o.order_id WHERE o.order_confirm='Y' AND oi.an=ipt.an AND oi.order_item_type='home-medication' AND o.pharmacist_order_status='accepted') AS count_item_accept_homemed,\
            ipt.hn,ipt.an,substring(CONCAT(spclty.name,' - ',w.name),1,200) AS sname,w.name AS ward_name,iptadm.bedno,\
            CONCAT(patient.pname,patient.fname,' ',patient.lname) AS pname,aa.income AS income,ptt.pcode AS rtcode,ptt.name AS rtname,\
            dt.name AS admdoctor_name,aa.admdate,aa.age_y,aa.age_m,aa.age_d,dc1.name AS dchtype_name,dc2.name AS dchstts_name,\
            (SELECT MIN(ADDTIME(CONVERT(order_date,DATETIME),order_time)) FROM ",kphis,".ipd_order_item oi JOIN ",kphis,".ipd_order o ON oi.order_id=o.order_id WHERE oi.an=ipt.an \
			    AND oi.order_item_type IN ('ivfluid','injection','home-medication','med','pharm') AND oi.icode IS NOT NULL \
			    AND o.pharmacist_order_status IS NULL) AS min_order_date_time,\
            (SELECT MAX(ADDTIME(CONVERT(order_date,DATETIME),order_time)) FROM ",kphis,".ipd_order_item oi JOIN ",kphis,".ipd_order o ON oi.order_id=o.order_id WHERE oi.an=ipt.an \
			    AND oi.order_item_type IN ('ivfluid','injection','home-medication','med','pharm') AND oi.icode IS NOT NULL \
			    AND o.pharmacist_order_status IS NULL) AS max_order_date_time \
        FROM ",hosxp,".ipt \
            LEFT JOIN ",kphis,".ipd_order ON ipd_order.an=ipt.an \
            LEFT JOIN ",hosxp,".spclty ON spclty.spclty=ipt.spclty \
            LEFT JOIN ",hosxp,".iptadm ON iptadm.an=ipt.an \
            LEFT JOIN ",hosxp,".patient ON patient.hn=ipt.hn \
            LEFT JOIN ",hosxp,".doctor dt ON dt.code=ipt.admdoctor \
            LEFT JOIN ",hosxp,".an_stat aa ON aa.an=ipt.an \
            LEFT JOIN ",hosxp,".ward w ON w.ward=ipt.ward \
            LEFT JOIN ",hosxp,".dchtype dc1 ON dc1.dchtype=ipt.dchtype \
            LEFT JOIN ",hosxp,".dchstts dc2 ON dc2.dchstts=ipt.dchstts \
            LEFT JOIN ",hosxp,".doctor di ON di.code=ipt.incharge_doctor \
            LEFT JOIN ",hosxp,".pttype ptt ON ptt.pttype=ipt.pttype \
            LEFT JOIN ",kphis,".ipd_doctor_in_charge ipd_dr ON ipd_dr.an=ipt.an AND ipd_dr.activated='on' \
            LEFT JOIN ",hosxp,".doctor d3 ON d3.`code`=ipd_dr.doctor \
            LEFT JOIN ",kphis,".opd_er_order_master om ON om.vn=ipt.vn \
        WHERE 1=1 ", &where_clause, " GROUP BY ipt.an \
        ORDER BY max_order_date_time DESC,ipt.dchdate DESC,ipt.dchtime DESC,ipt.regdate DESC,ipt.regtime DESC,LEFT(iptadm.bedno,3),MID(iptadm.bedno,4,999),iptadm.bedno \
        LIMIT 100;"
    ].concat()
}

// // SELECT MAX(CONCAT(ipd_order.order_date,' ',ipd_order.order_time)) AS new_last_order_time
// // FROM kphis.ipd_order
// //     LEFT JOIN hos.ipt on ipd_order.an=ipt.an
// //     LEFT JOIN hos.patient on patient.hn=ipt.hn
// //     LEFT JOIN kphis.ipd_doctor_in_charge ipd_dr ON ipd_dr.an=ipt.an AND ipd_dr.activated='on'
// // WHERE ipd_order.order_confirm='Y' AND ipd_order.pharmacist_order_status IS NULL
// pub fn select_pharmacy_new_last_order_time(
//     params: &IpdOrderPharmacyParams,
//     hlen: usize,
//     alen: usize,
//     hosxp: &str,
//     kphis: &str,
// ) -> String {
//     let patient = and_ipt_patient(&params.patient, hlen, alen, hosxp);
//     let is_inverse_ward_selection = params.inverse_ward_selection.as_ref().map(|ins| ins == "Y").unwrap_or_default();
//     let wards_sanitized = params.wards.as_ref().map(|words| {
//         words.split(',').map(|ward| ward.chars().filter(|c| c.is_alphanumeric()).collect()).collect::<Vec<String>>()
//     }).unwrap_or_default();
//     let wards = if wards_sanitized.is_empty() {
//         if is_inverse_ward_selection {String::new()} else {String::from(" AND false ")}
//     } else {
//         [
//             " AND ipt.ward",
//             if is_inverse_ward_selection {" NOT"} else {""},
//             " IN ('",&wards_sanitized.join("','"),"') "
//         ].concat()
//     };
//     let doctor_in_charge = if params.doctor_in_charge.is_some() {
//         [" AND ipt.an IN (SELECT an FROM ",kphis,".ipd_doctor_in_charge WHERE doctor=?) "].concat()
//     } else {
//         String::new()
//     }; // doctor_in_charge
//     let order_date_from = if params.order_date_from.is_some() {" AND ipd_order.order_date >= ? "} else {""}; // order_date_from
//     let order_date_to = if params.order_date_to.is_some() {" AND ipd_order.order_date <= ? "} else {""}; // order_date_to

//     let where_clause = [
//         &patient.clone().unwrap_or([wards, doctor_in_charge].concat()),
//         order_date_from,
//         order_date_to
//     ].concat();
//     [
//         "SELECT MAX(CONCAT(ipd_order.order_date,' ',ipd_order.order_time)) AS new_last_order_time \
//         FROM ",kphis,".ipd_order \
//             LEFT JOIN ",hosxp,".ipt on ipd_order.an=ipt.an \
//             LEFT JOIN ",hosxp,".patient on patient.hn=ipt.hn \
//             LEFT JOIN ",kphis,".ipd_doctor_in_charge ipd_dr ON ipd_dr.an=ipt.an AND ipd_dr.activated='on' \
//         WHERE ipd_order.order_confirm='Y' AND ipd_order.pharmacist_order_status IS NULL ",&where_clause,";"
//     ].concat()
// }

// // SELECT ipt.hn,ipt.an,SUBSTRING(CONCAT(spclty.name,' - ',w.name),1,200) AS sname,w.name AS ward_name,iptadm.bedno,
// //     CONCAT(patient.pname,patient.fname,' ',patient.lname) AS pname,aa.income AS income,ptt.pcode AS rtcode,ptt.name AS rtname,
// //     dt.name AS admdoctor_name,aa.admdate,aa.age_y,aa.age_m,aa.age_d,dc1.name AS dchtype_name,dc2.name AS dchstts_name,di.name AS incharge_doctor_name,
// //     GROUP_CONCAT(DISTINCT d3.name ORDER BY status DESC,doctor_in_charge_id ASC SEPARATOR ', ') AS kphis_incharge_doctor_name,
// //     (SELECT MAX(vs_datetime) FROM kphis.ipd_vs_vital_sign vs WHERE vs.an=ipt.an) AS max_vs_datetime,
// //     (SELECT MAX(CONCAT(fcnote_date,' ',fcnote_time)) FROM kphis.ipd_focus_note fcnote WHERE fcnote.an=ipt.an) AS max_fcnote_datetime,
// //     (SELECT MAX(CONCAT(order_date,' ',order_time)) FROM kphis.ipd_order WHERE ipd_order.an=ipt.an AND ipd_order.order_confirm='Y' AND ipd_order.order_owner_type='doctor') AS max_order_datetime
// // FROM hos.ipt
// //     LEFT JOIN hos.spclty ON spclty.spclty=ipt.spclty
// //     LEFT JOIN hos.iptadm ON iptadm.an=ipt.an
// //     LEFT JOIN hos.patient ON patient.hn=ipt.hn
// //     LEFT JOIN hos.doctor dt ON dt.code=ipt.admdoctor
// //--   LEFT JOIN hos.roomno ON roomno.roomno=iptadm.roomno
// //     LEFT JOIN hos.an_stat aa ON aa.an=ipt.an
// //     LEFT JOIN hos.ward w ON w.ward=ipt.ward
// //     LEFT JOIN hos.dchtype dc1 ON dc1.dchtype=ipt.dchtype
// //     LEFT JOIN hos.dchstts dc2 ON dc2.dchstts=ipt.dchstts
// //     LEFT JOIN hos.doctor di ON di.code=ipt.incharge_doctor
// //     LEFT JOIN hos.pttype ptt ON ptt.pttype=ipt.pttype
// //     LEFT JOIN kphis.ipd_doctor_in_charge ipd_dr ON ipd_dr.an=ipt.an AND ipd_dr.activated='on'
// //     LEFT JOIN hos.doctor d3 ON d3.`code`=ipd_dr.doctor
// //     LEFT JOIN kphis.opd_er_order_master om ON om.vn=ipt.vn
// pub fn select_pharmacy_ipt(
//     params: &IpdOrderPharmacyParams,
//     hlen: usize,
//     alen: usize,
//     hosxp: &str,
//     kphis: &str,
// ) -> String {
//     let patient = and_ipt_patient(&params.patient, hlen, alen, hosxp);
//     let is_inverse_ward_selection = params.inverse_ward_selection.as_ref().map(|ins| ins == "Y").unwrap_or_default();
//     let wards_sanitized = params.wards.as_ref().map(|words| {
//         words.split(',').map(|ward| ward.chars().filter(|c| c.is_alphanumeric()).collect()).collect::<Vec<String>>()
//     }).unwrap_or_default();
//     let wards = if wards_sanitized.is_empty() {
//         if is_inverse_ward_selection {String::new()} else {String::from(" AND false ")}
//     } else {
//         [
//             " AND ipt.ward",
//             if is_inverse_ward_selection {" NOT"} else {""},
//             " IN ('",&wards_sanitized.join("','"),"') "
//         ].concat()
//     };
//     let doctor_in_charge = if params.doctor_in_charge.is_some() {
//         [" AND ipt.an IN (SELECT an FROM ",kphis,".ipd_doctor_in_charge WHERE doctor=?) "].concat()
//     } else {
//         String::new()
//     }; // doctor_in_charge
//     let order_date_from = if params.order_date_from.is_some() {" AND ipd_order.order_date >= ? "} else {""}; // order_date_from
//     let order_date_to = if params.order_date_to.is_some() {" AND ipd_order.order_date <= ? "} else {""}; // order_date_to
//     let order_date = if params.order_date_from.is_some() || params.order_date_to.is_some() {
//         [" AND ipt.an NOT IN (SELECT an FROM ",kphis,".ipd_order WHERE ipd_order.an=ipt.an AND ipd_order.order_confirm='Y' ",order_date_from,order_date_to,") "].concat()
//     } else {
//         String::new()
//     };
//     let group_order_by = if patient.is_some() {
//         " GROUP BY ipt.an ORDER BY ipt.an DESC LIMIT 100;"
//     } else {
//         " GROUP BY ipt.an ORDER BY LEFT(iptadm.bedno,3),MID(iptadm.bedno,4,999),iptadm.bedno,ipt.regdate,ipt.regtime;"
//     };
//     let where_group_order = [
//         &patient.clone().unwrap_or([wards, doctor_in_charge].concat()),
//         &order_date,
//         group_order_by
//     ].concat();

//     [
//         "SELECT ipt.hn,ipt.an,SUBSTRING(CONCAT(spclty.name,' - ',w.name),1,200) AS sname,w.name AS ward_name,iptadm.bedno,\
//             CONCAT(patient.pname,patient.fname,' ',patient.lname) AS pname,aa.income AS income,ptt.pcode AS rtcode,ptt.name AS rtname,\
//             dt.name AS admdoctor_name,aa.admdate,aa.age_y,aa.age_m,aa.age_d,dc1.name AS dchtype_name,dc2.name AS dchstts_name,di.name AS incharge_doctor_name,\
//             GROUP_CONCAT(DISTINCT d3.name ORDER BY status DESC,doctor_in_charge_id ASC SEPARATOR ', ') AS kphis_incharge_doctor_name,\
//             (SELECT MAX(vs_datetime) FROM ",kphis,".ipd_vs_vital_sign vs WHERE vs.an=ipt.an) AS max_vs_datetime,\
//             (SELECT MAX(CONCAT(fcnote_date,' ',fcnote_time)) FROM ",kphis,".ipd_focus_note fcnote WHERE fcnote.an=ipt.an) AS max_fcnote_datetime,\
//             (SELECT MAX(CONCAT(order_date,' ',order_time)) FROM ",kphis,".ipd_order WHERE ipd_order.an=ipt.an AND ipd_order.order_confirm='Y' AND ipd_order.order_owner_type='doctor') AS max_order_datetime \
//         FROM ",hosxp,".ipt \
//             LEFT JOIN ",hosxp,".spclty ON spclty.spclty=ipt.spclty \
//             LEFT JOIN ",hosxp,".iptadm ON iptadm.an=ipt.an \
//             LEFT JOIN ",hosxp,".patient ON patient.hn=ipt.hn \
//             LEFT JOIN ",hosxp,".doctor dt ON dt.code=ipt.admdoctor \
//             LEFT JOIN ",hosxp,".an_stat aa ON aa.an=ipt.an \
//             LEFT JOIN ",hosxp,".ward w ON w.ward=ipt.ward \
//             LEFT JOIN ",hosxp,".dchtype dc1 ON dc1.dchtype=ipt.dchtype \
//             LEFT JOIN ",hosxp,".dchstts dc2 ON dc2.dchstts=ipt.dchstts \
//             LEFT JOIN ",hosxp,".doctor di ON di.code=ipt.incharge_doctor \
//             LEFT JOIN ",hosxp,".pttype ptt ON ptt.pttype=ipt.pttype \
//             LEFT JOIN ",kphis,".ipd_doctor_in_charge ipd_dr ON ipd_dr.an=ipt.an AND ipd_dr.activated='on' \
//             LEFT JOIN ",hosxp,".doctor d3 ON d3.`code`=ipd_dr.doctor \
//             LEFT JOIN ",kphis,".opd_er_order_master om ON om.vn=ipt.vn \
//             WHERE ipt.dchstts IS NULL AND (om.opd_er_order_master_id IS NULL OR (om.opd_er_order_master_id IS NOT NULL AND om.er_patient_status_id=7)) ", &where_group_order
//         ].concat()
// }

// SELECT ipt.hn,ipt.an,SUBSTRING(CONCAT(spclty.name,' - ',w.name),1,200) AS sname,w.name AS ward_name,iptadm.bedno,
//     CONCAT(patient.pname,patient.fname,' ',patient.lname) AS pname,aa.income AS income,ptt.pcode AS rtcode,
//     ptt.name AS rtname,dt.name AS admdoctor_name,aa.admdate,aa.age_y,aa.age_m,aa.age_d,
//     dc1.name AS dchtype_name,dc2.name AS dchstts_name,di.name AS incharge_doctor_name,
//     GROUP_CONCAT(DISTINCT d3.name ORDER BY status DESC,doctor_in_charge_id ASC SEPARATOR ', ') AS kphis_incharge_doctor_name,
//     (SELECT MAX(vs_datetime) FROM kphis.ipd_vs_vital_sign vs WHERE vs.an=ipt.an) AS max_vs_datetime,
//     (SELECT MAX(CONCAT(fcnote_date,' ',fcnote_time)) FROM kphis.ipd_focus_note fcnote WHERE fcnote.an=ipt.an) AS max_fcnote_datetime,
//     (SELECT MAX(CONCAT(order_date,' ',order_time)) FROM kphis.ipd_order
//     WHERE ipd_order.an=ipt.an AND ipd_order.order_confirm='Y' AND ipd_order.order_owner_type='doctor') AS max_order_datetime
// FROM hos.ipt
//     LEFT JOIN hos.spclty ON spclty.spclty=ipt.spclty
//     LEFT JOIN hos.iptadm ON iptadm.an=ipt.an
//     LEFT JOIN hos.patient ON patient.hn=ipt.hn
//     LEFT JOIN hos.doctor dt ON dt.code=ipt.admdoctor
//--   LEFT JOIN hos.roomno ON roomno.roomno=iptadm.roomno
//     LEFT JOIN hos.an_stat aa ON aa.an=ipt.an
//     LEFT JOIN hos.ward w ON w.ward=ipt.ward
//     LEFT JOIN hos.dchtype dc1 ON dc1.dchtype=ipt.dchtype
//     LEFT JOIN hos.dchstts dc2 ON dc2.dchstts=ipt.dchstts
//     LEFT JOIN hos.doctor di ON di.code=ipt.incharge_doctor
//     LEFT JOIN hos.pttype ptt ON ptt.pttype=ipt.pttype
//     LEFT JOIN kphis.ipd_doctor_in_charge ipd_dr ON ipd_dr.an=ipt.an AND ipd_dr.activated='on'
//     LEFT JOIN hos.doctor d3 ON d3.`code`=ipd_dr.doctor
// WHERE ipt.dchstts IS NOT NULL
pub fn select_admit_history(
    params: &IpdOrderPharmacyParams,
    hlen: usize,
    alen: usize,
    hosxp: &str,
    kphis: &str,
) -> String {
    let patient = and_ipt_patient(&params.patient, hlen, alen, hosxp).unwrap_or_default();

    [
        "SELECT ipt.hn,ipt.an,SUBSTRING(CONCAT(spclty.name,' - ',w.name),1,200) AS sname,w.name AS ward_name,iptadm.bedno,\
            CONCAT(patient.pname,patient.fname,' ',patient.lname) AS pname,aa.income AS income,ptt.pcode AS rtcode,\
            ptt.name AS rtname,dt.name AS admdoctor_name,aa.admdate,aa.age_y,aa.age_m,aa.age_d,\
            dc1.name AS dchtype_name,dc2.name AS dchstts_name,di.name AS incharge_doctor_name,\
            GROUP_CONCAT(DISTINCT d3.name ORDER BY status DESC,doctor_in_charge_id ASC SEPARATOR ', ') AS kphis_incharge_doctor_name,\
            (SELECT MAX(vs_datetime) FROM ",kphis,".ipd_vs_vital_sign vs WHERE vs.an=ipt.an) AS max_vs_datetime,\
            (SELECT MAX(ADDTIME(CONVERT(fcnote_date,DATETIME),fcnote_time)) FROM ",kphis,".ipd_focus_note fcnote WHERE fcnote.an=ipt.an) AS max_fcnote_datetime,\
            (SELECT MAX(ADDTIME(CONVERT(order_date,DATETIME),order_time)) FROM ",kphis,".ipd_order \
            WHERE ipd_order.an=ipt.an AND ipd_order.order_confirm='Y' AND ipd_order.order_owner_type='doctor') AS max_order_datetime \
        FROM ",hosxp,".ipt \
            LEFT JOIN ",hosxp,".spclty ON spclty.spclty=ipt.spclty \
            LEFT JOIN ",hosxp,".iptadm ON iptadm.an=ipt.an \
            LEFT JOIN ",hosxp,".patient ON patient.hn=ipt.hn \
            LEFT JOIN ",hosxp,".doctor dt ON dt.code=ipt.admdoctor \
            LEFT JOIN ",hosxp,".an_stat aa ON aa.an=ipt.an \
            LEFT JOIN ",hosxp,".ward w ON w.ward=ipt.ward \
            LEFT JOIN ",hosxp,".dchtype dc1 ON dc1.dchtype=ipt.dchtype \
            LEFT JOIN ",hosxp,".dchstts dc2 ON dc2.dchstts=ipt.dchstts \
            LEFT JOIN ",hosxp,".doctor di ON di.code=ipt.incharge_doctor \
            LEFT JOIN ",hosxp,".pttype ptt ON ptt.pttype=ipt.pttype \
            LEFT JOIN ",kphis,".ipd_doctor_in_charge ipd_dr ON ipd_dr.an=ipt.an AND ipd_dr.activated='on' \
            LEFT JOIN ",hosxp,".doctor d3 ON d3.`code`=ipd_dr.doctor \
        WHERE ipt.dchstts IS NOT NULL ",&patient,
        " GROUP BY ipt.an ORDER BY ipt.an DESC;"
    ].concat()
}

/// med_plan_number, an, doctor, icode, qty, offdate, orderdate, orderstatus, drugusage, sp_use, first_qty, last_update, first_update
pub fn insert_medplan_ipd(hosxp: &str) -> String {
    ["INSERT INTO ",hosxp,".medplan_ipd \
        (med_plan_number,an,doctor,icode,qty,offdate,orderdate,orderstatus,drugusage,sp_use,frequency,icode_type,med_interval_type_id,first_qty,frequency_2,last_update,first_update) \
    VALUES (?,?,?,?,?,?,?,?,?,?,1,'1',0,?,0,?,?);"].concat()
}

//UPDATE hos.medplan_ipd SET offdate=? WHERE med_plan_number IN ('xxx');
/// offdate, last_update
pub fn update_medplan_ipd_off(med_plan_numbers: &[i32], hosxp: &str) -> String {
    let numbers = med_plan_numbers.iter().map(|i| i.to_string()).collect::<Vec<String>>().join("','");
    ["UPDATE ",hosxp,".medplan_ipd SET offdate=?,last_update=? WHERE med_plan_number IN ('",&numbers,"');"].concat()
}

// SELECT drugusage FROM hos.drugusage WHERE CONCAT(IFNULL(name1,''),' ',IFNULL(name2,''),' ',IFNULL(name3,''))=? AND status='Y';
/// name1+name2+name2
pub fn select_drugusage(hosxp: &str) -> String {
    ["SELECT drugusage FROM ",hosxp,".drugusage WHERE TRIM(REGEXP_REPLACE(CONCAT(IFNULL(name1,''),' ',IFNULL(name2,''),' ',IFNULL(name3,'')),'[ .]{2,}',' ')) = ? AND status='Y';"].concat()
}

/// sp_use,name1,name2,name3,user
pub fn insert_sp_use(hosxp: &str) -> String {
    ["INSERT INTO ",hosxp,".sp_use \
        (sp_use,sp_name,name1,name2,name3,`user`) \
    VALUES (?,'',?,?,?,?);"].concat()
}