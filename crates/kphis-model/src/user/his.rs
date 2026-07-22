use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use derive_demo::Demo;
use encoding_rs::WINDOWS_874;
use futures_signals::{signal::Mutable, signal_vec::MutableVec};
use js_sys::JsString;
use md5::{Digest, Md5};
use serde_derive::{Deserialize, Serialize};
use sqlx::FromRow;
use std::rc::Rc;
use utoipa::ToSchema;
use wasm_bindgen::JsCast;

use kphis_util::{
    datetime::get_timestamp_wasm,
    error::{AppError, Source},
    util::add_u64_with_i64,
};

use crate::{
    app::{AppState, AppStatus},
    endpoint::EndPoint,
    fetch::fetch_json_api,
    image::ImageBase64,
    user::permission::Permission,
};

pub fn hash(plain: &str) -> argon2::password_hash::Result<String> {
    let data = md5_enc(plain);
    argon2_enc(&data)
}

fn md5_enc(plain: &str) -> Vec<u8> {
    // convert Thai password to match database
    let (w874_bytes, _a, _b) = WINDOWS_874.encode(plain);
    // generate MD5
    let mut md5_hasher = Md5::new();
    md5_hasher.update(w874_bytes);
    md5_hasher.finalize().to_vec()
}

fn argon2_enc(data: &[u8]) -> argon2::password_hash::Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2.hash_password(data, &salt).map(|h| h.to_string())
}

//====================//
// Flow of user model //
//====================//

// 0. user request for authentication
/// User Request for log-in
#[derive(Debug, Demo, Serialize, Deserialize, ToSchema)]
#[schema(example = json!(UserRequest::demo()))]
pub struct UserRequest {
    /// username (POST) or Ulid of state_id (PUT)
    #[Demo(value = r#"String::from("user")"#)]
    pub username: String,
    #[Demo(value = r#"String::from("$argon2id$v=19$m=19456,t=2,p=1$/XFasj8WyfzzGzV2fnouWQ$OfHASwUrzgJmchn9LvM+T7IHtvI//W+BMgBe70jDnqU")"#)]
    pub password: String,
}

/// User Request for checking 2FA
#[derive(Debug, Demo, Serialize, Deserialize, ToSchema)]
#[schema(example = json!(UserRequest2fa::demo()))]
pub struct UserRequest2fa {
    #[Demo(value = r#"String::from("user")"#)]
    pub username: String,
    #[Demo(value = r#"String::from("123456")"#)]
    pub token_2fa: String,
    /// if 'is_sub' is true, 'username' is token's sub
    #[Demo(value = "true")]
    pub is_sub: bool,
}

/// User Request for confirm
#[derive(Debug, Demo, Serialize, Deserialize, ToSchema)]
#[schema(example = json!(UserRequestFull::demo()))]
pub struct UserRequestFull {
    #[Demo(value = r#"String::from("01J15C32QJE9R3RPHT04QJB9AD")"#)]
    pub username: String,
    #[Demo(value = r#"String::from("$argon2id$v=19$m=19456,t=2,p=1$/XFasj8WyfzzGzV2fnouWQ$OfHASwUrzgJmchn9LvM+T7IHtvI//W+BMgBe70jDnqU")"#)]
    pub password: String,
    #[Demo(value = r#"String::from("123456")"#)]
    pub token_2fa: String,
}

// 1. from database
/// user from database
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct UserDb {
    pub loginname: String,
    pub passweb: String,
    pub name: String,
    pub doctorcode: Option<String>,
    pub groupname: Option<String>,
    pub accessright: String,
    pub entryposition: Option<String>,
    pub licenseno: Option<String>,
    pub image: Option<ImageBase64>,
    pub theme: Option<String>,
    pub wide_screen: Option<String>,
    pub totp: Option<String>,
    pub ts: Option<u64>,
    pub failed: Option<i8>,
    pub totp_done: Option<bool>,
    pub can_passcode: bool,
    pub wards: Vec<String>,
    pub spclty_ids: Vec<u32>,
}

// 2. backend usage without credentials
// user without credentials
/// User data
#[derive(Clone, Debug, Demo, Deserialize, Serialize, PartialEq, ToSchema)]
#[schema(example = json!(User::demo()))]
pub struct User {
    #[Demo(value = r#"String::from("user")"#)]
    pub name: String,
    #[Demo(value = r#"String::from("007")"#)]
    pub doctorcode: String,
    #[Demo(value = r#"String::from("แพทย์")"#)]
    pub groupname: String,
    #[Demo(value = r#"String::from("[]")"#)]
    pub accessright: String,
    #[Demo(value = r#"String::from("นายแพทย์")"#)]
    pub entryposition: String,
    #[Demo(value = r#"String::from("ว00000")"#)]
    pub licenseno: String,
    #[Demo(value = "Some(ImageBase64::demo())")]
    pub image: Option<ImageBase64>,
    #[Demo(value = "true")]
    pub can_passcode: bool,
    #[Demo(value = r#"String::from("dark")"#)]
    pub theme: String,
    #[Demo(value = r#"String::from("card")"#)]
    pub wide_screen: String,
    #[Demo(value = "Some(true)")]
    pub totp_done: Option<bool>,
    #[Demo(value = r#"vec![String::from("01")]"#)]
    pub wards: Vec<String>,
    #[Demo(value = r#"vec![1]"#)]
    pub spclty_ids: Vec<u32>,
}

impl From<UserDb> for User {
    fn from(item: UserDb) -> Self {
        Self {
            name: item.name,
            doctorcode: item.doctorcode.unwrap_or_default(),
            groupname: item.groupname.unwrap_or_default(),
            accessright: item.accessright,
            entryposition: item.entryposition.unwrap_or_default(),
            licenseno: item.licenseno.unwrap_or_default(),
            image: item.image,
            can_passcode: item.can_passcode,
            theme: item.theme.unwrap_or(String::from("light")),
            wide_screen: item.wide_screen.unwrap_or(String::from("table")),
            totp_done: item.totp_done,
            wards: item.wards,
            spclty_ids: item.spclty_ids,
        }
    }
}

impl From<&UserDb> for User {
    fn from(item: &UserDb) -> Self {
        Self {
            name: item.name.clone(),
            doctorcode: item.doctorcode.clone().unwrap_or_default(),
            groupname: item.groupname.clone().unwrap_or_default(),
            accessright: item.accessright.clone(),
            entryposition: item.entryposition.clone().unwrap_or_default(),
            licenseno: item.licenseno.clone().unwrap_or_default(),
            image: item.image.clone(),
            can_passcode: item.can_passcode,
            theme: item.theme.clone().unwrap_or(String::from("light")),
            wide_screen: item.wide_screen.clone().unwrap_or(String::from("table")),
            totp_done: item.totp_done,
            wards: item.wards.to_owned(),
            spclty_ids: item.spclty_ids.to_owned(),
        }
    }
}

// 3. send to wasm client
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct UserClient {
    pub user: User,
    pub roles: Vec<CurrentUserRole>,
    pub permissions: Vec<Permission>,
    pub token: String,
    pub sub: String,
    pub iat: u64,
    pub exp: u64,
    pub earlier_second: i64,
}

impl UserClient {
    pub fn authorized(&self) -> bool {
        let now = get_timestamp_wasm();
        self.iat <= now && now <= self.exp
    }
}

// 4. use in wasm
#[derive(Clone, Debug)]
pub struct UserClientMutable {
    pub user: UserMutable,
    pub roles: MutableVec<Rc<CurrentUserRoleMutable>>,
    pub permissions: Mutable<Vec<Permission>>,
    pub token: Mutable<String>,
    pub sub: Mutable<String>,
    pub iat: Mutable<u64>,
    pub exp: Mutable<u64>,
    pub earlier_second: Mutable<i64>,
}

impl UserClientMutable {
    // 60 sec token, 10 sec after issued: iat = x1050 exp = x1110 server = x1060
    // earlier  45 sec: client = x1015 now_adjusted = x1060, (true, true, life=50 sec)
    // earlier  90 sec: client = x0970 now_adjusted = x1060, (true, true, life=50 sec)
    // earlier -45 sec: client = x1105 now_adjusted = x1060, (true, true, life=50 sec)
    // earlier -90 sec: client = x1150 now_adjusted = x1060, (true, true, life=50 sec)
    /// check now, iat and exp with 9 seconds transport time added
    pub fn authorized(&self) -> bool {
        let now = get_timestamp_wasm();
        let iat = self.iat.get();
        let exp = self.exp.get();
        let earlier = self.earlier_second.get();
        let now_adjusted = add_u64_with_i64(now, earlier).saturating_add(9);
        // log::debug!("Access Token TS: iat {} now {} exp {} earlier {}", iat, now, exp, earlier);
        (iat < now_adjusted) && (now_adjusted < exp)
    }
}

impl From<UserClient> for UserClientMutable {
    fn from(item: UserClient) -> Self {
        Self {
            user: item.user.into(),
            roles: MutableVec::new_with_values(item.roles.into_iter().map(|role| Rc::new(role.into())).collect()),
            permissions: Mutable::new(item.permissions),
            token: Mutable::new(item.token),
            sub: Mutable::new(item.sub),
            iat: Mutable::new(item.iat),
            exp: Mutable::new(item.exp),
            earlier_second: Mutable::new(item.earlier_second),
        }
    }
}
impl From<UserClientMutable> for UserClient {
    fn from(item: UserClientMutable) -> Self {
        Self {
            user: item.user.into(),
            roles: item.roles.lock_ref().iter().map(|role| role.as_ref().to_owned().into()).collect(),
            permissions: item.permissions.get_cloned(),
            token: item.token.get_cloned(),
            sub: item.sub.get_cloned(),
            iat: item.iat.get(),
            exp: item.exp.get(),
            earlier_second: item.earlier_second.get(),
        }
    }
}

// 5. use in wasm UserClientMutable
#[derive(Clone, Debug)]
pub struct UserMutable {
    pub name: Mutable<String>,
    pub doctorcode: Mutable<String>,
    pub groupname: Mutable<String>,
    pub accessright: Mutable<String>,
    pub entryposition: Mutable<String>,
    pub licenseno: Mutable<String>,
    pub image: Mutable<Rc<ImageBase64>>,
    pub can_passcode: bool,
    pub theme: Mutable<String>,
    pub wide_screen: Mutable<String>,
    pub totp_done: Mutable<Option<bool>>,
    pub wards: Mutable<Vec<String>>,
    pub spclty_ids: Mutable<Vec<u32>>,
}

impl From<User> for UserMutable {
    fn from(item: User) -> Self {
        Self {
            name: Mutable::new(item.name),
            doctorcode: Mutable::new(item.doctorcode),
            groupname: Mutable::new(item.groupname),
            accessright: Mutable::new(item.accessright),
            entryposition: Mutable::new(item.entryposition),
            licenseno: Mutable::new(item.licenseno),
            image: Mutable::new(Rc::new(item.image.unwrap_or(ImageBase64::new_user()))),
            can_passcode: item.can_passcode,
            theme: Mutable::new(item.theme),
            wide_screen: Mutable::new(item.wide_screen),
            totp_done: Mutable::new(item.totp_done),
            wards: Mutable::new(item.wards),
            spclty_ids: Mutable::new(item.spclty_ids),
        }
    }
}

impl From<UserMutable> for User {
    fn from(item: UserMutable) -> Self {
        Self {
            name: item.name.get_cloned(),
            doctorcode: item.doctorcode.get_cloned(),
            groupname: item.groupname.get_cloned(),
            accessright: item.accessright.get_cloned(),
            entryposition: item.entryposition.get_cloned(),
            licenseno: item.licenseno.get_cloned(),
            image: Some(item.image.get_cloned().as_ref().to_owned()),
            can_passcode: item.can_passcode,
            theme: item.theme.get_cloned(),
            wide_screen: item.wide_screen.get_cloned(),
            totp_done: item.totp_done.get(),
            wards: item.wards.get_cloned(),
            spclty_ids: item.spclty_ids.get_cloned(),
        }
    }
}

/// Login Response
#[derive(Clone, Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(LoginResponse::demo()))]
pub struct LoginResponse {
    #[Demo(value = r#"String::from("v9.public.eyJzdWIi")"#)]
    pub token: String,
    #[Demo(value = "1777777777")]
    pub timestamp: u64, // second
    #[Demo(value = r#"String::from("58Wh6szG")"#)]
    pub public: String,
    #[Demo(value = "AppStatus::demo()")]
    pub app_status: AppStatus,
    #[Demo(value = "User::demo()")]
    pub user: User,
    #[Demo(value = "vec![CurrentUserRole::demo()]")]
    pub roles: Vec<CurrentUserRole>,
    #[Demo(value = "vec![Permission::demo_lab_view()]")]
    pub permissions: Vec<Permission>,
}

impl LoginResponse {
    /// POST `EndPoint::User`<br>
    /// get User from password with access token (JSON) and refresh token (Cookie)<br>
    /// NULL for next process
    pub async fn call_api_post_access(username: &str, password: &str, app: Rc<AppState>) -> Result<Option<Self>, AppError> {
        let body_json = serde_json::to_string(&UserRequest {
            username: username.to_owned(),
            password: password.to_owned(),
        })
        .map_err(|e| Source::SerdeJson.to_teapot_error(e, "Authentication"))?;

        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Authentication"))?;

        match fetch_json_api(&EndPoint::User.base(), "POST", Some(&body), app).await {
            Ok((response, true)) => {
                let response: Option<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Login"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Login"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("DISCONNECTED")), "Fetch Json")),
        }
    }

    /// PATCH `EndPoint::User`<br>
    /// get User from 2fa-token with access token (JSON) and refresh token (Cookie)<br>
    /// NULL if timeout
    pub async fn call_api_patch_access_2fa(is_sub: bool, username: &str, token_2fa: &str, app: Rc<AppState>) -> Result<Option<Self>, AppError> {
        let body_json = serde_json::to_string(&UserRequest2fa {
            username: username.to_owned(),
            token_2fa: token_2fa.to_owned(),
            is_sub,
        })
        .map_err(|e| Source::SerdeJson.to_teapot_error(e, "Authentication"))?;

        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Authentication"))?;

        match fetch_json_api(&EndPoint::User.base(), "PATCH", Some(&body), app).await {
            Ok((response, true)) => {
                let response: Option<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Login"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Login"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("DISCONNECTED")), "Fetch Json")),
        }
    }

    /// GET `EndPoint::User`<br>
    /// get User with access token (JSON)
    pub async fn call_api_get_access_renew(app: Rc<AppState>) -> Result<Self, AppError> {
        match fetch_json_api(&EndPoint::User.base(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Self = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Refresh"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Refresh"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("DISCONNECTED")), "Fetch Json")),
        }
    }

    /// PUT `EndPoint::User`<br>
    /// get User with access token (JSON) and refresh token (Cookie)
    pub async fn call_api_put_refresh_renew(raw_password: &str, token_2fa: &str, app: Rc<AppState>) -> Result<Self, AppError> {
        let password = hash(raw_password).map_err(|e| Source::PasswordHash.to_teapot_error(e, "Renew Refresh"))?;

        match app.token_sub() {
            Some(sub) => {
                let body_json = serde_json::to_string(&UserRequestFull {
                    username: sub,
                    password,
                    token_2fa: token_2fa.to_owned(),
                })
                .map_err(|e| Source::SerdeJson.to_teapot_error(e, "Renew Refresh"))?;

                let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Renew Refresh"))?;

                match fetch_json_api(&EndPoint::User.base(), "PUT", Some(&body), app).await {
                    Ok((response, true)) => {
                        let response: Self = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Login"))?;
                        Ok(response)
                    }
                    Ok((app_error, false)) => {
                        let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Login"))?;
                        Err(error)
                    }
                    Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("DISCONNECTED")), "Fetch Json")),
                }
            }
            None => Err(Source::App.to_teapot_error("Token sub not found", "Renew Refresh")),
        }
    }
}

// general usage
/// Current User Role
#[derive(Clone, Debug, Demo, Deserialize, FromRow, Serialize, PartialEq, ToSchema)]
#[schema(example = json!(CurrentUserRole::demo()))]
pub struct CurrentUserRole {
    #[Demo(value = r#"String::from("DOCTOR_STAFF")"#)]
    pub role: String,
    #[Demo(value = r#"String::from("แพทย์ STAFF")"#)]
    pub role_desc: String,
}

#[derive(Clone, Debug)]
pub struct CurrentUserRoleMutable {
    pub role: Mutable<String>,
    pub role_desc: Mutable<String>,
}

impl From<CurrentUserRole> for CurrentUserRoleMutable {
    fn from(item: CurrentUserRole) -> Self {
        Self {
            role: Mutable::new(item.role),
            role_desc: Mutable::new(item.role_desc),
        }
    }
}

impl From<CurrentUserRoleMutable> for CurrentUserRole {
    fn from(item: CurrentUserRoleMutable) -> Self {
        Self {
            role: item.role.get_cloned(),
            role_desc: item.role_desc.get_cloned(),
        }
    }
}

#[cfg(test)]
mod test_hash {

    use super::*;

    #[test]
    fn test_md5_enc() {
        let result: String = md5_enc("1234").iter().map(|byte| format!("{:02x}", byte)).collect();
        assert_eq!(result.as_str(), "81dc9bdb52d04dc20036dbd8313ed055");
    }

    #[test]
    fn test_argon2() {
        let result = argon2_enc(&md5_enc("1234")).unwrap();
        assert_ne!(result.as_str(), "$argon2id$v=19$m=19456,t=2,p=1$/XFasj8WyfzzGzV2fnouWQ$OfHASwUrzgJmchn9LvM+T7IHtvI//W+BMgBe70jDnqU");
    }

    // fn md5(plain: &str) -> Vec<u8> {
    //     // generate MD5
    //     let mut md5_hasher = Md5::new();
    //     md5_hasher.update(plain.as_bytes());
    //     md5_hasher.finalize().to_vec()
    // }
    // #[test]
    // fn test_md5() {
    //     let plain_password = "ใช้ชื่อจริง";
    //     let md5: String = md5(plain_password).iter().map(|byte| format!("{:02x}", byte)).collect();
    //     assert_eq!(md5.as_str(), "9c44a542d55e547dbb449c211d1fcf9f");
    //     let windows874_md5: String = md5_enc(plain_password).iter().map(|byte| format!("{:02x}", byte)).collect();
    //     assert_eq!(windows874_md5.as_str(), "467f7e270cd2025dbb5cf3befc5c56be");
    // }
}
