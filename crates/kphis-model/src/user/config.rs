use derive_demo::Demo;
use js_sys::JsString;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use utoipa::ToSchema;
use wasm_bindgen::JsCast;

use kphis_util::error::{AppError, Source};

use crate::{
    app::AppState,
    endpoint::EndPoint,
    fetch::{ExecuteResponse, fetch_json_api},
};

/// User Client Configuration
#[derive(Debug, Demo, Default, Serialize, Deserialize, ToSchema)]
#[schema(example = json!(UserConfig::demo()))]
pub struct UserConfig {
    /// "light", "dark", "auto"
    #[Demo(value = r#"Some(String::from("dark"))"#)]
    pub theme: Option<String>,
    /// "table", "card"
    #[Demo(value = r#"Some(String::from("card"))"#)]
    pub wide_screen: Option<String>,
    /// None mean 'not change' TOTP
    #[Demo(value = "Some(true)")]
    pub totp: Option<bool>,
}

#[derive(Debug, Demo, Serialize, Deserialize, ToSchema)]
#[schema(example = json!(UserConfigResponse::demo()))]
pub struct UserConfigResponse {
    #[Demo(
        value = r#"Some(String::from("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"))"#
    )]
    pub totp: Option<String>,
    #[Demo(value = r#"ExecuteResponse::default()"#)]
    pub result: ExecuteResponse,
}

impl UserConfigResponse {
    /// POST `EndPoint::UserConfig`
    pub async fn call_api_post(user_config: &UserConfig, app: Rc<AppState>) -> Result<Self, AppError> {
        let body_json = serde_json::to_string(user_config).unwrap();
        let body = serde_wasm_bindgen::to_value(&body_json).unwrap();
        match fetch_json_api(&EndPoint::UserConfig.base(), "POST", Some(&body), app).await {
            Ok((response, true)) => {
                let response: Self = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Execute Fetch"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Execute Fetch"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}

/// User Client Configuration edit command
#[derive(Debug, Demo, Serialize, Deserialize, ToSchema)]
#[schema(example = json!(UserConfigCommand::demo_clear2fa(String::from("user"))))]
pub enum UserConfigCommand {
    Clear2fa(String),
}

impl UserConfigCommand {
    /// PATCH `EndPoint::UserConfig`
    pub async fn call_api_patch(&self, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        let body_json = serde_json::to_string(self).unwrap();
        let body = serde_wasm_bindgen::to_value(&body_json).unwrap();
        match fetch_json_api(&EndPoint::UserConfig.base(), "PATCH", Some(&body), app).await {
            Ok((response, true)) => {
                let response: ExecuteResponse = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Execute Fetch"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Execute Fetch"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}
