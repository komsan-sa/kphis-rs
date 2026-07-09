use sqlx::{
    AssertSqlSafe, FromRow, MySql, Pool, Row,
    mysql::{MySqlQueryResult, MySqlRow},
};

use kphis_model::{
    fetch::ExecuteResponse,
    user::{
        permission::Permission,
        role::{Role, RolePermission, RolePermissionList, RolePermissionSave, UserRole, UserRoleList, UserRoleOptions, UserRoleParams, UserRoleSave},
    },
};
use kphis_sql::user::role;
use kphis_util::error::{AppError, Source};

use crate::{execute1, query_all};

/// select_roles_zero_count
pub async fn get_all_role(pool: &Pool<MySql>, kphis: &str) -> Result<Vec<Role>, AppError> {
    let roles_sql = role::select_roles_zero_count(kphis);
    let roles = query_all(&roles_sql, pool, "Select Roles")
        .await?
        .iter()
        .map(Role::from_row)
        .collect::<sqlx::Result<Vec<Role>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select Roles"))?;
    Ok(roles)
}

// GET user-role/role
pub async fn get_role_permission_list(params: UserRoleParams, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<RolePermissionList>, AppError> {
    let roles_sql = role::select_roles_permissions(&params, kphis);
    let mut roles_query = sqlx::query(AssertSqlSafe(roles_sql));
    if let Some(permission) = &params.permission {
        roles_query = roles_query.bind(permission);
    }
    if let Some(roles) = &params.role {
        for role in roles.split(',') {
            roles_query = roles_query.bind(role);
        }
    }
    roles_query
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select RolesPermissions"))?
        .iter()
        .map(role_permission_list_from_row)
        .collect::<sqlx::Result<Vec<RolePermissionList>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select RolesPermissions"))
}
fn role_permission_list_from_row(row: &MySqlRow) -> sqlx::Result<RolePermissionList> {
    let permissions_opt: Option<String> = row.try_get("permissions")?;
    let permissions = permissions_opt.map(|concat| concat.split(',').map(Permission::new).collect());

    Ok(RolePermissionList {
        role: row.try_get("role")?,
        role_desc: row.try_get("role_desc")?,
        parent_role: row.try_get("parent_role")?,
        parent_role_desc: row.try_get("parent_role_desc")?,
        permissions,
    })
}

// system-ac-role-user-list-data.php
// GET user-role/user
pub async fn get_user_role_list(params: UserRoleParams, pool: &Pool<MySql>, hosxp: &str, kphis: &str, kphis_extra: &str) -> Result<UserRoleList, AppError> {
    let user_roles = select_users_role_list(params, pool, hosxp, kphis, kphis_extra).await?;
    let roles = select_roles_with_count(pool, kphis).await?;

    Ok(UserRoleList { user_roles, roles })
}

async fn select_users_role_list(params: UserRoleParams, pool: &Pool<MySql>, hosxp: &str, kphis: &str, kphis_extra: &str) -> Result<Vec<UserRole>, AppError> {
    let list_sql = role::select_users_role_list(&params, hosxp, kphis, kphis_extra);
    let mut query = sqlx::query(AssertSqlSafe(list_sql));
    if let Some(loginname) = &params.loginname {
        query = query.bind(["%", loginname.trim(), "%"].concat());
    }
    if let Some(name) = &params.name {
        query = query.bind(["%", name.trim(), "%"].concat());
    }
    if let Some(role) = &params.role {
        query = query.bind(role);
    }
    if let Some(hosxp_group) = &params.hosxp_group {
        query = query.bind(hosxp_group);
    }
    query
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select User Roles"))?
        .iter()
        .map(UserRole::from_row)
        .collect::<sqlx::Result<Vec<UserRole>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select User Roles"))
}

async fn select_roles_with_count(pool: &Pool<MySql>, kphis: &str) -> Result<Vec<Role>, AppError> {
    let roles_sql = role::select_roles_with_count(kphis);
    query_all(&roles_sql, pool, "Select Roles")
        .await?
        .iter()
        .map(Role::from_row)
        .collect::<sqlx::Result<Vec<Role>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select Roles"))
}

// GET user-role/prelude
pub async fn get_user_role_prelude(pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<UserRoleOptions, AppError> {
    let roles = get_all_role(pool, kphis).await?;
    let hosxp_groups = select_hosxp_groups(pool, hosxp).await?;
    let role_permissions = select_role_permission(pool, kphis).await?;
    let permissions = select_permission(pool, kphis).await?;

    Ok(UserRoleOptions {
        roles,
        hosxp_groups,
        role_permissions,
        permissions,
    })
}

async fn select_hosxp_groups(pool: &Pool<MySql>, hosxp: &str) -> Result<Vec<String>, AppError> {
    let hosxp_group_sql = role::select_hosxp_groups(hosxp);
    query_all(&hosxp_group_sql, pool, "Select HosXP Groups")
        .await?
        .iter()
        .filter_map(|row| row.try_get("hosxp_group").transpose())
        .collect::<sqlx::Result<Vec<String>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select HosXP Groups"))
}

async fn select_role_permission(pool: &Pool<MySql>, kphis: &str) -> Result<Vec<RolePermission>, AppError> {
    let role_permissions_sql = role::select_role_permission(kphis);
    query_all(&role_permissions_sql, pool, "Select RolePermissions")
        .await?
        .iter()
        .map(role_permission_from_row)
        .collect::<sqlx::Result<Vec<RolePermission>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select RolePermissions"))
}

async fn select_permission(pool: &Pool<MySql>, kphis: &str) -> Result<Vec<Permission>, AppError> {
    let permissions_sql = role::select_permission(kphis);
    query_all(&permissions_sql, pool, "Select Permissions")
        .await?
        .iter()
        .map(permission_from_row)
        .collect::<sqlx::Result<Vec<Permission>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select Permissions"))
}

fn role_permission_from_row(row: &MySqlRow) -> sqlx::Result<RolePermission> {
    let name: String = row.try_get("permission")?;
    Ok(RolePermission {
        role: row.try_get("role")?,
        permission: Permission::new(&name),
    })
}

fn permission_from_row(row: &MySqlRow) -> sqlx::Result<Permission> {
    let name: String = row.try_get("permission")?;
    Ok(Permission::new(&name))
}

// POST user-role/user
pub async fn post_user_role(payload: UserRoleSave, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<ExecuteResponse>, AppError> {
    let mut results = Vec::with_capacity(2);
    let delete_result = delete_role_user_by_loginname(&payload.loginname, pool, kphis).await?;
    results.push(ExecuteResponse::from_query_result(delete_result, "Delete RoleUserByName"));
    if !payload.roles.is_empty() {
        let insert_result = insert_roles_user(&payload.roles, &payload.loginname, user, pool, kphis).await?;
        results.push(ExecuteResponse::from_query_result(insert_result, "Insert RoleUser"));
    }

    Ok(results)
}

async fn delete_role_user_by_loginname(loginname: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let delete_sql = role::delete_role_user_by_loginname(kphis);
    sqlx::query(AssertSqlSafe(delete_sql))
        .bind(loginname)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete RoleUserByName"))
}

async fn insert_roles_user(roles: &[String], loginname: &str, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    if !roles.is_empty() {
        let insert_sql = role::insert_roles_user(roles.len(), kphis);
        let mut insert_query = sqlx::query(AssertSqlSafe(insert_sql));
        for role in roles {
            insert_query = insert_query.bind(loginname).bind(role).bind(user).bind(user);
        }
        insert_query.execute(pool).await.map_err(|e| Source::SQLx.to_error(500, e, "Insert RoleUser"))
    } else {
        Ok(MySqlQueryResult::default())
    }
}

// // system-ac-role-permission-save.php
// // SUMMMARY
// [NOT has role_prv] -> INSERT INTO system_ac_role
//  ==> fn insert_role_new()
// [has role_prv]
// x [system_ac_role has role=role_prv] x=> error 403
// - -> [old role.role!=new role]
// - - x [system_ac_role NOT has role=role] x=> error 403
// - - - - INSERT INTO system_ac_role with old createuser/datetime
//          ==> fn insert_role_from_prev()
// - - - - UPDATE system_ac_role parent_role=role+updateuser/datetime+(version+1) where parent_role=role_prv
//          ==> fn update_parent_role()
// - - - - DELETE system_ac_role WHERE role=role_prv
//          ==> fn delete_role()
// - -> [old role.role==new role]
// - - - UPDATE system_ac_role role_desc,parent_role,updateuser/datetime,(version+1) where role=role_prv
//          ==> fn update_role()
// DELETE system_ac_role_permission where role=role AND permssion NOT in (permissions)
//  ==> fn delete_role_not_keep_permission()
// INSERT INTO system_ac_role_permission for each permission
//  ==> fn insert_role_permissions()
// POST user-role/role
pub async fn post_role_permission(payload: RolePermissionSave, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<ExecuteResponse>, AppError> {
    let mut results = Vec::with_capacity(5);
    let mut will_add_permissions = false;

    if let Some(role_prev) = payload.role_prev.as_ref() {
        // edit previous role
        if &payload.role == role_prev {
            // using the same `role`
            let update_result = update_role(&payload.role_desc, &payload.parent_role, role_prev, user, pool, kphis).await?;
            let is_updated = update_result.rows_affected() > 0;
            results.push(ExecuteResponse::from_query_result(update_result, "Update Role"));

            if is_updated {
                // we clear all permission
                // has_prev is false because role == role_prev
                let delete_rp_result = delete_role_not_keep_permission(&payload.role, &None, pool, kphis).await?;
                results.push(ExecuteResponse::from_query_result(delete_rp_result, "Delete RolePermission"));

                will_add_permissions = true;
            }
        } else {
            // change `role` = insert new + update parent + delete old
            let insert_result = insert_role_from_prev(&payload.role, &payload.role_desc, &payload.parent_role, role_prev, user, pool, kphis).await?;
            let is_inserted = insert_result.rows_affected() > 0;
            results.push(ExecuteResponse::from_query_result(insert_result, "Insert RoleFromPrevious"));

            if is_inserted {
                // clear a foreign key constraint on role.parent_role before delete role
                let update_result = update_parent_role(&Some(payload.role.clone()), role_prev, user, pool, kphis).await?;
                results.push(ExecuteResponse::from_query_result(update_result, "Update ParentRole"));

                // clear a foreign key constraint on system_ac_role_permission table before delete role
                // we clear all permission
                let delete_rp_result = delete_role_not_keep_permission(&payload.role, &payload.role_prev, pool, kphis).await?;
                results.push(ExecuteResponse::from_query_result(delete_rp_result, "Delete RolePermission"));

                let delete_role_result = delete_role(role_prev, pool, kphis).await?;
                results.push(ExecuteResponse::from_query_result(delete_role_result, "Delete Role"));

                will_add_permissions = true;
            }
        }
    } else {
        // new role = insert new
        let insert_result = insert_role_new(&payload.role, &payload.role_desc, &payload.parent_role, user, pool, kphis).await?;
        let is_inserted = insert_result.rows_affected() > 0;
        results.push(ExecuteResponse::from_query_result(insert_result, "Insert RoleNew"));

        if is_inserted {
            will_add_permissions = true;
        }
    }

    if will_add_permissions {
        let permissions_len = payload.permissions.len();
        if permissions_len > 0 {
            let insert_result = insert_role_permissions(&payload.role, &payload.permissions, user, pool, kphis).await?;
            results.push(ExecuteResponse::from_query_result(insert_result, "Insert RolePermissions"));
        }
    }

    Ok(results)
}

async fn update_role(role_desc: &str, parent_role: &Option<String>, role_prev: &str, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let update_sql = role::update_role(kphis);
    sqlx::query(AssertSqlSafe(update_sql))
        .bind(role_desc)
        .bind(parent_role)
        .bind(user)
        .bind(role_prev)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Update Role"))
}

async fn delete_role_not_keep_permission(role: &str, role_prev: &Option<String>, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let delete_rp_sql = role::delete_role_keep_permission(role_prev.is_some(), 0, kphis);
    let mut delete_rp_query = sqlx::query(AssertSqlSafe(delete_rp_sql)).bind(role);
    if let Some(role_prev) = role_prev {
        delete_rp_query = delete_rp_query.bind(role_prev);
    }
    delete_rp_query.execute(pool).await.map_err(|e| Source::SQLx.to_error(500, e, "Delete RolePermission"))
}

async fn insert_role_from_prev(role: &str, role_desc: &str, parent_role: &Option<String>, role_prev: &str, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let insert_sql = role::insert_role_from_prev(kphis);
    sqlx::query(AssertSqlSafe(insert_sql))
        .bind(role)
        .bind(role_desc)
        .bind(parent_role)
        .bind(user)
        .bind(role_prev)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert RoleFromPrevious"))
}

async fn update_parent_role(new_parent_role: &Option<String>, old_parent_role: &str, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let update_sql = role::update_parent_role(kphis);
    sqlx::query(AssertSqlSafe(update_sql))
        .bind(new_parent_role)
        .bind(user)
        .bind(old_parent_role)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Update ParentRole"))
}

async fn delete_role(role_prev: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let delete_sql = role::delete_role(kphis);
    execute1(role_prev, &delete_sql, pool, "Delete Role").await
}

async fn insert_role_new(role: &str, role_desc: &str, parent_role: &Option<String>, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let insert_sql = role::insert_role_new(kphis);
    sqlx::query(AssertSqlSafe(insert_sql))
        .bind(role)
        .bind(role_desc)
        .bind(parent_role)
        .bind(user)
        .bind(user)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert RoleNew"))
}

async fn insert_role_permissions(role: &str, permissions: &[Permission], user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let insert_sql = role::insert_role_permissions(permissions.len(), kphis);
    let mut insert_query = sqlx::query(AssertSqlSafe(insert_sql));
    for permission in permissions {
        insert_query = insert_query.bind(role).bind(permission.str()).bind(user).bind(user);
    }
    insert_query.execute(pool).await.map_err(|e| Source::SQLx.to_error(500, e, "Insert RolePermissions"))
}

// DELETE user-role/role
pub async fn delete_role_permission(params: UserRoleParams, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<ExecuteResponse>, AppError> {
    let mut results = Vec::with_capacity(4);
    if let Some(role) = params.role.as_ref() {
        // clear a foreign key constraint on role.parent_role before delete role
        // update parent_role from params.role -> params.parent_role
        let update_result = update_parent_role(&params.parent_role, role, user, pool, kphis).await?;
        results.push(ExecuteResponse::from_query_result(update_result, "Update ParentRole"));

        // clear a foreign key constraint on system_ac_role_permission table before delete role
        let delete_rp_result = delete_role_not_keep_permission(role, &None, pool, kphis).await?;
        results.push(ExecuteResponse::from_query_result(delete_rp_result, "Delete RolePermission"));

        let delete_ru_result = delete_role_user_by_role(role, pool, kphis).await?;
        results.push(ExecuteResponse::from_query_result(delete_ru_result, "Delete RoleUserByRole"));

        let delete_role_result = delete_role(role, pool, kphis).await?;
        results.push(ExecuteResponse::from_query_result(delete_role_result, "Delete Role"));
    }
    Ok(results)
}

async fn delete_role_user_by_role(role: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let delete_ru_sql = role::delete_role_user_by_role(kphis);
    sqlx::query(AssertSqlSafe(delete_ru_sql))
        .bind(role)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete RoleUserByRole"))
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;
    use kphis_sqlx_tester::MySqlTester;

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_all_role() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/system_ac_role.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/system_ac_role.sql")).execute(&tester.db_pool).await.unwrap();
        let found = get_all_role(&tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(found.len(), 13);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_role_permission_list() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/system_ac_role.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/system_ac_operation.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/system_ac_resource.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/system_acc_permission.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/system_acc_role_permission.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/system_ac_role.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/system_ac_operation.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/system_ac_resource.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/system_acc_permission.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/system_acc_role_permission.sql")).execute(&tester.db_pool).await.unwrap();

        let all = get_role_permission_list(UserRoleParams::default(), &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(all.len(), 13);

        // permission selector will filter 'permissions' column data, not row
        let permission_found = get_role_permission_list(UserRoleParams {permission: Some(String::from("EMR_VIEW")),..Default::default()},&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(permission_found.len(), 13);
        assert_eq!(permission_found.iter().map(|p| p.permissions.as_ref().map(|ps| ps.len()).unwrap_or_default()).sum::<usize>(),1);
        let permission_not_found = get_role_permission_list(UserRoleParams {permission: Some(String::from("USER_VIEW")),..Default::default()},&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(permission_not_found.iter().map(|p| p.permissions.as_ref().map(|ps| ps.len()).unwrap_or_default()).sum::<usize>(),0);

        let role_found = get_role_permission_list(UserRoleParams {role: Some(String::from("DOCTOR")),..Default::default()},&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(role_found.len(), 1);
        let role_not_found = get_role_permission_list(UserRoleParams {role: Some(String::from("ADMIN")),..Default::default()},&tester.db_pool,&tester.kphis).await.unwrap();
        assert!(role_not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_users_role_list() {
        let tester = MySqlTester::new_hosxp_and_kphis_and_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/opduser.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/system_ac_role.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/system_acc_role_user.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/user_config.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/opduser.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/system_ac_role.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/system_acc_role_user.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/user_config.sql")).execute(&tester.db_pool).await.unwrap();

        let all = select_users_role_list( UserRoleParams::default(),&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert_eq!(all.len(), 4);
        let enabled = select_users_role_list(UserRoleParams {account_disable: Some(String::from("N")),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert_eq!(enabled.len(), 3);
        let disabled = select_users_role_list(UserRoleParams {account_disable: Some(String::from("Y")),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert_eq!(disabled.len(), 1);

        let loginname_found = select_users_role_list(UserRoleParams {loginname: Some(String::from("este")),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert_eq!(loginname_found.len(), 1);
        let loginname_not_found = select_users_role_list(UserRoleParams {loginname: Some(String::from("xxxx")),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert!(loginname_not_found.is_empty());

        let name_found = select_users_role_list(UserRoleParams {name: Some(String::from("erMa")),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert_eq!(name_found.len(), 2);
        let name_not_found = select_users_role_list(UserRoleParams {name: Some(String::from("xxxx")),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert!(name_not_found.is_empty());

        let role_found = select_users_role_list(UserRoleParams {role: Some(String::from("DOCTOR")),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert_eq!(role_found.len(), 1);
        let role_not_found = select_users_role_list(UserRoleParams {role: Some(String::from("ADMIN")),..Default::default()
            },&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert!(role_not_found.is_empty());

        let group_found = select_users_role_list(UserRoleParams {hosxp_group: Some(String::from("ER")),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert_eq!(group_found.len(), 2);
        let group_not_found = select_users_role_list(UserRoleParams {hosxp_group: Some(String::from("ADMIN")),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert!(group_not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_hosxp_groups() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/opdgroup.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/opdgroup.sql")).execute(&tester.db_pool).await.unwrap();

        let all = select_hosxp_groups(&tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(all.len(), 2);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_roles_with_count() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/system_ac_role.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/system_acc_role_user.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/system_ac_role.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/system_acc_role_user.sql")).execute(&tester.db_pool).await.unwrap();

        let all = select_roles_with_count(&tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(all.len(), 13); // table `system_ac_role` has 6 rows
        assert_eq!(all.iter().map(|r| r.user_count).sum::<i64>(), 5); // 1 DOCTOR + 4 DOCTOR_STAFF
        assert_eq!(all.iter().filter(|r| r.role == *"DOCTOR_STAFF").map(|r| r.user_count).sum::<i64>(),3);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_permission() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/system_ac_operation.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/system_ac_resource.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/system_acc_permission.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/system_ac_operation.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/system_ac_resource.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/system_acc_permission.sql")).execute(&tester.db_pool).await.unwrap();

        let all = select_permission(&tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(all.len(), 9);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_role_permission() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/system_ac_role.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/system_ac_operation.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/system_ac_resource.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/system_acc_permission.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/system_acc_role_permission.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/system_ac_role.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/system_ac_operation.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/system_ac_resource.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/system_acc_permission.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/system_acc_role_permission.sql")).execute(&tester.db_pool).await.unwrap();

        let found = select_role_permission(&tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(found.len(), 3);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_role() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/system_ac_role.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/system_ac_role.sql")).execute(&tester.db_pool).await.unwrap();

        // parent_role MUST valid in 'system_ac_role' table
        let fail_parent = update_role("MUSCULAR DOCTOR",&Some(String::from("M&M")),"DOCTOR_STAFF","user",&tester.db_pool,&tester.kphis).await;
        assert!(fail_parent.is_err());
        let not_found = update_role("SUPER DOCTOR",&Some(String::from("MSO")),"SUPERMAN","user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(not_found.rows_affected(), 0);
        let success = update_role("SUPER DOCTOR",&Some(String::from("MSO")),"DOCTOR_STAFF","user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_parent_role() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/system_ac_role.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/system_ac_role.sql")).execute(&tester.db_pool).await.unwrap();

        // new_parent_role MUST valid in 'system_ac_role' table
        let fail_parent = update_parent_role(&Some(String::from("M&M")),"DOCTOR","user",&tester.db_pool,&tester.kphis).await;
        assert!(fail_parent.is_err());
        let not_found = update_parent_role(&Some(String::from("MSO")),"SUPERMAN","user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(not_found.rows_affected(), 0);
        let success = update_parent_role(&Some(String::from("MSO")),"DOCTOR","user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 2);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_role_new() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/system_ac_role.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/system_ac_role.sql")).execute(&tester.db_pool).await.unwrap();

        let duplicate_pk = insert_role_new("MSO","NEW ROLE",&Some(String::from("DOCTOR_STAFF")),"user",&tester.db_pool,&tester.kphis).await;
        assert!(duplicate_pk.is_err());
        // parent_role MUST valid in 'system_ac_role' table
        let fail_parent = insert_role_new("NEW","NEW ROLE",&Some(String::from("SUPERMAN")),"user",&tester.db_pool,&tester.kphis).await;
        assert!(fail_parent.is_err());
        let success = insert_role_new("NEW","NEW ROLE",&Some(String::from("DOCTOR_STAFF")),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_role_from_prev() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/system_ac_role.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/system_ac_role.sql")).execute(&tester.db_pool).await.unwrap();

        let duplicate_pk = insert_role_from_prev("DOCTOR","NEW ROLE",&Some(String::from("MSO")),"DOCTOR_STAFF","user",&tester.db_pool,&tester.kphis).await;
        assert!(duplicate_pk.is_err());
        // parent_role MUST valid in 'system_ac_role' table
        let fail_parent = insert_role_from_prev("DOCTOR_SUPER","NEW ROLE",&Some(String::from("SUPERMAN")),"DOCTOR_STAFF","user",&tester.db_pool,&tester.kphis).await;
        assert!(fail_parent.is_err());
        let prev_not_found = insert_role_from_prev("DOCTOR_SUPER","NEW ROLE",&Some(String::from("MSO")),"SUPERMAN","user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(prev_not_found.rows_affected(), 0);
        let success = insert_role_from_prev("DOCTOR_SUPER","NEW ROLE",&Some(String::from("MSO")),"DOCTOR_STAFF","user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_role_permissions() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/system_ac_role.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/system_ac_operation.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/system_ac_resource.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/system_acc_permission.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/system_acc_role_permission.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/system_ac_role.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/system_ac_operation.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/system_ac_resource.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/system_acc_permission.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/system_acc_role_permission.sql")).execute(&tester.db_pool).await.unwrap();

        // role + permission is PK
        let duplicate_pk = insert_role_permissions("MSO",&[Permission::demo_io_view()],"user",&tester.db_pool,&tester.kphis).await;
        assert!(duplicate_pk.is_err());
        // role MUST valid in 'system_ac_role' table
        let fail_role = insert_role_permissions("NEW",&[Permission::demo_io_view()],"user",&tester.db_pool,&tester.kphis).await;
        assert!(fail_role.is_err());
        // permission MUST valid in 'system_ac_permission' table
        let fail_permission = insert_role_permissions("DOCTOR",&[Permission::demo_lab_view()],"user",&tester.db_pool,&tester.kphis).await;
        assert!(fail_permission.is_err());
        let success = insert_role_permissions("DOCTOR",&[Permission::demo_io_view()],"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_roles_user() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/system_ac_role.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/system_acc_role_user.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/system_ac_role.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/system_acc_role_user.sql")).execute(&tester.db_pool).await.unwrap();

        let no_role = insert_roles_user(&[],"user","user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(no_role.rows_affected(), 0);
        // loginname + role is PK
        let duplicate_pk = insert_roles_user(&[String::from("DOCTOR")],"user","user",&tester.db_pool,&tester.kphis).await;
        assert!(duplicate_pk.is_err());
        // role MUST valid in 'system_ac_role' table
        let fail_role = insert_roles_user(&[String::from("SUPERMAN")],"user","user",&tester.db_pool,&tester.kphis).await;
        assert!(fail_role.is_err());
        let success = insert_roles_user(&[String::from("DOCTOR")],"tester","user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_role() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/system_ac_role.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/system_ac_role.sql")).execute(&tester.db_pool).await.unwrap();

        let not_found = delete_role("SUPERMAN", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(not_found.rows_affected(), 0);
        let success = delete_role("DOCTOR_STAFF", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_role_user_by_role() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/system_ac_role.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/system_acc_role_user.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/system_ac_role.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/system_acc_role_user.sql")).execute(&tester.db_pool).await.unwrap();

        let not_found = delete_role_user_by_role("SUPERMAN", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(not_found.rows_affected(), 0);
        let success = delete_role_user_by_role("DOCTOR_STAFF", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 3);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_role_user_by_loginname() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/system_ac_role.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/system_acc_role_user.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/system_ac_role.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/system_acc_role_user.sql")).execute(&tester.db_pool).await.unwrap();

        let not_found = delete_role_user_by_loginname("robin", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(not_found.rows_affected(), 0);
        let success = delete_role_user_by_loginname("user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 2);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_role_keep_permission() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/system_ac_role.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/system_ac_operation.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/system_ac_resource.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/system_acc_permission.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/system_acc_role_permission.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/system_ac_role.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/system_ac_operation.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/system_ac_resource.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/system_acc_permission.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/system_acc_role_permission.sql")).execute(&tester.db_pool).await.unwrap();

        let not_found = delete_role_not_keep_permission("SUPERMAN", &None, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(not_found.rows_affected(), 0);
        let success_with_prev = delete_role_not_keep_permission("DOCTOR",&Some(String::from("MSO")),&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success_with_prev.rows_affected(), 3);

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/system_acc_role_permission.sql")).execute(&tester.db_pool).await.unwrap();
        let success_without_prev = delete_role_not_keep_permission("DOCTOR", &None, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success_without_prev.rows_affected(), 1);
    }
}
