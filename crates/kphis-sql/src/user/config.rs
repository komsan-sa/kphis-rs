use crate::{TABLE_CREATE_COLUMNS, TABLE_CREATE_PREPARED};

// INSERT INTO kphis_extra.user_config (loginname,theme,wide_screen,totp,create_user,create_datetime,update_user,update_datetime,version) VALUES
//   (?,?,?,?,?,NOW(),?,NOW(),1)
// ON DUPLICATE KEY UPDATE theme=VALUES(theme),wide_screen=VALUES(wide_screen),totp=VALUES(totp),update_user=VALUES(update_user),update_datetime=NOW(),version=(version+1);
/// loginname, theme, wide_screen, totp, ts, loginname, loginname
pub fn insert_dup_config_user(is_update_totp: bool, kphis_extra: &str) -> String {
    let totp = if is_update_totp {",totp=VALUES(totp),ts=VALUES(ts),totp_done=NULL"} else {""};
    [
        "INSERT INTO ",kphis_extra,".user_config (loginname,theme,wide_screen,totp,ts",TABLE_CREATE_COLUMNS,") VALUES \
            (?,?,?,?,?",TABLE_CREATE_PREPARED,") \
        ON DUPLICATE KEY UPDATE theme=VALUES(theme),wide_screen=VALUES(wide_screen)",totp,",update_user=VALUES(update_user),update_datetime=NOW(),version=(version+1);"
    ].concat()
}

// INSERT INTO kphis_extra.user_config (loginname,wards,spcltys,create_user,create_datetime,update_user,update_datetime,version) VALUES
//   (?,?,?,?,NOW(),?,NOW(),1)
// ON DUPLICATE KEY UPDATE wards=VALUES(wards),spcltys=VALUES(spcltys),update_user=VALUES(update_user),update_datetime=NOW(),version=(version+1);
/// loginname, wards, spcltys, loginname, loginname
pub fn insert_dup_config_sse(kphis_extra: &str) -> String {
    [
        "INSERT INTO ",kphis_extra,".user_config (loginname,wards,spcltys",TABLE_CREATE_COLUMNS,") VALUES \
            (?,?,?",TABLE_CREATE_PREPARED,") \
        ON DUPLICATE KEY UPDATE wards=VALUES(wards),spcltys=VALUES(spcltys),update_user=VALUES(update_user),update_datetime=NOW(),version=(version+1);"
    ].concat()
}

// UPDATE kphis_extra.user_config SET ts=? WHERE loginname=? AND totp IS NOT NULL;
/// ts, loginname
pub fn update_ts(kphis_extra: &str) -> String {
    ["UPDATE ",kphis_extra,".user_config SET ts=? WHERE loginname=? AND totp IS NOT NULL;"].concat()
}

// INSERT INTO kphis_extra.user_config (loginname,failed,create_user,create_datetime,update_user,update_datetime,version) VALUES
//   (?,?,?,NOW(),?,NOW(),1)
// ON DUPLICATE KEY UPDATE failed=VALUES(failed),update_user=VALUES(update_user),update_datetime=NOW(),version=(version+1);
/// loginname, failed, loginname, loginname, 
pub fn insert_dup_failed(kphis_extra: &str) -> String {
    [
        "INSERT INTO ",kphis_extra,".user_config (loginname,failed",TABLE_CREATE_COLUMNS,") VALUES \
            (?,?",TABLE_CREATE_PREPARED,") \
        ON DUPLICATE KEY UPDATE failed=VALUES(failed),update_user=VALUES(update_user),update_datetime=NOW(),version=(version+1);"
    ].concat()

}

// UPDATE kphis_extra.user_config SET totp_done=1 WHERE loginname=? AND totp IS NOT NULL AND ts IS NOT NULL;
/// loginname
pub fn update_totp_done(kphis_extra: &str) -> String {
    ["UPDATE ",kphis_extra,".user_config SET totp_done=1 WHERE loginname=? AND totp IS NOT NULL AND ts IS NOT NULL;"].concat()
}

// UPDATE kphis_extra.user_config SET totp=NULL,ts=NULL,totp_done=NULL,update_user=?,update_datetime=NOW(),version=(version+1) WHERE loginname=?;
/// update_user, target_loginname
pub fn remove_totp(kphis_extra: &str) -> String {
    [
        "UPDATE ",kphis_extra,".user_config SET totp=NULL,ts=NULL,totp_done=NULL,update_user=?,update_datetime=NOW(),version=(version+1) WHERE loginname=?;"
    ].concat()
}