use super::opd_er::or_opd_patient_or_qn;
use time::Date;

use crate::{TABLE_CREATE_COLUMNS, TABLE_CREATE_PREPARED, TABLE_UPDATE_SET};

// // pharmacy-prescription-screen-utils.php
// SELECT ovst.vstdate,ovst.oqueue,ovst.hn,ovst.vn,p.cid,RTRIM(CONCAT(patient.pname,patient.fname,' ',patient.lname)) AS fullname,vn_stat.age_y,
//    CONCAT_WS(' ',p.addrpart,'หมู่',p.moopart,tambol.full_address_name) AS homeaddr,p.hometel
// FROM hos.ovst
//     LEFT JOIN hos.patient ON patient.hn=ovst.hn
//     LEFT JOIN hos.tambol ON tambol.tambol_code=CONCAT(p.chwpart,p.amppart,p.tmbpart) \
//     LEFT JOIN hos.vn_stat ON vn_stat.vn=ovst.vn
// WHERE 1=0
pub fn select_info_qn(
    search_text: &Option<String>,
    hlen: usize,
    vlen: usize,
    hosxp: &str,
) -> String {
    let patient = or_opd_patient_or_qn(search_text, hlen, vlen, hosxp).unwrap_or_default();
    [
        "SELECT ovst.vstdate,ovst.oqueue,ovst.hn,ovst.vn,p.cid,RTRIM(CONCAT(p.pname,p.fname,' ',p.lname)) AS fullname,\
            vn_stat.age_y,vn_stat.age_m,vn_stat.age_d,sex.`name` AS sex_name,CONCAT_WS(' ',p.addrpart,'หมู่',p.moopart,tambol.full_address_name) AS homeaddr,p.hometel \
        FROM ",hosxp,".ovst \
            LEFT JOIN ",hosxp,".patient p ON p.hn=ovst.hn \
            LEFT JOIN ",hosxp,".tambol ON tambol.tambol_code=CONCAT(p.chwpart,p.amppart,p.tmbpart) \
            LEFT JOIN ",hosxp,".vn_stat ON vn_stat.vn=ovst.vn \
            LEFT JOIN ",hosxp,".sex ON sex.code=p.sex \
        WHERE 1=0 ", &patient, "ORDER BY ovst.vn DESC LIMIT 1;"
    ].concat()
}

// SELECT ovst.vn,ovst.vstdate,ovst.vsttime,ovst.an
// FROM hos.ovst WHERE hn=? ORDER BY ovst.vstdate DESC,ovst.vsttime DESC;
/// hn
pub fn select_info_date(hosxp: &str) -> String {
    [
        "SELECT ovst.vn,ovst.vstdate,ovst.vsttime,ovst.an \
        FROM ",hosxp,".ovst WHERE hn=? ORDER BY ovst.vstdate DESC,ovst.vsttime DESC;"
    ].concat()
}

// // note_FROM_hosxp()
// SELECT GROUP_CONCAT(CONCAT(pt.ptnote_id,'|',pt.note_datetime,'|',pt.plain_text) SEPARATOR '\n') AS plain
// FROM hos.ptnote pt WHERE hn=? ORDER BY hn ASC LIMIT 100;
// // we not concat it
// SELECT ptnote_id,note_datetime,plain_text
// FROM hos.ptnote WHERE hn=? ORDER BY hn ASC LIMIT 100;
/// hn
pub fn select_note_from_hosxp(hosxp: &str) -> String {
    [
        "SELECT ptnote_id,note_datetime,plain_text FROM ",hosxp,".ptnote WHERE hn=? ORDER BY hn ASC LIMIT 100;"
    ].concat()
}

// // drug_allergy()
// SELECT CONCAT(agent,'=',IF(symptom IS NULL,'',symptom ),IF(note IS NULL,'',CONCAT(' [',note,']'))) AS drugallergy
// FROM hos.opd_allergy WHERE hn IN (?) ORDER BY display_order LIMIT 100;
// // fix to remove '=' result
// SELECT CONCAT(IF(agent >'',CONCAT(agent,'=',IF(symptom IS NULL,'',symptom),IF(note IS NULL,'',CONCAT(' [',note,']'))),NULL)) AS drugallergy
// FROM hos.opd_allergy WHERE hn=? ORDER BY display_order LIMIT 100;
/// hn
pub fn select_drug_allergy(hosxp: &str) -> String {
    [
        "SELECT CONCAT(IF(agent >'',CONCAT(agent,'=',IF(symptom IS NULL,'',symptom),IF(note IS NULL,'',CONCAT(' [',note,']'))),NULL)) AS drugallergy \
        FROM ",hosxp,".opd_allergy WHERE hn=? ORDER BY display_order LIMIT 100;"
    ].concat()
}

/// is_last using hn, else using vn
pub fn select_labs(lab_codes: &[(String, Vec<u64>)], is_last: bool, hosxp: &str) -> String {
    lab_codes.iter().map(|(lab, codes)| select_lab_item(lab, codes, is_last, hosxp)).collect::<Vec<String>>().join(" UNION ALL ")
}

// (SELECT 'INR' AS lab_name,lo.lab_order_result,lh.order_date,lh.hn,lh.vn
//     FROM hos.lab_head lh INNER JOIN hos.lab_order lo ON lh.lab_order_number=lo.lab_order_number AND lo.lab_items_code IN ( 458 )
//     WHERE lo.confirm='Y' AND lh.vn=? ORDER BY lh.lab_order_number DESC LIMIT 1)
// (SELECT 'INR_last' AS lab_name,lo.lab_order_result,lh.order_date,lh.hn,lh.vn
//     FROM hos.lab_head lh INNER JOIN hos.lab_order lo ON lh.lab_order_number=lo.lab_order_number AND lo.lab_items_code IN ( 458 )
//     WHERE lo.confirm='Y' AND lh.hn=? ORDER BY lh.lab_order_number DESC LIMIT 1)
// // Add data for LabHistory
// SELECT 'lab' AS lab_name,lo.lab_order_result,lh.order_date,lh.hn,lh.vn,lo.lab_items_code,lo.lab_items_name_ref,lo.lab_order_number,li.lab_items_unit,lab_items_normal_value
// FROM hos.lab_head lh 
// 	INNER JOIN hos.lab_order lo ON lh.lab_order_number=lo.lab_order_number AND lo.lab_items_code IN (77)
// 	INNER JOIN hos.lab_items li ON lo.lab_items_code=li.lab_items_code
// WHERE lo.confirm='Y' AND lh.hn='0074318' ORDER BY lh.lab_order_number DESC LIMIT 1;
fn select_lab_item(lab: &str, codes: &[u64], is_last: bool, hosxp: &str) -> String {
    [
        "(SELECT '",lab,"' AS lab_name,lo.lab_order_result,lh.order_date,lh.hn,lh.vn,lo.lab_items_code,lo.lab_items_name_ref,lo.lab_order_number,li.lab_items_unit,lab_items_normal_value \
        FROM ",hosxp,".lab_head lh 
            INNER JOIN ",hosxp,".lab_order lo ON lh.lab_order_number=lo.lab_order_number AND lo.lab_items_code IN \
                (", &codes.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(","), ") \
            INNER JOIN ",hosxp,".lab_items li ON lo.lab_items_code=li.lab_items_code \
        WHERE lo.confirm='Y' AND lh.",if is_last { "hn" } else { "vn" },"=? ORDER BY lh.lab_order_number DESC LIMIT 1)"
    ].concat()
}

// SELECT 'eGFR' AS lab_name,lo.lab_order_result,lh.order_date,lh.hn,lh.vn,lo.lab_items_code,lo.lab_items_name_ref,lo.lab_order_number,i.lab_items_unit,i.lab_items_normal_value
// FROM hos.lab_head lh
//   INNER JOIN hos.lab_order lo ON lh.lab_order_number=lo.lab_order_number AND lo.lab_items_code IN (364) 
//   INNER JOIN hos.lab_items i ON lo.lab_items_code=i.lab_items_code
// WHERE lo.confirm='Y' AND lh.hn='0010707' ORDER BY lh.lab_order_number DESC LIMIT 1
/// is_last using hn, else using vn
pub fn select_egfr(codes: &[u64], is_last: bool, not_after: &Option<Date>, hosxp: &str) -> String {
    let max_date = not_after
        .map(|date| [" AND lh.order_date <= '", &date.to_string(), "' "].concat())
        .unwrap_or_default();
    [
        "SELECT 'eGFR' AS lab_name,lo.lab_order_result,lh.order_date,lh.hn,lh.vn,lo.lab_items_code,lo.lab_items_name_ref,lo.lab_order_number,li.lab_items_unit,li.lab_items_normal_value \
        FROM ",hosxp,".lab_head lh INNER JOIN ",hosxp,".lab_order lo ON lh.lab_order_number=lo.lab_order_number AND lo.lab_items_code IN (",
            &codes.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(","),
        ") INNER JOIN ",hosxp,".lab_items li ON lo.lab_items_code=li.lab_items_code \
        WHERE lo.confirm='Y' AND lh.",if is_last { "hn" } else { "vn" },"=? ",&max_date,
        "ORDER BY lh.lab_order_number DESC LIMIT 1;"
    ].concat()
}

// SELECT 'CrCl' AS lab_name,lh.order_date,lh.hn,lh.vn,,lo.lab_items_code,lo.lab_items_name_ref,lo.lab_order_number,li.lab_items_unit,NULL AS lab_items_normal_value,
// IF(opdscreen.bw IS NOT NULL AND opdscreen.bw <> 0 AND vn_stat.age_y IS NOT NULL AND lo.lab_order_result IS NOT NULL AND lo.lab_order_result <> 0,
//     ROUND(IF(vn_stat.sex='2',((140-vn_stat.age_y)*opdscreen.bw)/(lo.lab_order_result*72)*0.85,((140-vn_stat.age_y)*opdscreen.bw)/(lo.lab_order_result*72)),2),'-') AS lab_order_result
// FROM hos.lab_head lh
// LEFT JOIN hos.lab_order lo ON lo.lab_order_number=lh.lab_order_number
// LEFT JOIN hos.lab_items li ON li.lab_items_code=lo.lab_items_code
// JOIN hos.vn_stat ON lh.vn=vn_stat.vn
// JOIN hos.opdscreen ON lh.vn=opdscreen.vn
// WHERE lo.confirm ='Y' AND lh.hn='0010707' AND lo.lab_items_code IN (78)
// ORDER BY lh.lab_order_number DESC LIMIT 1;
/// is_last using hn, else using vn
pub fn select_crcl(codes: &[u64], is_last: bool, not_after: &Option<Date>, hosxp: &str) -> String {
    let max_date = not_after.map(|date| [" AND lh.order_date <= '", &date.to_string(), "' "].concat()).unwrap_or_default();
    [
        "SELECT 'CrCl' AS lab_name,lh.order_date,lh.hn,lh.vn,lo.lab_items_code,lo.lab_items_name_ref,lo.lab_order_number,li.lab_items_unit,NULL AS lab_items_normal_value,\
            IF(opdscreen.bw IS NOT NULL AND opdscreen.bw <> 0 AND vn_stat.age_y IS NOT NULL AND lo.lab_order_result IS NOT NULL AND lo.lab_order_result <> 0,\
                ROUND(IF(vn_stat.sex='2',((140-vn_stat.age_y)*opdscreen.bw)/(lo.lab_order_result*72)*0.85,((140-vn_stat.age_y)*opdscreen.bw)/(lo.lab_order_result*72)),2),'-') AS lab_order_result \
        FROM ",hosxp,".lab_head lh \
            LEFT JOIN ",hosxp,".lab_order lo ON lo.lab_order_number=lh.lab_order_number \
            LEFT JOIN ",hosxp,".lab_items li ON li.lab_items_code=lo.lab_items_code \
            JOIN ",hosxp,".vn_stat ON lh.vn=vn_stat.vn \
            JOIN ",hosxp,".opdscreen ON lh.vn=opdscreen.vn \
        WHERE lo.confirm='Y' AND lh.",if is_last {"hn"} else {"vn"},"=? ",&max_date," AND lo.lab_items_code IN (",
            &codes.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(","),
        ") ORDER BY lh.lab_order_number DESC LIMIT 1;"
    ].concat()
}

// SELECT ovst.oqueue,ovst.vstdate,ovst.vsttime,ovst.hn,ovst.vn,ovst.an,doc.`name` AS doctor_name,pt.`name` AS pttype_name,opdc.cc,opdc.hpi,opdc.pe,
//     GROUP_CONCAT(CONCAT(ovd.icd10,' : ',icd.`name`) SEPARATOR '\n') AS diag,
//     opdc.temperature,opdc.bps,opdc.bpd,opdc.bw,opdc.height,opdc.bmi,opdc.fbs
//     doc1.`name` AS pharmacist_accept_name,sc.pharmacist_accept_time,
//     doc2.`name` AS pharmacist_check_name,sc.pharmacist_check_time,
//     doc3.`name` AS pharmacist_done_name,sc.pharmacist_done_time
//     sc.postal_status,doc4.`name` AS postal_doctor_name,sc.postal_time,
//     sc.telemed_add,sc.telemed_dose_up,sc.telemed_dose_down,sc.telemed_off,sc.telemed_other,
//     doc5.`name` AS telemed_doctor_name,sc.telemed_time
// FROM hos.ovst
//     LEFT JOIN hos.vn_stat vns ON vns.vn=ovst.vn
//     LEFT JOIN hos.doctor doc ON doc.`code`=ovst.doctor
//     LEFT JOIN hos.pttype pt ON pt.pttype=ovst.pttype
//     LEFT JOIN hos.opdscreen opdc ON opdc.vn=ovst.vn
//     LEFT JOIN hos.ovstdiag ovd ON  ovd.vn=ovst.vn
//     LEFT JOIN hos.icd101 icd ON  icd.`code`=ovd.icd10
//     LEFT JOIN kphis_extra,".prescription_screen sc ON sc.vn=ovst.vn
//     LEFT JOIN hos.doctor doc1 ON doc1.`code`=sc.pharmacist_accept
//     LEFT JOIN hos.doctor doc2 ON doc2.`code`=sc.pharmacist_check
//     LEFT JOIN hos.doctor doc3 ON doc3.`code`=sc.pharmacist_done
//     LEFT JOIN hos.doctor doc4 ON doc4.`code`=sc.postal_doctor
//     LEFT JOIN hos.doctor doc5 ON doc5.`code`=sc.telemed_doctor
// WHERE ovst.vn='660726084730' GROUP BY ovst.vn ORDER BY ovst.vn DESC;
/// vn
pub fn select_info_vn(hosxp: &str, kphis_extra: &str) -> String {
    [
        "SELECT ovst.oqueue,ovst.vstdate,ovst.vsttime,ovst.hn,ovst.vn,ovst.an,doc.`name` AS doctor_name,pt.`name` AS pttype_name,opdc.cc,opdc.hpi,opdc.pe,\
            GROUP_CONCAT(CONCAT(ovd.icd10,' : ',icd.`name`) SEPARATOR '\n') AS diag,\
            opdc.temperature,opdc.bps,opdc.bpd,opdc.bw,opdc.height,opdc.bmi,opdc.fbs,\
            doc1.`name` AS pharmacist_accept_name,sc.pharmacist_accept_time,\
            doc2.`name` AS pharmacist_check_name,sc.pharmacist_check_time,\
            doc3.`name` AS pharmacist_done_name,sc.pharmacist_done_time,\
            sc.postal_status,doc4.`name` AS postal_doctor_name,sc.postal_time,\
            sc.telemed_add,sc.telemed_dose_up,sc.telemed_dose_down,sc.telemed_off,sc.telemed_other,\
            doc5.`name` AS telemed_doctor_name,sc.telemed_time,\
            sc.pharmacy_care,doc6.`name` AS pharmacy_care_doctor_name,sc.pharmacy_care_time \
        FROM ",hosxp,".ovst \
            LEFT JOIN ",hosxp,".vn_stat vns ON vns.vn=ovst.vn \
            LEFT JOIN ",hosxp,".doctor doc ON doc.`code`=ovst.doctor \
            LEFT JOIN ",hosxp,".pttype pt ON pt.pttype=ovst.pttype \
            LEFT JOIN ",hosxp,".opdscreen opdc ON opdc.vn=ovst.vn \
            LEFT JOIN ",hosxp,".ovstdiag ovd ON ovd.vn=ovst.vn \
            LEFT JOIN ",hosxp,".icd101 icd ON icd.`code`=ovd.icd10 \
            LEFT JOIN ",kphis_extra,".prescription_screen sc ON sc.vn=ovst.vn \
            LEFT JOIN ",hosxp,".doctor doc1 ON doc1.`code`=sc.pharmacist_accept \
            LEFT JOIN ",hosxp,".doctor doc2 ON doc2.`code`=sc.pharmacist_check \
            LEFT JOIN ",hosxp,".doctor doc3 ON doc3.`code`=sc.pharmacist_done \
            LEFT JOIN ",hosxp,".doctor doc4 ON doc4.`code`=sc.postal_doctor \
            LEFT JOIN ",hosxp,".doctor doc5 ON doc5.`code`=sc.telemed_doctor \
            LEFT JOIN ",hosxp,".doctor doc6 ON doc6.`code`=sc.pharmacy_care_doctor \
        WHERE ovst.vn=? GROUP BY ovst.vn ORDER BY ovst.vn DESC;"
    ].concat()
}

// // info_medicine
// SELECT o.vn,o.an,o.hn,CONCAT(s.NAME,' ',s.strength,' ',s.units) AS name_drugitems,o.qty,o.sp_use,
//     IF(o.sp_use <> '',(SELECT CONCAT(IFNULL(name1,''),' ',IFNULL(name2,''),' ',IFNULL(name3,'')) FROM hos.sp_use WHERE sp_use=o.sp_use),dr.shortlist) AS shortlist,
//     o.icode,o.drugusage,o.rxdate,o.rxtime,d.generic_name,s.strength,s.icode
// FROM hos.opitemrece o
//     INNER JOIN hos.drugitems d ON d.icode=o.icode
//     LEFT JOIN hos.s_drugitems s ON s.icode=o.icode
//     LEFT JOIN hos.drugusage dr ON dr.drugusage=o.drugusage
// WHERE 1=1
// // info_medicine_last_drug
// SELECT IF(o1.vn IS NULL,'AN','VN') AS type_,IF(o1.vn IS NULL,o1.an,o1.vn) AS type_data,
//     CONCAT(:strength,:drugusage)<>CONCAT(d1.strength,o1.drugusage) AS drug_change,
//     IF(o1.sp_use <> '',(SELECT CONCAT(IFNULL(name1,''),' ',IFNULL(name2,''),' ',IFNULL(name3,'')) FROM hos.sp_use WHERE sp_use=o1.sp_use),dr.shortlist) AS shortlist,
//     TIMESTAMP(o1.rxdate,o1.rxtime) AS timestamp_1,
//     d1.generic_name,d1.strength,d1.units,o1.drugusage,o1.qty,o1.sp_use,
//     CONCAT(s.NAME,' ',s.strength,' ',s.units) AS name_drugitems,s.icode
// FROM hos.opitemrece o1
//     INNER JOIN hos.drugitems d1 ON d1.icode=o1.icode
//     LEFT JOIN hos.s_drugitems s ON s.icode=o1.icode
//     LEFT JOIN hos.drugusage dr ON dr.drugusage=o1.drugusage
// WHERE o1.hn=:hn AND o1.icode=:icode AND TIMESTAMP(o1.rxdate,o1.rxtime) < TIMESTAMP(:rxdate,:rxtime)
//     AND d1.generic_name=:generic_name AND TIMESTAMPDIFF(MONTH,o1.rxdate,:rxdate) <= 6
// ORDER BY TIMESTAMP(o1.rxdate) DESC LIMIT 1;
// // we change GET 'info_medicine' and USE 'info_medicine_last_drug'
// // to USE 'info_medicine' WITH 'previous prescription concat data'
// // and we remove hos.s_drugitems usage
// SELECT CONCAT(d.NAME,' ',d.strength,' ',d.units) AS name_drugitems,d.generic_name,d.strength,o.qty,o.icode,o.rxdate,o.rxtime,o.drugusage,o.vn,o.an,o.hn,o.sp_use,
//     IF(o.sp_use <> '',(SELECT CONCAT(IFNULL(name1,''),' ',IFNULL(name2,''),' ',IFNULL(name3,'')) FROM hos.sp_use WHERE sp_use=o.sp_use),dr.shortlist) AS shortlist,
//     (SELECT CONCAT(IF(o1.vn IS NULL,'AN','VN'),'^',IF(o1.vn IS NULL,o1.an,o1.vn),'^',CONCAT(d1.NAME,' ',d1.strength,' ',d1.units),'^',d1.strength,'^',o1.qty,'^',o1.icode,'^',o1.rxdate,' ',o1.rxtime,'^',
// 	 	IF(o1.sp_use <> '',(SELECT CONCAT(IFNULL(name1,''),' ',IFNULL(name2,''),' ',IFNULL(name3,'')) FROM hos.sp_use WHERE sp_use=o1.sp_use),dr1.shortlist))
//         FROM hos.opitemrece o1 INNER JOIN hos.drugitems d1 ON d1.icode=o1.icode LEFT JOIN hos.drugusage dr1 ON dr1.drugusage=o1.drugusage
// 		WHERE o1.hn=o.hn AND TIMESTAMP(o1.rxdate,o1.rxtime) < TIMESTAMP(o.rxdate,o.rxtime) AND d1.generic_name=d.generic_name
//         AND TIMESTAMPDIFF(MONTH,o1.rxdate,o.rxdate) <= 6 ORDER BY TIMESTAMP(o1.rxdate) DESC LIMIT 1) AS last_prescription
// FROM hos.opitemrece o INNER JOIN hos.drugitems d ON d.icode=o.icode LEFT JOIN hos.drugusage dr ON dr.drugusage=o.drugusage
// WHERE 1=1 AND o.vn='660726084730'
/// an || vn
pub fn select_info_medicine(is_admit: bool, hosxp: &str) -> String {
    let an_or_vn = if is_admit {" WHERE o.item_type='H' AND o.an=? "} else {" WHERE o.vn=? "};
    [
        "SELECT CONCAT(d.NAME,' ',d.strength,' ',d.units) AS name_drugitems,d.generic_name,d.strength,o.qty,o.icode,ADDTIME(CONVERT(o.rxdate,DATETIME),o.rxtime) AS rxdatetime,o.drugusage,o.vn,o.an,o.hn,o.sp_use,\
            IF(o.sp_use <> '',(SELECT CONCAT(IFNULL(name1,''),' ',IFNULL(name2,''),' ',IFNULL(name3,'')) FROM ",hosxp,".sp_use WHERE sp_use=o.sp_use),dr.shortlist) AS shortlist,\
            (SELECT CONCAT(IF(o1.vn IS NULL,'AN','VN'),'^',IF(o1.vn IS NULL,o1.an,o1.vn),'^',CONCAT(d1.NAME,' ',d1.strength,' ',d1.units),'^',d1.strength,'^',o1.qty,'^',o1.icode,'^',o1.rxdate,' ',o1.rxtime,'^',\
                IF(o1.sp_use <> '',(SELECT CONCAT(IFNULL(name1,''),' ',IFNULL(name2,''),' ',IFNULL(name3,'')) FROM ",hosxp,".sp_use WHERE sp_use=o1.sp_use),dr1.shortlist)) \
                FROM ",hosxp,".opitemrece o1 INNER JOIN ",hosxp,".drugitems d1 ON d1.icode=o1.icode LEFT JOIN ",hosxp,".drugusage dr1 ON dr1.drugusage=o1.drugusage \
                WHERE o1.hn=o.hn AND TIMESTAMP(o1.rxdate,o1.rxtime) < TIMESTAMP(o.rxdate,o.rxtime) AND d1.generic_name=d.generic_name \
                    AND TIMESTAMPDIFF(MONTH,o1.rxdate,o.rxdate) <= 6 ORDER BY TIMESTAMP(o1.rxdate) DESC LIMIT 1) AS last_prescription \
        FROM ",hosxp,".opitemrece o INNER JOIN ",hosxp,".drugitems d ON d.icode=o.icode LEFT JOIN ",hosxp,".drugusage dr ON dr.drugusage=o.drugusage ",
        an_or_vn,"ORDER BY o.item_no;"
    ].concat()
}

// info_drug_interaction
// SELECT drugname1,drugname2,severity,note FROM hos.drug_interaction WHERE drugname1 IN
// (SELECT DISTINCT generic_name FROM hos.drugitems WHERE icode IN (SELECT icode FROM hos.opitemrece WHERE vn='660726084730')) AND drugname2 IN
// (SELECT DISTINCT generic_name FROM hos.drugitems WHERE icode IN (SELECT icode FROM hos.opitemrece WHERE vn='660726084730')) ORDER BY drugname1, drugname2;
/// vn, vn
pub fn select_info_drug_interaction(hosxp: &str) -> String {
    [
        "SELECT drugname1,drugname2,severity,note FROM ",hosxp,".drug_interaction WHERE drugname1 IN \
        (SELECT DISTINCT generic_name FROM ",hosxp,".drugitems WHERE icode IN (SELECT icode FROM ",hosxp,".opitemrece WHERE vn=?)) AND drugname2 IN \
        (SELECT DISTINCT generic_name FROM ",hosxp,".drugitems WHERE icode IN (SELECT icode FROM ",hosxp,".opitemrece WHERE vn=?)) ORDER BY drugname1, drugname2;"
    ].concat()
}

// // next_app
// SELECT hn,GROUP_CONCAT(CONCAT(DATE_FORMAT(nextdate,'%e/%m/'),YEAR(nextdate)+543),' [',DATEDIFF(nextdate,DATE(NOW())),']' SEPARATOR '\n') AS next_app
// FROM hos.oapp WHERE oapp.hn=(SELECT hn FROM hos.ovst WHERE vn=?) AND oapp.nextdate >= DATE(NOW());
// // we not concat, and find 'hn' directly
// SELECT nextdate,DATEDIFF(nextdate,DATE(NOW())) AS days FROM hos.oapp WHERE oapp.hn='0023215' AND oapp.nextdate >= DATE(NOW());
// // edit
// SELECT nextdate,DATEDIFF(nextdate,vstdate) AS days FROM ",hosxp,".oapp WHERE oapp.hn=? AND oapp.vn=?;
/// vn
pub fn select_next_app(hosxp: &str) -> String {
    [
        "SELECT nextdate,c.`name` AS clinic_name,DATEDIFF(nextdate,vstdate) AS days \
        FROM ",hosxp,".oapp \
            LEFT JOIN ",hosxp,".clinic c ON c.clinic=oapp.clinic \
        WHERE oapp.vn=?;"
    ].concat()
}

// // info_mess_vn
pub fn select_info_message(messages: &[(String, Vec<String>)], hosxp: &str) -> String {
    messages.iter().map(|(message, icodes)| select_icode_message(message, icodes, hosxp)).collect::<Vec<String>>().join("UNION ALL")
}
// (SELECT CONCAT('message',' : ','drug:(',GROUP_CONCAT(d.name),')') AS message FROM hos.opitemrece o INNER JOIN hos.drugitems d ON d.icode=o.icode
// WHERE o.icode IN ('1000152','1520066','1000110','1000111','16000026','1000182','1520187','1600039','1550008') AND o.vn=? GROUP BY vn HAVING count(*) > 1 LIMIT 1)
/// vn
fn select_icode_message(message: &str, icodes: &[String], hosxp: &str) -> String {
    [
        "(SELECT CONCAT('",message,"',' : ','drug:(',GROUP_CONCAT(d.name),')') AS message FROM ",hosxp,".opitemrece o INNER JOIN ",hosxp,".drugitems d ON d.icode=o.icode \
        WHERE o.icode IN ('",&icodes.join("','"),"') AND o.vn=? GROUP BY vn HAVING count(*) > 1 LIMIT 1)"
    ].concat()
}

/// vn
pub fn select_info_ckd_message(
    messages: &[(String, u64, Vec<String>)],
    lab: &str,
    pt_value: &str,
    hosxp: &str,
) -> String {
    messages.iter().map(|(message, target, icodes)| {select_icode_ckd_message(message, lab, pt_value, &target.to_string(), icodes, hosxp)}).collect::<Vec<String>>().join("UNION ALL")
}
// (SELECT CONCAT('message',' : ','eGFR:',?) AS message FROM hos.opitemrece o
// WHERE o.icode IN ('1000152','1520066','1000110','1000111','1600026','1000182','1520187','1600039','1550008') AND ? < 45 AND o.vn=? LIMIT 1)
fn select_icode_ckd_message(
    message: &str,
    lab: &str,
    pt_value: &str,
    target: &str,
    icodes: &[String],
    hosxp: &str,
) -> String {
    [
        "(SELECT CONCAT('",message," ",lab," : ',",pt_value,") AS message FROM ",hosxp,".opitemrece o \
        WHERE o.icode IN ('",&icodes.join("','"),"') AND ",pt_value," < ",target," AND o.vn=? LIMIT 1)"
    ].concat()
}

/// vn, doctorcode, user, user
pub fn insert_duplicate_update_accept_prescription_screen(kphis_extra: &str) -> String {
    [
        "INSERT INTO ",kphis_extra,".prescription_screen (vn,pharmacist_accept,pharmacist_accept_time",TABLE_CREATE_COLUMNS,") \
            VALUES (?,?,NOW()",TABLE_CREATE_PREPARED,") \
        ON DUPLICATE KEY UPDATE \
            pharmacist_accept=VALUES(pharmacist_accept),pharmacist_accept_time=VALUE(pharmacist_accept_time),update_user=VALUE(update_user),update_datetime=VALUE(update_datetime),version=(version+1);"
    ].concat()
}

/// doctorcode, user, vn
pub fn update_check_prescription_screen(kphis_extra: &str) -> String {
    ["UPDATE `",kphis_extra,"`.`prescription_screen` SET pharmacist_check=?,pharmacist_check_time=NOW()",TABLE_UPDATE_SET," WHERE vn=? AND pharmacist_accept_time IS NOT NULL;"].concat()
}

/// doctorcode, user, vn
pub fn update_done_prescription_screen(kphis_extra: &str) -> String {
    ["UPDATE `",kphis_extra,"`.`prescription_screen` SET pharmacist_done=?,pharmacist_done_time=NOW()",TABLE_UPDATE_SET," WHERE vn=? AND pharmacist_accept_time IS NOT NULL AND pharmacist_check_time IS NOT NULL;"].concat()
}

/// postal_status, doctorcode, user, vn
pub fn update_postal_prescription_screen(kphis_extra: &str) -> String {
    ["UPDATE `",kphis_extra,"`.`prescription_screen` SET postal_status=?,postal_doctor=?,postal_time=NOW()",TABLE_UPDATE_SET," WHERE vn=?;"].concat()
}

/// vn, telemed_add, telemed_dose_up, telemed_dose_down, telemed_off, telemed_other, doctorcode, user, user
pub fn insert_duplicate_update_telemed_prescription_screen(kphis_extra: &str) -> String {
    [
        "INSERT INTO ",kphis_extra,".prescription_screen (vn,telemed_add,telemed_dose_up,telemed_dose_down,telemed_off,telemed_other,telemed_doctor,telemed_time",TABLE_CREATE_COLUMNS,") \
            VALUES (?,?,?,?,?,?,?,NOW()",TABLE_CREATE_PREPARED,") \
        ON DUPLICATE KEY UPDATE \
            telemed_add=VALUES(telemed_add),telemed_dose_up=VALUES(telemed_dose_up),telemed_dose_down=VALUES(telemed_dose_down),telemed_off=VALUES(telemed_off),telemed_other=VALUES(telemed_other),\
            telemed_doctor=VALUES(telemed_doctor),telemed_time=VALUES(telemed_time),update_user=VALUE(update_user),update_datetime=VALUE(update_datetime),version=(version+1);"
    ].concat()
}

/// pharmacy_care, doctorcode, user, vn
pub fn update_pharmacy_care_prescription_screen(kphis_extra: &str) -> String {
    ["UPDATE `",kphis_extra,"`.`prescription_screen` SET pharmacy_care=?,pharmacy_care_doctor=?,pharmacy_care_time=NOW()",TABLE_UPDATE_SET," WHERE vn=?;"].concat()
}