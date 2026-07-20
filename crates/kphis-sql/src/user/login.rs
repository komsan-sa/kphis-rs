// SELECT u.loginname,u.passweb,u.`name`,u.doctorcode,u.groupname,u.accessright,u.entryposition,d.licenseno,u.picture AS image,c.wards,c.spcltys,c.theme,c.wide_screen,c.totp,c.ts,c.failed,c.totp_done,
//   (SELECT EXISTS(SELECT * FROM kphis.ipd_ward_passcode_user WHERE loginname=u.loginname)) AS can_passcode
// FROM hos.opduser u
//   LEFT JOIN hos.doctor d ON d.`code`=u.doctorcode
//   LEFT JOIN kphis_extra.user_config c ON c.loginname=u.loginname
// WHERE u.loginname=? AND (u.account_disable IS NULL OR u.account_disable <> 'Y');
/// loginname
pub fn check_login(hosxp: &str, kphis: &str, kphis_extra: &str) -> String {
    [
        "SELECT u.loginname,u.passweb,u.`name`,u.doctorcode,u.groupname,u.accessright,u.entryposition,d.licenseno,u.picture AS image,c.wards,c.spcltys,c.theme,c.wide_screen,c.totp,c.ts,c.failed,c.totp_done,\
            (SELECT EXISTS(SELECT * FROM ",kphis,".ipd_ward_passcode_user WHERE loginname=u.loginname)) AS can_passcode \
        FROM ",hosxp,".opduser u \
            LEFT JOIN ",hosxp,".doctor d ON d.`code`=u.doctorcode \
            LEFT JOIN ",kphis_extra,".user_config c ON c.loginname=u.loginname \
        WHERE u.loginname=? AND (u.account_disable IS NULL OR u.account_disable <> 'Y');"
    ].concat()
}

// SELECT ru.role, r.role_desc
// FROM hos.opduser u
// JOIN kphis.system_ac_role_user ru ON u.loginname = ru.loginname
// JOIN kphis.system_ac_role r ON r.role = ru.role
// WHERE u.loginname = 'pitchapong'
// ORDER BY r.role_desc;
pub fn get_current_user_roles(hosxp: &str, kphis: &str) -> String {
    [
        "SELECT ru.role, r.role_desc \
        FROM ",hosxp,".opduser u \
            JOIN ",kphis,".system_ac_role_user ru ON u.loginname = ru.loginname \
            JOIN ",kphis,".system_ac_role r ON r.role = ru.role \
        WHERE u.loginname = ? \
        ORDER BY r.role_desc;"
    ].concat()
}

// select u.loginname, ru.role, r.role_desc, rp.permission, p.resource, p.operation
// from hos.opduser u
// join kphis.system_ac_role_user ru on u.loginname = ru.loginname
// join kphis.system_ac_role r on r.role in (
//     SELECT r1.role
//     from kphis.system_ac_role r1
//     where r1.role = ru.role
//     union
//     SELECT r2.role
//     from kphis.system_ac_role r1
//     join kphis.system_ac_role r2 on r1.parent_role = r2.role
//     where r1.role = ru.role
//     union
//     SELECT r3.role
//     from kphis.system_ac_role r1
//     join kphis.system_ac_role r2 on r1.parent_role = r2.role
//     join kphis.system_ac_role r3 on r2.parent_role = r3.role
//     where r1.role = ru.role
//     union
//     SELECT r4.role
//     from kphis.system_ac_role r1
//     join kphis.system_ac_role r2 on r1.parent_role = r2.role
//     join kphis.system_ac_role r3 on r2.parent_role = r3.role
//     join kphis.system_ac_role r4 on r3.parent_role = r4.role
//     where r1.role = ru.role
//     union
//     SELECT r5.role
//     from kphis.system_ac_role r1
//     join kphis.system_ac_role r2 on r1.parent_role = r2.role
//     join kphis.system_ac_role r3 on r2.parent_role = r3.role
//     join kphis.system_ac_role r4 on r3.parent_role = r4.role
//     join kphis.system_ac_role r5 on r4.parent_role = r5.role
//     where r1.role = ru.role
// )
// join kphis.system_ac_role_permission rp
//     on r.role = rp.role
// join kphis.system_ac_permission p on p.permission = rp.permission
// where u.loginname = 'pitchapong' and p.resource = 'SYSTEM_AC_ROLE' and p.operation = 'REMOVE';
// // or
// where u.loginname = 'pitchapong' and p.permission = 'SYSTEM_AC_ROLE_REMOVE';
// pub fn check_permission(hosxp: &str, kphis: &str) -> String {
//     [
//         "SELECT u.loginname, ru.role, r.role_desc, rp.permission, p.resource, p.operation \
//         FROM ",hosxp,".opduser u \
//         JOIN ",kphis,".system_ac_role_user ru ON u.loginname = ru.loginname \
//         JOIN ",kphis,".system_ac_role r ON r.role in ( \
//             SELECT r1.role \
//             FROM ",kphis,".system_ac_role r1 \
//             WHERE r1.role = ru.role \
//             UNION \
//                 SELECT r2.role \
//                 FROM ",kphis,".system_ac_role r1 \
//                 JOIN ",kphis,".system_ac_role r2 ON r1.parent_role = r2.role \
//                 WHERE r1.role = ru.role \
//             UNION \
//                 SELECT r3.role \
//                 FROM ",kphis,".system_ac_role r1 \
//                 JOIN ",kphis,".system_ac_role r2 ON r1.parent_role = r2.role \
//                 JOIN ",kphis,".system_ac_role r3 ON r2.parent_role = r3.role \
//                 WHERE r1.role = ru.role \
//             UNION \
//                 SELECT r4.role \
//                 FROM ",kphis,".system_ac_role r1 \
//                 JOIN ",kphis,".system_ac_role r2 ON r1.parent_role = r2.role \
//                 JOIN ",kphis,".system_ac_role r3 ON r2.parent_role = r3.role \
//                 JOIN ",kphis,".system_ac_role r4 ON r3.parent_role = r4.role \
//                 WHERE r1.role = ru.role \
//             UNION \
//                 SELECT r5.role \
//                 FROM ",kphis,".system_ac_role r1 \
//                 JOIN ",kphis,".system_ac_role r2 ON r1.parent_role = r2.role \
//                 JOIN ",kphis,".system_ac_role r3 ON r2.parent_role = r3.role \
//                 JOIN ",kphis,".system_ac_role r4 ON r3.parent_role = r4.role \
//                 JOIN ",kphis,".system_ac_role r5 ON r4.parent_role = r5.role \
//                 WHERE r1.role = ru.role \
//         ) \
//         JOIN ",kphis,".system_ac_role_permission rp ON r.role = rp.role \
//         JOIN ",kphis,".system_ac_permission p ON p.permission = rp.permission "
//     )
// }