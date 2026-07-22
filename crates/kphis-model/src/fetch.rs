pub use http::Method;

use js_sys::{Array, ArrayBuffer, JsString, Uint8Array};
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlQueryResult;
use std::rc::Rc;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Blob, BlobPropertyBag, File, FormData, Headers, RequestInit, Response};

use kphis_util::{
    abort::Abort,
    error::{AppError, Source},
};

use crate::{app::AppState, endpoint::EndPoint};

/// Database execution result
#[derive(Debug, Default, Deserialize, Serialize, utoipa::ToSchema, utoipa::ToResponse)]
pub struct ExecuteResponse {
    /// 'Last Inserted ID' reported by Database. Error will be 0
    #[schema(example = "0")]
    pub last_insert_id: u64,
    /// 'Row Affected' reported by Database. Error will be 0
    #[schema(example = "0")]
    pub rows_affected: u64,
    /// Error source and message. None is no error
    #[schema(example = "source: App, message: Tea Pot Broken")]
    pub error: Option<String>,
    /// Action that generate this error. None is no error
    #[schema(example = "MakeTea")]
    pub action: Option<String>,
}

impl ExecuteResponse {
    pub fn from_query_result(result: MySqlQueryResult, action: &str) -> Self {
        Self {
            last_insert_id: result.last_insert_id(),
            rows_affected: result.rows_affected(),
            action: Some(action.to_owned()),
            ..Default::default()
        }
    }
    pub fn with_action(mut self, action: &str) -> Self {
        self.action = Some(action.to_owned());
        self
    }
}

impl From<AppError> for ExecuteResponse {
    fn from(item: AppError) -> Self {
        Self {
            error: Some(item.string()),
            action: Some(item.action),
            ..Default::default()
        }
    }
}

/// GET `EndPoint::ExistsKeyId`
pub async fn call_api_get_exists_key_id(key: &str, id: &str, app: Rc<AppState>) -> Result<bool, AppError> {
    match fetch_json_api(&[&EndPoint::ExistsKeyId.base(), key, id].concat(), "GET", None, app).await {
        Ok((response, true)) => {
            let response = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch ExistsFromUrl"))?;
            Ok(response)
        }
        Ok((app_error, false)) => {
            let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch ExistsFromUrl"))?;
            Err(error)
        }
        Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
    }
}

pub async fn get_text_from_url(url: &str, app: Rc<AppState>) -> Result<Option<String>, AppError> {
    match fetch_text_api(url, app).await {
        Ok((response, true)) => {
            let response = response.dyn_ref::<JsString>().map(|s| s.into());
            Ok(response)
        }
        Ok((app_error, false)) => {
            let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch TextFromUrl"))?;
            Err(error)
        }
        Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Text")),
    }
}

pub async fn get_blob_from_url(url: &str, mime: &str, app: Rc<AppState>) -> Result<Blob, AppError> {
    match fetch_blob_api(url, mime, app).await {
        Ok((response, true)) => Ok(response.unchecked_into::<Blob>()),
        Ok((app_error, false)) => {
            let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch BlobFromUrl"))?;
            Err(error)
        }
        Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Blob")),
    }
}

/// return ((value, is_ok), error), not check token
pub async fn fetch_json_api(url: &str, method: &str, body: Option<&JsValue>, app: Rc<AppState>) -> Result<(JsValue, bool), JsValue> {
    let abort = Abort::new()?;

    let headers = Headers::new()?;
    headers.set("Accept", "application/json")?;
    headers.set("Content-Type", "application/json")?;
    if app.no_cache_mode.get() {
        headers.set("Cache-Control", "no-cache")?;
        headers.set("Pragma", "no-cache")?;
    }
    if let Some(bearer) = app.token() {
        headers.set("Authorization", &["Bearer ", &bearer].concat())?;
    }

    let future = app.window.with(|w| {
        let init = RequestInit::new();
        init.set_method(method);
        init.set_headers(&headers);
        if let Some(b) = body {
            init.set_body(b);
        }
        init.set_signal(Some(&abort.signal()));
        w.fetch_with_str_and_init(url, &init)
    });

    let response = JsFuture::from(future).await?.unchecked_into::<Response>();

    let value = JsFuture::from(response.json()?).await?;

    // if response.status() == 401 {
    //     log::debug!("401 from server, remove user and redirect to index page");
    //     app.remove_user_and_go_index();
    // }

    if response.ok() { Ok((value, true)) } else { Ok((value, false)) }
}

// pub async fn fetch_json_api(url: &str, method: &str, body: Option<&JsValue>, app: Rc<AppState>) -> Result<(JsValue, bool), JsValue> {
//     let abort = Abort::new()?;

//     let headers = Headers::new()?;
//     headers.set("Accept", "application/json")?;
//     headers.set("Content-Type", "application/json")?;
//     if app.no_cache_mode.get() {
//         headers.set("Cache-Control", "no-cache")?;
//         headers.set("Pragma", "no-cache")?;
//     }
//     if let Some(bearer) = app.token() {
//         headers.set("Authorization", &["Bearer ", &bearer].concat())?;
//     }

//     let future = app.window.with(|w| {
//         let init = RequestInit::new();
//         init.set_method(method);
//         init.set_headers(&headers);
//         if let Some(b) = body {
//             init.set_body(b);
//         }
//         init.set_signal(Some(&abort.signal()));
//         w.fetch_with_str_and_init(url, &init)
//     });
//     let response = JsFuture::from(future).await?.unchecked_into::<Response>();
//     let value = JsFuture::from(response.json()?).await?;

//     match response.status() {
//         200 => Ok((value, true)),
//         401 => {
//             log::debug!("401 from server, remove user and redirect to index page");
//             app.remove_user_and_go_index();
//             Ok((value, false))
//         }
//         429 => {
//             // try again after waiting (once)
//             TimeoutFuture::new(1000).await;
//             let future_2 = app.window.with(|w| {
//                 let init_2 = RequestInit::new();
//                 init_2.set_method(method);
//                 init_2.set_headers(&headers);
//                 if let Some(b) = body {
//                     init_2.set_body(b);
//                 }
//                 init_2.set_signal(Some(&abort.signal()));
//                 w.fetch_with_str_and_init(url, &init_2)
//             });
//             let response_2 = JsFuture::from(future_2).await?.unchecked_into::<Response>();
//             let value_2 = JsFuture::from(response_2.json()?).await?;
//             match response.status() {
//                 200 => Ok((value_2, true)),
//                 401 => {
//                     log::debug!("401 from server, remove user and redirect to index page");
//                     app.remove_user_and_go_index();
//                     Ok((value_2, false))
//                 }
//                 _ => Ok((value_2, false)),
//             }
//         }
//         _ => Ok((value, false)),
//     }
// }

/// only GET method, return ((value, is_ok), error), not check token
pub async fn fetch_text_api(url: &str, app: Rc<AppState>) -> Result<(JsValue, bool), JsValue> {
    let abort = Abort::new()?;

    let headers = Headers::new()?;
    headers.set("Accept", "text/plain;charset=UTF-8")?;
    if app.no_cache_mode.get() {
        headers.set("Cache-Control", "no-cache")?;
        headers.set("Pragma", "no-cache")?;
    }
    if let Some(bearer) = app.token() {
        headers.set("Authorization", &["Bearer ", &bearer].concat())?;
    }

    let future = app.window.with(|w| {
        let init = RequestInit::new();
        init.set_method("GET");
        init.set_headers(&headers);
        init.set_signal(Some(&abort.signal()));
        w.fetch_with_str_and_init(url, &init)
    });

    let response = JsFuture::from(future).await?.unchecked_into::<Response>();

    // if response.status() == 401 {
    //     log::debug!("401 from server, remove user and redirect to index page");
    //     app.remove_user_and_go_index();
    // }

    if response.ok() {
        let value = JsFuture::from(response.text()?).await?;
        Ok((value, true))
    } else {
        let value = JsFuture::from(response.json()?).await?;
        Ok((value, false))
    }
}

/// only GET method, return ((value, is_ok), error), not check token
pub async fn fetch_blob_api(url: &str, mime: &str, app: Rc<AppState>) -> Result<(JsValue, bool), JsValue> {
    let abort = Abort::new()?;

    let headers = Headers::new()?;
    headers.set("Accept", mime)?;
    if app.no_cache_mode.get() {
        headers.set("Cache-Control", "no-cache")?;
        headers.set("Pragma", "no-cache")?;
    }
    if let Some(bearer) = app.token() {
        headers.set("Authorization", &["Bearer ", &bearer].concat())?;
    }

    let future = app.window.with(|w| {
        let init = RequestInit::new();
        init.set_method("GET");
        init.set_headers(&headers);
        init.set_signal(Some(&abort.signal()));
        w.fetch_with_str_and_init(url, &init)
    });

    let response = JsFuture::from(future).await?.unchecked_into::<Response>();

    // if response.status() == 401 {
    //     log::debug!("401 from server, remove user and redirect to index page");
    //     app.remove_user_and_go_index();
    // }

    if response.ok() {
        let value = JsFuture::from(response.blob()?).await?;
        Ok((value, true))
    } else {
        let value = JsFuture::from(response.json()?).await?;
        Ok((value, false))
    }
}

pub async fn post_multipart(url: &str, body: &FormData, app: Rc<AppState>) -> Result<(JsValue, bool), JsValue> {
    let abort = Abort::new()?;

    let headers = Headers::new()?;
    headers.set("Accept", "multipart/form-data")?;
    if let Some(bearer) = app.token() {
        headers.set("Authorization", &["Bearer ", &bearer].concat())?;
    }

    let future = app.window.with(|w| {
        let init = RequestInit::new();
        init.set_method("POST");
        init.set_headers(&headers);
        init.set_body(body);
        init.set_signal(Some(&abort.signal()));
        w.fetch_with_str_and_init(url, &init)
    });

    let response = JsFuture::from(future).await?.unchecked_into::<Response>();

    // if response.status() == 401 {
    //     log::debug!("401 from server, remove user and redirect to index page");
    //     app.remove_user_and_go_index();
    // }

    if response.ok() {
        let value = JsFuture::from(response.json()?).await?;
        Ok((value, true))
    } else {
        let value = JsFuture::from(response.json()?).await?;
        Ok((value, false))
    }
}

/// `execute` api, return single ExecuteResponse
pub async fn execute_fetch(path: &str, method: &str, body: Option<&JsValue>, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
    match fetch_json_api(path, method, body, app).await {
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

/// `execute` api, return single String
pub async fn execute_fetch_text(path: &str, method: &str, body: Option<&JsValue>, app: Rc<AppState>) -> Result<String, AppError> {
    match fetch_json_api(path, method, body, app).await {
        Ok((response, true)) => {
            let response = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Execute Fetch"))?;
            Ok(response)
        }
        Ok((app_error, false)) => {
            let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Execute Fetch"))?;
            Err(error)
        }
        Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
    }
}

/// `execute` api, return multiple ExecuteResponses
pub async fn execute_fetch_vec(path: &str, method: &str, body: Option<&JsValue>, app: Rc<AppState>) -> Result<Vec<ExecuteResponse>, AppError> {
    match fetch_json_api(path, method, body, app).await {
        Ok((response, true)) => {
            let response: Vec<ExecuteResponse> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Execute FetchVec"))?;
            Ok(response)
        }
        Ok((app_error, false)) => {
            let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Execute FetchVec"))?;
            Err(error)
        }
        Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
    }
}

/// `execute` api, return single ExecuteResponse with u32
pub async fn execute_fetch_with_u32(path: &str, method: &str, body: Option<&JsValue>, app: Rc<AppState>) -> Result<(u32, ExecuteResponse), AppError> {
    match fetch_json_api(path, method, body, app).await {
        Ok((response, true)) => {
            let response: (u32, ExecuteResponse) = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Execute Fetch+u32"))?;
            Ok(response)
        }
        Ok((app_error, false)) => {
            let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Execute Fetch+u32"))?;
            Err(error)
        }
        Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
    }
}

/// `execute` api, return multiple ExecuteResponses with u32
pub async fn execute_fetch_vec_with_u32(path: &str, method: &str, body: Option<&JsValue>, app: Rc<AppState>) -> Result<(u32, Vec<ExecuteResponse>), AppError> {
    match fetch_json_api(path, method, body, app).await {
        Ok((response, true)) => {
            let response: (u32, Vec<ExecuteResponse>) = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Execute FetchVec+u32"))?;
            Ok(response)
        }
        Ok((app_error, false)) => {
            let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Execute FetchVec+u32"))?;
            Err(error)
        }
        Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
    }
}

//============//
// Conversion //
//============//
// Rust <-> JavaScript conversions in [serde-wasm-bindgen](https://docs.rs/serde-wasm-bindgen/latest/serde_wasm_bindgen/)
// supported only Unit8Array/ArrayBuffer/Array <-> Vec<u8>

pub async fn file_to_bytes(file: &File) -> Result<Vec<u8>, JsValue> {
    let buffer = JsFuture::from(file.array_buffer()).await?.unchecked_into::<ArrayBuffer>();
    buffer_to_bytes(&buffer)
}

pub async fn blob_to_bytes(blob: &Blob) -> Result<Vec<u8>, JsValue> {
    let buffer = JsFuture::from(blob.array_buffer()).await?.unchecked_into::<ArrayBuffer>();
    buffer_to_bytes(&buffer)
}

fn buffer_to_bytes(buffer: &ArrayBuffer) -> Result<Vec<u8>, JsValue> {
    // ArrayBuffer to Uint8Array
    let file_u8a = Uint8Array::new(buffer);
    // Unit8Array to [u8]
    // `copy_to` need both source and destination to have the same lentgh
    let mut file_buf = vec![0; file_u8a.length() as usize];
    file_u8a.copy_to(&mut file_buf);
    Ok(file_buf)
}

pub fn bytes_to_blob(bytes: &[u8], content_type: &str) -> Result<Blob, JsValue> {
    // [u8] to Uint8Array
    let img_u8a = Uint8Array::new_with_length(bytes.len() as u32);
    img_u8a.copy_from(bytes);
    // Uint8Array to Array sequence
    let img_array = Array::new_with_length(1);
    img_array.set(0, img_u8a.into());
    // Array sequence of Uint8Array to Blob
    let option = BlobPropertyBag::new();
    option.set_type(content_type);
    Blob::new_with_u8_array_sequence_and_options(&img_array, &option)
}
