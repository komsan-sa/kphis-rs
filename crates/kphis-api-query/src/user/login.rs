use sqlx::{FromRow, MySql, Pool, Row, mysql::MySqlRow};

use kphis_model::user::his::{CurrentUserRole, UserDb};
use kphis_sql::user::login;
use kphis_util::error::{AppError, Source};

use crate::{image::image_from_row, query1_all, query1_opt};

// project/function/SessionManager.php::checklogin()
pub async fn get_user(username: &str, pool: &Pool<MySql>, hosxp: &str, kphis: &str, kphis_extra: &str) -> Result<Option<UserDb>, AppError> {
    let sql = login::check_login(hosxp, kphis, kphis_extra);

    query1_opt(username, &sql, pool, "Select User").await?.as_ref().map(user_row).transpose()
}
fn user_row(row: &MySqlRow) -> Result<UserDb, AppError> {
    let wards_str: Option<String> = row.try_get("wards").map_err(|e| Source::SQLx.to_error(500, e, "Select User"))?;
    let spcltys_str: Option<String> = row.try_get("spcltys").map_err(|e| Source::SQLx.to_error(500, e, "Select User"))?;
    Ok(UserDb {
        loginname: row.try_get("loginname").map_err(|e| Source::SQLx.to_error(500, e, "Select User"))?,
        passweb: row.try_get("passweb").map_err(|e| Source::SQLx.to_error(500, e, "Select User"))?,
        name: row.try_get("name").map_err(|e| Source::SQLx.to_error(500, e, "Select User"))?,
        doctorcode: row.try_get("doctorcode").map_err(|e| Source::SQLx.to_error(500, e, "Select User"))?,
        groupname: row.try_get("groupname").map_err(|e| Source::SQLx.to_error(500, e, "Select User"))?,
        accessright: row.try_get("accessright").map_err(|e| Source::SQLx.to_error(500, e, "Select User"))?,
        entryposition: row.try_get("entryposition").map_err(|e| Source::SQLx.to_error(500, e, "Select User"))?,
        licenseno: row.try_get("licenseno").map_err(|e| Source::SQLx.to_error(500, e, "Select User"))?,
        image: image_from_row(row)?,
        theme: row.try_get("theme").map_err(|e| Source::SQLx.to_error(500, e, "Select User"))?,
        wide_screen: row.try_get("wide_screen").map_err(|e| Source::SQLx.to_error(500, e, "Select User"))?,
        totp: row.try_get("totp").map_err(|e| Source::SQLx.to_error(500, e, "Select User"))?,
        ts: row.try_get("ts").map_err(|e| Source::SQLx.to_error(500, e, "Select User"))?,
        failed: row.try_get("failed").map_err(|e| Source::SQLx.to_error(500, e, "Select User"))?,
        totp_done: row.try_get("totp_done").map_err(|e| Source::SQLx.to_error(500, e, "Select User"))?,
        can_passcode: row.try_get("can_passcode").map_err(|e| Source::SQLx.to_error(500, e, "Select User"))?,
        wards: wards_str.map(|s| s.split(',').map(|c| c.to_owned()).collect::<Vec<String>>()).unwrap_or_default(),
        spclty_ids: spcltys_str.map(|s| s.split(',').filter_map(|c| c.parse::<u32>().ok()).collect::<Vec<u32>>()).unwrap_or_default(),
    })
}

// project/function/SessionManager.php::getCurrentUserRoles()
pub async fn get_user_roles(loginname: &str, pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<Vec<CurrentUserRole>, AppError> {
    let sql = login::get_current_user_roles(hosxp, kphis);
    let roles = query1_all(loginname, &sql, pool, "Select UserRoles").await?;

    roles
        .iter()
        .map(CurrentUserRole::from_row)
        .collect::<sqlx::Result<Vec<CurrentUserRole>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select Role"))
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;
    use kphis_sqlx_tester::MySqlTester;

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_user() {
        let tester = MySqlTester::new_hosxp_and_kphis_and_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/opduser.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_ward_passcode_user.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/user_config.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/opduser.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_ward_passcode_user.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/user_config.sql")).execute(&tester.db_pool).await.unwrap();

        let disable_null = get_user("tester", &tester.db_pool, &tester.hosxp, &tester.kphis, &tester.kphis_extra).await.unwrap();
        assert!(disable_null.is_some());
        let disable_n = get_user("user", &tester.db_pool, &tester.hosxp, &tester.kphis, &tester.kphis_extra).await.unwrap();
        assert!(disable_n.is_some());
        let disable_y = get_user("rayman", &tester.db_pool, &tester.hosxp, &tester.kphis, &tester.kphis_extra).await.unwrap();
        assert!(disable_y.is_none());
        let not_found = get_user("unknown", &tester.db_pool, &tester.hosxp, &tester.kphis, &tester.kphis_extra).await.unwrap();
        assert!(not_found.is_none());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_user_roles() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/opduser.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/system_ac_role.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/system_acc_role_user.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/opduser.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/system_ac_role.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/system_acc_role_user.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_user_roles("user", &tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert_eq!(found.len(), 2);
        let not_found = get_user_roles("unknown", &tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }
}
