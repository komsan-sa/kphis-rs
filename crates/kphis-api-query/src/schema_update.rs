use sqlx::{AssertSqlSafe, MySql, Pool, mysql::MySqlQueryResult};

use kphis_sql::schema_update;

pub async fn update_kphis(pool: &Pool<MySql>, kphis: &str) -> sqlx::Result<Vec<MySqlQueryResult>> {
    let sqls = schema_update::update_kphis(kphis);
    let mut results = Vec::with_capacity(sqls.len());
    for sql in sqls.into_iter() {
        results.push(sqlx::query(AssertSqlSafe(sql)).execute(pool).await?);
    }

    Ok(results)
}

pub async fn update_kphis_log(pool: &Pool<MySql>, kphis_log: &str) -> sqlx::Result<Vec<MySqlQueryResult>> {
    let sqls = schema_update::update_kphis_log(kphis_log);
    let mut results = Vec::with_capacity(sqls.len());
    for sql in sqls.into_iter() {
        results.push(sqlx::query(AssertSqlSafe(sql)).execute(pool).await?);
    }

    Ok(results)
}

pub async fn update_kphis_extra(pool: &Pool<MySql>, kphis_extra: &str) -> sqlx::Result<Vec<MySqlQueryResult>> {
    let sqls = schema_update::update_kphis_extra(kphis_extra);
    let mut results = Vec::with_capacity(sqls.len());
    for sql in sqls.into_iter() {
        results.push(sqlx::query(AssertSqlSafe(sql)).execute(pool).await?);
    }

    Ok(results)
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;
    use kphis_sqlx_tester::MySqlMocker;

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_kphis() {
        let mocker = MySqlMocker::new_kphis().await;

        let success = update_kphis(&mocker.db_pool, &mocker.kphis).await.unwrap();
        assert_eq!(success.len(), 222);
        let again = update_kphis(&mocker.db_pool, &mocker.kphis).await.unwrap();
        assert_eq!(again.len(), 222);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_kphis_log() {
        let mocker = MySqlMocker::new_kphis_log().await;

        let success = update_kphis_log(&mocker.db_pool, &mocker.kphis_log).await.unwrap();
        assert_eq!(success.len(), 5);
        let again = update_kphis_log(&mocker.db_pool, &mocker.kphis_log).await.unwrap();
        assert_eq!(again.len(), 5);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_kphis_extra() {
        let mocker = MySqlMocker::new_kphis_extra().await;

        let success = update_kphis_extra(&mocker.db_pool, &mocker.kphis_extra).await.unwrap();
        assert_eq!(success.len(), 29);
        let again = update_kphis_extra(&mocker.db_pool, &mocker.kphis_extra).await.unwrap();
        assert_eq!(again.len(), 29);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_kphis_extra_from_no_db() {
        let mut mocker = MySqlMocker::new_no_database().await;
        // this update will create kphis_extra database
        let success = update_kphis_extra(&mocker.db_pool, &mocker.kphis_extra).await.unwrap();
        assert_eq!(success.len(), 29);
        let again = update_kphis_extra(&mocker.db_pool, &mocker.kphis_extra).await.unwrap();
        assert_eq!(again.len(), 29);
        // for delete kphis_extra database on drop
        mocker.has_kphis_extra = true;
    }
}
