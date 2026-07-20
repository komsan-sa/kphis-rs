use sqlx::{AssertSqlSafe, MySql, Pool, mysql::MySqlQueryResult};

use kphis_model::{
    fetch::ExecuteResponse,
    sse::SseGroup,
    user::config::{UserConfig, UserConfigResponse},
};
use kphis_sql::user::config;
use kphis_util::{
    datetime::get_timestamp_server,
    error::{AppError, Source},
};

use super::totp;

// POST user-config
pub async fn insert_dup_user_config(payload: &UserConfig, user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<(UserConfigResponse, Option<Option<String>>), AppError> {
    let (totp, pk, ts) = if payload.totp.unwrap_or_default() {
        let (qr, secret) = totp::new_totp_encoded_key(user, "KPHIS")?;
        (Some(qr), Some(secret), get_timestamp_server().ok())
    } else {
        (None, None, None)
    };

    let sql = config::insert_dup_config_user(payload.totp.is_some(), kphis_extra);
    let result = sqlx::query(AssertSqlSafe(sql))
        .bind(user)
        .bind(&payload.theme)
        .bind(&payload.wide_screen)
        .bind(&pk)
        .bind(ts)
        .bind(user)
        .bind(user)
        .execute(pool)
        .await
        .map(|res| ExecuteResponse::from_query_result(res, "Post User Config"))
        .map_err(|e| Source::SQLx.to_error(500, e, "Post User Config"))?;

    // Some is changed, None is not changed
    let changed_pk = if payload.totp.is_some() { Some(pk) } else { None };

    Ok((UserConfigResponse { totp, result }, changed_pk))
}

pub async fn insert_dup_config_sse(payload: &SseGroup, user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<ExecuteResponse, AppError> {
    let sql = config::insert_dup_config_sse(kphis_extra);
    let wards = if payload.wards.is_empty() { None } else { Some(payload.wards.join(",")) };
    let spcltys = if payload.spclty_ids.is_empty() {
        None
    } else {
        Some(payload.spclty_ids.iter().map(|u| u.to_string()).collect::<Vec<String>>().join(","))
    };
    sqlx::query(AssertSqlSafe(sql))
        .bind(user)
        .bind(wards)
        .bind(spcltys)
        .bind(user)
        .bind(user)
        .execute(pool)
        .await
        .map(|res| ExecuteResponse::from_query_result(res, "Post User SSE Group"))
        .map_err(|e| Source::SQLx.to_error(500, e, "Post User SSE Group"))
}

pub async fn update_ts(target_loginname: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<MySqlQueryResult, AppError> {
    let sql = config::update_ts(kphis_extra);
    sqlx::query(AssertSqlSafe(sql))
        .bind(get_timestamp_server().ok())
        .bind(target_loginname)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Update TS"))
}

pub async fn update_failed(failed: i8, target_loginname: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<MySqlQueryResult, AppError> {
    let sql = config::update_failed(kphis_extra);
    sqlx::query(AssertSqlSafe(sql))
        .bind(failed)
        .bind(target_loginname)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Update failed"))
}

pub async fn update_totp_done(target_loginname: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<MySqlQueryResult, AppError> {
    let sql = config::update_totp_done(kphis_extra);
    sqlx::query(AssertSqlSafe(sql))
        .bind(target_loginname)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Update TOTP Done"))
}

pub async fn remove_totp(target_loginname: &str, user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<ExecuteResponse, AppError> {
    let sql = config::remove_totp(kphis_extra);
    sqlx::query(AssertSqlSafe(sql))
        .bind(user)
        .bind(target_loginname)
        .execute(pool)
        .await
        .map(|res| ExecuteResponse::from_query_result(res, "Remove TOTP"))
        .map_err(|e| Source::SQLx.to_error(500, e, "Remove TOTP"))
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;
    use sqlx::Row;
    use kphis_sqlx_tester::MySqlTester;

    async fn select_user_config(user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> (Option<String>,Option<String>,Option<String>) {
        let sql = ["SELECT * FROM ",kphis_extra,".user_config WHERE loginname=?;"].concat();
        let row = sqlx::query(AssertSqlSafe(sql)).bind(user).fetch_one(pool).await.unwrap();
        let theme = row.try_get("theme").unwrap();
        let wide_screen = row.try_get("wide_screen").unwrap();
        let totp = row.try_get("totp").unwrap();
        (theme, wide_screen, totp)
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_dup_user_config() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/user_config.sql")).execute(&tester.db_pool).await.unwrap();

        // when payload.totp is Some(true), insert/update with a new PK, return new QR-CODE
        let mut payload = UserConfig::demo();
        let success = insert_dup_user_config(&payload,"user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert!(success.0.totp.is_some());
        assert_eq!(success.0.result.rows_affected, 1); // 1 is insert, 2 is update

        let again_success = insert_dup_user_config(&payload,"user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert!(again_success.0.totp.is_some());
        assert_eq!(again_success.0.result.rows_affected, 2); // 1 is insert, 2 is update
        let (_,_,totp) = select_user_config("user", &tester.db_pool, &tester.kphis_extra).await;
        assert!(totp.is_some());

        // when payload.totp is None, insert with totp = NULL, do not update totp, return NONE QR-CODE
        payload.totp = None;
        let again_without_totp = insert_dup_user_config(&payload,"user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert!(again_without_totp.0.totp.is_none());
        assert_eq!(again_without_totp.0.result.rows_affected, 2); // 1 is insert, 2 is update
        let (_,_,totp_again) = select_user_config("user", &tester.db_pool, &tester.kphis_extra).await;
        assert_eq!(totp_again, totp);

        // when payload.totp is Some(false), insert/update with totp = NULL, return NONE QR-CODE
        payload.totp = Some(false);
        let again_without_totp = insert_dup_user_config(&payload,"user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert!(again_without_totp.0.totp.is_none());
        assert_eq!(again_without_totp.0.result.rows_affected, 2); // 1 is insert, 2 is update
        let (_,_,totp_again) = select_user_config("user", &tester.db_pool, &tester.kphis_extra).await;
        assert!(totp_again.is_none()); 
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_dup_config_sse() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/user_config.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_dup_config_sse(&SseGroup::demo(),"user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected, 1);
        let again_success = insert_dup_config_sse(&SseGroup::demo(),"user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(again_success.rows_affected, 2);
        let again_empty = insert_dup_config_sse(&SseGroup {wards: Vec::new(), spclty_ids: Vec::new()},"user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(again_empty.rows_affected, 2);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_ts() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/user_config.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/user_config.sql")).execute(&tester.db_pool).await.unwrap();

        let no_totp = update_ts("user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(no_totp.rows_affected(), 0);

        let update_totp = insert_dup_user_config(&UserConfig::demo(),"user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert!(update_totp.0.totp.is_some());

        let success = update_ts("user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = update_ts("user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
        let not_found = update_ts("admin",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(not_found.rows_affected(), 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_failed() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/user_config.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/user_config.sql")).execute(&tester.db_pool).await.unwrap();

        let success = update_failed(99,"user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let not_found = update_failed(99,"admin",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(not_found.rows_affected(), 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_totp_done() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/user_config.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/user_config.sql")).execute(&tester.db_pool).await.unwrap();

        let no_totp = update_totp_done("user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(no_totp.rows_affected(), 0);

        let update_totp = insert_dup_user_config(&UserConfig::demo(),"user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert!(update_totp.0.totp.is_some());

        let success = update_totp_done("user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = update_totp_done("user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
        let not_found = update_totp_done("admin",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(not_found.rows_affected(), 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_remove_totp() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/user_config.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/user_config.sql")).execute(&tester.db_pool).await.unwrap();

        let success = remove_totp("user","admin",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected, 1);
        let again_success = remove_totp("user","admin",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(again_success.rows_affected, 1);
        let not_found = remove_totp("admin","user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(not_found.rows_affected, 0);
    }
}
