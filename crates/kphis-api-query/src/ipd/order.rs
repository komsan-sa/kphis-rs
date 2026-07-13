use sqlx::{
    AssertSqlSafe, FromRow, MySql, Pool, Row,
    mysql::{MySqlQueryResult, MySqlRow},
};
use std::{cmp::Ordering, collections::HashMap};
use time::{Date, PrimitiveDateTime, Time};

use kphis_model::{
    app::VisitTypeId,
    fetch::ExecuteResponse,
    ipd::pharmacy_monitor::{IpdOrderPharmacy, IpdOrderPharmacyMonitor, IpdOrderPharmacyParams, PharmacyIpt},
    order::{MedOrderItem, MedPlanItem, Order, OrderDate, OrderItem, OrderItemOnly, OrderItemPatch, OrderItemSave, OrderOnly, OrderParams, OrderPatch, OrderPatchAction, OrderSave},
};
use kphis_sql::ipd::order;
use kphis_util::{
    datetime::now,
    error::{AppError, Source},
    util::{opt_empty_none, sanity_dot_space, sanity_tis620, split_to_three, zero_none},
};

use super::{
    index_action::{get_index_action_only, insert_index_action_only},
    index_monitor::{get_index_monitor_only, insert_index_monitors_only},
    index_plan::{get_index_plan_only, insert_index_plan_only},
};
use crate::{
    app::{bump_and_get_serial, bump_and_get_sp_use},
    execute6, query1_all, query2_all,
};

// ipd-dr-order.php
pub async fn get_order_date(an: &str, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<OrderDate>, AppError> {
    let sql = order::select_ipd_order_date(false, kphis);
    query2_all(an, an, &sql, pool, "Select OrderDate")
        .await?
        .iter()
        .map(OrderDate::from_row)
        .collect::<sqlx::Result<Vec<OrderDate>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select OrderDate"))
}

// ipd-dr-order-one-day-data.php
// ipd-dr-order-continuous-data.php
pub async fn get_order(params: &OrderParams, doctorcode: &Option<String>, intern_roles: &[String], pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<Vec<Order>, AppError> {
    let sql = order::select_order(params, intern_roles, hosxp, kphis);
    let mut query = sqlx::query(AssertSqlSafe(sql));
    if let Some(order_id) = params.order_id.as_ref() {
        query = query.bind(order_id);
    }
    if let Some(an) = params.an.as_ref() {
        query = query.bind(an);
    }
    if let Some(current_date) = params.current_date.as_ref() {
        query = query.bind(current_date);
    }
    if let Some(order_confirm) = params.order_confirm.as_ref() {
        query = query.bind(order_confirm);
    }
    if params.doctor_not_confirm_as.as_ref().map(|s| s.as_str() == "Y").unwrap_or_default() {
        query = query.bind(doctorcode);
    }
    query
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select Order"))?
        .iter()
        .map(order_from_row)
        .collect::<sqlx::Result<Vec<Order>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select Order"))
}
fn order_from_row(row: &MySqlRow) -> sqlx::Result<Order> {
    let an: String = row.try_get("an")?;
    Ok(Order {
        visit_type: VisitTypeId::Ipd(an.clone()),
        order_id: row.try_get("order_id")?,
        hn: row.try_get("hn")?,
        fullname: row.try_get("fullname")?,
        ward_name: row.try_get("ward_name")?,
        bedno: row.try_get("bedno")?,
        display_bedno: None,
        bed_type_name: None,
        bed_type_color: None,
        order_date: row.try_get("order_date")?,
        order_time: row.try_get("order_time")?,
        order_doctor: row.try_get("order_doctor")?,
        order_type: row.try_get("order_type")?,
        order_owner_type: row.try_get("order_owner_type")?,
        order_confirm: row.try_get("order_confirm")?,
        nurse_order_as: row.try_get("nurse_order_as")?,
        doctor_confirm_time: row.try_get("doctor_confirm_time")?,
        nurse_accept: row.try_get("nurse_accept")?,
        nurse_accept_time: row.try_get("nurse_accept_time")?,
        pharmacist_accept: row.try_get("pharmacist_accept")?,
        pharmacist_accept_time: row.try_get("pharmacist_accept_time")?,
        pharmacist_check: row.try_get("pharmacist_check")?,
        pharmacist_check_time: row.try_get("pharmacist_check_time")?,
        pharmacist_done: row.try_get("pharmacist_done")?,
        pharmacist_done_time: row.try_get("pharmacist_done_time")?,
        pharmacist_order_status: row.try_get("pharmacist_order_status")?,
        pre_order_id: row.try_get("pre_order_id")?,
        pre_order_date: row.try_get("pre_order_date")?,
        pre_order_time: row.try_get("pre_order_time")?,

        order_doctor_name: row.try_get("order_doctor_name")?,
        order_doctor_licenseno: row.try_get("order_doctor_licenseno")?,
        order_doctor_entryposition: row.try_get("order_doctor_entryposition")?,
        order_doctor_is_intern: row.try_get("order_doctor_is_intern")?,

        nurse_order_as_name: row.try_get("nurse_order_as_name")?,
        nurse_order_as_licenseno: row.try_get("nurse_order_as_licenseno")?,
        nurse_order_as_entryposition: row.try_get("nurse_order_as_entryposition")?,
        nurse_order_as_is_intern: row.try_get("nurse_order_as_is_intern")?,

        nurse_accept_name: row.try_get("nurse_accept_name")?,
        nurse_accept_licenseno: row.try_get("nurse_accept_licenseno")?,
        nurse_accept_entryposition: row.try_get("nurse_accept_entryposition")?,

        pharmacist_accept_name: row.try_get("pharmacist_accept_name")?,
        pharmacist_accept_licenseno: row.try_get("pharmacist_accept_licenseno")?,
        pharmacist_accept_entryposition: row.try_get("pharmacist_accept_entryposition")?,

        pharmacist_check_name: row.try_get("pharmacist_check_name")?,
        pharmacist_check_licenseno: row.try_get("pharmacist_check_licenseno")?,
        pharmacist_check_entryposition: row.try_get("pharmacist_check_entryposition")?,

        pharmacist_done_name: row.try_get("pharmacist_done_name")?,
        pharmacist_done_licenseno: row.try_get("pharmacist_done_licenseno")?,
        pharmacist_done_entryposition: row.try_get("pharmacist_done_entryposition")?,

        order_item_types: Vec::new(),
    })
}

pub async fn get_order_only_bundle(an: &str, pool: &Pool<MySql>, kphis: &str, kphis_extra: &str) -> Result<Vec<OrderOnly>, AppError> {
    let mut orders = get_order_only(an, pool, kphis).await?;
    for order in orders.iter_mut() {
        let mut order_items = get_order_item_only(order.order_id, pool, kphis).await?;
        for order_item in order_items.iter_mut() {
            let mut index_plans = get_index_plan_only(order_item.order_item_id, pool, kphis).await?;
            for index_plan in index_plans.iter_mut() {
                let mut index_actions = get_index_action_only(index_plan.plan_id, pool, kphis, kphis_extra).await?;
                for index_action in index_actions.iter_mut() {
                    if index_action.has_monitor {
                        let index_monitors = get_index_monitor_only(index_action.action_id, pool, kphis_extra).await?;
                        index_action.index_monitors = index_monitors;
                    }
                }
                index_plan.index_actions = index_actions;
            }
            order_item.index_plans = index_plans;
        }
        order.order_items = order_items;
    }

    Ok(orders)
}

async fn get_order_only(an: &str, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<OrderOnly>, AppError> {
    let sql = order::select_order_only(kphis);
    sqlx::query(AssertSqlSafe(sql))
        .bind(an)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select OrderOnly"))?
        .iter()
        .map(OrderOnly::from_row)
        .collect::<sqlx::Result<Vec<OrderOnly>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select OrderOnly"))
}

pub async fn get_order_types(ids: &[u32], order_type: &Option<String>, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<(u32, String)>, AppError> {
    if ids.is_empty() {
        Ok(Vec::new())
    } else {
        let sql = order::select_order_types(ids, order_type.is_some(), kphis);
        let mut query = sqlx::query(AssertSqlSafe(sql));
        if let Some(otype) = order_type {
            query = query.bind(otype);
        }
        query.fetch_all(pool).await.map_err(|e| Source::SQLx.to_error(500, e, "Select OrderType")).map(|rows| {
            rows.iter()
                .filter_map(|row| {
                    let item_type: Option<String> = row.try_get("order_item_type").ok();
                    let order_id: Option<u32> = row.try_get("order_id").ok();
                    order_id.zip(item_type)
                })
                .collect()
        })
    }
}

pub async fn get_order_item(
    // order_date: Date,
    order_id: Option<u32>,
    order_item_type: Option<String>,
    params: &OrderParams,
    order_item_ids: &[u32],
    pool: &Pool<MySql>,
    hosxp: &str,
    kphis: &str,
) -> Result<Vec<OrderItem>, AppError> {
    let sql = order::select_order_item(params, order_id.is_some(), order_item_type.is_some(), order_item_ids, hosxp, kphis);
    let mut query = sqlx::query(AssertSqlSafe(sql)); //.bind(order_date);
    if let Some(an) = params.an.as_ref() {
        query = query.bind(an);
    }
    if let Some(order_date) = params.current_date {
        query = query.bind(order_date);
    }
    if let Some(order_id) = order_id {
        query = query.bind(order_id);
    }
    if let Some(order_item_id) = params.order_item_id {
        query = query.bind(order_item_id);
    }
    if let Some(order_item_type) = order_item_type.as_ref() {
        query = query.bind(order_item_type);
    }
    if let Some(order_type) = params.order_type.as_ref() {
        query = query.bind(order_type);
    }
    query
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select OrderItem"))?
        .iter()
        .map(order_item_from_row)
        .collect::<sqlx::Result<Vec<OrderItem>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select OrderItem"))
}
fn order_item_from_row(row: &MySqlRow) -> sqlx::Result<OrderItem> {
    let an: String = row.try_get("an")?;
    Ok(OrderItem {
        visit_type: VisitTypeId::Ipd(an.clone()),
        order_item_id: row.try_get("order_item_id")?,
        order_id: row.try_get("order_id")?,
        order_date: row.try_get("order_date")?,
        order_time: row.try_get("order_time")?,
        order_type: row.try_get("order_type")?,
        order_owner_type: row.try_get("order_owner_type")?,
        order_doctor_name: row.try_get("order_doctor_name")?,
        order_doctor_licenseno: row.try_get("order_doctor_licenseno")?,

        order_item_type: row.try_get("order_item_type")?,
        order_item_detail: row.try_get("order_item_detail")?,
        stat: row.try_get("stat")?,
        off_order_item_id: row.try_get("off_order_item_id")?,
        icode: row.try_get("icode")?,
        nurse_assign: row.try_get("nurse_assign")?,
        off_by_datetime: row.try_get("off_by_datetime")?,
        med_name: row.try_get("med_name")?,
        displaycolor: row.try_get("displaycolor")?,
        addict_type_id: row.try_get("addict_type_id")?,
        habit_forming_type: row.try_get("habit_forming_type")?,
        generic_name: row.try_get("generic_name")?,
        dosageform: row.try_get("dosageform")?,
        off_icode: row.try_get("off_icode")?,
        off_med_name: row.try_get("off_med_name")?,
        off_displaycolor: row.try_get("off_displaycolor")?,
        off_order_item_detail: row.try_get("off_order_item_detail")?,
        allergy_agent_symptom: row.try_get("allergy_agent_symptom")?,
        first_qty: row.try_get("first_qty")?,
        qty: row.try_get("qty")?,

        med_reconciliation_item_id: row.try_get("med_reconciliation_item_id")?,
        old_drugusage: row.try_get("old_drugusage")?,
        receive_from: row.try_get("receive_from")?,
        receive_date: row.try_get("receive_date")?,
        receive_qty: row.try_get("receive_qty")?,
        last_dose_taken_time: row.try_get("last_dose_taken_time")?,
        last_dose_taken_remark: row.try_get("last_dose_taken_remark")?,
        used: row.try_get("used")?,

        due_usage: row.try_get("due_usage")?,
        due_status: row.try_get("due_status")?,
        due_doctor: row.try_get("due_doctor")?,
        due_doctor_note: row.try_get("due_doctor_note")?,
        due_pharm: row.try_get("due_pharm")?,
        due_pharm_note: row.try_get("due_pharm_note")?,

        info: row.try_get("info")?,
        info_status: row.try_get("info_status")?,

        order_duration: None,
        duration1: None,
        exceed_duration1_color: None,
        duration2: None,
        exceed_duration2_color: None,
        duration3: None,
        exceed_duration3_color: None,

        monitor: row.try_get("monitor")?,
        monitor_count: row.try_get("monitor_count")?,
        monitor_duration: row.try_get("monitor_duration")?,
        monitor_status: row.try_get("monitor_status")?,

        index_plans: Vec::new(),
    })
}

pub async fn select_order_item_ids_by_an_and_plan_date(an: &str, plan_date: Date, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<u32>, AppError> {
    let sql = order::select_order_item_ids_by_an_and_plan_date(kphis);
    sqlx::query(AssertSqlSafe(sql))
        .bind(an)
        .bind(plan_date)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select OrderItemIDs"))?
        .iter()
        .filter_map(|row| row.try_get("order_item_id").transpose())
        .collect::<sqlx::Result<Vec<u32>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select OrderItemIDs"))
}

async fn get_order_item_only(order_id: u32, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<OrderItemOnly>, AppError> {
    let sql = order::select_order_item_only(kphis);
    sqlx::query(AssertSqlSafe(sql))
        .bind(order_id)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select OrderItemOnly"))?
        .iter()
        .map(OrderItemOnly::from_row)
        .collect::<sqlx::Result<Vec<OrderItemOnly>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select OrderItemOnly"))
}

// ipd-dr-order-continuous-previous-data.php
pub async fn get_previous_order(params: &OrderParams, pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<Vec<OrderItem>, AppError> {
    let sql = order::select_previous(params, hosxp, kphis);
    if let (Some(current_date), Some(an)) = (params.current_date.as_ref(), params.an.as_ref()) {
        let mut query = sqlx::query(AssertSqlSafe(sql)).bind(current_date).bind(current_date).bind(an);
        // let mut query = sqlx::query(AssertSqlSafe(sql)).bind(current_date).bind(current_date).bind(current_date).bind(an);
        if let Some(order_type) = &params.order_type {
            query = query.bind(order_type);
        }
        // if let Some(order_item_type) = &params.order_item_type {
        //     query = query.bind(order_item_type);
        // }
        if params.with_offed.is_none() {
            query = query.bind(current_date);
        }
        query
            .fetch_all(pool)
            .await
            .map_err(|e| Source::SQLx.to_error(500, e, "Select PreviousOrder"))?
            .iter()
            .map(prev_order_item_from_row)
            .collect::<sqlx::Result<Vec<OrderItem>>>()
            .map_err(|e| Source::SQLx.to_error(500, e, "Select PreviousOrder"))
    } else {
        Ok(Vec::new())
    }
}
fn prev_order_item_from_row(row: &MySqlRow) -> sqlx::Result<OrderItem> {
    let an: String = row.try_get("an")?;
    Ok(OrderItem {
        visit_type: VisitTypeId::Ipd(an.clone()),
        order_item_id: row.try_get("order_item_id")?,
        order_id: row.try_get("order_id")?,
        order_date: row.try_get("order_date")?,
        order_time: row.try_get("order_time")?,
        order_type: row.try_get("order_type")?,
        order_owner_type: row.try_get("order_owner_type")?,
        order_doctor_name: row.try_get("order_doctor_name")?,
        order_doctor_licenseno: row.try_get("order_doctor_licenseno")?,

        order_item_type: row.try_get("order_item_type")?,
        order_item_detail: row.try_get("order_item_detail")?,
        stat: row.try_get("stat")?,
        off_order_item_id: row.try_get("off_order_item_id")?,
        icode: row.try_get("icode")?,
        nurse_assign: row.try_get("nurse_assign")?,
        off_by_datetime: row.try_get("off_by_datetime")?,
        med_name: row.try_get("med_name")?,
        displaycolor: row.try_get("displaycolor")?,
        addict_type_id: row.try_get("addict_type_id")?,
        habit_forming_type: row.try_get("habit_forming_type")?,
        generic_name: row.try_get("generic_name")?,
        dosageform: row.try_get("dosageform")?,
        off_icode: None,
        off_med_name: None,
        off_displaycolor: None,
        off_order_item_detail: None,
        allergy_agent_symptom: row.try_get("allergy_agent_symptom")?,
        first_qty: row.try_get("first_qty")?,
        qty: row.try_get("qty")?,

        due_usage: row.try_get("due_usage")?,
        due_status: row.try_get("due_status")?,
        due_doctor: row.try_get("due_doctor")?,
        due_doctor_note: row.try_get("due_doctor_note")?,
        due_pharm: row.try_get("due_pharm")?,
        due_pharm_note: row.try_get("due_pharm_note")?,

        info: row.try_get("info")?,
        info_status: row.try_get("info_status")?,

        med_reconciliation_item_id: row.try_get("med_reconciliation_item_id")?,
        old_drugusage: row.try_get("old_drugusage")?,
        receive_from: row.try_get("receive_from")?,
        receive_date: row.try_get("receive_date")?,
        receive_qty: row.try_get("receive_qty")?,
        last_dose_taken_time: row.try_get("last_dose_taken_time")?,
        last_dose_taken_remark: row.try_get("last_dose_taken_remark")?,
        used: row.try_get("used")?,

        order_duration: row.try_get("order_duration")?,
        duration1: row.try_get("duration1")?,
        exceed_duration1_color: row.try_get("exceed_duration1_color")?,
        duration2: row.try_get("duration2")?,
        exceed_duration2_color: row.try_get("exceed_duration2_color")?,
        duration3: row.try_get("duration3")?,
        exceed_duration3_color: row.try_get("exceed_duration3_color")?,

        monitor: row.try_get("monitor")?,
        monitor_count: row.try_get("monitor_count")?,
        monitor_duration: row.try_get("monitor_duration")?,
        monitor_status: row.try_get("monitor_status")?,

        index_plans: Vec::new(),
    })
}

// ipd-dr-order-previous-one-day-order-data.php
pub async fn get_previous_one_day_order(an: &str, pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<Vec<MedOrderItem>, AppError> {
    let sql = order::select_one_day_previous(hosxp, kphis);
    query1_all(an, &sql, pool, "Select PreviousOneDayOrderItem")
        .await?
        .iter()
        .map(MedOrderItem::from_row)
        // .map(prev_oneday_order_item_from_row)
        .collect::<sqlx::Result<Vec<MedOrderItem>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select PreviousOneDayOrderItem"))
}
// fn prev_oneday_order_item_from_row(row: &MySqlRow) -> sqlx::Result<MedOrderItem> {
//     Ok(MedOrderItem {
//         icode: row.try_get("icode")?,
//         med_reconciliation_item_id: row.try_get("med_reconciliation_item_id")?,
//         med_name: row.try_get("med_name")?,
//         generic_name: row.try_get("generic_name")?,
//         order_item_detail: row.try_get("order_item_detail")?,
//         order_item_type: row.try_get("order_item_type")?,
//         off_by_datetime: row.try_get("off_by_datetime")?,
//         displaycolor: None,

//         old_drugusage
//         receive_from
//         receive_date
//         receive_qty
//         last_dose_taken_time
//         last_dose_taken_remark
//     })
// }

// ipd-dr-order-one-day-save.php
// ipd-dr-order-continuous-save.php
pub async fn post_order(save: &OrderSave, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<(u32, Vec<ExecuteResponse>), AppError> {
    match &save.visit_type {
        VisitTypeId::Ipd(an) | VisitTypeId::PreAdmit(an) => {
            let order_id;
            let mut results = Vec::with_capacity(3);
            if let Some(id) = save.order_id.and_then(zero_none) {
                order_id = id;
                match get_order_confirm(order_id, pool, kphis).await? {
                    Some(yn) => {
                        if yn == *"Y" {
                            return Err(Source::App.to_error(304, "Already Confirmed", "Update Order"));
                        } else {
                            let update_result = update_order(order_id, &save.order_doctor, user, pool, kphis).await?;
                            results.push(ExecuteResponse::from_query_result(update_result, "Update Order"));

                            let delete_result = delete_order_item(order_id, pool, kphis).await?;
                            results.push(ExecuteResponse::from_query_result(delete_result, "Delete OrderItem"));
                        }
                    }
                    None => {
                        return Err(AppError::app_404("Update Order"));
                    }
                }
            } else {
                let insert_result = insert_order(an, save, user, pool, kphis).await?;
                order_id = insert_result.last_insert_id() as u32;
                results.push(ExecuteResponse::from_query_result(insert_result, "Insert Order"));
            }
            if order_id > 0 {
                let insert_order_item_result = insert_order_items(order_id, an, &save.order_items, user, pool, kphis).await?;
                results.push(ExecuteResponse::from_query_result(insert_order_item_result, "Insert OrderItem"));
            }

            Ok((order_id, results))
        }
        VisitTypeId::OpdEr(_, _) | VisitTypeId::Visit(_) => Err(AppError::app_400("Post Order")),
    }
}

async fn get_order_confirm(order_id: u32, pool: &Pool<MySql>, kphis: &str) -> Result<Option<String>, AppError> {
    let check_order_sql = order::get_order_confirm(kphis);
    sqlx::query(AssertSqlSafe(check_order_sql))
        .bind(order_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select ConfirmOrder"))?
        .map(|row| row.try_get::<Option<String>, &str>("order_confirm"))
        .transpose()
        .map(|opt| opt.flatten())
        .map_err(|e| Source::SQLx.to_error(500, e, "Select ConfirmOrder"))
}

async fn update_order(order_id: u32, order_doctor: &str, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let update_sql = order::update_order(kphis);
    sqlx::query(AssertSqlSafe(update_sql))
        .bind(order_doctor)
        .bind(user)
        .bind(order_id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Update Order"))
}

async fn insert_order(an: &str, save: &OrderSave, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let insert_sql = order::insert_order(kphis);
    execute6(an, &save.order_doctor, &save.order_type, &save.order_owner_type, user, user, &insert_sql, pool, "Insert Order").await
}

pub async fn insert_orders_only_bundle(an: &str, orders_only: &[OrderOnly], pool: &Pool<MySql>, kphis: &str, kphis_extra: &str) -> Result<Vec<MySqlQueryResult>, AppError> {
    let mut results = Vec::new();

    // Assumption is `off-order MUST comes after offed-order` so order_item_id of `off-order` MUST more than `offed-order`
    // 1. Insert all `order` and create `order_id_map`
    // 2. Collect all `order_item` and sort by `order_item_id`
    // 3. Iterate over order_item and update `off_order_item_id` with `order_item_id_map`
    // 4. Insert `order_item`, `index_plan` and `index_action`

    // insert `order` and collect new/old order_id in `order_id_map`
    // key = old order_id, value = new order_id
    let mut order_id_map = HashMap::new();
    for order in orders_only.iter() {
        let order_result = insert_order_only(an, order, pool, kphis).await?;
        let new_order_id = order_result.last_insert_id() as u32;
        order_id_map.insert(order.order_id, new_order_id);
        results.push(order_result);
    }

    // collect `offed-order` over sorted `order_item_id` and then use it when `off-order` was detected
    // key = old order_item_id, value = new order_item_id
    let mut order_item_id_map = HashMap::new();
    let mut order_items = orders_only.iter().flat_map(|order| order.order_items.clone()).collect::<Vec<OrderItemOnly>>();
    order_items.sort_by(|a, b| a.order_item_id.cmp(&b.order_item_id));

    for order_item in order_items.iter_mut() {
        if let Some(old_order_id) = order_item.order_id {
            if let Some(new_order_id) = order_id_map.get(&old_order_id) {
                // update off_order_item_id
                if let Some(off_order_item_id) = order_item.off_order_item_id {
                    if let Some(new_off_order_item_id) = order_item_id_map.get(&off_order_item_id) {
                        order_item.off_order_item_id = Some(*new_off_order_item_id);
                    }
                }
                // insert `order_item`
                let order_item_result = insert_order_item_only(*new_order_id, an, order_item, pool, kphis).await?;
                let new_order_item_id = order_item_result.last_insert_id() as u32;
                // insert `order_item_id` to `order_item_id_map` for updating `off_order_item_id` in the next iteration
                order_item_id_map.insert(order_item.order_item_id, new_order_item_id);
                results.push(order_item_result);
                // insert `index_plan`
                for index_plan in order_item.index_plans.iter_mut() {
                    let index_plan_result = insert_index_plan_only(new_order_item_id, an, index_plan, pool, kphis).await?;
                    let new_index_plan_id = index_plan_result.last_insert_id() as u32;
                    results.push(index_plan_result);
                    // insert `index_action` and `index_monitor`
                    for index_action in index_plan.index_actions.iter_mut() {
                        let index_action_result = insert_index_action_only(new_index_plan_id, an, index_action, pool, kphis).await?;
                        let new_index_action_id = index_action_result.last_insert_id() as u32;
                        results.push(index_action_result);
                        let index_monitor_result = insert_index_monitors_only(new_index_action_id, an, &index_action.index_monitors, pool, kphis_extra).await?;
                        results.push(index_monitor_result);
                    }
                }
            }
        }
    }

    Ok(results)
}

async fn insert_order_only(an: &str, only: &OrderOnly, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    only.insert(Some("order_id"), Some("ipd_order"), ",an", ",?", &[an], pool, kphis)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert OrderOnly"))
}

async fn insert_order_items(order_id: u32, an: &str, order_items: &[OrderItemSave], user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let insert_order_item_sql = order::insert_order_items(order_items.len(), kphis);
    let mut query = sqlx::query(AssertSqlSafe(insert_order_item_sql));
    for order_item in order_items {
        query = query
            .bind(order_id)
            .bind(an)
            .bind(&order_item.order_item_type)
            .bind(&order_item.order_item_detail)
            .bind(&order_item.stat)
            .bind(order_item.off_order_item_id)
            .bind(&order_item.icode)
            .bind(order_item.med_reconciliation_item_id)
            .bind(order_item.first_qty)
            .bind(order_item.qty)
            .bind(user)
            .bind(user);
    }
    query.execute(pool).await.map_err(|e| Source::SQLx.to_error(500, e, "Insert OrderItem"))
}

async fn insert_order_item_only(order_id: u32, an: &str, only: &mut OrderItemOnly, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    only.order_id = Some(order_id);
    only.insert(Some("order_item_id"), Some("ipd_order_item"), ",an", ",?", &[an], pool, kphis)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert OrderItemsOnly"))
}

// async fn insert_order_items_only(order_id: u32, an: &str, order_items_only: &[OrderItemOnly], pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
//     let insert_order_item_sql = order::insert_order_items_only(order_id, an, order_items_only, kphis);
//     sqlx::query(AssertSqlSafe(insert_order_item_sql))
//         .execute(pool)
//         .await
//         .map_err(|e| Source::SQLx.to_error(500, e, "Insert OrderItemsOnly"))
// }

// ipd-dr-order-continuous-order-to-home-med-data.php
pub async fn get_home_med_from_cont(an: &str, pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<Vec<MedOrderItem>, AppError> {
    let sql = order::get_home_med_from_cont(hosxp, kphis);
    query1_all(an, &sql, pool, "Select MedOrderItem")
        .await?
        .iter()
        .map(MedOrderItem::from_row)
        .collect::<sqlx::Result<Vec<MedOrderItem>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select MedOrderItem"))
}

// ipd-dr-order-one-day-confirm.php, ipd-dr-order-continuous-confirm.php
// ipd-dr-order-one-day-nurse_accept.php, ipd-dr-order-continuous-nurse_accept.php
// ipd-dr-order-one-day-pharmacist_accept.php, ipd-dr-order-continuous-pharmacist_accept.php
// ipd-dr-order-one-day-pharmacist_done.php, ipd-dr-order-continuous-pharmacist_done.php
// PATCH /ipd/order/order
pub async fn patch_order(payload: &OrderPatch, doctor_code: &Option<String>, med_rec_icode: &str, user: &str, pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<Vec<ExecuteResponse>, AppError> {
    if match payload.action {
        OrderPatchAction::ConfirmAs | OrderPatchAction::EditAs => payload.order_id > 0 && payload.nurse_order_as.is_some(),
        // payload.order_id > 0 will update all order_id
        OrderPatchAction::DoctorConfirm => true,
        OrderPatchAction::Confirm | OrderPatchAction::NurseAccept | OrderPatchAction::PharmacistAccept | OrderPatchAction::PharmacistCheck | OrderPatchAction::PharmacistDone => payload.order_id > 0,
    } {
        let mut results = Vec::with_capacity(2 + (payload.medplans.len() * 2));
        // Confirm         : (order_time),                            doctor_code, loginname, order_id
        // ConfirmAs       : (order_time), nurse_order_as,            doctor_code, loginname, order_id
        // EditAs          :               nurse_order_as,                         loginname, order_id
        // DoctorConfirm   : (order_time),                 loginname, doctor_code,           (order_id)
        // NurseAccept     : (order_time),                            doctor_code, loginname, order_id
        // PharmacistAccept: (order_time),                            doctor_code, loginname, order_id
        // PharmacistCheck : (order_time),                            doctor_code, loginname, order_id
        // PharmacistDone  : (order_time),                            doctor_code, loginname, order_id
        let is_fixed_time = payload.order_time.is_some();
        let (sql, need_medplan) = match payload.action {
            OrderPatchAction::Confirm => (order::update_confirm_order(is_fixed_time, kphis), false),
            OrderPatchAction::ConfirmAs => (order::update_confirm_order_as(is_fixed_time, kphis), false),
            OrderPatchAction::EditAs => (order::update_edit_order_as(kphis), false),
            OrderPatchAction::DoctorConfirm => (order::update_doctor_confirm_order(payload.order_id > 0, is_fixed_time, kphis), false),
            OrderPatchAction::NurseAccept => (order::update_nurse_accept_order(is_fixed_time, kphis), false),
            OrderPatchAction::PharmacistAccept => (order::update_pharmacist_accept_order(is_fixed_time, kphis), true),
            OrderPatchAction::PharmacistCheck => (order::update_pharmacist_check_order(is_fixed_time, kphis), false),
            OrderPatchAction::PharmacistDone => (order::update_pharmacist_done_order(is_fixed_time, kphis), false),
        };
        let mut query = sqlx::query(AssertSqlSafe(sql));
        if !matches!(payload.action, OrderPatchAction::EditAs) {
            if let Some(order_time) = payload.order_time {
                query = query.bind(order_time);
            }
        }
        if matches!(payload.action, OrderPatchAction::ConfirmAs | OrderPatchAction::EditAs) {
            query = query.bind(payload.nurse_order_as.clone().unwrap_or_default());
        }
        if payload.action == OrderPatchAction::DoctorConfirm {
            query = query.bind(user);
        }
        if payload.action != OrderPatchAction::EditAs {
            query = query.bind(doctor_code);
        }
        if payload.action != OrderPatchAction::DoctorConfirm {
            query = query.bind(user);
        }
        if payload.order_id > 0 {
            query = query.bind(payload.order_id);
        }
        let update_result = query.execute(pool).await.map_err(|e| Source::SQLx.to_error(500, e, "Update Order"))?;
        results.push(ExecuteResponse::from_query_result(update_result, "Update Order"));

        if need_medplan {
            for medplan in &payload.medplans {
                if medplan.order_item_id > 0 {
                    results.push(update_order_item_qty(&medplan.first_qty, &medplan.qty, medplan.order_item_id, user, pool, kphis).await?);
                }
                results.push(insert_medplan_ipd(&medplan, med_rec_icode, user, pool, hosxp).await?);
            }
            if !payload.off_med_plan_numbers.is_empty() {
                results.push(off_medplan_ipd(&payload.off_med_plan_numbers, &payload.order_time, pool, hosxp).await?);
            }
        }

        Ok(results)
    } else {
        Err(AppError::app_400("Patch Order"))
    }
}

pub async fn update_order_item_nurse_assign(payload: &OrderItemPatch, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<ExecuteResponse, AppError> {
    let sql = order::update_order_item_nurse_assign(kphis);
    sqlx::query(AssertSqlSafe(sql))
        .bind(&payload.nurse_assign)
        .bind(user)
        .bind(payload.order_item_id)
        .execute(pool)
        .await
        .map(|res| ExecuteResponse::from_query_result(res, "Update OrderItemNurseAssign"))
        .map_err(|e| Source::SQLx.to_error(500, e, "Update OrderItemNurseAssign"))
}

pub async fn update_order_item_type(payload: &OrderItemPatch, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<ExecuteResponse, AppError> {
    let sql = order::update_order_item_type(kphis);
    sqlx::query(AssertSqlSafe(sql))
        .bind(&payload.order_item_type)
        .bind(user)
        .bind(payload.order_item_id)
        .execute(pool)
        .await
        .map(|res| ExecuteResponse::from_query_result(res, "Update OrderItemType"))
        .map_err(|e| Source::SQLx.to_error(500, e, "Update OrderItemType"))
}

pub async fn update_order_item_due_doctor(payload: &OrderItemPatch, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<ExecuteResponse, AppError> {
    let sql = order::update_order_item_due_doctor(kphis);
    sqlx::query(AssertSqlSafe(sql))
        .bind(&payload.due_doctor)
        .bind(&payload.due_doctor_note)
        .bind(user)
        .bind(payload.order_item_id)
        .execute(pool)
        .await
        .map(|res| ExecuteResponse::from_query_result(res, "Update OrderItemDueDoctor"))
        .map_err(|e| Source::SQLx.to_error(500, e, "Update OrderItemDueDoctor"))
}

pub async fn update_order_item_due_pharm(payload: &OrderItemPatch, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<ExecuteResponse, AppError> {
    let sql = order::update_order_item_due_pharm(kphis);
    sqlx::query(AssertSqlSafe(sql))
        .bind(&payload.due_pharm)
        .bind(&payload.due_pharm_note)
        .bind(user)
        .bind(payload.order_item_id)
        .execute(pool)
        .await
        .map(|res| ExecuteResponse::from_query_result(res, "Update OrderItemDuePharm"))
        .map_err(|e| Source::SQLx.to_error(500, e, "Update OrderItemDuePharm"))
}

pub async fn update_order_item_qty(first_qty: &Option<i32>, qty: &Option<i32>, order_item_id: u32, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<ExecuteResponse, AppError> {
    let sql = order::update_order_item_qty(kphis);
    sqlx::query(AssertSqlSafe(sql))
        .bind(first_qty)
        .bind(qty)
        .bind(user)
        .bind(order_item_id)
        .execute(pool)
        .await
        .map(|res| ExecuteResponse::from_query_result(res, "Update OrderItemQty"))
        .map_err(|e| Source::SQLx.to_error(500, e, "Update OrderItemQty"))
}

// ipd-pharmacy-order-monitor-table.php
pub async fn get_pharmacy_order(params: &IpdOrderPharmacyParams, hlen: usize, alen: usize, pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<IpdOrderPharmacyMonitor, AppError> {
    let orders_sql = order::select_pharmacy_order(params, hlen, alen, hosxp, kphis);
    let admits_sql = order::select_admit_history(params, hlen, alen, hosxp, kphis);

    let mut orders_query = sqlx::query(AssertSqlSafe(orders_sql));
    let mut admits_query = sqlx::query(AssertSqlSafe(admits_sql));

    if let Some(patient) = params.patient.as_ref().and_then(|s| urlencoding::decode(s).ok()) {
        let wildcard = ["%", patient.trim(), "%"].concat();
        match patient.parse::<u64>().is_ok() {
            true => {
                if patient.len() == 13 {
                    orders_query = orders_query.bind(patient.clone());
                    admits_query = admits_query.bind(patient);
                } else {
                    match hlen.cmp(&alen) {
                        Ordering::Equal => {
                            orders_query = orders_query.bind(wildcard.clone()).bind(wildcard.clone());
                            admits_query = admits_query.bind(wildcard.clone()).bind(wildcard.clone());
                        }
                        Ordering::Greater | Ordering::Less => {
                            orders_query = orders_query.bind(wildcard.clone());
                            admits_query = admits_query.bind(wildcard.clone());
                        }
                    }
                }
            }
            false => {
                orders_query = orders_query.bind(wildcard.clone());
            }
        }
    } else if let Some(doctor_in_charge) = params.doctor_in_charge.as_ref() {
        orders_query = orders_query.bind(doctor_in_charge);
    }
    if let Some(order_date_from) = params.order_date_from.as_ref() {
        orders_query = orders_query.bind(order_date_from);
    }
    if let Some(order_date_to) = params.order_date_to.as_ref() {
        orders_query = orders_query.bind(order_date_to).bind(order_date_to);
    }

    let orders = orders_query
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select OrderPharmacy"))?
        .iter()
        .map(IpdOrderPharmacy::from_row)
        .collect::<sqlx::Result<Vec<IpdOrderPharmacy>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select OrderPharmacy"))?;

    let admits = if let Some(patient) = &params.patient {
        if patient.parse::<u64>().is_ok() {
            admits_query
                .fetch_all(pool)
                .await
                .map_err(|e| Source::SQLx.to_error(500, e, "Select PharmacyIptPrev"))?
                .iter()
                .map(PharmacyIpt::from_row)
                .collect::<sqlx::Result<Vec<PharmacyIpt>>>()
                .map_err(|e| Source::SQLx.to_error(500, e, "Select PharmacyIptPrev"))?
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };

    Ok(IpdOrderPharmacyMonitor { orders, admits })
}

// ipd-dr-order-one-day-delete.php, ipd-dr-order-continuous-delete.php
/// delete order and order_item
pub async fn delete_order(order_id: u32, pool: &Pool<MySql>, kphis: &str) -> Result<ExecuteResponse, AppError> {
    let sql = order::delete_order(kphis);
    let delete_result = sqlx::query(AssertSqlSafe(sql))
        .bind(order_id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete Order"))?;

    Ok(ExecuteResponse::from_query_result(delete_result, "Delete Order"))
}

/// delete order, order_item, index_plan and index_action<br>
/// an
pub async fn delete_order_bundle(an: &str, pool: &Pool<MySql>, kphis: &str) -> Result<ExecuteResponse, AppError> {
    let sql = order::delete_order_bundle(kphis);
    let delete_result = sqlx::query(AssertSqlSafe(sql))
        .bind(an)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete OrderBundle"))?;

    Ok(ExecuteResponse::from_query_result(delete_result, "Delete OrderBundle"))
}

async fn delete_order_item(order_id: u32, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let delete_sql = order::delete_order_item(kphis);
    sqlx::query(AssertSqlSafe(delete_sql))
        .bind(order_id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete OrderItem"))
}

async fn insert_medplan_ipd(medplan: &MedPlanItem, med_rec_icode: &str, user: &str, pool: &Pool<MySql>, hosxp: &str) -> Result<ExecuteResponse, AppError> {
    let is_ivfluid = medplan.order_item_type.as_ref().map(|s| s == "ivfluid").unwrap_or_default();
    let (drugusage_opt, order_item_detail_opt) = if let Some(order_item_detail) = opt_empty_none(medplan.order_item_detail.clone()) {
        if medplan.icode.as_ref().map(|icode| icode == med_rec_icode).unwrap_or_default() {
            (None, Some([&medplan.med_name.clone().unwrap_or_default(), " ", &order_item_detail].concat()))
        } else {
            (if is_ivfluid { None } else { get_drugusage(&order_item_detail, pool, hosxp).await? }, Some(order_item_detail))
        }
    } else {
        (None, None)
    };
    let (drugusage, sp_use) = match drugusage_opt {
        Some(du) => (Some(du), None),
        None => {
            let sp_use_opt = if let Some(order_item_detail) = opt_empty_none(order_item_detail_opt) {
                if let Some(sp_use) = bump_and_get_sp_use(pool, hosxp).await? {
                    let (line1, line2, line3) = split_to_three(&order_item_detail, is_ivfluid);
                    if insert_sp_use(&sp_use, &line1, &line2, &line3, user, pool, hosxp).await.is_ok() {
                        Some(sp_use)
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            };
            (None, sp_use_opt)
        }
    };
    insert_medplan_ipd_inner(medplan, drugusage, sp_use, pool, hosxp).await
}

async fn insert_medplan_ipd_inner(medplan: &MedPlanItem, drugusage: Option<String>, sp_use: Option<String>, pool: &Pool<MySql>, hosxp: &str) -> Result<ExecuteResponse, AppError> {
    if let Some(serial) = bump_and_get_serial("med_plan_number", pool, hosxp).await? {
        let sql = order::insert_medplan_ipd(hosxp);
        let order_date_midnight = medplan.order_date.map(|order_date| PrimitiveDateTime::new(order_date, Time::MIDNIGHT));
        let order_datetime = medplan.order_date.map(|order_date| PrimitiveDateTime::new(order_date, medplan.order_time.unwrap_or(Time::MIDNIGHT)));
        sqlx::query(AssertSqlSafe(sql))
            .bind(serial)
            .bind(&medplan.an)
            .bind(&medplan.order_doctor)
            .bind(&medplan.icode)
            .bind(&medplan.qty)
            .bind((medplan.order_type == Some(String::from("oneday"))).then(|| order_date_midnight))
            .bind(order_date_midnight)
            .bind(if medplan.order_type == Some(String::from("oneday")) { "S" } else { "C" })
            .bind(drugusage)
            .bind(sp_use)
            .bind(&medplan.first_qty)
            .bind(order_datetime)
            .bind(order_datetime)
            .execute(pool)
            .await
            .map(|result| ExecuteResponse::from_query_result(result, "Insert MedPlanIpd"))
            .map_err(|e| Source::SQLx.to_error(500, e, "Insert MedPlanIpd"))
    } else {
        Err(Source::App.to_error(500, "Cannot create serial number", "Insert MedPlanIpd"))
    }
}

async fn off_medplan_ipd(med_plan_numbers: &[i32], order_time: &Option<Time>, pool: &Pool<MySql>, hosxp: &str) -> Result<ExecuteResponse, AppError> {
    let sql = order::update_medplan_ipd_off(med_plan_numbers, hosxp);
    let dt_now = now();
    let offdate = PrimitiveDateTime::new(dt_now.date(), Time::MIDNIGHT);
    let last_update = PrimitiveDateTime::new(dt_now.date(), order_time.unwrap_or(dt_now.time()));
    sqlx::query(AssertSqlSafe(sql))
        .bind(offdate)
        .bind(last_update)
        .execute(pool)
        .await
        .map(|result| ExecuteResponse::from_query_result(result, "Update MedPlanIpdOff"))
        .map_err(|e| Source::SQLx.to_error(500, e, "Update MedPlanIpdOff"))
}

async fn get_drugusage(order_item_detail: &str, pool: &Pool<MySql>, hosxp: &str) -> Result<Option<String>, AppError> {
    let sql = order::select_drugusage(hosxp);
    sqlx::query(AssertSqlSafe(sql))
        // order_item_detail is `utf8` but `name1`+`name2`+`name2` is `tis-620`
        .bind(sanity_dot_space(&sanity_tis620(order_item_detail)))
        .fetch_optional(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select Drugusage"))?
        .map(|row| row.try_get::<String, usize>(0))
        .transpose()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select Drugusage"))
}

async fn insert_sp_use(sp_use: &str, line1: &str, line2: &str, line3: &str, user: &str, pool: &Pool<MySql>, hosxp: &str) -> Result<ExecuteResponse, AppError> {
    let sql = order::insert_sp_use(hosxp);
    sqlx::query(AssertSqlSafe(sql))
        .bind(sp_use)
        .bind(line1)
        .bind(line2)
        .bind(line3)
        .bind(user)
        .execute(pool)
        .await
        .map(|result| ExecuteResponse::from_query_result(result, "Insert SpUse"))
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert SpUse"))
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;
    use kphis_sqlx_tester::MySqlTester;
    use kphis_util::datetime::date_8601;
    use time::macros::{date, time};

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_order_date() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_progress_note.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_progress_note.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_order_date("660001234", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(found.len(), 5); // 4 order + 1 progress
        let not_found = get_order_date("660006666", &tester.db_pool, &tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_order() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/system_ac_role.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/system_acc_role_user.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/ipt.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/iptadm.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/ward.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/patient.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/opduser.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/system_ac_role.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/system_acc_role_user.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/ipt.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/iptadm.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/ward.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/patient.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/opduser.sql")).execute(&tester.db_pool).await.unwrap();

        let default = get_order(&OrderParams::default(),&None,&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(default.len(), 16);
        let found_order_id = get_order(&OrderParams {order_id: Some(1),..Default::default()},&None,&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_order_id.len(), 1);
        let found_an = get_order(&OrderParams {an: Some(String::from("660001234")),..Default::default()},&None,&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_an.len(), 7);
        let found_current_date = get_order(&OrderParams {current_date: date_8601("2024-11-05"),..Default::default()},&None,&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_current_date.len(), 2);
        let found_order_confirm_y = get_order(&OrderParams {order_confirm: Some(String::from("Y")),..Default::default()},&None,&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_order_confirm_y.len(), 14);
        let found_order_confirm_n = get_order(&OrderParams {order_confirm: Some(String::from("N")),..Default::default()},&None,&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_order_confirm_n.len(), 2);
        let found_order_type = get_order(&OrderParams {order_type: Some(String::from("continuous")),..Default::default()},&None,&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_order_type.len(), 3);
        let found_order_owner_type = get_order(&OrderParams {order_owner_types: Some(String::from("nurse")),..Default::default()},&None,&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_order_owner_type.len(), 5);

        // order_owner_type IN ('doctor','nurse')
        let found_order_view_by_doctor = get_order(&OrderParams {view_by: Some(String::from("doctor")),..Default::default()},&None,&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_order_view_by_doctor.len(), 16);
        // order_owner_type IN ('doctor','nurse')
        let found_order_view_by_nurse = get_order(&OrderParams {view_by: Some(String::from("nurse")),..Default::default()},&None,&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_order_view_by_nurse.len(), 16);
        // order_owner_type IN ('doctor','nurse') && order_confirm = 'Y'
        let found_order_view_by_pharm = get_order(&OrderParams {view_by: Some(String::from("pharmacist")),..Default::default()},&None,&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_order_view_by_pharm.len(), 14);
        // order_owner_type IN ('doctor','nurse') && order_confirm = 'Y'
        let found_order_view_by_other = get_order(&OrderParams {view_by: Some(String::from("other")),..Default::default()},&None,&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_order_view_by_other.len(), 14);
        let order_view_by_unknown = get_order(&OrderParams {view_by: Some(String::from("xxxx")),..Default::default()},&None,&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert!(order_view_by_unknown.is_empty());

        let order_not_confirm_as = get_order(&OrderParams {doctor_not_confirm_as: Some(String::from("Y")),..Default::default()},&Some(String::from("007")),&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(order_not_confirm_as.len(), 1);

        let not_found = get_order(&OrderParams {order_id: Some(999),..Default::default()},&None,&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_order_only() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_order.sql")).execute(&tester.db_pool).await.unwrap();

        let found_an = get_order_only("660001234",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(found_an.len(), 7);
        let not_found = get_order_only("660006666",&tester.db_pool,&tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_order_types() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_order_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_order_item_type.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_order_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_order_item_type.sql")).execute(&tester.db_pool).await.unwrap();

        let default = get_order_types(&[], &None, &tester.db_pool, &tester.kphis).await.unwrap();
        assert!(default.is_empty());
        let found = get_order_types(&[1],&Some(String::from("continuous")),&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(found.len(), 1);
        let not_found = get_order_types(&[1],&Some(String::from("oneday")),&tester.db_pool,&tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_order_item() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_order_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_med_reconciliation_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/kphis_drug_use_duration.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/drugitems.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/ipt.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/opd_allergy.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_order_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_med_reconciliation_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/kphis_drug_use_duration.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/drugitems.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/ipt.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/opd_allergy.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();

        // order_date only used for display offed details
        let default = get_order_item(None,None,&OrderParams::default(),&[],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(default.len(), 18);
        let found_an = get_order_item(None,None,&OrderParams {an: Some(String::from("660001234")),..Default::default()},&[],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_an.len(), 10);
        let found_order_id = get_order_item(Some(1),None,&OrderParams {an: Some(String::from("660001234")),..Default::default()},&[],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_order_id.len(), 1);
        let found_order_item_id = get_order_item(None,None,&OrderParams {order_item_id: Some(1),an: Some(String::from("660001234")),..Default::default()},&[],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_order_item_id.len(), 1);
        let found_order_type = get_order_item(None,None,&OrderParams {order_type: Some(String::from("continuous")),an: Some(String::from("660001234")),..Default::default()},&[],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();assert_eq!(found_order_type.len(), 5);
        let found_order_item_type = get_order_item(None,Some(String::from("med")),&OrderParams {an: Some(String::from("660001234")),..Default::default()},&[],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_order_item_type.len(), 3);
        let not_found = get_order_item(None,None,&OrderParams {an: Some(String::from("660006666")),..Default::default()},&[],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_order_item_ids_by_an_and_plan_date() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_nurse_index_plan.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_nurse_index_plan.sql")).execute(&tester.db_pool).await.unwrap();

        let found_order_item_ids = select_order_item_ids_by_an_and_plan_date("670001234", date!(2024-11-01), &tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(found_order_item_ids.len(), 1);
        let not_found = select_order_item_ids_by_an_and_plan_date("670001234", date!(2024-01-01), &tester.db_pool,&tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_order_item_only() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_order_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_order_item.sql")).execute(&tester.db_pool).await.unwrap();

        let found_order_id = get_order_item_only(1,&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(found_order_id.len(), 1);
        let not_found = get_order_item_only(999,&tester.db_pool,&tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_previous_order() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_order_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/kphis_drug_use_duration.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_med_reconciliation_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/drugitems.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/ipt.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/opd_allergy.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_order_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/kphis_drug_use_duration.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_med_reconciliation_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/drugitems.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/ipt.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/opd_allergy.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();

        // o.order_confirm='Y' && oi.off_order_item_id IS NULL
        // MUST have 'current_date' and 'an' in params
        let default = get_previous_order(&OrderParams::default(),&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert!(default.is_empty());
        let found_not_offed = get_previous_order(&OrderParams {current_date: date_8601("2024-11-11"),an: Some(String::from("660001234")),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_not_offed.len(), 5); // not included 2 offed items and 2 offing item
        let found_with_offed = get_previous_order(&OrderParams {with_offed: Some(String::from("Y")),current_date: date_8601("2024-11-11"),an: Some(String::from("660001234")),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_with_offed.len(), 7);

        let found_order_type = get_previous_order(&OrderParams {order_type: Some(String::from("continuous")),current_date: date_8601("2024-11-11"),an: Some(String::from("660001234")),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_order_type.len(), 3);
        let found_order_item_types = get_previous_order(&OrderParams {order_item_types: Some(String::from("med")),current_date: date_8601("2024-11-11"),an: Some(String::from("660001234")),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_order_item_types.len(), 2);

        // o.order_owner_type='doctor'
        let found_view_by_doctor = get_previous_order(&OrderParams {view_by: Some(String::from("doctor")),current_date: date_8601("2024-11-11"),an: Some(String::from("660001234")),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_view_by_doctor.len(), 5);
        // offed + o.order_owner_type='doctor' OR o.order_owner_type='nurse'
        let found_view_by_nurse = get_previous_order(&OrderParams {view_by: Some(String::from("nurse")),current_date: date_8601("2024-11-11"),an: Some(String::from("660001234")),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_view_by_nurse.len(), 5);
        // offed + o.order_owner_type='doctor' OR o.order_owner_type='nurse'
        let found_view_by_pharm = get_previous_order(&OrderParams {view_by: Some(String::from("pharmacist")),current_date: date_8601("2024-11-11"),an: Some(String::from("660001234")),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_view_by_pharm.len(), 5);
        // offed + o.order_owner_type='doctor' OR o.order_owner_type='nurse'
        let found_view_by_other = get_previous_order(&OrderParams {view_by: Some(String::from("other")),current_date: date_8601("2024-11-11"),an: Some(String::from("660001234")),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_view_by_other.len(), 5);
        let view_by_unknown = get_previous_order(&OrderParams {view_by: Some(String::from("unknown")),current_date: date_8601("2024-11-11"),an: Some(String::from("660001234")),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert!(view_by_unknown.is_empty());

        let not_found = get_previous_order(&OrderParams {current_date: date_8601("2024-11-11"),an: Some(String::from("660006666")),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_previous_one_day_order() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_order_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_med_reconciliation_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/kphis_drug_use_duration.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/drugitems.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_order_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_med_reconciliation_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/kphis_drug_use_duration.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/drugitems.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_previous_one_day_order("670001234", &tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert_eq!(found.len(), 1);
        let not_found = get_previous_one_day_order("660006666", &tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_order_confirm() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_order.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_order_confirm(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert!(found.map(|s| s.as_str() == "Y").unwrap_or_default());
        let not_found = get_order_confirm(999, &tester.db_pool, &tester.kphis).await.unwrap();
        assert!(not_found.is_none());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_home_med_from_cont() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_order_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_med_reconciliation_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/kphis_drug_use_duration.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/drugitems.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_order_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_med_reconciliation_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/kphis_drug_use_duration.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/drugitems.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_home_med_from_cont("660001234", &tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert_eq!(found.len(), 2); // minus 1 offed
        let not_found = get_home_med_from_cont("660006666", &tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_pharmacy_order() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_order_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_doctor_in_charge.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_vs_vital_sign.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_focus_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_order_master.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/ipt.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/spclty.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/iptadm.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/patient.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/an_stat.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/ward.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/dchtype.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/dchstts.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/pttype.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_order_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_doctor_in_charge.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_vs_vital_sign.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_focus_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_order_master.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/ipt.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/spclty.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/iptadm.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/patient.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/an_stat.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/ward.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/dchtype.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/dchstts.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/pttype.sql")).execute(&tester.db_pool).await.unwrap();

        let default = get_pharmacy_order(&IpdOrderPharmacyParams::default(),7,9,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        // orders, new_last_order_time and ipts MUST have 'patient' or 'wards' in params OR params.inverse_ward_select = "Y"
        assert!(default.orders.is_empty());
        // MUST have numeric 'patient' in params + ipt.dchstts IS NOT NUL
        assert!(default.admits.is_empty());

        let found_hn = get_pharmacy_order(&IpdOrderPharmacyParams {patient: Some(String::from("1234")),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_hn.orders.len(), 2);
        assert_eq!(found_hn.admits.len(), 1);

        let found_an = get_pharmacy_order(&IpdOrderPharmacyParams {patient: Some(String::from("70001234")),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_an.orders.len(), 1);
        assert!(found_an.admits.is_empty());

        let found_cid = get_pharmacy_order(&IpdOrderPharmacyParams {patient: Some(String::from("1111111111111")),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_cid.orders.len(), 2);
        assert_eq!(found_cid.admits.len(), 1);

        let found_name = get_pharmacy_order(&IpdOrderPharmacyParams {patient: Some(String::from("มุติ")),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_name.orders.len(), 2);
        assert!(found_an.admits.is_empty());

        let found_wards = get_pharmacy_order(&IpdOrderPharmacyParams {wards: Some(String::from("01")),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_wards.orders.len(), 2);

        let found_inverse_ward_selection = get_pharmacy_order(&IpdOrderPharmacyParams {inverse_ward_select: Some(String::from("Y")),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_inverse_ward_selection.orders.len(), 14);

        let found_doctor_in_charge = get_pharmacy_order(&IpdOrderPharmacyParams {doctor_in_charge: Some(String::from("007")),inverse_ward_select: Some(String::from("Y")),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_doctor_in_charge.orders.len(), 2);

        let found_order_date_from = get_pharmacy_order(&IpdOrderPharmacyParams {order_date_from: date_8601("2024-11-09"),inverse_ward_select: Some(String::from("Y")),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_order_date_from.orders.len(), 13);

        let found_order_date_to = get_pharmacy_order(&IpdOrderPharmacyParams {order_date_to: date_8601("2024-11-09"),inverse_ward_select: Some(String::from("Y")),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_order_date_to.orders.len(), 9);

        let found_order_date_between = get_pharmacy_order(&IpdOrderPharmacyParams {order_date_from: date_8601("2024-11-09"),order_date_to: date_8601("2024-11-11"),inverse_ward_select: Some(String::from("Y")),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_order_date_between.orders.len(), 8);

        let found_is_discharged = get_pharmacy_order(&IpdOrderPharmacyParams {is_discharged: Some(String::from("Y")),inverse_ward_select: Some(String::from("Y")),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_is_discharged.orders.len(), 1);

        let found_not_discharge = get_pharmacy_order(&IpdOrderPharmacyParams {is_discharged: Some(String::from("N")),inverse_ward_select: Some(String::from("Y")),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_not_discharge.orders.len(), 13);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_order() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_order.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_order("660001234",&OrderSave::demo(), "user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = insert_order("660001234",&OrderSave::demo(), "user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_order_only() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_order.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_order_only("660001234", &OrderOnly::demo(), &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = insert_order_only("", &OrderOnly::demo(), &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_order_items() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_order_item.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_order_items(1,"660001234",&[OrderItemSave::demo()],"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = insert_order_items(1,"660001234",&[OrderItemSave::demo()],"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_order_item_only() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_order_item.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_order_item_only(1,"660001234",&mut OrderItemOnly::demo(),&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = insert_order_item_only(1,"",&mut OrderItemOnly::demo(),&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    // #[tokio::test]
    // #[ignore]
    // async fn sqlx_insert_order_items_only() {
    //     let tester = MySqlTester::new_kphis().await;
    //     sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_order_item.sql")).execute(&tester.db_pool).await.unwrap();

    //     let success = insert_order_items_only(1,"660001234",&[OrderItemOnly::demo()],&tester.db_pool,&tester.kphis).await.unwrap();
    //     assert_eq!(success.rows_affected(), 1);
    //     let again_success = insert_order_items_only(1,"",&[OrderItemOnly::demo()],&tester.db_pool,&tester.kphis).await.unwrap();
    //     assert_eq!(again_success.rows_affected(), 1);
    // }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_order() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_order.sql")).execute(&tester.db_pool).await.unwrap();

        let success = update_order(1, "007", "user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = update_order(1, "007", "user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_patch_order_now() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_order_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/medplan_ipd.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/drugusage.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/serial.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/sys_var.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_order_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/medplan_ipd.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/drugusage.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/serial.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/sys_var.sql")).execute(&tester.db_pool).await.unwrap();

        let mut patch = OrderPatch::demo();
        patch.order_time = None;
        patch.order_id = 0;
        let no_order_id = patch_order(&patch,&Some(String::from("007")),"1111111","user",&tester.db_pool,&tester.hosxp,&tester.kphis).await;
        assert!(no_order_id.is_err());
        patch.order_id = 15;
        let success_confirm = patch_order(&patch,&Some(String::from("007")),"1111111","user",&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(success_confirm[0].rows_affected, 1);

        patch.action = OrderPatchAction::ConfirmAs;
        patch.order_id = 16;
        patch.nurse_order_as = None;
        let confirm_as_no_order_as = patch_order(&patch,&Some(String::from("007")),"1111111","user",&tester.db_pool,&tester.hosxp,&tester.kphis).await;
        assert!(confirm_as_no_order_as.is_err());
        patch.nurse_order_as = Some(String::from("008"));
        let success_confirm_as = patch_order(&patch,&Some(String::from("007")),"1111111","user",&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(success_confirm_as[0].rows_affected, 1);

        patch.action = OrderPatchAction::EditAs;
        patch.nurse_order_as = Some(String::from("007"));
        let success_edit_as = patch_order(&patch,&Some(String::from("007")),"1111111","user",&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(success_edit_as[0].rows_affected, 1);

        patch.action = OrderPatchAction::DoctorConfirm;
        let success_doctor_confirm = patch_order(&patch,&Some(String::from("007")),"1111111","user",&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(success_doctor_confirm[0].rows_affected, 1);

        patch.action = OrderPatchAction::NurseAccept;
        patch.order_id = 15;
        let success_nurse_accept = patch_order(&patch,&Some(String::from("007")),"1111111","user",&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(success_nurse_accept[0].rows_affected, 1);

        patch.action = OrderPatchAction::PharmacistAccept;
        let success_pharm_accept = patch_order(&patch,&Some(String::from("007")),"1111111","user",&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(success_pharm_accept[0].rows_affected, 1);

        patch.action = OrderPatchAction::PharmacistDone;
        let success_pharm_done = patch_order(&patch,&Some(String::from("007")),"1111111","user",&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(success_pharm_done[0].rows_affected, 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_patch_order_fixed_time() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_order_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/medplan_ipd.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/drugusage.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/serial.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/sys_var.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_order_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/medplan_ipd.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/drugusage.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/serial.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/sys_var.sql")).execute(&tester.db_pool).await.unwrap();

        let mut patch = OrderPatch::demo();
        patch.order_id = 0;
        let no_order_id = patch_order(&patch,&Some(String::from("007")),"1111111","user",&tester.db_pool,&tester.hosxp,&tester.kphis).await;
        assert!(no_order_id.is_err());
        patch.order_id = 15;
        let success_confirm = patch_order(&patch,&Some(String::from("007")),"1111111","user",&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(success_confirm[0].rows_affected, 1);

        patch.action = OrderPatchAction::ConfirmAs;
        patch.order_id = 16;
        patch.nurse_order_as = None;
        let confirm_as_no_order_as = patch_order(&patch,&Some(String::from("007")),"1111111","user",&tester.db_pool,&tester.hosxp,&tester.kphis).await;
        assert!(confirm_as_no_order_as.is_err());
        patch.nurse_order_as = Some(String::from("008"));
        let success_confirm_as = patch_order(&patch,&Some(String::from("007")),"1111111","user",&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(success_confirm_as[0].rows_affected, 1);

        patch.action = OrderPatchAction::EditAs;
        patch.nurse_order_as = Some(String::from("007"));
        let success_edit_as = patch_order(&patch,&Some(String::from("007")),"1111111","user",&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(success_edit_as[0].rows_affected, 1);

        patch.action = OrderPatchAction::DoctorConfirm;
        let success_doctor_confirm = patch_order(&patch,&Some(String::from("007")),"1111111","user",&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(success_doctor_confirm[0].rows_affected, 1);

        patch.action = OrderPatchAction::NurseAccept;
        patch.order_id = 15;
        let success_nurse_accept = patch_order(&patch,&Some(String::from("007")),"1111111","user",&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(success_nurse_accept[0].rows_affected, 1);

        patch.action = OrderPatchAction::PharmacistAccept;
        let success_pharm_accept = patch_order(&patch,&Some(String::from("007")),"1111111","user",&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(success_pharm_accept[0].rows_affected, 1);

        patch.action = OrderPatchAction::PharmacistCheck;
        let success_pharm_check = patch_order(&patch,&Some(String::from("007")),"1111111","user",&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(success_pharm_check[0].rows_affected, 1);

        patch.action = OrderPatchAction::PharmacistDone;
        let success_pharm_done = patch_order(&patch,&Some(String::from("007")),"1111111","user",&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(success_pharm_done[0].rows_affected, 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_order_item_nurse_assign() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_order_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_order_item.sql")).execute(&tester.db_pool).await.unwrap();

        let mut patch = OrderItemPatch::demo();
        patch.order_item_id = 0;
        let no_order_item_id = update_order_item_nurse_assign(&patch,"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(no_order_item_id.rows_affected, 0);
        patch.order_item_id = 1;
        let success_nurse_assign = update_order_item_nurse_assign(&patch,"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success_nurse_assign.rows_affected, 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_order_item_type() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_order_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_order_item.sql")).execute(&tester.db_pool).await.unwrap();

        let mut patch = OrderItemPatch::demo();
        patch.order_item_id = 0;
        let no_order_item_id = update_order_item_type(&patch,"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(no_order_item_id.rows_affected, 0);
        patch.order_item_id = 1;
        let success_nurse_assign = update_order_item_type(&patch,"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success_nurse_assign.rows_affected, 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_order_item_due_doctor() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_order_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_order_item.sql")).execute(&tester.db_pool).await.unwrap();

        let mut patch = OrderItemPatch::demo();
        patch.order_item_id = 0;
        let no_order_item_id = update_order_item_due_doctor(&patch,"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(no_order_item_id.rows_affected, 0);
        patch.order_item_id = 1;
        let success_nurse_assign = update_order_item_due_doctor(&patch,"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success_nurse_assign.rows_affected, 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_order_item_due_pharm() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_order_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_order_item.sql")).execute(&tester.db_pool).await.unwrap();

        let mut patch = OrderItemPatch::demo();
        patch.order_item_id = 0;
        let no_order_item_id = update_order_item_due_pharm(&patch,"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(no_order_item_id.rows_affected, 0);
        patch.order_item_id = 1;
        let success_nurse_assign = update_order_item_due_pharm(&patch,"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success_nurse_assign.rows_affected, 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_order_item_qty() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_order_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_order_item.sql")).execute(&tester.db_pool).await.unwrap();

        let no_order_item_id = update_order_item_qty(&Some(10),&Some(10),0,"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(no_order_item_id.rows_affected, 0);
        let success_nurse_assign = update_order_item_qty(&Some(10),&Some(10),1,"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success_nurse_assign.rows_affected, 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_patch_order_doctor_confirm_all() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_order.sql")).execute(&tester.db_pool).await.unwrap();

        let mut patch = OrderPatch::demo();
        patch.order_id = 0;
        patch.action = OrderPatchAction::DoctorConfirm;
        let no_doctor_confirm = patch_order(&patch,&Some(String::from("999")),"1111111","user",&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(no_doctor_confirm[0].rows_affected, 0);
        let success_doctor_confirm = patch_order(&patch,&Some(String::from("007")),"1111111","user",&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(success_doctor_confirm[0].rows_affected, 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_order() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_order_item.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_order_item.sql")).execute(&tester.db_pool).await.unwrap();

        let confirmed_failure = delete_order(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(confirmed_failure.rows_affected, 0);
        let success = delete_order(15, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected, 2);
        let again_not_found = delete_order(15, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_not_found.rows_affected, 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_order_bundle() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_order_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_nurse_index_plan.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_nurse_index_action.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_order_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_nurse_index_plan.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_nurse_index_action.sql")).execute(&tester.db_pool).await.unwrap();

        let success = delete_order_bundle("670001234", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected, 23); // 9 + 8 + 3 + 3
        let again_not_found = delete_order_bundle("670001234", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_not_found.rows_affected, 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_order_item() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_order_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_order_item.sql")).execute(&tester.db_pool).await.unwrap();

        let success = delete_order_item(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_not_found = delete_order_item(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_not_found.rows_affected(), 0);
    }

    // use serde::{Deserialize, Serialize};
    // #[derive(Debug, Serialize, Deserialize, FromRow)]
    // struct MedPlanIpd {
    //     med_plan_number: i32,
    //     an: Option<String>,
    //     doctor: Option<String>,
    //     icode: Option<String>,
    //     qty: Option<i32>,
    //     orderdate: Option<PrimitiveDateTime>,
    //     orderstatus: Option<String>,
    //     drugusage: Option<String>,
    //     first_qty: Option<i32>,
    //     last_update: Option<PrimitiveDateTime>,
    //     first_update: Option<PrimitiveDateTime>,
    // }
    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_medplan_ipd_inner() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/medplan_ipd.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/drugusage.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/serial.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/medplan_ipd.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/drugusage.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/serial.sql")).execute(&tester.db_pool).await.unwrap();

        let result = insert_medplan_ipd_inner(&MedPlanItem::demo(), Some(String::from("0000888")), Some(String::from("0000777")), &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(result.rows_affected, 1);
        let again = insert_medplan_ipd_inner(&MedPlanItem::demo(), Some(String::from("0000888")), Some(String::from("0000777")), &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(again.rows_affected, 1);
        // // use with MedPlanIpd above
        // let all = sqlx::query("SELECT * FROM hos.medplan_ipd;").fetch_all(&tester.db_pool).await.unwrap()
        //     .iter().map(MedPlanIpd::from_row).collect::<sqlx::Result<Vec<MedPlanIpd>>>().unwrap();
        // dbg!(all);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_off_medplan_ipd() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/medplan_ipd.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/medplan_ipd.sql")).execute(&tester.db_pool).await.unwrap();

        let success = off_medplan_ipd(&[1], &Some(time!(12:59:59)), &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(success.rows_affected, 1);
        let again = off_medplan_ipd(&[1], &Some(time!(12:59:59)), &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(again.rows_affected, 1);
        let empty = off_medplan_ipd(&[], &Some(time!(12:59:59)), &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(empty.rows_affected, 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_drugusage() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/drugusage.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/drugusage.sql")).execute(&tester.db_pool).await.unwrap();

        let found_exact = get_drugusage("รับประทาน ครั้งละ  1   เม็ด วันละ..2...เวลา หลังอาหารเช้า - เย็น", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(found_exact, Some(String::from("3333")));
        let found_neat = get_drugusage("รับประทาน ครั้งละ 1 เม็ด วันละ 2 เวลา หลังอาหารเช้า - เย็น", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(found_neat, Some(String::from("3333")));
        let found_varient = get_drugusage("รับประทาน  ครั้งละ ..1 ..เม็ด  วันละ 2   เวลา   หลังอาหารเช้า  - เย็น", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(found_varient, Some(String::from("3333")));
        let not_found = get_drugusage("1*9pc☑", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(not_found.is_none());
    }

    // use serde::{Deserialize, Serialize};
    // #[derive(Debug, Serialize, Deserialize, FromRow)]
    // struct SpUse {
    //     sp_use: String,
    //     sp_name: Option<String>,
    //     name1: Option<String>,
    //     name2: Option<String>,
    //     name3: Option<String>,
    //     user: Option<String>,
    // }
    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_sp_use() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/sp_use.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/serial.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/sp_use.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/serial.sql")).execute(&tester.db_pool).await.unwrap();

        let result = insert_sp_use("0000778", "รับประทาน","ครั้งละ 1 เม็ด", "เช้า กลางวัน เย็น", "user", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(result.rows_affected, 1);
        let again = insert_sp_use("0000779", "รับประทาน","ครั้งละ 1 เม็ด", "เช้า กลางวัน เย็น", "user", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(again.rows_affected, 1);
        // // use with SpUse above
        // let all = sqlx::query("SELECT * FROM hos.sp_use;").fetch_all(&tester.db_pool).await.unwrap()
        //     .iter().map(SpUse::from_row).collect::<sqlx::Result<Vec<SpUse>>>().unwrap();
        // dbg!(all);
    }
}
