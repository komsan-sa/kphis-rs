use std::collections::HashMap;

use kphis_model::{
    order::{OrderParams, OrderItemSave}, opd_er::pharmacy_monitor::OpdErOrderPharmacyParams,
    pre_order::order::PreOrderSave,
};

use crate::{
    TABLE_CREATE_COLUMNS, TABLE_CREATE_PREPARED, TABLE_UPDATE_SET,
    opd_er::and_opd_patient,
};

// // opd-er-order-one-day-data.php
// SELECT o.*,d.name AS order_doctor_name,s.name AS nurse_order_as_name,
//     IF(o.order_owner_type = 'doctor',d.licenseno,s.licenseno) AS doctor_licenseno,
//     n.name AS nurse_accept_name,pa.name AS pharmacist_accept_name,pc.name AS pharmacist_check_name,pd.name AS pharmacist_done_name
// FROM kphis.opd_er_order o
//     LEFT JOIN hos.doctor d ON d.code = o.order_doctor
//     LEFT JOIN hos.doctor s ON s.code = o.nurse_order_as
//     LEFT JOIN hos.doctor n ON n.code = o.nurse_accept
//     LEFT JOIN hos.doctor pa ON pa.code = o.pharmacist_accept
//     LEFT JOIN hos.doctor pc ON pc.code = o.pharmacist_check
//     LEFT JOIN hos.doctor pd ON pd.code = o.pharmacist_done
// WHERE 1=1
// ORDER BY o.order_date,o.order_time,o.order_id;
/// (order_id), (opd_er_order_master_id), (order_confirm), (doctor_not_confirm_as='Y' => doctorcode)
pub fn select_order(params: &OrderParams, intern_roles: &[String], hosxp: &str, kphis: &str) -> String {
    let order_id = if params.order_id.is_some() {" AND o.order_id=? "} else {""};
    let opd_er_order_master_id = if params.opd_er_order_master_id.is_some() {" AND o.opd_er_order_master_id=? "} else {""};
    let order_confirm = if params.order_confirm.is_some() {" AND o.order_confirm=? "} else {""};
    let order_type = params.order_type.as_ref().map(|t| {
        if ["oneday", "continuous"].contains(&t.as_str()) {
            [" AND o.order_type ='", t, "' "].concat()
        } else {
            String::new()
        }
    }).unwrap_or_default();
    let order_owner_types = params.order_owner_types.as_ref().map(|types| {
        let in_c = types.split(',').map(|s| s.trim())
            .filter(|t| ["doctor", "nurse", "pharmacist"].contains(t))
            .collect::<Vec<&str>>().join("','");
        [" AND o.order_owner_type IN ('", &in_c, "') "].concat()
    }).unwrap_or_default();
    let view_by = if params.doctor_not_confirm_as.as_ref().map(|s| s.as_str() == "Y").unwrap_or_default() {
        " AND o.order_owner_type = 'nurse' AND o.order_confirm = 'Y' AND o.nurse_order_as=? AND o.doctor_confirm_time IS NULL "
    } else {
        params.view_by.as_ref().map(|v| match v.as_str() {
            // "doctor" => " AND ((o.order_owner_type = 'nurse' AND o.order_confirm = 'Y') OR (o.order_owner_type = 'doctor')) ",
            // "nurse" => " AND ((o.order_owner_type = 'doctor' AND o.order_confirm = 'Y') OR (o.order_owner_type = 'nurse')) ",
            "doctor"|"nurse" => " AND o.order_owner_type IN ('doctor','nurse') ",
            "pharmacist"|"other" => " AND (o.order_owner_type IN ('doctor','nurse') AND o.order_confirm = 'Y') ",
            _ => " AND 0=1 ",
        }).unwrap_or_default()
    };

    [
        "SELECT o.*,ovst.hn,om.vn,CONCAT(p.pname,p.fname,' ',p.lname) AS fullname,b.bedno AS display_bedno,bt.bed_type_name,bt.bed_type_color,\
            d1.`name` AS order_doctor_name,d1.licenseno AS order_doctor_licenseno,\
            (SELECT entryposition FROM ",hosxp,".opduser WHERE doctorcode=o.order_doctor AND (account_disable IS NULL OR account_disable='N') LIMIT 1) AS order_doctor_entryposition,\
            (SELECT EXISTS(SELECT * FROM ",hosxp,".opduser ou LEFT JOIN ",kphis,".system_ac_role_user ru ON ou.loginname=ru.loginname \
                WHERE ou.doctorcode=o.order_doctor AND (ou.account_disable IS NULL OR ou.account_disable='N') AND ru.role IN ('",&intern_roles.join("','"),"'))) AS order_doctor_is_intern,\
            d2.`name` AS nurse_order_as_name,d2.licenseno AS nurse_order_as_licenseno,\
            (SELECT entryposition FROM ",hosxp,".opduser WHERE doctorcode=o.nurse_order_as AND (account_disable IS NULL OR account_disable='N') LIMIT 1) AS nurse_order_as_entryposition,\
            (SELECT EXISTS(SELECT * FROM ",hosxp,".opduser ou LEFT JOIN ",kphis,".system_ac_role_user ru ON ou.loginname=ru.loginname \
                WHERE ou.doctorcode=o.nurse_order_as AND (ou.account_disable IS NULL OR ou.account_disable='N') AND ru.role IN ('",&intern_roles.join("','"),"'))) AS nurse_order_as_is_intern,\
            d3.`name` AS nurse_accept_name,d3.licenseno AS nurse_accept_licenseno,\
            (SELECT entryposition FROM ",hosxp,".opduser WHERE doctorcode=o.nurse_accept AND (account_disable IS NULL OR account_disable='N') LIMIT 1) AS nurse_accept_entryposition,\
            d4.`name` AS pharmacist_accept_name,d4.licenseno AS pharmacist_accept_licenseno,\
            (SELECT entryposition FROM ",hosxp,".opduser WHERE doctorcode=o.pharmacist_accept AND (account_disable IS NULL OR account_disable='N') LIMIT 1) AS pharmacist_accept_entryposition,\
            d5.`name` AS pharmacist_check_name,d5.licenseno AS pharmacist_check_licenseno,\
            (SELECT entryposition FROM ",hosxp,".opduser WHERE doctorcode=o.pharmacist_check AND (account_disable IS NULL OR account_disable='N') LIMIT 1) AS pharmacist_check_entryposition,\
            d6.`name` AS pharmacist_done_name,d6.licenseno AS pharmacist_done_licenseno,\
            (SELECT entryposition FROM ",hosxp,".opduser WHERE doctorcode=o.pharmacist_done AND (account_disable IS NULL OR account_disable='N') LIMIT 1) AS pharmacist_done_entryposition \
        FROM ",kphis,".opd_er_order o \
            LEFT JOIN ",kphis,".opd_er_order_master om ON om.opd_er_order_master_id=o.opd_er_order_master_id \
            LEFT JOIN ",kphis,".opd_er_bed b ON b.opd_er_bed_id=om.bedno \
            LEFT JOIN ",kphis,".opd_er_bed_type bt ON bt.bed_type=b.bed_type \
            LEFT JOIN ",hosxp,".ovst ON ovst.vn=om.vn \
            LEFT JOIN ",hosxp,".patient p ON p.hn=ovst.hn \
            LEFT JOIN ",hosxp,".doctor d1 ON d1.`code`=o.order_doctor \
            LEFT JOIN ",hosxp,".doctor d2 ON d2.`code`=o.nurse_order_as \
            LEFT JOIN ",hosxp,".doctor d3 ON d3.`code`=o.nurse_accept \
            LEFT JOIN ",hosxp,".doctor d4 ON d4.`code`=o.pharmacist_accept \
            LEFT JOIN ",hosxp,".doctor d5 ON d5.`code`=o.pharmacist_check \
            LEFT JOIN ",hosxp,".doctor d6 ON d6.`code`=o.pharmacist_done \
        WHERE 1=1 ", &order_type, order_id, opd_er_order_master_id, order_confirm, &order_owner_types, view_by,
        "ORDER BY o.order_date,o.order_time,o.order_id;"
    ].concat()
}

/// opd_er_order_master_id
pub fn select_order_only(kphis: &str) -> String {
    [
        "SELECT * FROM ",kphis,".opd_er_order WHERE opd_er_order_master_id=? ORDER BY order_date,order_time,order_id;"
    ].concat()
}

// SELECT DISTINCT oi.order_item_type, oi.order_id
// FROM kphis.opd_er_order_item oi
// JOIN kphis.opd_er_order o ON o.order_id = oi.order_id
// LEFT JOIN kphis.ipd_order_item_type oit ON oi.order_item_type = oit.order_item_type AND o.order_type = oit.order_type
// WHERE oi.order_id=? AND o.order_type=?
// ORDER BY oit.display_order, oi.order_item_id;
/// (order_type)
pub fn select_order_types(ids: &[u32], has_order_type: bool, kphis: &str) -> String {
    let in_c = ids.iter().map(|id| id.to_string()).collect::<Vec<String>>().join(",");
    let order_type = if has_order_type {" AND o.order_type=? "} else {""};
    [
        "SELECT DISTINCT oi.order_item_type, oi.order_id \
        FROM ",kphis,".opd_er_order_item oi \
            JOIN ",kphis,".opd_er_order o ON o.order_id = oi.order_id \
            LEFT JOIN ",kphis,".ipd_order_item_type oit ON oi.order_item_type = oit.order_item_type AND o.order_type = oit.order_type \
        WHERE oi.order_id IN (",&in_c,") ", order_type,
        "ORDER BY oit.display_order, oi.order_item_id;"
    ].concat()
}

// SELECT oi.*,o.order_date,o.order_time,o.order_type,o.order_owner_type,ooi.order_item_detail AS off_order_item_detail,
//     (SELECT TIMESTAMP(off_by_order.order_date,off_by_order.order_time) FROM kphis.opd_er_order_item obi
//         JOIN kphis.opd_er_order off_by_order ON obi.order_id = off_by_order.order_id
//             AND off_by_order.order_confirm = 'Y'
//         WHERE obi.off_order_item_id = oi.order_item_id
//             AND off_by_order.opd_er_order_master_id = oi.opd_er_order_master_id LIMIT 1) AS off_by_datetime,
//     IF(mr.custom_med_name IS NOT NULL,mr.custom_med_name,CONCAT(di.`name`,' ',di.strength,' ',di.units)) AS med_name,di.displaycolor,di.generic_name,ooi.icode AS off_icode,
//     CONCAT(off_di.`name`,' ',off_di.strength,' ',off_di.units) AS off_med_name,off_di.displaycolOR AS off_displaycolor,
//     GROUP_CONCAT(DISTINCT(CONCAT(allergy.agent,'=',IFNULL(allergy.symptom,''))) ORDER BY allergy.agent) AS allergy_agent_symptom
// FROM kphis.opd_er_order_item oi
// JOIN kphis.opd_er_order o ON o.order_id = oi.order_id
// LEFT JOIN kphis.opd_er_order_item ooi ON ooi.order_item_id = oi.off_order_item_id
// LEFT JOIN kphis.opd_er_med_reconciliation_item mr ON mr.med_reconciliation_item_id=oi.med_reconciliation_item_id
// LEFT JOIN hos.drugitems di ON di.icode = oi.icode
// LEFT JOIN hos.drugitems off_di ON off_di.icode = ooi.icode
// LEFT JOIN kphis.opd_er_order_master om ON o.opd_er_order_master_id = om.opd_er_order_master_id
// LEFT JOIN hos.ovst ON ovst.vn = om.vn
// LEFT JOIN hos.opd_allergy allergy ON (
//     (allergy.agent LIKE CONCAT('%',di.generic_name,'%') AND allergy.hn=ovst.hn AND di.generic_name IS NOT NULL AND TRIM(di.generic_name) <> '')
//     OR (di.generic_name LIKE CONCAT('%',allergy.agent,'%') AND allergy.hn=ovst.hn AND allergy.agent IS NOT NULL AND TRIM(allergy.agent) <> ''))
// WHERE oi.order_id=? AND oi.order_item_type=? AND o.order_type=? GROUP BY oi.order_item_id ORDER BY oi.order_item_id;
/// (opd_er_order_master_id), (order_id), (order_item_id), (order_item_type), (order_type)<br>
/// NOTE: order_id MUST use outside PARAMS
pub fn select_order_item(
    params: &OrderParams,
    has_order_id: bool,
    has_order_item_type: bool,
    hosxp: &str,
    kphis: &str,
) -> String {
    let (opd_er_order_master_id, limit) = if params.opd_er_order_master_id.is_some() {(" AND o.opd_er_order_master_id=? ","")} else {("", " LIMIT 100")};
    let vb = if params.view_by.as_ref().map(|vb| vb.as_str() == "doctor").unwrap_or_default()
        {""} else {" AND off_by_order.order_confirm = 'Y' "};
    let order_id = if has_order_id {" AND o.order_id=? "} else {""};
    let order_item_id = if params.order_item_id.is_some() {" AND oi.order_item_id=? "} else {""};
    let order_item_type = if has_order_item_type {" AND oi.order_item_type=? "} else {""};
    let order_type = if params.order_type.is_some() {" AND o.order_type=? "} else {""};
    [
        "SELECT oi.*,om.vn,o.order_date,o.order_time,o.order_type,o.order_owner_type,ooi.order_item_detail AS off_order_item_detail,d.`name` AS order_doctor_name,d.`licenseno` AS order_doctor_licenseno,\
        dud.`usage` AS due_usage,dud.status AS due_status,dud.monitor,dud.monitor_count,dud.monitor_duration,dud.monitor_status,dud.info,dud.info_status,\
            (SELECT TIMESTAMP(off_by_order.order_date,off_by_order.order_time) FROM ",kphis,".opd_er_order_item obi \
                JOIN ",kphis,".opd_er_order off_by_order ON obi.order_id=off_by_order.order_id ",vb,
                "WHERE obi.off_order_item_id=oi.order_item_id AND off_by_order.opd_er_order_master_id=oi.opd_er_order_master_id LIMIT 1) AS off_by_datetime,\
            IF(mr.custom_med_name IS NULL OR mr.custom_med_name='',CONCAT(di.`name`,' ',di.strength,' ',di.units),mr.custom_med_name) AS med_name,\
            di.displaycolor,di.generic_name,di.dosageform,di.addict_type_id,di.habit_forming_type,ooi.icode AS off_icode,\
            IF(omr.custom_med_name IS NULL OR omr.custom_med_name='',CONCAT(off_di.`name`,' ',off_di.strength,' ',off_di.units),omr.custom_med_name) AS off_med_name,off_di.displaycolOR AS off_displaycolor,\
            mr.old_drugusage,mr.receive_from,mr.receive_date,mr.receive_qty,mr.last_dose_taken_time,mr.last_dose_taken_remark,mr.`use` AS used,\
            GROUP_CONCAT(DISTINCT(CONCAT(allergy.agent,'=',IFNULL(allergy.symptom,''))) ORDER BY allergy.agent) AS allergy_agent_symptom \
        FROM ",kphis,".opd_er_order_item oi \
            JOIN ",kphis,".opd_er_order o ON o.order_id=oi.order_id \
            LEFT JOIN ",kphis,".kphis_drug_use_duration dud ON dud.icode=oi.icode \
            LEFT JOIN ",kphis,".opd_er_order_item ooi ON ooi.order_item_id=oi.off_order_item_id \
            LEFT JOIN ",kphis,".opd_er_med_reconciliation_item mr ON mr.med_reconciliation_item_id=oi.med_reconciliation_item_id \
            LEFT JOIN ",kphis,".opd_er_med_reconciliation_item omr ON omr.med_reconciliation_item_id=ooi.med_reconciliation_item_id \
            LEFT JOIN ",hosxp,".drugitems di ON di.icode=oi.icode \
            LEFT JOIN ",hosxp,".drugitems off_di ON off_di.icode=ooi.icode \
            LEFT JOIN ",kphis,".opd_er_order_master om ON o.opd_er_order_master_id=om.opd_er_order_master_id \
            LEFT JOIN ",hosxp,".ovst ON ovst.vn=om.vn \
            LEFT JOIN ",hosxp,".doctor d ON d.`code`=IF(o.nurse_order_as IS NULL,o.order_doctor,o.nurse_order_as) \
            LEFT JOIN ",hosxp,".opd_allergy allergy ON (\
                (allergy.agent LIKE CONCAT('%',di.generic_name,'%') AND allergy.hn=ovst.hn AND di.generic_name IS NOT NULL AND TRIM(di.generic_name) <> '') \
                OR (di.generic_name LIKE CONCAT('%',allergy.agent,'%') AND allergy.hn=ovst.hn AND allergy.agent IS NOT NULL AND TRIM(allergy.agent) <> '')) \
        WHERE 1=1 ", opd_er_order_master_id, order_id, order_item_id, order_item_type, order_type, " GROUP BY oi.order_item_id ORDER BY oi.order_item_id", limit, ";"
    ].concat()
}

/// order_id
pub fn select_order_item_only(kphis: &str) -> String {
    [
        "SELECT * FROM ",kphis,".opd_er_order_item WHERE order_id=? ORDER BY order_item_id;"
    ].concat()
}

// // SELECT ia.action_id,ia.plan_id,ia.opd_er_order_master_id,ia.action_result,ia.action_remark,ia.action_date,ia.action_time,ia.action_report_back,ia.action_blood_had,ia.action_person_1,ia.action_person_2,
// //     ip.order_item_id,ip.plan_detail,ip.plan_date,ip.plan_time,ip.plan_sch_type
// // FROM kphis.opd_er_nurse_index_action AS ia INNER JOIN kphis.opd_er_nurse_index_plan AS ip ON ia.plan_id = ip.plan_id
// // WHERE ip.order_item_id=:order_item_id ORDER BY ip.plan_date, ip.plan_time, ia.action_date, ia.action_time;
// /// order_item_id, (order_date)
// pub fn select_order_action(with_plan_date: bool, kphis: &str) -> String {
//     let plan_date = if with_plan_date {" AND ip.plan_date=? "} else {""};
//     [
//         "SELECT ia.action_id,ia.plan_id,ia.opd_er_order_master_id,ia.action_result,ia.action_remark,ia.action_date,ia.action_time,ia.action_report_back,ia.action_blood_had,ia.action_person_1,ia.action_person_2,\
//             ip.order_item_id,ip.plan_detail,ip.plan_date,ip.plan_time,ip.plan_sch_type \
//         FROM ",kphis,".opd_er_nurse_index_action AS ia INNER JOIN ",kphis,".opd_er_nurse_index_plan AS ip ON ia.plan_id = ip.plan_id \
//         WHERE ip.order_item_id=? ", plan_date, " ORDER BY ip.plan_date, ip.plan_time, ia.action_date, ia.action_time;"
//     ].concat()
// }

// INSERT INTO kphis.opd_er_order (opd_er_order_master_id,order_date,order_time,pre_order_id,pre_order_date,pre_order_time,order_doctor,order_type,order_owner_type,order_confirm,create_user,create_datetime,update_user,update_datetime,version)
// VALUES (?, DATE(NOW()), TIME(NOW()),?,?,?,'N',?,NOW(),?,NOW(),1);
/// opd_er_order_master_id, order_doctor, order_type, order_owner_type, loginname, loginname
pub fn insert_many_orders_with_pre(
    pre_orders: &[PreOrderSave],
    opd_er_order_master_id: u32,
    loginname: &str,
    doctorcode: &str,
    kphis: &str,
) -> String {
    pre_orders.iter().map(|order| {
        [
            "INSERT INTO ",kphis,".opd_er_order (opd_er_order_master_id,order_date,order_time,pre_order_id,pre_order_date,pre_order_time,order_doctor,order_type,order_owner_type,order_confirm",TABLE_CREATE_COLUMNS,") \
                VALUES (",&opd_er_order_master_id.to_string(),", DATE(NOW()), TIME(NOW()),",
                &order.order_id.to_string(),",'",
                &order.order_date.to_string(),"','",
                &order.order_time.to_string(),"','",
                &doctorcode,"','",
                &order.order_type,"','",
                &order.order_owner_type,"','N','",
                loginname, "',NOW(),'",loginname,"',NOW(),1);"
        ].concat()
    }).collect::<Vec<String>>().concat()
}

// INSERT INTO kphis.opd_er_order_item (order_id,opd_er_order_master_id,order_item_type,order_item_detail,stat,off_order_item_id,icode,create_user,create_datetime,update_user,update_datetime,version)
// VALUES (?,?,?,?,?,?,?,?,NOW(),?,NOW(),1);
/// order_id, opd_er_order_master_id, order_item_type, order_item_detail, stat, off_order_item_id, icode, loginname, loginname
pub fn insert_pre_to_order_items(
    order_items: &[OrderItemSave],
    order_id_map: &HashMap<u32, u64>,
    opd_er_order_master_id: u32,
    loginname: &str,
    kphis: &str,
) -> String {
    let values = order_items.iter().map(|item| {
        [
            "(",&order_id_map.get(&item.order_id.unwrap_or_default()).map(|id| id.to_string()).unwrap_or(String::from("NULL")),",",
            &opd_er_order_master_id.to_string(),",",
            &item.order_item_type.as_ref().map(|s| ["'", s, "'"].concat()).unwrap_or(String::from("NULL")),",",
            &item.order_item_detail.as_ref().map(|s| ["'", s, "'"].concat()).unwrap_or(String::from("NULL")),",",
            &item.stat.as_ref().map(|s| ["'", s, "'"].concat()).unwrap_or(String::from("NULL")),",",
            &item.off_order_item_id.map(|id| id.to_string()).unwrap_or(String::from("NULL")),",",
            &item.icode.as_ref().map(|s| ["'", s, "'"].concat()).unwrap_or(String::from("NULL")),",'",
            loginname,"',NOW(),'",loginname,"',NOW(),1)"
        ].concat()
        }).collect::<Vec<String>>().join(",");
    [
        "INSERT INTO ",kphis,".opd_er_order_item (order_id,opd_er_order_master_id,order_item_type,order_item_detail,stat,off_order_item_id,icode",TABLE_CREATE_COLUMNS,") \
        VALUES ",&values,";"
    ].concat()
}

// pub fn insert_order_items_only(order_id: u32, opd_er_order_master_id: u32, order_item_onlys: &[OrderItemOnly], kphis: &str) -> String {
//     let values = order_item_onlys.iter().map(|item| {
//         [
//             "(",&order_id.to_owned(),",",
//             &opd_er_order_master_id.to_owned(),",",
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
//         "INSERT INTO ",kphis,".opd_er_order_item (order_id,opd_er_order_master_id,order_item_type,order_item_detail,stat,off_order_item_id,icode,med_reconciliation_item_id,create_user,create_datetime,update_user,update_datetime,version) \
//         VALUES ", &values,";"
//     ].concat()
// }

// SELECT order_confirm FROM kphis.opd_er_order WHERE order_id=?;
/// order_id
pub fn get_order_confirm(kphis: &str) -> String {
    [
        "SELECT order_confirm FROM ",kphis,".opd_er_order WHERE order_id=?;"
    ].concat()
}

// UPDATE kphis.opd_er_order SET order_date=DATE(NOW()),order_time=TIME(NOW()),order_doctor=?,update_user=?,update_datetime=NOW(),version=(version+1) WHERE order_id=?;
/// order_doctor, loginname, order_id
pub fn update_order(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".opd_er_order SET order_date=NOW(),order_time=NOW(),order_doctor=?",TABLE_UPDATE_SET,
        " WHERE order_id=?;"
    ].concat()
}

// UPDATE kphis.opd_er_order_item SET nurse_assign=?, update_user=?, update_datetime=NOW(), version=(version+1) WHERE order_item_id=?;
/// nurse_assing, loginname, order_item_id
pub fn update_order_item_nurse_assign(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".opd_er_order_item SET nurse_assign=?",TABLE_UPDATE_SET," WHERE order_item_id=?;"
    ].concat()
}

// UPDATE kphis.opd_er_order_item SET order_item_type=?, update_user=?, update_datetime=NOW(), version=(version+1) WHERE order_item_id=?;
/// order_item_type, loginname, order_item_id
pub fn update_order_item_type(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".opd_er_order_item SET order_item_type=?",TABLE_UPDATE_SET," WHERE order_item_id=?;"
    ].concat()
}

// UPDATE kphis.opd_er_order_item SET due_doctor=?, due_doctor_note=?, update_user=?, update_datetime=NOW(), version=(version+1) WHERE order_item_id=?;
/// due_doctor, due_doctor_note, loginname, order_item_id
pub fn update_order_item_due_doctor(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".opd_er_order_item SET due_doctor=?,due_doctor_note=?",TABLE_UPDATE_SET," WHERE order_item_id=?;"
    ].concat()
}

// UPDATE kphis.opd_er_order_item SET due_pharm=?, due_pharm_note=?, update_user=?, update_datetime=NOW(), version=(version+1) WHERE order_item_id=?;
/// due_pharm, due_pharm_note, loginname, order_item_id
pub fn update_order_item_due_pharm(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".opd_er_order_item SET due_pharm=?,due_pharm_note=?",TABLE_UPDATE_SET," WHERE order_item_id=?;"
    ].concat()
}

// INSERT INTO kphis.opd_er_order (opd_er_order_master_id,order_date,order_time,order_doctor,order_type,order_owner_type,order_confirm,create_user,create_datetime,update_user,update_datetime,version)
// VALUES (?,DATE(NOW()),TIME(NOW()),?,?,?,'N',?,NOW(),?,NOW(),1);
/// opd_er_order_master_id, order_doctor, order_type, order_owner_type, loginname, loginname
pub fn insert_order(kphis: &str) -> String {
    [
        "INSERT INTO ",kphis,".opd_er_order (opd_er_order_master_id,order_date,order_time,order_doctor,order_type,order_owner_type,order_confirm",TABLE_CREATE_COLUMNS,") \
        VALUES (?,DATE(NOW()),TIME(NOW()),?,?,?,'N'",TABLE_CREATE_PREPARED,");"
    ].concat()
}

// INSERT INTO kphis.opd_er_order_item (order_id,opd_er_order_master_id,order_item_type,order_item_detail,stat,off_order_item_id,icode,med_reconciliation_item_id,first_qty,qty,create_user,create_datetime,update_user,update_datetime,version)
// VALUES (?,?,?,?,?,?,?,?,?,?,?,NOW(),?,NOW(),1);
/// order_id, opd_er_order_master_id, order_item_type, order_item_detail, stat, off_order_item_id, icode, med_reconciliation_item_id, first_qty, qty, loginname, loginname
pub fn insert_order_items(item_len: usize, kphis: &str) -> String {
    let values = vec!["(?,?,?,?,?,?,?,?,?,?,?,NOW(),?,NOW(),1)";item_len].join(",");
    [
        "INSERT INTO ",kphis,".opd_er_order_item (order_id,opd_er_order_master_id,order_item_type,order_item_detail,stat,off_order_item_id,icode,med_reconciliation_item_id,first_qty,qty",TABLE_CREATE_COLUMNS,") \
        VALUES ", &values,";"
    ].concat()
}

// // opd-er-order-one-day-confirm.php, opd-er-order-continuous-confirm.php
// --1 SELECT order_confirm FROM kphis.opd_er_order WHERE order_id = ?;
// --2 UPDATE kphis.opd_er_order SET order_confirm ='Y', order_date = NOW(), order_time = NOW(), order_doctor = ?, update_user = ?, update_datetime = NOW(), version = (version+1) WHERE order_id = ? AND order_confirm != 'Y';
// --we change two sqls into one
// UPDATE kphis.opd_er_order SET order_confirm ='Y', order_date = NOW(), order_time = NOW(), order_doctor = ?, update_user = ?, update_datetime = NOW(), version = (version+1) WHERE order_id = ? AND order_confirm != 'Y';
/// (order_time), doctor_code, loginname, order_id
pub fn update_confirm_order(is_fixed_time: bool, kphis: &str) -> String {
    let order_time = if is_fixed_time {"?"} else {"NOW()"};
    [
        "UPDATE ",kphis,".opd_er_order SET order_confirm ='Y',order_date=NOW(),order_time=",order_time,",order_doctor=?",TABLE_UPDATE_SET,
        " WHERE order_id=? AND order_confirm !='Y';"
    ].concat()
}

// UPDATE kphis.opd_er_order SET nurse_order_as = ?, order_confirm ='Y', order_date = NOW(), order_time = NOW(), order_doctor = ?, update_user = ?, update_datetime = NOW(), version = (version+1) WHERE order_id = ? AND order_confirm != 'Y';
/// (order_time), nurse_order_as, doctor_code, loginname, order_id
pub fn update_confirm_order_as(is_fixed_time: bool, kphis: &str) -> String {
    let order_time = if is_fixed_time {"?"} else {"NOW()"};
    [
        "UPDATE ",kphis,".opd_er_order SET order_date=NOW(), order_time=",order_time,", nurse_order_as=?, order_confirm='Y', order_doctor=?",TABLE_UPDATE_SET,
        " WHERE order_id=? AND order_confirm != 'Y';"
    ].concat()
}

// UPDATE kphis.opd_er_order SET nurse_order_as=?, update_user=?, update_datetime=NOW(), version=(version+1) WHERE order_id=? AND doctor_confirm_time IS NULL;
/// nurse_order_as, loginname, order_id
pub fn update_edit_order_as(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".opd_er_order SET nurse_order_as=?",TABLE_UPDATE_SET," WHERE order_id=? AND doctor_confirm_time IS NULL;"
    ].concat()
}

// UPDATE kphis.opd_er_order SET doctor_confirm_time=NOW(), update_user=?, update_datetime=NOW(), version=(version+1) WHERE doctor_confirm_time IS NULL AND nurse_order_as=? AND order_id=?;
/// (order_time), loginname, doctor_code, (order_id)
pub fn update_doctor_confirm_order(has_order_id: bool, is_fixed_time: bool, kphis: &str) -> String {
    let order_id = if has_order_id {" AND order_id=?"} else {""};
    let doctor_confirm_time = if is_fixed_time {"TIMESTAMP(CURDATE(),?)"} else {"NOW()"};
    [
        "UPDATE ",kphis,".opd_er_order SET doctor_confirm_time=",doctor_confirm_time,TABLE_UPDATE_SET,
        " WHERE doctor_confirm_time IS NULL AND nurse_order_as=?", order_id, ";"
    ].concat()
}

// // opd-er-order-one-day-nurse-accept.php
// --1 SELECT nurse_accept, nurse_accept_time FROM kphis.opd_er_order WHERE nurse_accept_time IS NULL AND order_id = ?;
// --2 UPDATE kphis.opd_er_order SET nurse_accept_time=NOW(), nurse_accept=?, update_user=?, update_datetime=NOW(), version=(version+1) WHERE order_id=?;
// --we change two sqls into one
// UPDATE kphis.opd_er_order SET nurse_accept_time=NOW(), nurse_accept=?, update_user=?, update_datetime=NOW(), version=(version+1) WHERE nurse_accept_time IS NULL AND order_id=?;
/// (order_time), doctor_code, loginname, order_id
pub fn update_nurse_accept_order(is_fixed_time: bool, kphis: &str) -> String {
    let nurse_accept_time = if is_fixed_time {"TIMESTAMP(CURDATE(),?)"} else {"NOW()"};
    [
        "UPDATE ",kphis,".opd_er_order SET nurse_accept_time=",nurse_accept_time,", nurse_accept=?",TABLE_UPDATE_SET,
        " WHERE nurse_accept_time IS NULL AND order_id=?;"
    ].concat()
}

// // opd-er-order-one-day-pharmacist-accept.php
// --1 SELECT pharmacist_accept, pharmacist_accept_time FROM kphis.opd_er_order WHERE pharmacist_accept_time IS NULL AND order_id = ?;
// --2 UPDATE kphis.opd_er_order SET pharmacist_accept_time=NOW(), pharmacist_accept=?, pharmacist_order_status='accepted', update_user=?, update_datetime=NOW(), version=(version+1) WHERE order_id=?;
// --we change two sqls into one
// UPDATE kphis.opd_er_order SET pharmacist_accept_time=NOW(), pharmacist_accept=?, pharmacist_order_status='accepted', update_user=?, update_datetime=NOW(), version=(version+1) WHERE pharmacist_accept_time IS NULL AND order_id=?;
/// (order_time), doctor_code, loginname, order_id
pub fn update_pharmacist_accept_order(is_fixed_time: bool, kphis: &str) -> String {
    let pharmacist_accept_time = if is_fixed_time {"TIMESTAMP(CURDATE(),?)"} else {"NOW()"};
    [
        "UPDATE ",kphis,".opd_er_order SET pharmacist_accept_time=",pharmacist_accept_time,", pharmacist_accept=?, pharmacist_order_status='accepted'",TABLE_UPDATE_SET,
        " WHERE pharmacist_accept_time IS NULL AND order_id=?;"
    ].concat()
}

// UPDATE kphis.opd_er_order SET pharmacist_check_time=NOW(), pharmacist_check=?, pharmacist_order_status='check', update_user=?, update_datetime=NOW(), version=(version+1) WHERE pharmacist_check_time IS NULL AND order_id=?;
/// (order_time), doctor_code, loginname, order_id
pub fn update_pharmacist_check_order(is_fixed_time: bool, kphis: &str) -> String {
    let pharmacist_check_time = if is_fixed_time {"TIMESTAMP(CURDATE(),?)"} else {"NOW()"};
    [
        "UPDATE ",kphis,".opd_er_order SET pharmacist_check_time=",pharmacist_check_time,", pharmacist_check=?, pharmacist_order_status='checked'",TABLE_UPDATE_SET,
        " WHERE pharmacist_check_time IS NULL AND order_id=?;"
    ].concat()
}

// // opd-er-order-one-day-pharmacist-done.php
// --1 SELECT pharmacist_done, pharmacist_done_time FROM kphis.opd_er_order WHERE pharmacist_done_time IS NULL AND order_id = ?;
// --2 UPDATE kphis.opd_er_order SET pharmacist_done_time=NOW(), pharmacist_accept=?, pharmacist_order_status='done', update_user=?, update_datetime=NOW(), version=(version+1) WHERE order_id=?;
// --we change two sqls into one
// UPDATE kphis.opd_er_order SET pharmacist_done_time=NOW(), pharmacist_done=?, pharmacist_order_status='done', update_user=?, update_datetime=NOW(), version=(version+1) WHERE pharmacist_done_time IS NULL AND order_id=?;
/// (order_time), doctor_code, loginname, order_id
pub fn update_pharmacist_done_order(is_fixed_time: bool, kphis: &str) -> String {
    let pharmacist_done_time = if is_fixed_time {"TIMESTAMP(CURDATE(),?)"} else {"NOW()"};
    [
        "UPDATE ",kphis,".opd_er_order SET pharmacist_done_time=",pharmacist_done_time,", pharmacist_done=?, pharmacist_order_status='done'",TABLE_UPDATE_SET,
        " WHERE pharmacist_done_time IS NULL AND order_id=?;"
    ].concat()
}

// // opd-er-order-one-day-delete.php, opd-er-order-continuous-delete.php
// DELETE o, i FROM kphis.opd_er_order AS o
//     LEFT JOIN kphis.opd_erorder_item AS i ON i.order_id = o.order_id
// WHERE o.order_id = ? AND o.order_confirm != 'Y';
// *** canNOT use alias IN delete `https://bugs.mysql.com/bug.php?id=82189` ***
/// order_id
pub fn delete_order(kphis: &str) -> String {
    [
        "DELETE ",kphis,".opd_er_order, ",kphis,".opd_er_order_item FROM ",kphis,".opd_er_order \
            LEFT JOIN ",kphis,".opd_er_order_item ON opd_er_order_item.order_id = opd_er_order.order_id \
        WHERE opd_er_order.order_id = ? AND opd_er_order.order_confirm != 'Y';"
    ].concat()
}

/// delete `order`, `order_item`, `index_plan` and `index_action`<br>
/// opd_er_order_master_id
pub fn delete_order_bundle(kphis: &str) -> String {
    [
        "DELETE ",kphis,".opd_er_order,",kphis,".opd_er_order_item,",kphis,".opd_er_nurse_index_plan,",kphis,".opd_er_nurse_index_action \
        FROM ",kphis,".opd_er_order \
            LEFT JOIN ",kphis,".opd_er_order_item ON opd_er_order_item.order_id=opd_er_order.order_id \
            LEFT JOIN ",kphis,".opd_er_nurse_index_plan ON opd_er_nurse_index_plan.order_item_id=opd_er_order_item.order_item_id \
            LEFT JOIN ",kphis,".opd_er_nurse_index_action ON opd_er_nurse_index_action.plan_id=opd_er_nurse_index_plan.plan_id \
        WHERE opd_er_order.opd_er_order_master_id=?;"
    ].concat()
}

// DELETE FROM kphis.opd_er_order_item WHERE order_id=?;
/// order_id
pub fn delete_order_item(kphis: &str) -> String {
    [
        "DELETE FROM ",kphis,".opd_er_order_item WHERE order_id=?;"
    ].concat()
}

// // opd-er-pharmacy-order-monitor-table.php
// // we change
// IF((SUM(IF(opd_er_order.pharmacist_order_status IS NULL,1,0)) > 0) AND ((SELECT COUNT(*) FROM kphis.opd_er_order_item oi JOIN kphis.opd_er_order o ON oi.order_id=o.order_id WHERE o.opd_er_order_master_id=opd_er_order_master.opd_er_order_master_id AND o.order_confirm='Y' AND o.pharmacist_order_status IS NULL AND oi.stat='Y') > 0),'ยังไม่ได้รับ - มี Stat Order',
//     IF((SUM(IF(opd_er_order.pharmacist_order_status IS NULL,1,0)) > 0) AND ((SELECT COUNT(*) FROM kphis.opd_er_order_item oi JOIN kphis.opd_er_order o ON oi.order_id=o.order_id WHERE o.opd_er_order_master_id=opd_er_order_master.opd_er_order_master_id AND o.order_confirm='Y' AND o.pharmacist_order_status IS NULL AND oi.order_item_type='home-medication') > 0),'ยังไม่ได้รับ - มี Home-Med Order',
//     IF(SUM(IF(opd_er_order.pharmacist_order_status IS NULL,1,0)) > 0,'ยังไม่ได้รับ',
//     IF((SUM(IF(opd_er_order.pharmacist_order_status='accepted',1,0)) > 0) AND ((SELECT COUNT(*) FROM kphis.opd_er_order_item oi JOIN kphis.opd_er_order o ON oi.order_id=o.order_id WHERE o.opd_er_order_master_id=opd_er_order_master.opd_er_order_master_id AND o.order_confirm='Y' AND o.pharmacist_order_status='accepted' AND oi.stat='Y') > 0),'รับแล้ว - มี Stat Order',
//     IF((SUM(IF(opd_er_order.pharmacist_order_status='accepted',1,0)) > 0) AND ((SELECT COUNT(*) FROM kphis.opd_er_order_item oi JOIN kphis.opd_er_order o ON oi.order_id=o.order_id WHERE o.opd_er_order_master_id=opd_er_order_master.opd_er_order_master_id AND o.order_confirm='Y' AND o.pharmacist_order_status='accepted' AND oi.order_item_type='home-medication') > 0),'รับแล้ว - มี Home-Med Order',
//     IF(SUM(IF(opd_er_order.pharmacist_order_status='accepted',1,0)) > 0,'รับแล้ว',
//     IF((SUM(IF(opd_er_order.pharmacist_order_status='done',1,0)) > 0) AND ((SELECT COUNT(*) FROM kphis.opd_er_order_item oi JOIN kphis.opd_er_order o ON oi.order_id=o.order_id WHERE o.opd_er_order_master_id=opd_er_order_master.opd_er_order_master_id AND o.order_confirm='Y' AND o.pharmacist_order_status='accepted' AND oi.stat='Y') > 0),'ตรวจสอบแล้ว - มี Stat Order',
//     IF((SUM(IF(opd_er_order.pharmacist_order_status='done',1,0)) > 0) AND ((SELECT COUNT(*) FROM kphis.opd_er_order_item oi JOIN kphis.opd_er_order o ON oi.order_id=o.order_id WHERE o.opd_er_order_master_id=opd_er_order_master.opd_er_order_master_id AND o.order_confirm='Y' AND o.pharmacist_order_status='accepted' AND oi.order_item_type='home-medication') > 0),'ตรวจสอบแล้ว - มี Home-Med Order',
//     IF(SUM(IF(opd_er_order.pharmacist_order_status='done',1,0)) > 0,'ตรวจสอบแล้ว', NULL))))))))) AS order_priority_text,
// IF((SUM(IF(opd_er_order.pharmacist_order_status IS NULL,1,0)) > 0) AND ((SELECT COUNT(*) FROM kphis.opd_er_order_item oi JOIN kphis.opd_er_order o ON oi.order_id=o.order_id WHERE o.opd_er_order_master_id=opd_er_order_master.opd_er_order_master_id AND o.order_confirm='Y' AND o.pharmacist_order_status IS NULL AND oi.stat='Y') > 0),1,
//     IF((SUM(IF(opd_er_order.pharmacist_order_status IS NULL,1,0)) > 0) AND ((SELECT COUNT(*) FROM kphis.opd_er_order_item oi JOIN kphis.opd_er_order o ON oi.order_id=o.order_id WHERE o.opd_er_order_master_id=opd_er_order_master.opd_er_order_master_id AND o.order_confirm='Y' AND o.pharmacist_order_status IS NULL AND oi.order_item_type='home-medication') > 0),2,
//     IF(SUM(IF(opd_er_order.pharmacist_order_status IS NULL,1,0)) > 0,3,
//     IF((SUM(IF(opd_er_order.pharmacist_order_status='accepted',1,0)) > 0) AND ((SELECT COUNT(*) FROM kphis.opd_er_order_item oi JOIN kphis.opd_er_order o ON oi.order_id=o.order_id WHERE o.opd_er_order_master_id=opd_er_order_master.opd_er_order_master_id AND o.order_confirm='Y' AND o.pharmacist_order_status='accepted' AND oi.stat='Y') > 0),4,
//     IF((SUM(IF(opd_er_order.pharmacist_order_status='accepted',1,0)) > 0) AND ((SELECT COUNT(*) FROM kphis.opd_er_order_item oi JOIN kphis.opd_er_order o ON oi.order_id=o.order_id WHERE o.opd_er_order_master_id=opd_er_order_master.opd_er_order_master_id AND o.order_confirm='Y' AND o.pharmacist_order_status='accepted' AND oi.order_item_type='home-medication') > 0),5,
//     IF(SUM(IF(opd_er_order.pharmacist_order_status='accepted',1,0)) > 0,6,
//     IF((SUM(IF(opd_er_order.pharmacist_order_status='done',1,0)) > 0) AND ((SELECT COUNT(*) FROM kphis.opd_er_order_item oi JOIN kphis.opd_er_order o ON oi.order_id=o.order_id WHERE o.opd_er_order_master_id=opd_er_order_master.opd_er_order_master_id AND o.order_confirm='Y' AND o.pharmacist_order_status='accepted' AND oi.stat='Y') > 0),7,
//     IF((SUM(IF(opd_er_order.pharmacist_order_status='done',1,0)) > 0) AND ((SELECT COUNT(*) FROM kphis.opd_er_order_item oi JOIN kphis.opd_er_order o ON oi.order_id=o.order_id WHERE o.opd_er_order_master_id=opd_er_order_master.opd_er_order_master_id AND o.order_confirm='Y' AND o.pharmacist_order_status='accepted' AND oi.order_item_type='home-medication') > 0),8,
//     IF(SUM(IF(opd_er_order.pharmacist_order_status='done',1,0)) > 0,9,10))))))))) AS order_priority,
// // to count_item_not_accept_stat, count_item_not_accept_homemed, count_item_accept_stat, count_item_accept_homemed for calculate client side
// SELECT IF(SUM(IF((SELECT COUNT(*) FROM kphis.opd_er_order_item oi WHERE oi.order_id=opd_er_order.order_id AND oi.stat='Y') > 0,1,0)) > 0,'Y','N') AS has_stat,
//     SUM(IF((SELECT COUNT(*) FROM kphis.opd_er_order_item oi WHERE oi.order_id=opd_er_order.order_id AND oi.stat='Y') > 0,1,0)) AS count_stat,
//     SUM(IF(opd_er_order.pharmacist_order_status IS NULL,1,0)) AS count_not_accept,
//     SUM(IF(opd_er_order.pharmacist_order_status='accepted',1,0)) AS count_accept,
//     SUM(IF(opd_er_order.pharmacist_order_status='done',1,0)) AS count_done,
//     (SELECT COUNT(*) FROM kphis.opd_er_order_item oi JOIN kphis.opd_er_order o ON oi.order_id=o.order_id WHERE o.opd_er_order_master_id=om.opd_er_order_master_id AND o.order_confirm='Y' AND o.pharmacist_order_status IS NULL AND oi.stat='Y') AS count_item_not_accept_stat,
//     (SELECT COUNT(*) FROM kphis.opd_er_order_item oi JOIN kphis.opd_er_order o ON oi.order_id=o.order_id WHERE o.opd_er_order_master_id=om.opd_er_order_master_id AND o.order_confirm='Y' AND o.pharmacist_order_status IS NULL AND oi.order_item_type='home-medication') AS count_item_not_accept_homemed,
//     (SELECT COUNT(*) FROM kphis.opd_er_order_item oi JOIN kphis.opd_er_order o ON oi.order_id=o.order_id WHERE o.opd_er_order_master_id=om.opd_er_order_master_id AND o.order_confirm='Y' AND o.pharmacist_order_status='accepted' AND oi.stat='Y') AS count_item_accept_stat,
//     (SELECT COUNT(*) FROM kphis.opd_er_order_item oi JOIN kphis.opd_er_order o ON oi.order_id=o.order_id WHERE o.opd_er_order_master_id=om.opd_er_order_master_id AND o.order_confirm='Y' AND o.pharmacist_order_status='accepted' AND oi.order_item_type='home-medication') AS count_item_accept_homemed,
//     om.opd_er_order_master_id,om.order_date,om.order_time,CONCAT(om.order_date,' ',om.order_time) AS order_date_time,
//     om.note,ovst.hn,ovst.vn,opd_er_bed.bedno,CONCAT(patient.pname,patient.fname,' ',patient.lname) AS fullname,aa.age_y,aa.age_m,aa.age_d,
//     IF(SUM(IF(opd_er_order.pharmacist_order_status IS NULL,1,0)) > 0,MIN(IF(opd_er_order.pharmacist_order_status IS NULL,CONCAT(opd_er_order.order_date,' ',opd_er_order.order_time),NULL)),
//         IF(SUM(IF(opd_er_order.pharmacist_order_status='accepted',1,0)) > 0,MIN(IF(opd_er_order.pharmacist_order_status='accepted',CONCAT(opd_er_order.order_date,' ',opd_er_order.order_time),NULL)),
//         MIN(CONCAT(opd_er_order.order_date,' ',opd_er_order.order_time)))) AS min_order_date_time,
//     IF(SUM(IF(opd_er_order.pharmacist_order_status IS NULL,1,0)) > 0,MAX(IF(opd_er_order.pharmacist_order_status IS NULL,CONCAT(opd_er_order.order_date,' ',opd_er_order.order_time),NULL)),
//         IF(SUM(IF(opd_er_order.pharmacist_order_status = 'accepted',1,0)) > 0,MAX(IF(opd_er_order.pharmacist_order_status = 'accepted',CONCAT(opd_er_order.order_date,' ',opd_er_order.order_time),NULL)),
//         MAX(CONCAT(opd_er_order.order_date,' ',opd_er_order.order_time)))) AS max_order_date_time,
//     IF(SUM(IF((SELECT COUNT(*) FROM kphis.opd_er_order_item oi WHERE oi.order_id=opd_er_order.order_id AND (opd_er_order.pharmacist_order_status IS NULL OR opd_er_order.pharmacist_order_status <> 'done')
//         AND (oi.order_item_type IN ('med') OR (opd_er_order.order_owner_type='nurse' AND oi.order_item_type='other'))) > 0,1,0)) > 0,'Y','N') AS containing_med_order_item
// FROM kphis.opd_er_order
//     INNER JOIN kphis.opd_er_order_master om ON opd_er_order.opd_er_order_master_id=om.opd_er_order_master_id
//     LEFT JOIN kphis.opd_er_bed ON om.bedno=opd_er_bed.opd_er_bed_id
//     LEFT JOIN hos.ovst ON om.vn=ovst.vn
//     LEFT JOIN hos.patient ON patient.hn=ovst.hn
//     LEFT JOIN hos.vn_stat aa ON aa.vn=ovst.vn
// WHERE opd_er_order.order_confirm='Y'
//
// // 2025-09-03: only show 'ivfluid','injection','home-medication','med', can hide/show discharged item
// SELECT (SELECT COUNT(*) FROM kphis.opd_er_order_item oi JOIN kphis.opd_er_order o ON oi.order_id=o.order_id WHERE o.order_confirm='Y' AND o.opd_er_order_master_id=om.opd_er_order_master_id AND oi.order_item_type IN ('ivfluid','injection','home-medication','med') AND oi.icode IS NOT NULL AND oi.stat='Y') AS count_stat,
//     (SELECT COUNT(*) FROM kphis.opd_er_order_item oi JOIN kphis.opd_er_order o ON oi.order_id=o.order_id WHERE o.order_confirm='Y' AND o.opd_er_order_master_id=om.opd_er_order_master_id AND oi.order_item_type IN ('ivfluid','injection','home-medication','med') AND oi.icode IS NOT NULL AND o.pharmacist_order_status IS NULL) AS count_not_accept,
//     (SELECT COUNT(*) FROM kphis.opd_er_order_item oi JOIN kphis.opd_er_order o ON oi.order_id=o.order_id WHERE o.order_confirm='Y' AND o.opd_er_order_master_id=om.opd_er_order_master_id AND oi.order_item_type IN ('ivfluid','injection','home-medication','med') AND oi.icode IS NOT NULL AND o.pharmacist_order_status='accepted') AS count_accept,
//     (SELECT COUNT(*) FROM kphis.opd_er_order_item oi JOIN kphis.opd_er_order o ON oi.order_id=o.order_id WHERE o.order_confirm='Y' AND o.opd_er_order_master_id=om.opd_er_order_master_id AND oi.order_item_type IN ('ivfluid','injection','home-medication','med') AND oi.icode IS NOT NULL AND o.pharmacist_order_status='checked') AS count_check,
//     (SELECT COUNT(*) FROM kphis.opd_er_order_item oi JOIN kphis.opd_er_order o ON oi.order_id=o.order_id WHERE o.order_confirm='Y' AND o.opd_er_order_master_id=om.opd_er_order_master_id AND oi.order_item_type IN ('ivfluid','injection','home-medication','med') AND oi.icode IS NOT NULL AND o.pharmacist_order_status='done') AS count_done,
//     (SELECT COUNT(*) FROM kphis.opd_er_order_item oi JOIN kphis.opd_er_order o ON oi.order_id=o.order_id WHERE o.order_confirm='Y' AND o.opd_er_order_master_id=om.opd_er_order_master_id AND oi.order_item_type='pharm') AS count_pharm_notify,
//     (SELECT COUNT(*) FROM kphis.opd_er_order_item oi JOIN kphis.opd_er_order o ON oi.order_id=o.order_id WHERE o.order_confirm='Y' AND o.opd_er_order_master_id=om.opd_er_order_master_id AND oi.order_item_type IN ('ivfluid','injection','home-medication','med') AND oi.icode IS NOT NULL AND o.pharmacist_order_status IS NULL AND oi.stat='Y') AS count_item_not_accept_stat,
//     (SELECT COUNT(*) FROM kphis.opd_er_order_item oi JOIN kphis.opd_er_order o ON oi.order_id=o.order_id WHERE o.order_confirm='Y' AND o.opd_er_order_master_id=om.opd_er_order_master_id AND oi.order_item_type='home-medication' AND o.pharmacist_order_status IS NULL) AS count_item_not_accept_homemed,
//     (SELECT COUNT(*) FROM kphis.opd_er_order_item oi JOIN kphis.opd_er_order o ON oi.order_id=o.order_id WHERE o.order_confirm='Y' AND o.opd_er_order_master_id=om.opd_er_order_master_id AND oi.order_item_type IN ('ivfluid','injection','home-medication','med') AND oi.icode IS NOT NULL AND o.pharmacist_order_status='accepted' AND oi.stat='Y') AS count_item_accept_stat,
//     (SELECT COUNT(*) FROM kphis.opd_er_order_item oi JOIN kphis.opd_er_order o ON oi.order_id=o.order_id WHERE o.order_confirm='Y' AND o.opd_er_order_master_id=om.opd_er_order_master_id AND oi.order_item_type='home-medication' AND o.pharmacist_order_status='accepted') AS count_item_accept_homemed,
//     om.opd_er_order_master_id,om.order_date,om.order_time,CONCAT(om.order_date,' ',om.order_time) AS order_date_time,
//     om.note,ovst.hn,ovst.vn,opd_er_bed.bedno,CONCAT(patient.pname,patient.fname,' ',patient.lname) AS fullname,aa.age_y,aa.age_m,aa.age_d,
//     (SELECT MIN(CONCAT(o.order_date,' ',o.order_time)) FROM kphis.opd_er_order_item oi JOIN kphis.opd_er_order o ON oi.order_id=o.order_id WHERE o.opd_er_order_master_id=om.opd_er_order_master_id
//         AND oi.order_item_type IN ('ivfluid','injection','home-medication','med','pharm') AND oi.icode IS NOT NULL
//         AND o.pharmacist_order_status IS NULL) AS min_order_date_time,
//     (SELECT MAX(CONCAT(o.order_date,' ',o.order_time)) FROM kphis.opd_er_order_item oi JOIN kphis.opd_er_order o ON oi.order_id=o.order_id WHERE o.opd_er_order_master_id=om.opd_er_order_master_id 
//         AND oi.order_item_type IN ('ivfluid','injection','home-medication','med','pharm') AND oi.icode IS NOT NULL 
//         AND o.pharmacist_order_status IS NULL) AS max_order_date_time 
// FROM kphis.opd_er_order_master om
//     LEFT JOIN kphis.opd_er_order ON opd_er_order.opd_er_order_master_id=om.opd_er_order_master_id 
//     LEFT JOIN kphis.opd_er_bed ON om.bedno=opd_er_bed.opd_er_bed_id 
//     LEFT JOIN hos.ovst ON om.vn=ovst.vn 
//     LEFT JOIN hos.patient ON patient.hn=ovst.hn 
//     LEFT JOIN hos.vn_stat aa ON aa.vn=ovst.vn 
// WHERE opd_er_order.order_confirm='Y' GROUP BY om.opd_er_order_master_id
// ORDER BY max_order_date_time DESC,om.order_date DESC,om.order_time DESC,opd_er_bed.display_order,opd_er_bed.bedno
// LIMIT 100;
pub fn select_pharmacy_order(
    params: &OpdErOrderPharmacyParams,
    hlen: usize,
    vlen: usize,
    hosxp: &str,
    kphis: &str,
) -> String {
    let patient = and_opd_patient(&params.patient, hlen, vlen, hosxp).unwrap_or_default();
    let order_date_from = if params.order_date_from.is_some() {" AND opd_er_order.order_date >= ? "} else {""};
    let order_date_to = if params.order_date_to.is_some() {" AND opd_er_order.order_date <= ? "} else {""};
    let is_discharged = match params.is_discharged.clone().unwrap_or_default().as_str() {
        "Y" => " AND om.er_patient_status_id IN (7,10) ",
        "N" => " AND om.er_patient_status_id NOT IN (7,10) ",
        _ => "",
    };

    [
        "SELECT (SELECT COUNT(*) FROM ",kphis,".opd_er_order_item oi JOIN ",kphis,".opd_er_order o ON oi.order_id=o.order_id WHERE o.order_confirm='Y' AND o.opd_er_order_master_id=om.opd_er_order_master_id AND oi.order_item_type IN ('ivfluid','injection','home-medication','med') AND oi.icode IS NOT NULL AND oi.stat='Y') AS count_stat,\
            (SELECT COUNT(*) FROM ",kphis,".opd_er_order_item oi JOIN ",kphis,".opd_er_order o ON oi.order_id=o.order_id WHERE o.order_confirm='Y' AND o.opd_er_order_master_id=om.opd_er_order_master_id AND oi.order_item_type IN ('ivfluid','injection','home-medication','med') AND oi.icode IS NOT NULL AND o.pharmacist_order_status IS NULL) AS count_not_accept,\
            (SELECT COUNT(*) FROM ",kphis,".opd_er_order_item oi JOIN ",kphis,".opd_er_order o ON oi.order_id=o.order_id WHERE o.order_confirm='Y' AND o.opd_er_order_master_id=om.opd_er_order_master_id AND oi.order_item_type IN ('ivfluid','injection','home-medication','med') AND oi.icode IS NOT NULL AND o.pharmacist_order_status='accepted') AS count_accept,\
            (SELECT COUNT(*) FROM ",kphis,".opd_er_order_item oi JOIN ",kphis,".opd_er_order o ON oi.order_id=o.order_id WHERE o.order_confirm='Y' AND o.opd_er_order_master_id=om.opd_er_order_master_id AND oi.order_item_type IN ('ivfluid','injection','home-medication','med') AND oi.icode IS NOT NULL AND o.pharmacist_order_status='checked') AS count_check,\
            (SELECT COUNT(*) FROM ",kphis,".opd_er_order_item oi JOIN ",kphis,".opd_er_order o ON oi.order_id=o.order_id WHERE o.order_confirm='Y' AND o.opd_er_order_master_id=om.opd_er_order_master_id AND oi.order_item_type IN ('ivfluid','injection','home-medication','med') AND oi.icode IS NOT NULL AND o.pharmacist_order_status='done') AS count_done,\
            (SELECT COUNT(*) FROM ",kphis,".opd_er_order_item oi JOIN ",kphis,".opd_er_order o ON oi.order_id=o.order_id WHERE o.order_confirm='Y' AND o.opd_er_order_master_id=om.opd_er_order_master_id AND oi.order_item_type='pharm') AS count_pharm_notify,\
            (SELECT COUNT(*) FROM ",kphis,".opd_er_order_item oi JOIN ",kphis,".opd_er_order o ON oi.order_id=o.order_id WHERE o.order_confirm='Y' AND o.opd_er_order_master_id=om.opd_er_order_master_id AND oi.order_item_type IN ('ivfluid','injection','home-medication','med') AND oi.icode IS NOT NULL AND o.pharmacist_order_status IS NULL AND oi.stat='Y') AS count_item_not_accept_stat,\
            (SELECT COUNT(*) FROM ",kphis,".opd_er_order_item oi JOIN ",kphis,".opd_er_order o ON oi.order_id=o.order_id WHERE o.order_confirm='Y' AND o.opd_er_order_master_id=om.opd_er_order_master_id AND oi.order_item_type='home-medication' AND o.pharmacist_order_status IS NULL) AS count_item_not_accept_homemed,\
            (SELECT COUNT(*) FROM ",kphis,".opd_er_order_item oi JOIN ",kphis,".opd_er_order o ON oi.order_id=o.order_id WHERE o.order_confirm='Y' AND o.opd_er_order_master_id=om.opd_er_order_master_id AND oi.order_item_type IN ('ivfluid','injection','home-medication','med') AND oi.icode IS NOT NULL AND o.pharmacist_order_status='accepted' AND oi.stat='Y') AS count_item_accept_stat,\
            (SELECT COUNT(*) FROM ",kphis,".opd_er_order_item oi JOIN ",kphis,".opd_er_order o ON oi.order_id=o.order_id WHERE o.order_confirm='Y' AND o.opd_er_order_master_id=om.opd_er_order_master_id AND oi.order_item_type='home-medication' AND o.pharmacist_order_status='accepted') AS count_item_accept_homemed,\
            om.opd_er_order_master_id,om.er_patient_status_id,om.order_date,om.order_time,ADDTIME(CONVERT(om.order_date,DATETIME),om.order_time) AS order_date_time,\
            om.note,ovst.hn,ovst.vn,opd_er_bed.bedno,CONCAT(patient.pname,patient.fname,' ',patient.lname) AS fullname,aa.age_y,aa.age_m,aa.age_d,\
            (SELECT MIN(ADDTIME(CONVERT(o.order_date,DATETIME),o.order_time)) FROM ",kphis,".opd_er_order_item oi JOIN ",kphis,".opd_er_order o ON oi.order_id=o.order_id WHERE o.opd_er_order_master_id=om.opd_er_order_master_id \
                AND oi.order_item_type IN ('ivfluid','injection','home-medication','med','pharm') AND oi.icode IS NOT NULL \
                AND o.pharmacist_order_status IS NULL) AS min_order_date_time,
            (SELECT MAX(ADDTIME(CONVERT(o.order_date,DATETIME),o.order_time)) FROM ",kphis,".opd_er_order_item oi JOIN ",kphis,".opd_er_order o ON oi.order_id=o.order_id WHERE o.opd_er_order_master_id=om.opd_er_order_master_id \
                AND oi.order_item_type IN ('ivfluid','injection','home-medication','med','pharm') AND oi.icode IS NOT NULL \
                AND o.pharmacist_order_status IS NULL) AS max_order_date_time \
        FROM ",kphis,".opd_er_order_master om \
            LEFT JOIN ",kphis,".opd_er_order ON opd_er_order.opd_er_order_master_id=om.opd_er_order_master_id \
            LEFT JOIN ",kphis,".opd_er_bed ON om.bedno=opd_er_bed.opd_er_bed_id \
            LEFT JOIN ",hosxp,".ovst ON om.vn=ovst.vn \
            LEFT JOIN ",hosxp,".patient ON patient.hn=ovst.hn \
            LEFT JOIN ",hosxp,".vn_stat aa ON aa.vn=ovst.vn \
        WHERE 1=1 ", is_discharged, &patient, order_date_from, order_date_to, " GROUP BY om.opd_er_order_master_id \
        ORDER BY max_order_date_time DESC,om.order_date DESC,om.order_time DESC,opd_er_bed.display_order,opd_er_bed.bedno \
        LIMIT 1;"
    ].concat()
}

// // SELECT MAX(CONCAT(opd_er_order.order_date,' ',opd_er_order.order_time)) AS new_last_order_time
// // FROM kphis.opd_er_order
// //     INNER JOIN kphis.opd_er_order_master ON opd_er_order.opd_er_order_master_id=opd_er_order_master.opd_er_order_master_id
// //     LEFT JOIN hos.ovst ON opd_er_order_master.vn=ovst.vn
// //     LEFT JOIN hos.patient ON patient.hn=ovst.hn
// // WHERE opd_er_order.order_confirm='Y' AND opd_er_order.pharmacist_order_status IS NULL;
// pub fn select_pharmacy_new_last_order_time(
//     params: &OpdErOrderPharmacyParams,
//     hlen: usize,
//     vlen: usize,
//     hosxp: &str,
//     kphis: &str,
// ) -> String {
//     let patient = and_opd_patient(&params.patient, hlen, vlen, hosxp).unwrap_or_default();
//     let order_date_from = if params.order_date_from.is_some() {" AND opd_er_order.order_date >= ? "} else {""}; // order_date_from
//     let order_date_to = if params.order_date_to.is_some() {" AND opd_er_order.order_date <= ? "} else {""}; // order_date_to

//     [
//         "SELECT MAX(CONCAT(opd_er_order.order_date,' ',opd_er_order.order_time)) AS new_last_order_time \
//         FROM ",kphis,".opd_er_order \
//             INNER JOIN ",kphis,".opd_er_order_master ON opd_er_order.opd_er_order_master_id=opd_er_order_master.opd_er_order_master_id \
//             LEFT JOIN ",hosxp,".ovst ON opd_er_order_master.vn=ovst.vn \
//             LEFT JOIN ",hosxp,".patient ON patient.hn=ovst.hn \
//         WHERE opd_er_order.order_confirm='Y' AND opd_er_order.pharmacist_order_status IS NULL ", &patient, order_date_from, order_date_to, ";"
//     ].concat()
// }

// // SELECT om.opd_er_order_master_id,om.order_date,om.order_time,CONCAT(om.order_date,' ',om.order_time) AS order_date_time,
// //     om.note,ovst.hn,ovst.vn,opd_er_bed.bedno,CONCAT(patient.pname,patient.fname,' ',patient.lname) AS fullname,aa.age_y,aa.age_m,aa.age_d,
// //     (SELECT MAX(vs_datetime) FROM kphis.opd_er_vs_vital_sign vs WHERE vs.opd_er_order_master_id=om.opd_er_order_master_id) AS max_vs_datetime,
// //     (SELECT MAX(CONCAT(fcnote_date,' ',fcnote_time)) FROM kphis.opd_er_focus_note fcnote WHERE fcnote.opd_er_order_master_id=om.opd_er_order_master_id) AS max_fcnote_datetime,
// //     (SELECT MAX(CONCAT(order_date,' ',order_time)) FROM kphis.opd_er_order WHERE opd_er_order.opd_er_order_master_id=om.opd_er_order_master_id AND opd_er_order.order_confirm = 'Y' AND opd_er_order.order_owner_type = 'doctor') AS max_order_datetime
// // FROM kphis.opd_er_order_master om
// //     LEFT JOIN kphis.opd_er_bed ON om.bedno=opd_er_bed.opd_er_bed_id
// //     LEFT JOIN hos.ovst ON om.vn=ovst.vn
// //     LEFT JOIN hos.patient ON patient.hn=ovst.hn
// //     LEFT JOIN hos.vn_stat aa ON aa.vn=ovst.vn
// // WHERE 1=1
// pub fn select_pharmacy_order_master(
//     params: &OpdErOrderPharmacyParams,
//     hlen: usize,
//     vlen: usize,
//     hosxp: &str,
//     kphis: &str,
// ) -> String {
//     let patient = and_opd_patient(&params.patient, hlen, vlen, hosxp);
//     let order_date_from = if params.order_date_from.is_some() {" AND opd_er_order.order_date >= ? "} else {""}; // order_date_from
//     let order_date_to = if params.order_date_to.is_some() {" AND opd_er_order.order_date <= ? "} else {""}; // order_date_to
//     let order_date = if params.order_date_from.is_some() || params.order_date_to.is_some() {
//         [" AND om.opd_er_order_master_id NOT IN (SELECT opd_er_order_master_id FROM ",kphis,".opd_er_order \
//             WHERE opd_er_order.opd_er_order_master_id=om.opd_er_order_master_id AND opd_er_order.order_confirm='Y' ",
//             order_date_from, order_date_to, ") "].concat()
//     } else {
//         String::new()
//     };
//     let group_order_by = if patient.is_some() {
//         " GROUP BY om.opd_er_order_master_id ORDER BY om.opd_er_order_master_id DESC LIMIT 100;"
//     } else {
//         " GROUP BY om.opd_er_order_master_id ORDER BY opd_er_bed.display_order,opd_er_bed.bedno,om.order_date,om.order_time;"
//     };

//     [
//         "SELECT om.opd_er_order_master_id,om.order_date,om.order_time,CONCAT(om.order_date,' ',om.order_time) AS order_date_time,\
//             om.note,ovst.hn,ovst.vn,opd_er_bed.bedno,CONCAT(patient.pname,patient.fname,' ',patient.lname) AS fullname,aa.age_y,aa.age_m,aa.age_d,\
//             (SELECT MAX(vs_datetime) FROM ",kphis,".opd_er_vs_vital_sign vs WHERE vs.opd_er_order_master_id=om.opd_er_order_master_id) AS max_vs_datetime,\
//             (SELECT MAX(CONCAT(fcnote_date,' ',fcnote_time)) FROM ",kphis,".opd_er_focus_note fcnote WHERE fcnote.opd_er_order_master_id=om.opd_er_order_master_id) AS max_fcnote_datetime,\
//             (SELECT MAX(CONCAT(order_date,' ',order_time)) FROM ",kphis,".opd_er_order WHERE opd_er_order.opd_er_order_master_id=om.opd_er_order_master_id AND opd_er_order.order_confirm = 'Y' AND opd_er_order.order_owner_type = 'doctor') AS max_order_datetime \
//         FROM ",kphis,".opd_er_order_master om \
//             LEFT JOIN ",kphis,".opd_er_bed ON om.bedno=opd_er_bed.opd_er_bed_id \
//             LEFT JOIN ",hosxp,".ovst ON om.vn=ovst.vn \
//             LEFT JOIN ",hosxp,".patient ON patient.hn=ovst.hn \
//             LEFT JOIN ",hosxp,".vn_stat aa ON aa.vn=ovst.vn \
//         WHERE 1=1 ", &patient.clone().unwrap_or(" AND om.er_patient_status_id <> 7 ".to_owned()), &order_date, group_order_by
//     ].concat()
// }