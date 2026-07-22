use bitcode::{Decode, Encode};
use derive_demo::Demo;
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal},
    signal_vec::{MutableVec, SignalVecExt},
};
use http::Method;
use js_sys::{Array, JsString, Promise, Uint8Array};
use serde_derive::{Deserialize, Serialize};
use std::{cmp::Ordering, collections::HashMap, future::Future, rc::Rc, sync::Arc, thread::LocalKey};
use time::{Date, Duration, Time, ext::NumericalDuration, macros::time};
use utoipa::ToSchema;
use wasm_bindgen::{JsCast, JsValue, closure::Closure};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Blob, BlobPropertyBag, Document, Element, HtmlElement, Range, ScrollBehavior, ScrollToOptions, Selection, ServiceWorkerRegistration, Storage, Text, Url, Window, window};

use kphis_util::{
    datetime::{duration_hm, js_now, time_hm},
    error::{AppError, Source},
    loader::AsyncLoader,
};

use crate::{
    ASSETS_PREFIX,
    endpoint::EndPoint,
    fetch::{blob_to_bytes, execute_fetch_text, fetch_blob_api, fetch_json_api, get_blob_from_url},
    image::file_path::ImagePath,
    order::Order,
    route::Route,
    score::SupportedScore,
    search::searchbox::DrugUsage,
    select_utils::{ColorSelectOption, SelectOption},
    shift::{NurseShift, Shift},
    sse::{SseData, SseGroup, SseMessage},
    timer::Timeout,
    user::{his::UserClientMutable, permission::Permission},
    vital_sign::VsMode,
};

thread_local! {
    pub static WINDOW: Window = window().unwrap();
    static STORAGE: Storage = WINDOW.with(|w| w.local_storage().unwrap().unwrap());
}

/// UI state
pub struct AppState {
    pub window: &'static LocalKey<Window>,
    pub storage: &'static LocalKey<Storage>,
    loader: AsyncLoader,
    pub route: Mutable<Route>,
    pub host: Mutable<String>,
    pub app_status: Mutable<Option<Rc<AppStatus>>>,
    pub app_asset: Mutable<Option<Rc<AppAsset>>>,
    /// will `true` manually or by `new` AppState (when No `app` in localStorage)<br>
    /// NOTE: localStorage.clear() will called when<br>
    ///   1. Successfully install Service Worker (called when reload)<br>
    ///   2. `App::logout(app.clone(), true)`, (now only called when `clear cache`)<br>
    /// will `false` manually or after get `AppAsset`
    pub no_cache_mode: Mutable<bool>,

    pub ward_select: Mutable<String>,
    pub ward_multiple_select: Mutable<String>, // comma delimited
    pub inverse_ward_select: Mutable<String>,
    pub summary_status_select: Mutable<String>,
    pub inscl_select: Mutable<String>,
    pub adm_doctor_select: Mutable<String>,
    pub dch_doctor_select: Mutable<String>,
    pub spclty_select: Mutable<String>,

    pub monitor_refresh_interval: Mutable<String>,
    pub ipd_pharmacy_order_monitor_is_discharged: Mutable<String>,
    pub opd_er_pharmacy_order_monitor_is_discharged: Mutable<String>,
    pub order_monitor_new_order_sound_on: Mutable<String>,
    pub uploaded_images: MutableVec<Rc<ImagePath>>,

    // // Not to localstorage
    /// for sending edited data from med-reconcile form
    pub edit_order: Mutable<Option<Rc<Order>>>,
    pub report_select: Mutable<String>,
    pub aside_prev_percent: Mutable<f64>,
    pub aside_prev_percent_memoize: Mutable<f64>,
    pub user: Mutable<Option<Rc<UserClientMutable>>>,
    pub msg_private: MutableVec<SseData>,
    pub msg_ward: MutableVec<SseData>,
    pub msg_spclty: MutableVec<SseData>,
    pub msg_global: MutableVec<SseData>,
    pub clipboard_images: MutableVec<Rc<ImagePath>>,

    pub post_admit_count: Mutable<i64>,
    pub ipd_order_as: MutableVec<Rc<Order>>,
    pub opd_er_order_as: MutableVec<Rc<Order>>,
    pub vs_mode: Mutable<VsMode>,
    pub start_dchdate: Mutable<String>,
    pub end_dchdate: Mutable<String>,

    pub pharmacist_allow_non_med: Mutable<bool>,

    pub drag_start_state: Mutable<Option<DragStartState>>,

    scroll_position: Mutable<(f64, f64)>,
}

impl AppState {
    fn new(route_url: &str) -> Rc<Self> {
        let now = js_now().date();
        let host = Url::new(route_url).map(|u| u.host()).unwrap_or_default();
        Rc::new(Self {
            window: &WINDOW,
            storage: &STORAGE,
            loader: AsyncLoader::default(),
            route: Mutable::new(Route::from_url(route_url, &host)),
            host: Mutable::new(host),
            app_status: Mutable::new(None),
            app_asset: Mutable::new(None),
            no_cache_mode: Mutable::new(true),

            ward_select: Mutable::new(String::new()),
            ward_multiple_select: Mutable::new(String::new()),
            inverse_ward_select: Mutable::new(String::new()),
            summary_status_select: Mutable::new(String::new()),
            inscl_select: Mutable::new(String::new()),
            adm_doctor_select: Mutable::new(String::new()),
            dch_doctor_select: Mutable::new(String::new()),
            spclty_select: Mutable::new(String::new()),

            monitor_refresh_interval: Mutable::new(String::from("60")),
            ipd_pharmacy_order_monitor_is_discharged: Mutable::new(String::from("N")),
            opd_er_pharmacy_order_monitor_is_discharged: Mutable::new(String::from("N")),
            order_monitor_new_order_sound_on: Mutable::new(String::new()),
            uploaded_images: MutableVec::new(),

            // InMemory only
            edit_order: Mutable::new(None),
            report_select: Mutable::new(String::new()),
            aside_prev_percent: Mutable::new(100.0),
            aside_prev_percent_memoize: Mutable::new(55.0),
            user: Mutable::new(None),
            msg_private: MutableVec::new(),
            msg_ward: MutableVec::new(),
            msg_spclty: MutableVec::new(),
            msg_global: MutableVec::new(),
            clipboard_images: MutableVec::new(),

            post_admit_count: Mutable::new(0),
            ipd_order_as: MutableVec::new(),
            opd_er_order_as: MutableVec::new(),
            vs_mode: Mutable::new(VsMode::General),
            start_dchdate: Mutable::new((now - 12.weeks()).to_string()),
            end_dchdate: Mutable::new(now.to_string()),

            pharmacist_allow_non_med: Mutable::new(false),

            drag_start_state: Mutable::new(None),

            scroll_position: Mutable::new((0.0, 0.0)),
        })
    }

    pub fn new_from_local_storage(route_url: &str) -> Rc<Self> {
        // we use ok() instead of unwrap()
        STORAGE.with(|storage| {
            storage
                .get_item("app")
                .ok()
                .flatten()
                .and_then(|state_json| serde_json::from_str::<AppLocalStorage>(state_json.as_str()).ok())
                .map(|item| Rc::new(Self::from_local_storage(route_url, item)))
                .unwrap_or(Self::new(route_url))
        })
    }

    fn from_local_storage(route_url: &str, item: AppLocalStorage) -> Self {
        let now = js_now().date();
        let host = Url::new(route_url).map(|u| u.host()).unwrap_or_default();
        Self {
            window: &WINDOW,
            storage: &STORAGE,
            loader: AsyncLoader::new(),
            route: Mutable::new(Route::from_url(route_url, &host)),
            host: Mutable::new(host),
            app_status: Mutable::new(item.app_status.map(Rc::new)),
            app_asset: Mutable::new(None),
            no_cache_mode: Mutable::new(false),

            ward_select: Mutable::new(item.ward_select),
            ward_multiple_select: Mutable::new(item.ward_multiple_select),
            inverse_ward_select: Mutable::new(item.inverse_ward_select),
            summary_status_select: Mutable::new(item.summary_status_select),
            inscl_select: Mutable::new(item.inscl_select),
            adm_doctor_select: Mutable::new(item.adm_doctor_select),
            dch_doctor_select: Mutable::new(item.dch_doctor_select),
            spclty_select: Mutable::new(item.spclty_select),

            monitor_refresh_interval: Mutable::new(item.monitor_refresh_interval),
            ipd_pharmacy_order_monitor_is_discharged: Mutable::new(item.ipd_pharmacy_order_monitor_is_discharged),
            opd_er_pharmacy_order_monitor_is_discharged: Mutable::new(item.opd_er_pharmacy_order_monitor_is_discharged),
            order_monitor_new_order_sound_on: Mutable::new(item.order_monitor_new_order_sound_on),
            uploaded_images: MutableVec::new_with_values(item.uploaded_images.into_iter().map(Rc::new).collect()),

            // InMemory only
            edit_order: Mutable::new(None),
            report_select: Mutable::new(String::new()),
            aside_prev_percent: Mutable::new(100.0),
            aside_prev_percent_memoize: Mutable::new(55.0),
            user: Mutable::new(None),
            msg_private: MutableVec::new(),
            msg_ward: MutableVec::new(),
            msg_spclty: MutableVec::new(),
            msg_global: MutableVec::new(),
            clipboard_images: MutableVec::new(),

            post_admit_count: Mutable::new(0),
            ipd_order_as: MutableVec::new(),
            opd_er_order_as: MutableVec::new(),
            vs_mode: Mutable::new(VsMode::General),
            start_dchdate: Mutable::new((now - 4.weeks()).to_string()),
            end_dchdate: Mutable::new(now.to_string()),

            pharmacist_allow_non_med: Mutable::new(false),

            drag_start_state: Mutable::new(None),

            scroll_position: Mutable::new((0.0, 0.0)),
        }
    }

    pub fn to_local_storage(&self) {
        let app = AppLocalStorage::from(self);
        if let Ok(value) = serde_json::to_string(&app) {
            self.storage.with(|storage| {
                if let Err(e) = storage.set_item("app", &value) {
                    self.show_jsvalue_message(&e);
                }
            })
        }
    }

    pub fn clear_in_memory_except_user(&self) {
        let now = js_now().date();
        self.edit_order.set(None);
        self.aside_prev_percent.set(100.0);
        self.aside_prev_percent_memoize.set(55.0);
        self.msg_private.lock_mut().clear();
        self.msg_ward.lock_mut().clear();
        self.msg_spclty.lock_mut().clear();
        self.msg_global.lock_mut().clear();
        self.clipboard_images.lock_mut().clear();

        self.ipd_order_as.lock_mut().clear();
        self.opd_er_order_as.lock_mut().clear();
        self.vs_mode.set(VsMode::General);
        self.start_dchdate.set((now - 4.weeks()).to_string());
        self.end_dchdate.set(now.to_string());

        self.pharmacist_allow_non_med.set(false);

        self.drag_start_state.set(None);

        self.scroll_position.set((0.0, 0.0));
    }

    pub fn clear_local_storage(&self) {
        self.storage.with(|storage| {
            if let Err(e) = storage.clear() {
                self.show_jsvalue_message(&e);
            }
        })
    }

    pub fn hard_redirect(&self) {
        self.route.lock_ref().hard_redirect()
    }

    pub fn get_id(&self, id: &str) -> Option<Element> {
        self.window.with(|w| w.document().and_then(|d| d.get_element_by_id(id)))
    }

    // unwrap result to 0
    pub fn get_random(&self, numbers: &mut [u8]) -> u8 {
        self.window.with(|w| {
            w.crypto()
                .map(|c| {
                    if let Ok(obj) = c.get_random_values_with_u8_array(numbers) {
                        let u8a: Uint8Array = obj.unchecked_into();
                        let mut buf = vec![0u8; u8a.length() as usize];
                        u8a.copy_to(&mut buf);
                        buf.first().copied().unwrap_or_default()
                    } else {
                        0
                    }
                })
                .unwrap_or_default()
        })
    }

    pub fn query_selector(&self, selectors: &str) -> Option<Element> {
        self.window.with(|w| w.document().and_then(|d| d.query_selector(selectors).ok().flatten()))
    }

    pub fn scroll_into_view(&self, id: &str) {
        if let Some(elm) = self.get_id(id) {
            elm.scroll_into_view();
        }
    }

    /// return to last scroll_y position
    pub fn scroll_position_restore(&self) {
        let (x, y) = self.scroll_position.get();
        let option = ScrollToOptions::new();
        option.set_left(x);
        option.set_top(y);
        option.set_behavior(ScrollBehavior::Instant);
        self.window.with(|w| w.scroll_to_with_scroll_to_options(&option))
    }

    /// set current scroll_y position
    pub fn scroll_position_set(&self) {
        self.window.with(|w| {
            self.scroll_position.set((w.scroll_x().unwrap_or_default(), w.scroll_y().unwrap_or_default()));
        })
    }

    pub fn update_sw(&self) {
        // log::debug!("checking Service Worker for update");
        if let Some(elm) = self.get_id("checkUpdate").and_then(|elm| elm.dyn_into::<HtmlElement>().ok()) {
            elm.click();
        }
    }

    pub async fn unregister_sw(&self) {
        let (sw, location) = self.window.with(|w| (w.navigator().service_worker(), w.location()));
        let regs = JsFuture::from(sw.get_registrations()).await.unwrap();
        let reg_arr = Array::from(&regs);
        for reg_v in reg_arr {
            let reg = reg_v.dyn_into::<ServiceWorkerRegistration>().unwrap();
            let _ = JsFuture::from(reg.unregister().unwrap()).await.unwrap();
        }
        if let Err(e) = location.reload_with_forceget(true) {
            self.show_jsvalue_message(&e);
        }
    }
    pub async fn delete_caches(&self) {
        match self.window.with(|w| w.caches()) {
            Ok(caches) => {
                let mut delete_key = |key: JsValue, _id: u32, _arr: Array| {
                    let cache_name = key.as_string().unwrap_or_default();
                    JsValue::from(caches.delete(&cache_name))
                };
                let keylist = JsFuture::from(caches.keys()).await.unwrap();
                let keys = Array::from(&keylist);
                let result = Promise::all(&keys.map(&mut delete_key));
                let _ = JsFuture::from(result).await.unwrap();
            }
            Err(e) => self.show_jsvalue_message(&e),
        }
    }

    pub fn load_theme(&self) {
        self.set_bs_theme(&self.user.lock_ref().as_ref().map(|uc| uc.user.theme.get_cloned()).unwrap_or(String::from("light")));
    }
    pub fn set_theme(&self, theme: &str) {
        self.set_bs_theme(theme);
        if let Some(t) = self.user.lock_ref().as_ref().map(|uc| uc.user.theme.clone()) {
            t.set_neq(String::from(theme));
        }
    }
    fn set_bs_theme(&self, theme: &str) {
        let media_query = if theme == "auto" { self.get_browser_theme() } else { theme };
        self.window.with(|w| w.document().unwrap().body().unwrap().set_attribute("data-bs-theme", media_query).unwrap());
    }
    fn get_browser_theme(&self) -> &'static str {
        self.window.with(|w| match w.match_media("(prefers-color-scheme: dark)") {
            Ok(Some(media_query_list)) => {
                if media_query_list.matches() {
                    "dark"
                } else {
                    "light"
                }
            }
            Ok(None) | Err(_) => "light",
        })
    }

    pub fn set_wide_screen_mode(&self, wide_screen_mode: &str) {
        if let Some(w) = self.user.lock_ref().as_ref().map(|uc| uc.user.wide_screen.clone()) {
            w.set_neq(String::from(wide_screen_mode));
        }
    }

    pub fn loader_load<F>(&self, fut: F)
    where
        F: Future<Output = ()> + 'static,
    {
        self.loader.load(fut);
    }
    pub fn loader_is_loading(&self) -> impl Signal<Item = bool> + use<> {
        self.loader.is_loading()
    }
    pub fn loader_cancel(&self) {
        self.loader.cancel()
    }

    pub fn has_any_order_as(&self) -> impl Signal<Item = bool> + use<> {
        map_ref! {
            let ipd_as = self.ipd_order_as.signal_vec_cloned().len(),
            let opd_er_as = self.opd_er_order_as.signal_vec_cloned().len() =>
            *ipd_as > 0 || *opd_er_as > 0
        }
    }

    /// DELETE `EndPoint::Sse`
    pub async fn call_api_delete_sse_end(app: Rc<Self>) -> Result<String, AppError> {
        execute_fetch_text(&EndPoint::Sse.base(), "DELETE", None, app).await
    }

    pub fn sse_group(&self) -> SseGroup {
        SseGroup {
            wards: self.user.lock_ref().as_ref().map(|uc| uc.user.wards.get_cloned()).unwrap_or_default(),
            spclty_ids: self.user.lock_ref().as_ref().map(|uc| uc.user.spclty_ids.get_cloned()).unwrap_or_default(),
        }
    }
    pub fn set_sse_msg(&self, msg: SseMessage) {
        match msg {
            SseMessage::GlobalMsg(data) => self.msg_global.lock_mut().insert_cloned(0, data),
            SseMessage::WardMsg(data) => self.msg_ward.lock_mut().insert_cloned(0, data),
            SseMessage::SpcltyMsg(data) => self.msg_spclty.lock_mut().insert_cloned(0, data),
            SseMessage::DirectMsg(data) => self.msg_private.lock_mut().insert_cloned(0, data),
            SseMessage::Msg(_) | SseMessage::Logout(_) => {}
        }
    }
    pub fn count_unread_all_msg(&self) -> impl Signal<Item = usize> + use<> {
        map_ref! {
            let global = self.count_unread_global_msg(),
            let ward = self.count_unread_ward_msg(),
            let spclty = self.count_unread_spclty_msg(),
            let private = self.count_unread_private_msg() =>
            global + ward + spclty + private
        }
    }
    pub fn count_unread_global_msg(&self) -> impl Signal<Item = usize> + use<> {
        self.msg_global.signal_vec_cloned().filter(|msg| !msg.readed).len()
    }
    pub fn count_unread_ward_msg(&self) -> impl Signal<Item = usize> + use<> {
        self.msg_ward.signal_vec_cloned().filter(|msg| !msg.readed).len()
    }
    pub fn count_unread_spclty_msg(&self) -> impl Signal<Item = usize> + use<> {
        self.msg_spclty.signal_vec_cloned().filter(|msg| !msg.readed).len()
    }
    pub fn count_unread_private_msg(&self) -> impl Signal<Item = usize> + use<> {
        self.msg_private.signal_vec_cloned().filter(|msg| !msg.readed).len()
    }
    pub fn get_min_global_message_id(&self) -> Option<u32> {
        self.msg_global.lock_ref().iter().min_by(|a, b| a.message_id.cmp(&b.message_id)).map(|msg| msg.message_id)
    }
    pub fn get_min_ward_message_id(&self) -> Option<u32> {
        self.msg_ward.lock_ref().iter().min_by(|a, b| a.message_id.cmp(&b.message_id)).map(|msg| msg.message_id)
    }
    pub fn get_min_spclty_message_id(&self) -> Option<u32> {
        self.msg_spclty.lock_ref().iter().min_by(|a, b| a.message_id.cmp(&b.message_id)).map(|msg| msg.message_id)
    }
    pub fn get_min_private_message_id(&self) -> Option<u32> {
        self.msg_private.lock_ref().iter().min_by(|a, b| a.message_id.cmp(&b.message_id)).map(|msg| msg.message_id)
    }
    pub fn get_min_message_id(&self) -> Option<u32> {
        [
            self.get_min_global_message_id(),
            self.get_min_ward_message_id(),
            self.get_min_spclty_message_id(),
            self.get_min_private_message_id(),
        ]
        .into_iter()
        .flatten()
        .min()
    }
    pub fn load_messages(&self, messages: &[SseMessage]) {
        let mut globals = Vec::new();
        let mut wards = Vec::new();
        let mut spcltys = Vec::new();
        let mut privates = Vec::new();
        for message in messages.iter() {
            match message {
                SseMessage::GlobalMsg(data) => globals.push(data.clone()),
                SseMessage::WardMsg(data) => wards.push(data.clone()),
                SseMessage::SpcltyMsg(data) => spcltys.push(data.clone()),
                SseMessage::DirectMsg(data) => privates.push(data.clone()),
                SseMessage::Msg(_) | SseMessage::Logout(_) => {}
            }
        }
        if !globals.is_empty() {
            globals.extend(self.msg_global.lock_ref().iter().cloned());
            globals.sort();
            globals.dedup();
            globals.reverse();
            self.msg_global.lock_mut().replace_cloned(globals);
        }
        if !wards.is_empty() {
            wards.extend(self.msg_ward.lock_ref().iter().cloned());
            wards.sort();
            wards.dedup();
            wards.reverse();
            self.msg_ward.lock_mut().replace_cloned(wards);
        }
        if !spcltys.is_empty() {
            spcltys.extend(self.msg_spclty.lock_ref().iter().cloned());
            spcltys.sort();
            spcltys.dedup();
            spcltys.reverse();
            self.msg_spclty.lock_mut().replace_cloned(spcltys);
        }
        if !privates.is_empty() {
            privates.extend(self.msg_private.lock_ref().iter().cloned());
            privates.sort();
            privates.dedup();
            privates.reverse();
            self.msg_private.lock_mut().replace_cloned(privates);
        }
    }
    pub fn read_one_global_msg(&self, msg_id: u32) {
        let id_msg = self
            .msg_global
            .lock_ref()
            .iter()
            .enumerate()
            .find(|(_, msg)| msg.message_id == msg_id)
            .map(|(id, msg)| (id, msg.clone()));
        if let Some((idx, mut new)) = id_msg {
            new.readed = true;
            self.msg_global.lock_mut().set_cloned(idx, new);
        }
    }
    pub fn read_one_ward_msg(&self, msg_id: u32) {
        let id_msg = self.msg_ward.lock_ref().iter().enumerate().find(|(_, msg)| msg.message_id == msg_id).map(|(id, msg)| (id, msg.clone()));
        if let Some((idx, mut new)) = id_msg {
            new.readed = true;
            self.msg_ward.lock_mut().set_cloned(idx, new);
        }
    }
    pub fn read_one_spclty_msg(&self, msg_id: u32) {
        let id_msg = self
            .msg_spclty
            .lock_ref()
            .iter()
            .enumerate()
            .find(|(_, msg)| msg.message_id == msg_id)
            .map(|(id, msg)| (id, msg.clone()));
        if let Some((idx, mut new)) = id_msg {
            new.readed = true;
            self.msg_spclty.lock_mut().set_cloned(idx, new);
        }
    }
    pub fn read_one_private_msg(&self, msg_id: u32) {
        let id_msg = self
            .msg_private
            .lock_ref()
            .iter()
            .enumerate()
            .find(|(_, msg)| msg.message_id == msg_id)
            .map(|(id, msg)| (id, msg.clone()));
        if let Some((idx, mut new)) = id_msg {
            new.readed = true;
            self.msg_private.lock_mut().set_cloned(idx, new);
        }
    }
    pub fn read_all_global_msg(&self) {
        let mut buffer = self.msg_global.lock_ref().to_vec();
        buffer.iter_mut().for_each(|msg| {
            if !msg.readed {
                msg.readed = true
            }
        });
        self.msg_global.lock_mut().replace_cloned(buffer);
    }
    pub fn read_all_ward_msg(&self) {
        let mut buffer = self.msg_ward.lock_ref().to_vec();
        buffer.iter_mut().for_each(|msg| {
            if !msg.readed {
                msg.readed = true
            }
        });
        self.msg_ward.lock_mut().replace_cloned(buffer);
    }
    pub fn read_all_spclty_msg(&self) {
        let mut buffer = self.msg_spclty.lock_ref().to_vec();
        buffer.iter_mut().for_each(|msg| {
            if !msg.readed {
                msg.readed = true
            }
        });
        self.msg_spclty.lock_mut().replace_cloned(buffer);
    }
    pub fn read_all_private_msg(&self) {
        let mut buffer = self.msg_private.lock_ref().to_vec();
        buffer.iter_mut().for_each(|msg| {
            if !msg.readed {
                msg.readed = true
            }
        });
        self.msg_private.lock_mut().replace_cloned(buffer);
    }
    pub fn all_msg_ids(&self) -> Vec<u32> {
        let mut result = Vec::new();
        result.extend(self.msg_global.lock_ref().iter().filter_map(|msg| (!msg.readed).then(|| msg.message_id)));
        result.extend(self.msg_ward.lock_ref().iter().filter_map(|msg| (!msg.readed).then(|| msg.message_id)));
        result.extend(self.msg_spclty.lock_ref().iter().filter_map(|msg| (!msg.readed).then(|| msg.message_id)));
        result.extend(self.msg_private.lock_ref().iter().filter_map(|msg| (!msg.readed).then(|| msg.message_id)));
        result
    }

    pub fn show_jsvalue_message(&self, js_value: &JsValue) {
        self.window.with(|w| show_jsvalue_error_message_with_document(&w.document().unwrap(), js_value))
    }

    pub fn state_id(&self) -> Option<String> {
        self.user.lock_ref().as_ref().map(|user| user.sub.get_cloned())
    }
    pub fn user_name(&self) -> Option<String> {
        self.user.lock_ref().as_ref().map(|user| user.user.name.get_cloned())
    }
    pub fn authorized(&self) -> Option<bool> {
        self.user.lock_ref().as_ref().map(|user| user.authorized())
    }
    pub fn token(&self) -> Option<String> {
        self.user.lock_ref().as_ref().map(|user| user.token.get_cloned())
    }
    pub fn token_sub(&self) -> Option<String> {
        self.user.lock_ref().as_ref().map(|user| user.sub.get_cloned())
    }
    pub fn handshake_2fa_timeout_second(&self) -> u64 {
        self.app_status.lock_ref().as_ref().map(|status| status.handshake_2fa_timeout_second).unwrap_or(60)
    }
    pub fn remove_user_and_go_index(&self) {
        self.user.set(None);
        self.to_local_storage();
        Route::Index.hard_redirect();
    }

    pub fn is_production(&self) -> bool {
        self.app_status.lock_ref().as_ref().map(|status| status.is_production).unwrap_or(false)
    }
    pub fn is_pre_admit(&self, an: &str) -> bool {
        an.len() > self.hosxp_an_len()
    }
    pub fn is_checked_pharmacist_can_done(&self) -> bool {
        self.app_status.lock_ref().as_ref().map(|status| status.is_checked_pharmacist_can_done).unwrap_or(false)
    }
    pub fn can_sign_pdf(&self) -> bool {
        self.app_status.lock_ref().as_ref().map(|status| status.can_sign_pdf).unwrap_or(false)
    }
    pub fn is_read_only_mode(&self) -> bool {
        self.app_status.lock_ref().as_ref().map(|status| status.is_read_only_mode).unwrap_or(false)
    }
    pub fn has_covid_lab(&self) -> bool {
        self.app_status.lock_ref().as_ref().map(|status| status.has_covid_lab).unwrap_or(false)
    }
    pub fn concat_with_space(&self) -> bool {
        self.app_status.lock_ref().as_ref().map(|status| status.concat_with_space).unwrap_or(false)
    }
    pub fn report_coercions(&self) -> Option<Arc<HashMap<String, String>>> {
        self.app_status.lock_ref().as_ref().and_then(|status| status.report_coercions.clone())
    }
    pub fn hosxp_medrec_icode(&self) -> Option<String> {
        self.app_status.lock_ref().as_ref().map(|status| status.hosxp_med_reconcilation_icode.clone())
    }
    pub fn hosxp_hn_len(&self) -> usize {
        self.app_status.lock_ref().as_ref().map(|status| status.hosxp_hn_length).unwrap_or(7)
    }
    pub fn hosxp_an_len(&self) -> usize {
        self.app_status.lock_ref().as_ref().map(|status| status.hosxp_an_length).unwrap_or(9)
    }
    pub fn hosxp_vn_len(&self) -> usize {
        self.app_status.lock_ref().as_ref().map(|status| status.hosxp_vn_length).unwrap_or(12)
    }
    pub fn hosxp_hn_len_signal(&self) -> impl Signal<Item = usize> + use<> {
        self.app_status.signal_ref(|opt_status| opt_status.as_ref().map(|status| status.hosxp_hn_length).unwrap_or(7))
    }
    pub fn hosxp_an_len_signal(&self) -> impl Signal<Item = usize> + use<> {
        self.app_status.signal_ref(|opt_status| opt_status.as_ref().map(|status| status.hosxp_an_length).unwrap_or(9))
    }
    pub fn hosxp_vn_len_signal(&self) -> impl Signal<Item = usize> + use<> {
        self.app_status.signal_ref(|opt_status| opt_status.as_ref().map(|status| status.hosxp_vn_length).unwrap_or(12))
    }
    pub fn hospital_name_signal(&self) -> impl Signal<Item = String> + use<> {
        self.app_status
            .signal_ref(|opt_status| opt_status.as_ref().map(|status| status.hospital_name.clone()).unwrap_or_default())
    }
    pub fn hospital_short_name_signal(&self) -> impl Signal<Item = String> + use<> {
        self.app_status
            .signal_ref(|opt_status| opt_status.as_ref().map(|status| status.hospital_short_name.clone()).unwrap_or_default())
    }
    pub fn nurse_assign_groups(&self) -> Vec<String> {
        self.app_status.lock_ref().as_ref().map(|status| status.nurse_assign_groups.clone()).unwrap_or_default()
    }
    pub fn lab_alerts(&self) -> Vec<String> {
        self.app_status.lock_ref().as_ref().map(|status| status.lab_alerts.clone()).unwrap_or_default()
    }
    pub fn has_pacs_host(&self) -> bool {
        self.app_status.lock_ref().as_ref().map(|status| status.has_pacs_host).unwrap_or_default()
    }
    pub fn pacs_hn_url(&self, hn: &str) -> Option<String> {
        self.app_status
            .lock_ref()
            .as_ref()
            .and_then(|status| status.pacs_hn_url.clone().map(|hn_url| hn_url.replace("[HN]", hn)))
    }
    pub fn ekg_hn_url(&self, hn: &str) -> Option<String> {
        self.app_status
            .lock_ref()
            .as_ref()
            .and_then(|status| status.ekg_hn_url.clone().map(|hn_url| hn_url.replace("[HN]", hn)))
    }
    pub fn scan_hn_url(&self, hn: &str) -> Option<String> {
        self.app_status
            .lock_ref()
            .as_ref()
            .and_then(|status| status.scan_hn_url.clone().map(|hn_url| hn_url.replace("[HN]", hn)))
    }
    pub fn scan_an_url(&self, an: &str) -> Option<String> {
        self.app_status
            .lock_ref()
            .as_ref()
            .and_then(|status| status.scan_an_url.clone().map(|vn_url| vn_url.replace("[AN]", an)))
    }
    pub fn cart_vnan_url(&self, vnan: &str) -> Option<String> {
        self.app_status
            .lock_ref()
            .as_ref()
            .and_then(|status| status.cart_vnan_url.clone().map(|vnan_url| vnan_url.replace("[VNAN]", vnan)))
    }
    pub fn food_url(&self) -> Option<String> {
        self.app_status.lock_ref().as_ref().and_then(|status| status.food_url.clone())
    }

    pub fn doctor_code(&self) -> Option<String> {
        self.user.lock_ref().as_ref().map(|user| user.user.doctorcode.get_cloned())
    }
    pub fn doctor_name(&self) -> Option<String> {
        self.user.lock_ref().as_ref().map(|user| user.user.name.get_cloned())
    }
    pub fn doctor_licenseno(&self) -> Option<String> {
        self.user.lock_ref().as_ref().map(|user| user.user.licenseno.get_cloned())
    }
    pub fn doctor_entryposition(&self) -> Option<String> {
        self.user.lock_ref().as_ref().map(|user| user.user.entryposition.get_cloned())
    }
    pub fn guess_view_by(&self) -> &'static str {
        if self.has_permission(Permission::DataTypeDoctorUse) {
            "doctor"
        } else if self.has_permission(Permission::DataTypeNurseUse) {
            "nurse"
        } else if self.has_permission(Permission::DataTypePharmacyUse) {
            "pharmacist"
        } else {
            "other"
        }
    }
    pub fn has_permission(&self, permission: Permission) -> bool {
        if self.is_production() {
            let read_only_perm = if self.is_read_only_mode() { permission.read_only() } else { true };
            read_only_perm && self.user.lock_ref().as_ref().map(|user| user.permissions.lock_ref().contains(&permission)).unwrap_or_default()
        } else {
            true
        }
    }
    /// is_pre_admit == use VN as AN and check OPD-ER permission instead (if not exists will bypass checking)
    pub fn endpoint_is_allow(&self, method: &Method, endpoint: &EndPoint, is_pre_admit: bool) -> bool {
        if self.is_production() {
            if let Some(permissions) = self.user.lock_ref().as_ref().map(|user| user.permissions.lock_ref()) {
                endpoint.is_allow(method, &permissions, is_pre_admit)
            } else {
                false
            }
        } else {
            true
        }
    }
    pub fn can_change_ward_passcode(&self) -> bool {
        self.user.lock_ref().as_ref().map(|user| user.user.can_passcode).unwrap_or_default()
    }

    pub fn scores_tuple(&self) -> Option<(Vec<SupportedScore>, Vec<SupportedScore>, Vec<SupportedScore>)> {
        self.app_status
            .lock_ref()
            .as_ref()
            .map(|app_status| (app_status.score_ews.clone(), app_status.score_qsofa.clone(), app_status.score_sirs.clone()))
    }
    pub fn scores_table_header(&self) -> String {
        self.scores_tuple()
            .map(|(ews, qsofa, sirs)| {
                let parse_fn = |v: &[SupportedScore]| v.iter().map(|s| s.label()).collect::<Vec<&'static str>>().join("/");
                [parse_fn(&ews), parse_fn(&qsofa), parse_fn(&sirs)].join(", ")
            })
            .unwrap_or(String::from("MEWS, qSOFA, SIRS"))
    }

    /// return ('date of shift', shift)
    /// ex: if night shift start after 22:00 then at 24-01-2024 23:45 is night-shift of 25-01-2024
    /// so cal_shift(24-01-2024,23:45) -> (25-01-2024, Night)
    pub fn cal_shift(&self, date: Date, time: Time) -> Option<(Date, NurseShift)> {
        self.app_status
            .lock_ref()
            .as_ref()
            .map(|app_status| NurseShift::generate(app_status.shift_day_start, app_status.shift_evening_start, app_status.shift_night_start, date, time))
    }
    // generate Shift from NurseShift
    pub fn shift(&self, nurse_shift: NurseShift) -> Option<Shift> {
        self.app_status.lock_ref().as_ref().map(|app_status| {
            let shift_day_start = app_status.shift_day_start;
            let shift_evening_start = app_status.shift_evening_start;
            let shift_night_start = app_status.shift_night_start;
            let noon = Time::from_hms(12, 0, 0).unwrap();
            let day = Duration::days(1);

            let (duration, detail) = match nurse_shift {
                NurseShift::Night => {
                    let duration = if shift_night_start == Time::MIDNIGHT {
                        duration_hm(shift_day_start - Time::MIDNIGHT)
                    } else if shift_night_start > noon {
                        duration_hm(day - (shift_night_start - Time::MIDNIGHT) + (shift_day_start - Time::MIDNIGHT))
                    } else {
                        duration_hm(shift_day_start - shift_night_start)
                    };
                    let detail = [time_hm(&shift_night_start), time_hm(&shift_day_start)].join(" - ");
                    (duration, detail)
                }
                NurseShift::Day => {
                    let duration = duration_hm(shift_evening_start - shift_day_start);
                    let detail = [time_hm(&shift_day_start), time_hm(&shift_evening_start)].join(" - ");
                    (duration, detail)
                }
                NurseShift::Evening => {
                    let duration = if shift_night_start == Time::MIDNIGHT {
                        duration_hm(day - (shift_evening_start - Time::MIDNIGHT))
                    } else if shift_night_start > noon {
                        duration_hm(shift_night_start - shift_evening_start)
                    } else {
                        duration_hm(day - (shift_evening_start - Time::MIDNIGHT) + (shift_night_start - Time::MIDNIGHT))
                    };
                    let detail = [time_hm(&shift_evening_start), time_hm(&shift_night_start)].join(" - ");
                    (duration, detail)
                }
            };

            Shift { shift: nurse_shift, duration, detail }
        })
    }

    pub fn nurse_shift(&self, select_time_opt: &Option<Time>) -> Option<NurseShift> {
        self.app_status.lock_ref().as_ref().and_then(|app_status| {
            select_time_opt.map(|select_time| {
                if app_status.shift_night_start > app_status.shift_evening_start {
                    // night shift start before midnight
                    if select_time >= app_status.shift_night_start {
                        NurseShift::Night
                    } else if select_time >= app_status.shift_evening_start {
                        NurseShift::Evening
                    } else if select_time >= app_status.shift_day_start {
                        NurseShift::Day
                    } else {
                        NurseShift::Night
                    }
                } else {
                    // night shift start after midnight
                    if select_time >= app_status.shift_evening_start {
                        NurseShift::Evening
                    } else if select_time >= app_status.shift_day_start {
                        NurseShift::Day
                    } else if select_time >= app_status.shift_night_start {
                        NurseShift::Night
                    } else {
                        NurseShift::Evening
                    }
                }
            })
        })
    }

    pub fn set_title(&self, title: &str) {
        self.window.with(|w| w.document().unwrap().set_title(title))
    }

    /// return true if cannot go back
    pub fn go_back_else(&self) -> bool {
        self.window.with(|w| w.history().ok().and_then(|h| h.back().ok()).is_none())
    }

    pub fn open_response_blob(&self, blob: Blob, file_name: &str) {
        match Url::create_object_url_with_blob(&blob) {
            Ok(url) => {
                // // open with url as filename
                //self.window.with(|w| w.open_with_url(&url).unwrap_throw());
                self.open_with_filename(url, file_name);
            }
            Err(e) => self.show_jsvalue_message(&e),
        }
    }

    pub fn open_file_with_mime(&self, bytes: &[u8], file_name: &str, mime_type: &str) {
        let url = create_url_with_mime(bytes, mime_type);

        // // open with url as filename
        // self.window.with(|w| w.open_with_url(&url).expect_throw("cannot open url"));
        self.open_with_filename(url, file_name);
    }

    fn open_with_filename(&self, url: String, file_name: &str) {
        self.window.with(|w| {
            let document = w.document().unwrap();
            let body = document.body().unwrap();
            let a = document.create_element("a").unwrap().dyn_into::<HtmlElement>().unwrap();
            a.set_attribute("href", &url).unwrap();
            a.set_attribute("download", file_name).unwrap();
            body.append_child(&a).unwrap();
            a.click();

            let update = Timeout::new(0, move || {
                if let Err(e) = Url::revoke_object_url(&url) {
                    show_jsvalue_error_message_with_document(&document, &e);
                }
                if let Err(e) = body.remove_child(&a) {
                    show_jsvalue_error_message_with_document(&document, &e);
                }
            });
            update.forget();
        });
    }

    /// GET `EndPoint::ReportTemplateTypeId`
    pub async fn call_api_get_pdf_report(template_name: &str, report_type: &str, ids: &str, app: Rc<Self>) -> Result<Blob, AppError> {
        let path = [&EndPoint::ReportTemplateTypeId.base(), template_name, "/", report_type, "/", ids].concat();
        get_blob_from_url(&path, "application/pdf", app).await
    }

    pub fn get_selection(&self) -> Option<Selection> {
        // we use ok() instead of unwrap()
        self.window.with(|w| w.get_selection().ok().flatten())
    }

    pub fn create_range(&self) -> Range {
        self.window.with(|w| w.document().unwrap().create_range().unwrap())
    }

    pub fn create_text_node(&self, data: &str) -> Text {
        self.window.with(|w| w.document().unwrap().create_text_node(data))
    }

    pub fn onscroll<F>(&self, f: F)
    where
        F: FnMut() + 'static,
    {
        let f = Closure::wrap(Box::new(f) as Box<dyn FnMut()>);

        self.window.with(|w| w.set_onscroll(Some(f.as_ref().unchecked_ref())));

        f.forget();
    }

    pub fn window_scroll_y(&self) -> f64 {
        self.window.with(|w| w.scroll_y().unwrap())
    }

    pub fn local_storage_get(&self, key: &str) -> Option<String> {
        // we use ok() instead of unwrap()
        self.storage.with(|x| x.get_item(key).ok().flatten())
    }

    pub fn local_storage_set(&self, key: &str, value: &str) {
        self.storage.with(|x| {
            if let Err(e) = x.set_item(key, value) {
                self.show_jsvalue_message(&e);
            }
        })
    }

    pub fn set_interval<F>(&self, f: F, ms: i32) -> i32
    where
        F: FnMut() + 'static,
    {
        let f = Closure::wrap(Box::new(f) as Box<dyn FnMut()>);

        let handle = self.window.with(|w| w.set_interval_with_callback_and_timeout_and_arguments_0(f.as_ref().unchecked_ref(), ms).unwrap());

        f.forget();

        handle
    }

    pub fn clear_interval(&self, handle_id: i32) {
        self.window.with(|w| w.clear_interval_with_handle(handle_id))
    }

    pub fn show_jsvalue_error_message(&self, js_value: &JsValue) {
        if let Some(message) = js_value.dyn_ref::<JsString>().map(|s| Into::<String>::into(s)) {
            self.show_error_message(&message);
        }
    }

    pub fn show_error_message(&self, message: &str) {
        if let Some(elm) = self.get_id("errormessage") {
            elm.set_text_content(Some(message));
        }
        log::error!("{}", message);
    }

    // pub fn get_icd10(&self, icd10: &str) -> Option<Icd10Keywords> {
    //     self.app_asset.lock_ref().as_ref().and_then(|asset| Icd10Keywords::search_icd10_exact(icd10, &asset.icd10_keywords))
    // }
}

#[derive(Clone)]
pub enum DragStartState {
    Text(String),
    /// (ty, muable)
    ///
    /// ty in ["com", "sum", "rev"]
    Input((&'static str, Mutable<String>)),
}

impl DragStartState {
    pub fn new_text(text: &str) -> Self {
        Self::Text(text.to_owned())
    }

    pub fn new_input(ty: &'static str, mutable: Mutable<String>) -> Self {
        Self::Input((ty, mutable))
    }

    pub fn get_type(&self) -> &'static str {
        match self {
            Self::Text(_) => "text",
            Self::Input((ty, _)) => ty,
        }
    }

    pub fn get_value(&self) -> String {
        match self {
            Self::Text(text) => text.to_owned(),
            Self::Input((_, mutable)) => mutable.get_cloned(),
        }
    }

    pub fn set_input(&self, value: &str) {
        match self {
            Self::Text(_) => {}
            Self::Input((_, mutable)) => mutable.set_neq(value.to_owned()),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct AppLocalStorage {
    app_status: Option<AppStatus>,

    ward_select: String,
    ward_multiple_select: String,
    inverse_ward_select: String,
    summary_status_select: String,
    inscl_select: String,
    adm_doctor_select: String,
    dch_doctor_select: String,
    spclty_select: String,

    monitor_refresh_interval: String,
    ipd_pharmacy_order_monitor_is_discharged: String,
    opd_er_pharmacy_order_monitor_is_discharged: String,
    order_monitor_new_order_sound_on: String,
    uploaded_images: Vec<ImagePath>,
}

impl From<&AppState> for AppLocalStorage {
    fn from(item: &AppState) -> Self {
        AppLocalStorage {
            app_status: item.app_status.lock_ref().as_ref().map(|inner| inner.as_ref().to_owned()),

            ward_select: item.ward_select.get_cloned(),
            ward_multiple_select: item.ward_multiple_select.get_cloned(),
            inverse_ward_select: item.inverse_ward_select.get_cloned(),
            summary_status_select: item.summary_status_select.get_cloned(),
            inscl_select: item.inscl_select.get_cloned(),
            adm_doctor_select: item.adm_doctor_select.get_cloned(),
            dch_doctor_select: item.dch_doctor_select.get_cloned(),
            spclty_select: item.spclty_select.get_cloned(),

            monitor_refresh_interval: item.monitor_refresh_interval.get_cloned(),
            ipd_pharmacy_order_monitor_is_discharged: item.ipd_pharmacy_order_monitor_is_discharged.get_cloned(),
            opd_er_pharmacy_order_monitor_is_discharged: item.opd_er_pharmacy_order_monitor_is_discharged.get_cloned(),
            order_monitor_new_order_sound_on: item.order_monitor_new_order_sound_on.get_cloned(),
            uploaded_images: item.uploaded_images.lock_ref().iter().map(|im| im.as_ref().to_owned()).collect(),
        }
    }
}

/// API configuration
#[derive(Clone, Debug, Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(AppStatus::demo()))]
pub struct AppStatus {
    #[Demo(value = "true")]
    pub is_production: bool,
    #[Demo(value = "true")]
    pub is_read_only_mode: bool,
    #[Demo(value = "true")]
    pub is_checked_pharmacist_can_done: bool,
    #[Demo(value = "true")]
    pub has_covid_lab: bool,
    #[Demo(value = "true")]
    pub allow_insert_his: bool,
    #[Demo(value = "true")]
    pub can_sign_pdf: bool,
    #[Demo(value = "60")]
    pub handshake_2fa_timeout_second: u64,

    #[Demo(value = "7")]
    pub hosxp_hn_length: usize,
    #[Demo(value = "12")]
    pub hosxp_vn_length: usize,
    #[Demo(value = "9")]
    pub hosxp_an_length: usize,

    #[Demo(value = r#"String::from("11111")"#)]
    pub hospcode: String,
    #[Demo(value = r#"String::from("XXH")"#)]
    pub code_name: String,
    #[Demo(value = r#"String::from("Best Hospital")"#)]
    pub hospital_name: String,
    #[Demo(value = r#"String::from("Best")"#)]
    pub hospital_short_name: String,
    #[Demo(value = r#"String::from("Y")"#)]
    pub drug_notify_use: String,
    #[Demo(value = r#"String::from("Y")"#)]
    pub drug_notify_start_end_marker_use: String,
    #[Demo(value = r#"String::from("***")"#)]
    pub drug_notify_start_marker: String,
    #[Demo(value = r#"String::from("***")"#)]
    pub drug_notify_end_marker: String,

    #[Demo(value = r#"vec![String::from("Incharge"), String::from("Leader")]"#)]
    pub nurse_assign_groups: Vec<String>,

    #[Demo(value = "time!(8 AM)")]
    pub shift_day_start: Time,
    #[Demo(value = "time!(4 PM)")]
    pub shift_evening_start: Time,
    #[Demo(value = "Time::MIDNIGHT")]
    pub shift_night_start: Time,

    #[Demo(value = "vec![SupportedScore::demo_news(), SupportedScore::demo_pops(), SupportedScore::demo_mews()]")]
    pub score_ews: Vec<SupportedScore>,
    #[Demo(value = "vec![SupportedScore::demo_lq_sofa(), SupportedScore::demo_qsofa()]")]
    pub score_qsofa: Vec<SupportedScore>,
    #[Demo(value = "vec![SupportedScore::demo_psirs(), SupportedScore::demo_sirs()]")]
    pub score_sirs: Vec<SupportedScore>,

    #[Demo(value = "true")]
    pub concat_with_space: bool,
    #[Demo(value = "None")]
    pub report_coercions: Option<Arc<HashMap<String, String>>>,

    #[Demo(value = r#"String::from("IVFLUIDS")"#)]
    pub hosxp_ivfluid_dosageform: String,
    #[Demo(value = r#"vec![String::from("INJECTIONS"),String::from("NEBULIZERS")]"#)]
    pub hosxp_injection_dosageforms: Vec<String>,
    #[Demo(value = "Some(255)")]
    pub hosxp_had_displaycolor: Option<i32>,
    #[Demo(value = "Some(16711935)")]
    pub hosxp_lasa_displaycolor: Option<i32>,

    #[Demo(value = r#"String::from("1111111")"#)]
    pub hosxp_med_reconcilation_icode: String,
    #[Demo(value = r#"vec![String::from("HCT < 30%"), String::from("INR > 1.2")]"#)]
    pub lab_alerts: Vec<String>,
    #[Demo(value = "true")]
    pub has_pacs_host: bool,
    #[Demo(value = r#"Some(String::from("http://pacs/search?hn=[HN]"))"#)]
    pub pacs_hn_url: Option<String>,
    #[Demo(value = r#"Some(String::from("http://ekg/search?hn=[HN]"))"#)]
    pub ekg_hn_url: Option<String>,
    #[Demo(value = r#"Some(String::from("http://scan/search?hn=[HN]"))"#)]
    pub scan_hn_url: Option<String>,
    #[Demo(value = r#"Some(String::from("http://pacs/search?an=[AN]"))"#)]
    pub scan_an_url: Option<String>,
    #[Demo(value = r#"Some(String::from("http://cart/search?vnan=[VNAN]"))"#)]
    pub cart_vnan_url: Option<String>,
    #[Demo(value = r#"Some(String::from("http://food"))"#)]
    pub food_url: Option<String>,
}

// please update `kphis_model::report::SystemListType` enum too
/// HTML select element options and ICD10 search items
#[derive(Clone, Debug, Demo, Decode, Encode, PartialEq, Serialize, ToSchema)]
#[schema(example = json!(AppAsset::demo()))]
pub struct AppAsset {
    #[Demo(value = r#"vec![ColorSelectOption::demo(String::from("1"), String::from("1"))]"#)]
    pub fcnote_patient_type_select_options: Vec<ColorSelectOption>,
    #[Demo(value = r#"vec![ColorSelectOption::demo(String::from("1"), String::from("แดง 1"))]"#)]
    pub er_bed_select_options: Vec<ColorSelectOption>,
    #[Demo(value = r#"vec![SelectOption::demo(String::from("1"), String::from("รอตรวจ"))]"#)]
    pub er_patient_status_select_options: Vec<SelectOption>,
    #[Demo(value = r#"vec![SelectOption::demo(String::from("1"), String::from("กลับบ้าน"))]"#)]
    pub er_dch_type_select_options: Vec<SelectOption>,
    #[Demo(value = r#"vec![SelectOption::demo(String::from("01"), String::from("ตึกชาย"))]"#)]
    pub ward_select_option: Vec<SelectOption>,
    #[Demo(value = r#"vec![SelectOption::demo(String::from("001"), String::from("Dr.Doctor"))]"#)]
    pub doctor_select_option: Vec<SelectOption>,
    #[Demo(value = r#"vec![SelectOption::demo(String::from("001"), String::from("Dr.Doctor"))]"#)]
    pub all_doctor_select_option: Vec<SelectOption>,
    #[Demo(value = r#"vec![SelectOption::demo(String::from("01"), String::from("อายุรกรรม"))]"#)]
    pub spclty_select_option: Vec<SelectOption>,
    #[Demo(value = r#"vec![SelectOption::demo(String::from("1"), String::from("อายุรกรรมชาย"))]"#)]
    pub spclty_kphis_select_option: Vec<SelectOption>,
    #[Demo(value = r#"vec![SelectOption::demo(String::from("UCS"), String::from("สิทธิ UC"))]"#)]
    pub inscl_select_option: Vec<SelectOption>,
    #[Demo(value = r#"vec![SelectOption::demo(String::from("1"), String::from("ด่วน"))]"#)]
    pub emergency_select_option: Vec<SelectOption>,
    #[Demo(value = r#"vec![SelectOption::demo(String::from("1"), String::from("Resuscitate"))]"#)]
    pub emergency_level_select_option: Vec<SelectOption>,
    #[Demo(value = r#"vec![SelectOption::demo(String::from("1"), String::from("ใบ Consult ทั่วไป"))]"#)]
    pub consult_type_select_option: Vec<SelectOption>,
    #[Demo(value = r#"vec![SelectOption::demo(String::from("1"), String::from("รู้สึกตัวดี"))]"#)]
    pub conscious_select_option: Vec<SelectOption>,
    #[Demo(value = r#"vec![SelectOption::demo(String::from("1"), String::from("501-1000"))]"#)]
    pub urine_amount_select_option: Vec<SelectOption>,
    #[Demo(value = r#"vec![SelectOption::demo(String::from("1"), String::from("ปัสสาวะ/วัน"))]"#)]
    pub urine_duration_select_option: Vec<SelectOption>,
    #[Demo(value = r#"vec![SelectOption::demo(String::from("1"), String::from("C-Line"))]"#)]
    pub line_select_option: Vec<SelectOption>,
    #[Demo(value = r#"vec![SelectOption::demo(String::from("1"), String::from("React to light"))]"#)]
    pub cha_select_option: Vec<SelectOption>,
    #[Demo(value = r#"vec![SelectOption::demo(String::from("1"), String::from("A"))]"#)]
    pub va_select_option: Vec<SelectOption>,
    #[Demo(value = r#"vec![SelectOption::demo(String::from("1"), String::from("1"))]"#)]
    pub mass_select_option: Vec<SelectOption>,
    #[Demo(value = r#"vec![SelectOption::demo(String::from("1"), String::from("I"))]"#)]
    pub motor_select_option: Vec<SelectOption>,
    #[Demo(value = r#"vec![SelectOption::demo(String::from("1"), String::from("Canular"))]"#)]
    pub o2_select_option: Vec<SelectOption>,
    #[Demo(value = r#"vec![SelectOption::demo(String::from("1"), String::from("Et-tube"))]"#)]
    pub tube_select_option: Vec<SelectOption>,
    // #[Demo(value = r#"vec![SelectOption::demo(String::from("1"), String::from("Parenteral"))]"#)]
    // pub intake_select_option: Vec<SelectOption>,
    // #[Demo(value = r#"vec![SelectOption::demo(String::from("1"), String::from("Drain"))]"#)]
    // pub output_select_option: Vec<SelectOption>,
    #[Demo(value = r#"vec![SelectOption::demo(String::from("1"), String::from("-2"))]"#)]
    pub lr_sta_select_option: Vec<SelectOption>,
    #[Demo(value = r#"vec![SelectOption::demo(String::from("1"), String::from("R"))]"#)]
    pub lr_mem_select_option: Vec<SelectOption>,
    #[Demo(value = r#"vec![SelectOption::demo(String::from("1"), String::from("0"))]"#)]
    pub lr_moulding_select_option: Vec<SelectOption>,
    #[Demo(value = r#"vec![SelectOption::demo(String::from("1"), String::from("-ve"))]"#)]
    pub dipstick_select_option: Vec<SelectOption>,
    #[Demo(value = r#"vec![SelectOption::demo(String::from("1"), String::from("No distress"))]"#)]
    pub breathing_select_option: Vec<SelectOption>,
    #[Demo(value = r#"vec![SelectOption::demo(String::from("1"), String::from("Alert"))]"#)]
    pub avpu_select_option: Vec<SelectOption>,
    #[Demo(value = r#"vec![SelectOption::demo(String::from("1"), String::from("Well"))]"#)]
    pub gut_feeling_select_option: Vec<SelectOption>,
    #[Demo(value = r#"vec![SelectOption::demo(String::from("1"), String::from("NA"))]"#)]
    pub pops_other_select_option: Vec<SelectOption>,
    #[Demo(value = r#"vec![SelectOption::demo(String::from("1"), String::from("Pre-contemplation"))]"#)]
    pub stage_of_change_select_option: Vec<SelectOption>,
    #[Demo(value = r#"vec![SelectOption::demo(String::from("1"), String::from("รับส่งภายในจังหวัด"))]"#)]
    pub refer_type_select_option: Vec<SelectOption>,
    #[Demo(value = r#"vec![SelectOption::demo(String::from("1"), String::from("รับไว้รักษาต่อ"))]"#)]
    pub refer_cause_select_option: Vec<SelectOption>,
    #[Demo(value = r#"vec![SelectOption::demo(String::from("ER"), String::from("ER"))]"#)]
    pub refer_point_select_option: Vec<SelectOption>,
    #[Demo(value = r#"vec![SelectOption::demo(String::from("1"), String::from("ระบุวันหมดอายุของใบส่งตัว"))]"#)]
    pub moph_refer_expire_type_select_option: Vec<SelectOption>,
    // #[Demo(value = "vec![Icd10Keywords::demo()]")]
    // pub icd10_keywords: Vec<Icd10Keywords>,
    // #[Demo(value = "vec![Icd10Keywords::demo()]")]
    // pub icd10_keywords_ext: Vec<Icd10Keywords>,
    #[Demo(value = r#"vec![SelectOption::demo(String::from("1"), String::from("ใบยินยอม"))]"#)]
    pub document_type_select_option: Vec<SelectOption>,
    #[Demo(value = "vec![DrugUsage::demo()]")]
    pub drugusages: Vec<DrugUsage>,
}

impl AppAsset {
    /// GET /assets get AppState Assets
    pub async fn get_asset(app: Rc<AppState>) -> Result<Self, AppError> {
        match fetch_blob_api(ASSETS_PREFIX, "GET", app).await {
            Ok((response, true)) => {
                let blob = response.unchecked_into::<Blob>();
                let bytes = blob_to_bytes(&blob)
                    .await
                    .map_err(|e| Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch AppAsset"))?;
                bitcode::decode(&bytes).map_err(|e| Source::BitCode.to_teapot_error(e, "Fetch AppAsset"))
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch AppAsset"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Blob")),
        }
    }

    /// PATCH /assets set app_asset_cache_exp == now
    pub async fn patch_asset(app: Rc<AppState>) -> Result<bool, AppError> {
        match fetch_json_api(ASSETS_PREFIX, "PATCH", None, app).await {
            Ok((response, true)) => {
                let response = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Patch AppAsset"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Patch AppAsset"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}

fn create_url_with_mime(bytes: &[u8], mime_type: &str) -> String {
    let uint8 = js_sys::Uint8Array::new_with_length(bytes.len() as u32);
    uint8.copy_from(bytes);
    // Uint8Array => Array => Blob
    let array = js_sys::Array::new();
    array.push(&uint8.buffer());
    let bag = BlobPropertyBag::new();
    bag.set_type(mime_type);
    let blob = Blob::new_with_u8_array_sequence_and_options(&array, &bag).unwrap();

    Url::create_object_url_with_blob(&blob).unwrap()
}

/// Visit Type with ID
#[derive(Clone, Debug, Demo, Deserialize, Eq, Serialize, ToSchema)]
#[schema(example = json!(VisitTypeId::demo_ipd(String::from("660001234"))))]
pub enum VisitTypeId {
    /// true IPD (AN)
    Ipd(String),
    /// Pre-Admit(VN as AN)
    PreAdmit(String),
    /// OPD-ER (VN, opd_er_order_master_id)
    OpdEr(String, u32),
    /// HOSxP Visited (VN)
    Visit(String),
}

impl Default for VisitTypeId {
    fn default() -> Self {
        VisitTypeId::Visit(String::new())
    }
}

impl Ord for VisitTypeId {
    fn cmp(&self, other: &Self) -> Ordering {
        self.vnan().cmp(other.vnan())
    }
}

impl PartialOrd for VisitTypeId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.vnan().cmp(other.vnan()))
    }
}

impl PartialEq for VisitTypeId {
    fn eq(&self, other: &Self) -> bool {
        self.vnan().eq(other.vnan())
    }
}

impl VisitTypeId {
    pub fn is_ipd(&self) -> bool {
        matches!(self, Self::Ipd(_) | Self::PreAdmit(_))
    }
    pub fn is_admited(&self) -> bool {
        matches!(self, Self::Ipd(_))
    }
    pub fn is_pre_admit(&self) -> bool {
        matches!(self, Self::PreAdmit(_))
    }
    pub fn is_ipd_and_is_pre_admit(&self) -> (bool, bool) {
        match self {
            Self::Ipd(_) => (true, false),
            Self::PreAdmit(_) => (true, true),
            Self::OpdEr(_, _) | Self::Visit(_) => (false, false),
        }
    }
    pub fn an_and_is_pre_admit(&self) -> Option<(&String, bool)> {
        match self {
            Self::Ipd(an) => Some((an, false)),
            Self::PreAdmit(vn) => Some((vn, true)),
            Self::OpdEr(_, _) | Self::Visit(_) => None,
        }
    }
    pub fn an_and_is_pre_admit_owned(&self) -> Option<(String, bool)> {
        match self {
            Self::Ipd(an) => Some((an.to_owned(), false)),
            Self::PreAdmit(vn) => Some((vn.to_owned(), true)),
            Self::OpdEr(_, _) | Self::Visit(_) => None,
        }
    }
    /// is any value insided empty or zero
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Ipd(an) => an.is_empty(),
            Self::PreAdmit(vn) => vn.is_empty(),
            Self::OpdEr(vn, opd_er_order_master_id) => vn.is_empty() || *opd_er_order_master_id == 0,
            Self::Visit(vn) => vn.is_empty(),
        }
    }

    pub fn vnan(&self) -> &String {
        match self {
            Self::Ipd(an) => an,
            Self::PreAdmit(vn) | Self::OpdEr(vn, _) | Self::Visit(vn) => vn,
        }
    }
    pub fn vnan_and_is_pre_admit_owned(&self) -> (String, bool) {
        match self {
            Self::Ipd(an) => (an.to_owned(), false),
            Self::PreAdmit(vn) => (vn.to_owned(), true),
            Self::OpdEr(vn, _) | Self::Visit(vn) => (vn.to_owned(), false),
        }
    }
}

fn show_jsvalue_error_message_with_document(document: &Document, js_value: &JsValue) {
    if let Some(message) = js_value.dyn_ref::<JsString>().map(|s| Into::<String>::into(s)) {
        document.get_element_by_id("errormessage").unwrap().set_text_content(Some(&message));
        log::error!("{}", message);
    }
}
