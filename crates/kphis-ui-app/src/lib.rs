use async_once_cell::OnceCell;
use discard::Discard;
use dominator::{Dom, append_dom, clone, html, window_size};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, option},
    signal_vec::{MutableVec, SignalVecExt},
};
use js_sys::JsString;
use std::{
    collections::{HashMap, HashSet},
    future::Future,
    ops::Deref,
    rc::Rc,
    sync::Arc,
};
use wasm_bindgen::{JsCast, closure::Closure};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Event, EventSource, MessageEvent};

use kphis_drg_worker::{Thread as DrgThread, drg::model::I10vx};
use kphis_model::{
    SCREEN_WIDTH_EXTRA,
    app::{AppAsset, AppState},
    endpoint::EndPoint,
    fetch::{ExecuteResponse, Method},
    order::{Order, OrderParams},
    post_admit,
    route::Route,
    sse::{SseData, SseGroup, SseMenuTab, SseMessage, SseMessageParams, SsePostMessage},
    timer::Timeout,
    user::permission::Permission,
};
use kphis_typst_worker::Thread as TypstThread;
use kphis_ui_core::{
    class,
    popups::{PopupOkCancel, confirm::ConfirmPopup, dom_with_close::DomWithClosePopup, with_close::WithClosePopup},
    token,
};
use kphis_util::error::{AppError, CONTACT_ADMIN};

/// From https://developer.mozilla.org/en-US/docs/Web/API/EventSource/readyState
/// - EventSource.CONNECTING (0) : The connection is not yet open.
/// - EventSource.OPEN (1) : The connection is open and ready to communicate.
/// - EventSource.CLOSED (2) : The connection is closed or couldn't be opened.<br>
///
/// EventSource will auto-reconnect onError (except 502, EventSource will STOP auto-reconnect)
///
/// We use (3) for `EventSource error, retry` and set TimeOut to reconnect forever (solved 502 problem)
///
/// Mechanic in this module
/// - EventSource fired onOpen and onMessage => `sse_ready_state` mutable = 1
/// - EventSource fired onError => `sse_ready_state` mutable = 3
/// - Windows unloaded => `sse_ready_state` mutable = 2 + closing EventSource
///
/// Expected behavoir is
/// - Browser can use Anonymous-SSE or SSE-ID, not both
/// - User can has only 1 active SSE-ID (aka 1 browser tab)
/// - Kick all users off when server is down
/// - Update Service-Worker and connect SSE again with server is active again
///
/// `sse_ready_state` life cycle
/// - (0.) Start with 3
/// - (1.) GET `/sse` => onOpen/onMessage => Set `sse_ready_state` = 1 (4.)
///                   => onError => Set `sse_ready_state` = 3 (2.)
/// - Browser tab lost focus => close EventStream + Set `sse_ready_state` = 2 (2.)
/// - Browser tab gain focus + `sse_ready_state` = 2 => Set `sse_ready_state` = 0 + GET `/sse` or `/sse-id` (1.)
/// - (2.) Index page + `sse_ready_state` > 1 => Set reconnecting TimeOut (if TimeOut is already set, use old Timeout)
/// - (3.) Index page's reconnecting TimeOut evoked => Set `sse_ready_state` = 0 + GET `/sse` (1.)
/// - (4.) Index page + `sse_ready_state` = 1 => Clear reconnecting TimeOut handle
pub struct App {
    state: Rc<AppState>,
    typst_worker: Rc<OnceCell<TypstThread>>,
    typst_version: &'static str,
    drg_worker: Rc<OnceCell<DrgThread>>,
    sse: Mutable<Option<Rc<EventSource>>>,
    /// !document.hidden() state, browser tab was lost focus
    pub visible: Mutable<bool>,
    /// ready_state mutable for store `sse_new` interval checking result
    /// - 0 = connection
    /// - 1 = open
    /// - 2 = closed
    /// - 3 = restart
    pub sse_ready_state: Mutable<u16>,
    /// messages wait for sending
    pub messages: MutableVec<SsePostMessage>,
}

impl App {
    pub fn new(state: Rc<AppState>) -> Rc<Self> {
        Rc::new(Self {
            state,
            typst_worker: Rc::new(OnceCell::new()),
            typst_version: typst_utils::version().raw(),
            drg_worker: Rc::new(OnceCell::new()),
            sse: Mutable::new(None),
            visible: Mutable::new(true),
            sse_ready_state: Mutable::new(3),
            messages: MutableVec::new(),
        })
    }

    pub fn state(&self) -> Rc<AppState> {
        self.state.clone()
    }

    pub async fn typst_worker(&self) -> &TypstThread {
        self.typst_worker.get_or_init(async { kphis_typst_worker::spawn("/typst_worker_init.js").await }).await
    }

    pub fn typst_version(&self) -> &'static str {
        self.typst_version
    }

    pub async fn drg_worker(&self) -> &DrgThread {
        self.drg_worker.get_or_init(async { kphis_drg_worker::spawn("/drg_worker_init.js").await }).await
    }

    pub fn async_load<F>(&self, using_token: bool, fut: F)
    where
        F: Future<Output = ()> + 'static,
    {
        if using_token {
            let state = self.state.clone();
            self.loader_load(async move {
                if token::update_token(state.clone()).await {
                    fut.await;
                } else {
                    // log::debug!("Load with invalid Token, remove user and redirect to index page");
                    state.remove_user_and_go_index();
                }
            });
        } else {
            self.loader_load(fut);
        }
    }

    /// - None is NOT wide screen
    /// - Some(true) is wide screen card
    /// - Some(false) is wide screen table
    pub fn is_wide_screen_card_or_table(&self) -> impl Signal<Item = Option<bool>> + use<> {
        map_ref! {
            let is_wide_screen = window_size().map(|ws| ws.width > SCREEN_WIDTH_EXTRA).dedupe(),
            let is_wide_card = self.user.signal_cloned().map(|opt| option(opt.as_ref().map(|uc| uc.user.wide_screen.signal_cloned().map(|wide_mode| wide_mode.as_str() == "card")))).flatten() =>
            if *is_wide_screen {
                *is_wide_card
            } else {
                None
            }
        }
    }

    pub async fn set_clipboard(&self, message: &str) {
        let future = self.state.window.with(|w| w.navigator().clipboard().write_text(message));
        if JsFuture::from(future).await.is_ok() {
            self.alert("บันทึกสู่ Clipboard แล้ว", "ท่านสามารถ Paste (Ctrl + v) เพื่อวางข้อความได้")
        }
    }

    pub fn alert(&self, title: &str, message: &str) {
        if let Some(elm) = self.get_id("alert") {
            if let Some(title_elm) = elm.first_element_child() {
                title_elm.set_text_content(Some(title));
            }
            if let Some(message_elm) = elm.last_element_child() {
                message_elm.set_text_content(Some(message));
            }
            elm.class_list().add_1("show").unwrap();
            let show = Timeout::new(5000, move || {
                elm.class_list().remove_1("show").unwrap();
            });
            show.forget();
        }
    }

    pub async fn confirm(&self, message: &str) -> bool {
        let popup = ConfirmPopup::new("Confirm", message);
        // bootstrap modal will lock focus only within .modal-content
        // so we need to append to '.modal.show .modal-body' if exist
        match self.query_selector(".modal.show .modal-body").or(self.get_id("popup")) {
            Some(parent) => {
                let handle = append_dom(&parent, ConfirmPopup::render(popup.clone()));
                match popup.finished().wait_for(true).await {
                    Some(is_fin) => {
                        if is_fin {
                            handle.discard();
                            matches!(*popup.result.lock_ref(), PopupOkCancel::Ok)
                        } else {
                            false
                        }
                    }
                    None => false,
                }
            }
            None => false,
        }
    }

    pub async fn dom_with_close(&self, title: &str, content: Dom, is_error: bool) {
        let popup = DomWithClosePopup::new(title, is_error);
        // bootstrap modal will lock focus only within .modal-content
        // so we need to append to '.modal.show .modal-body' if exist
        match self.query_selector(".modal.show .modal-body").or(self.get_id("popup")) {
            Some(parent) => {
                let handle = append_dom(&parent, DomWithClosePopup::render(content, popup.clone()));
                match popup.finished().wait_for(true).await {
                    Some(is_fin) => {
                        if is_fin {
                            handle.discard();
                        }
                    }
                    None => {}
                }
            }
            None => {}
        }
    }

    pub async fn alert_with_close(&self, title: &str, message: &str, is_error: bool) {
        let popup = WithClosePopup::new(title, message, is_error);
        // bootstrap modal will lock focus only within .modal-content
        // so we need to append to '.modal.show .modal-body' if exist
        match self.query_selector(".modal.show .modal-body").or(self.get_id("popup")) {
            Some(parent) => {
                let handle = append_dom(&parent, WithClosePopup::render(popup.clone()));
                match popup.finished().wait_for(true).await {
                    Some(is_fin) => {
                        if is_fin {
                            handle.discard();
                        }
                    }
                    None => {}
                }
            }
            None => {}
        }
    }

    /// red alert popup + log to console + copy message to clipboard
    pub fn alert_error(&self, title: &str, message: &str) {
        // console
        log::error!("Error: {}", title);
        // red alert popup
        if let Some(elm) = self.get_id("alert") {
            if let Some(title_elm) = elm.first_element_child() {
                title_elm.set_text_content(Some(title));
            }
            if let Some(message_elm) = elm.last_element_child() {
                message_elm.set_text_content(Some(message));
            }
            elm.class_list().add_2("show", "danger").unwrap();
            let show = Timeout::new(7000, move || {
                elm.class_list().remove_2("show", "danger").unwrap();
            });
            show.forget();
        }
    }

    /// red alert popup + log to console + copy message to clipboard
    pub async fn alert_error_with_closed(&self, title: &str, message: &str) {
        // console
        log::error!("Error: {}", title);
        // red alert popup with close btn
        self.alert_with_close(title, message, true).await;
    }

    /// red alert popup + log to console + copy message to clipboard
    pub async fn alert_error_with_clipboard(&self, title: &str, message: &str) {
        // clipboard
        if let Err(e) = JsFuture::from(self.state.window.with(|w| w.navigator().clipboard().write_text(&message))).await {
            log::error!("{:?}", e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("Cannot save to Clipboard")));
        }
        // red alert popup
        self.alert_error_with_closed(title, message).await;
    }

    /// red alert popup + log to console + copy message to clipboard
    pub async fn alert_app_error(&self, err: &AppError) {
        self.alert_error_with_clipboard(&err.title.string(), &err.string()).await;
    }

    /// run fut when any response.rows_affected > 0
    pub async fn alert_execute_responses<F>(&self, responses: &[ExecuteResponse], fut: F)
    where
        F: Future<Output = ()> + 'static,
    {
        if let Some(response) = responses.iter().find(|res| res.error.is_some()) {
            if let Some(error) = &response.error {
                self.alert_error_with_clipboard(CONTACT_ADMIN, &["ExecuteResponse: ", error].concat()).await;
            }
        } else if responses.iter().any(|res| res.rows_affected > 0) {
            fut.await;
        } else {
            self.alert("ไม่มีการเปลี่ยนแปลง", "หากพบปัญหา กรุณาติดต่อผู้ดูแลระบบ");
        }
    }

    /// run fut when response.rows_affected > 0<br>
    /// Alert when error or rows_affected == 0
    pub async fn alert_execute_response<F>(&self, response: &ExecuteResponse, fut: F)
    where
        F: Future<Output = ()> + 'static,
    {
        if let Some(error) = &response.error {
            self.alert_error_with_clipboard(CONTACT_ADMIN, &["ExecuteResponse: ", error].concat()).await;
        } else if response.rows_affected > 0 {
            fut.await;
        } else {
            self.alert("ไม่มีการเปลี่ยนแปลง", "หากพบปัญหา กรุณาติดต่อผู้ดูแลระบบ");
        }
    }

    pub fn drug_alert_badge(&self, displaycolor: Option<i32>) -> Vec<Dom> {
        self.app_status
            .lock_ref()
            .as_ref()
            .map(|status| vec![status.hosxp_had_displaycolor, status.hosxp_lasa_displaycolor])
            .map(|had_lasa| {
                had_lasa
                    .into_iter()
                    .enumerate()
                    .filter_map(|(i, hl_opt)| {
                        if let (Some(hl), Some(dc)) = (hl_opt, displaycolor) {
                            if hl == dc {
                                // HAD
                                if i == 0 {
                                    Some(html!("span", {.class(class::BADGE_WRAP_R_RED).style("cursor","default").text("HAD")}))
                                } else {
                                    Some(html!("span", {.class(class::BADGE_WRAP_R_GOLD).style("cursor","default").text("LASA")}))
                                }
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<Dom>>()
            })
            .unwrap_or_default()
    }

    //=====//
    // SSE //
    //=====//
    pub fn start_sse_by_renew_token(app: Rc<Self>) {
        app.async_load(
            false,
            clone!(app => async move {
                if token::renew_access_token(app.state()).await {
                    App::sse_new(app.clone());
                    if matches!(*app.route.lock_ref(), Route::Index) {
                        Route::Info.hard_redirect();
                    } else {
                        app.get_initial_user_alert().await;
                    }
                } else {
                    App::sse_new_anonymous(app.clone());
                    if !matches!(*app.route.lock_ref(), Route::Index) {
                        Route::Index.hard_redirect();
                    }
                }
            }),
        );
    }

    pub fn sse_new_anonymous(app: Rc<Self>) {
        // log::debug!("Try new anonymous EventSource");
        if app.sse.get_cloned().is_none() {
            let event_source = EventSource::new("/sse/any").unwrap();
            {
                // on Open
                let open_cs = Closure::<dyn FnMut(_)>::new(clone!(app => move |_: Event| {
                    app.sse_ready_state.set(1);
                }));
                event_source.set_onopen(Some(open_cs.as_ref().unchecked_ref()));
                // event_source.add_event_listener_with_callback("open", open_cs.as_ref().unchecked_ref()).unwrap();
                open_cs.forget();
            }
            {
                // on Message (failed-safe when connected before on-open fired)
                let message_cs = Closure::<dyn FnMut(_)>::new(clone!(app => move |_: Event| {
                    app.sse_ready_state.set_neq(1);
                }));
                event_source.set_onmessage(Some(message_cs.as_ref().unchecked_ref()));
                // event_source.add_event_listener_with_callback("message", message_cs.as_ref().unchecked_ref()).unwrap();
                message_cs.forget();
            }
            {
                // on Error
                let error_cs = Closure::<dyn FnMut(_)>::new(clone!(app => move |_: Event| {
                    app.sse_ready_state.set(3);
                }));
                event_source.set_onerror(Some(error_cs.as_ref().unchecked_ref()));
                // event_source.add_event_listener_with_callback("error", error_cs.as_ref().unchecked_ref()).unwrap();
                error_cs.forget();
            }
            app.sse.set(Some(Rc::new(event_source.clone())));

            app.window.with(|w| {
                let cs = Closure::<dyn FnMut(_)>::new(move |_: Event| {
                    event_source.close();
                    app.sse_ready_state.set(2);
                });
                w.set_onbeforeunload(Some(cs.as_ref().unchecked_ref()));
                cs.forget();
            });
        }
    }

    pub fn sse_new(app: Rc<Self>) {
        // log::debug!("Try new EventSource with Id");
        if app.sse.get_cloned().is_none() {
            let event_source = EventSource::new(&["/sse/id/", &app.state_id().unwrap_or_default()].concat()).unwrap();
            {
                // on Open
                let open_cs = Closure::<dyn FnMut(_)>::new(clone!(app => move |_: Event| {
                    app.sse_ready_state.set(1);
                }));
                event_source.set_onopen(Some(open_cs.as_ref().unchecked_ref()));
                // event_source.add_event_listener_with_callback("open", open_cs.as_ref().unchecked_ref()).unwrap();
                open_cs.forget();
            }
            {
                // on Message (fail safe when connected before on-open fired)
                let message_cs = Closure::<dyn FnMut(_)>::new(clone!(app => move |_: Event| {
                    app.sse_ready_state.set_neq(1);
                }));
                event_source.set_onmessage(Some(message_cs.as_ref().unchecked_ref()));
                // event_source.add_event_listener_with_callback("message", message_cs.as_ref().unchecked_ref()).unwrap();
                message_cs.forget();
            }
            {
                // on Error
                let error_cs = Closure::<dyn FnMut(_)>::new(clone!(app => move |_: Event| {
                    app.sse_ready_state.set(3);
                }));
                event_source.set_onerror(Some(error_cs.as_ref().unchecked_ref()));
                // event_source.add_event_listener_with_callback("error", error_cs.as_ref().unchecked_ref()).unwrap();
                error_cs.forget();
            }
            {
                // global message
                let message_cs = Closure::<dyn FnMut(_)>::new(clone!(app => move |event: MessageEvent| {
                    let sse_text = event.data().dyn_into::<JsString>().unwrap().as_string().unwrap();
                    let sse_data = serde_json::from_str::<SseData>(&sse_text).unwrap();
                    app.set_sse_msg(SseMessage::GlobalMsg(sse_data));
                }));
                // event_source.set_onmessage(Some(message_cs.as_ref().unchecked_ref()));
                event_source.add_event_listener_with_callback("globalMsg", message_cs.as_ref().unchecked_ref()).unwrap();
                message_cs.forget();
            }
            {
                // ward message
                let ward_cs = Closure::<dyn FnMut(_)>::new(clone!(app => move |event: MessageEvent| {
                    let sse_text = event.data().dyn_into::<JsString>().unwrap().as_string().unwrap();
                    let sse_data = serde_json::from_str::<SseData>(&sse_text).unwrap();
                    app.set_sse_msg(SseMessage::WardMsg(sse_data));
                }));
                event_source.add_event_listener_with_callback("wardMsg", ward_cs.as_ref().unchecked_ref()).unwrap();
                ward_cs.forget();
            }
            {
                // spclty message
                let spclty_cs = Closure::<dyn FnMut(_)>::new(clone!(app => move |event: MessageEvent| {
                    let sse_text = event.data().dyn_into::<JsString>().unwrap().as_string().unwrap();
                    let sse_data = serde_json::from_str::<SseData>(&sse_text).unwrap();
                    app.set_sse_msg(SseMessage::SpcltyMsg(sse_data));
                }));
                event_source.add_event_listener_with_callback("spcltyMsg", spclty_cs.as_ref().unchecked_ref()).unwrap();
                spclty_cs.forget();
            }
            {
                // direct message
                let direct_cs = Closure::<dyn FnMut(_)>::new(clone!(app => move |event: MessageEvent| {
                    let sse_text = event.data().dyn_into::<JsString>().unwrap().as_string().unwrap();
                    let sse_data = serde_json::from_str::<SseData>(&sse_text).unwrap();
                    app.set_sse_msg(SseMessage::DirectMsg(sse_data));
                }));
                event_source.add_event_listener_with_callback("directMsg", direct_cs.as_ref().unchecked_ref()).unwrap();
                direct_cs.forget();
            }
            {
                // logout message
                let logout_cs = Closure::<dyn FnMut(_)>::new(clone!(app => move |event: MessageEvent| {
                    let sse_data = event.data().dyn_into::<JsString>().unwrap().as_string().unwrap();
                    if let Some(elm) = app.get_id("alert") {
                        if let Some(title_elm) = elm.first_element_child() {
                            title_elm.set_text_content(Some("บังคับออกจากระบบ"));
                        }
                        if let Some(message_elm) = elm.last_element_child() {
                            message_elm.set_text_content(Some(&sse_data));
                        }
                        elm.class_list().add_2("show", "danger").unwrap();
                        let show = Timeout::new(7000, clone!(app => move || {
                            elm.class_list().remove_2("show", "danger").unwrap();
                            App::logout(app, false);
                        }));
                        show.forget();
                    }
                }));
                event_source.add_event_listener_with_callback("logout", logout_cs.as_ref().unchecked_ref()).unwrap();
                logout_cs.forget();
            }
            app.sse.set(Some(Rc::new(event_source.clone())));
            app.window.with(|w| {
                let cs = Closure::<dyn FnMut(_)>::new(move |_: Event| {
                    event_source.close();
                });
                w.set_onbeforeunload(Some(cs.as_ref().unchecked_ref()));
                cs.forget();
            });
        }
    }

    pub fn send_sse(&self, message: SsePostMessage) {
        self.messages.lock_mut().push_cloned(message);
    }

    // with_reconnect will set `sse_ready_state` to 2, so IndexPage will reconnecting EventSource
    pub fn sse_end(&self, with_reconnect: bool) {
        // log::debug!("Try clearing any EventSource");
        // close eventstream
        if let Some(evs) = self.sse.lock_ref().as_ref() {
            evs.close();
        }
        self.sse.set(None);
        if with_reconnect {
            self.sse_ready_state.set_neq(2);
        }
    }

    pub async fn get_initial_user_alert(&self) {
        self.get_initial_message().await;
        if self.has_permission(Permission::IpdOrderCheck)
            && (self.endpoint_is_allow(&Method::GET, &EndPoint::IpdOrderOrder, true) || self.endpoint_is_allow(&Method::GET, &EndPoint::IpdOrderOrder, false))
        {
            self.get_ipd_order_as().await;
        }
        if self.has_permission(Permission::OpdErOrderCheck) && self.endpoint_is_allow(&Method::GET, &EndPoint::OpdErOrderOrder, false) {
            self.get_opd_er_order_as().await;
        }
        if self.doctor_code().is_some() && self.has_permission(Permission::DataTypeDoctorUse) && self.endpoint_is_allow(&Method::POST, &EndPoint::IpdSummary, false) {
            self.get_post_admit_count().await;
        }
    }

    /// GET `EndPoint::SseMessage`
    async fn get_initial_message(&self) {
        match SseMessage::call_api_get(&SseMessageParams::default(), self.state()).await {
            Ok(responses) => self.load_messages(&responses),
            Err(e) => {
                log::error!("Error:{}", e.message);
            }
        }
    }

    /// GET `EndPoint::IpdPostAdmitCount`
    pub async fn get_post_admit_count(&self) {
        match post_admit::get_post_admit_count(self.state()).await {
            Ok(count) => {
                self.post_admit_count.set_neq(count);
            }
            Err(e) => {
                self.alert_app_error(&e).await;
            }
        }
    }

    /// GET `EndPoint::IpdOrderOrder`
    pub async fn get_ipd_order_as(&self) {
        let params = OrderParams {
            without_plan: Some(String::from("Y")),
            doctor_not_confirm_as: Some(String::from("Y")),
            ..Default::default()
        };
        match Order::call_api_get_ipd(&params, self.state()).await {
            Ok(orders) => {
                let mut lock = self.ipd_order_as.lock_mut();
                lock.clear();
                lock.extend(orders.into_iter().map(Rc::new));
            }
            Err(e) => {
                self.alert_app_error(&e).await;
            }
        }
    }

    /// GET `EndPoint::OpdErOrderOrder`
    pub async fn get_opd_er_order_as(&self) {
        let params = OrderParams {
            without_plan: Some(String::from("Y")),
            doctor_not_confirm_as: Some(String::from("Y")),
            ..Default::default()
        };
        match Order::call_api_get_opd_er(&params, self.state()).await {
            Ok(orders) => {
                let mut lock = self.opd_er_order_as.lock_mut();
                lock.clear();
                lock.extend(orders.into_iter().map(Rc::new));
            }
            Err(e) => {
                self.alert_app_error(&e).await;
            }
        }
    }

    pub fn count_order_as(&self) -> impl Signal<Item = usize> + use<> {
        map_ref! {
            let ipd = self.ipd_order_as.signal_vec_cloned().len(),
            let opd_er = self.opd_er_order_as.signal_vec_cloned().len() =>
            ipd + opd_er
        }
    }

    /// GET `EndPoint::SseMessage`
    pub fn get_more_msg(&self, tab: SseMenuTab) {
        if let Some(params) = match tab {
            SseMenuTab::Global => Some(SseMessageParams {
                cat: Some(String::from("global")),
                min_id: self.get_min_global_message_id(),
            }),
            SseMenuTab::Ward => Some(SseMessageParams {
                cat: Some(String::from("ward")),
                min_id: self.get_min_ward_message_id(),
            }),
            SseMenuTab::Spclty => Some(SseMessageParams {
                cat: Some(String::from("splcty")),
                min_id: self.get_min_spclty_message_id(),
            }),
            SseMenuTab::Private => Some(SseMessageParams {
                cat: Some(String::from("private")),
                min_id: self.get_min_private_message_id(),
            }),
            SseMenuTab::Compose | SseMenuTab::Config => None,
        } {
            let state = self.state();
            self.async_load(true, async move {
                match SseMessage::call_api_get(&params, state.clone()).await {
                    Ok(responses) => state.load_messages(&responses),
                    Err(e) => {
                        log::error!("Error:{}", e.message);
                    }
                }
            })
        }
    }

    /// PATCH `EndPoint::SseMessage`
    pub fn set_msg_readed(&self, message_ids: Vec<u32>) {
        if !message_ids.is_empty() {
            let state = self.state();
            self.async_load(true, async move {
                match SseMessage::call_api_patch(&message_ids, state).await {
                    Ok(response) => {
                        if let Some(error) = response.error {
                            log::error!("Error:{}", error);
                        }
                    }
                    Err(e) => {
                        log::error!("Error:{}", e.message);
                    }
                }
            })
        }
    }

    /// POST `EndPoint::SseGroup`
    pub fn save_msg_group(&self) {
        let state = self.state();
        self.async_load(true, async move {
            if let Err(e) = SseGroup::call_api_post(&state.sse_group(), state).await {
                log::error!("Error:{}", e.message);
            }
        })
    }

    /// DELETE `EndPoint::Sse`<br>
    /// is_clean will clear localstorage, set server's app_asset_cache_exp == now
    pub fn logout(app: Rc<Self>, is_clean: bool) {
        app.async_load(
            true,
            clone!(app => async move {
                if let Err(e) = AppState::call_api_delete_sse_end(app.state()).await {
                    log::error!("Error:{}",e.message);
                }
                app.user.set(None);
                app.app_asset.set(None);
                app.sse_end(true);
                if is_clean {
                    if let Err(e) = AppAsset::patch_asset(app.state()).await {
                        log::error!("Error:{}",e.message);
                    }
                    app.clear_local_storage();
                    app.delete_caches().await;
                    app.unregister_sw().await;
                } else {
                    app.to_local_storage();
                    app.clear_in_memory_except_user();
                }
                Route::Index.hard_redirect();
            }),
        )
    }
}

impl Deref for App {
    type Target = Rc<AppState>;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

#[derive(Default)]
pub struct DaggerAsteriskState {
    // true: parsed at least once
    // false: never parse
    is_parsed: Mutable<bool>,
    is_parsing: Mutable<bool>,
    codes: Mutable<HashSet<String>>,
    /// `(Option<Dagger>, Asterisk)`
    pairs: Mutable<Vec<(Option<String>, String)>>,
    codes_vx: Mutable<HashMap<String, Arc<I10vx>>>,
}

impl DaggerAsteriskState {
    pub fn new() -> Rc<Self> {
        Rc::new(Self::default())
    }

    pub fn is_parsed_signal(&self) -> impl Signal<Item = bool> + use<> {
        self.is_parsed.signal()
    }

    pub fn start_parsing(&self) {
        self.is_parsing.set_neq(true);
    }

    pub fn is_parsing_signal(&self) -> impl Signal<Item = bool> + use<> {
        self.is_parsing.signal()
    }

    pub fn insert_code(&self, code: &str) {
        self.codes.lock_mut().insert(code.to_owned());
    }

    pub fn replace_code(&self, old: &str, new: &str) {
        let mut lock = self.codes.lock_mut();
        lock.remove(old);
        lock.insert(new.to_owned());
    }

    pub fn parse_codes(state: Rc<Self>, app: Rc<App>) {
        app.async_load(
            false,
            clone!(app => async move {
                let thread = app.drg_worker().await;
                let input = bitcode::encode(&state.codes.get_cloned());
                let bytes = thread.find_dagger_asterisk_pairs(input).await;
                if let Ok((pairs, codes_vx)) = bitcode::decode::<(Vec<(Option<String>, String)>, HashMap<String, Arc<I10vx>>)>(&bytes) {
                    state.pairs.set(pairs);
                    state.codes_vx.set(codes_vx);
                    state.is_parsing.set(false);
                    if !state.is_parsed.get() {
                        state.is_parsed.set(true);
                    }
                }
            }),
        )
    }

    pub fn get_status(&self, code: &str) -> DaggerAsteriskStatus {
        let codes_vx = self.codes_vx.get_cloned();
        if code == "???" || code.is_empty() {
            DaggerAsteriskStatus::None
        } else if let Some(vx) = codes_vx.get(code) {
            if vx.is_valid {
                if code.starts_with(['V', 'W', 'X', 'Y']) {
                    DaggerAsteriskStatus::Single(vx.clone())
                } else {
                    let pairs = self.pairs.lock_ref();
                    let daggers = pairs
                        .iter()
                        .filter_map(|(opt, aster)| (aster == code && opt.as_ref().map(|dagger| dagger != code).unwrap_or(true)).then(|| opt))
                        .collect::<Vec<&Option<String>>>();
                    let asters = pairs
                        .iter()
                        .filter_map(|(opt, aster)| (aster != code && opt.as_ref().map(|dagger| dagger == code).unwrap_or_default()).then(|| aster))
                        .collect::<Vec<&String>>();

                    match (daggers.len(), asters.len()) {
                        (0, 0) => DaggerAsteriskStatus::Single(vx.clone()),
                        (1, 0) => {
                            if let Some(dagger) = daggers.get(0).and_then(|opt| opt.as_ref().and_then(|dag| codes_vx.get(dag))) {
                                DaggerAsteriskStatus::AsteriskWith(vx.clone(), dagger.clone())
                            } else {
                                // included dagger's code_vx notfound
                                DaggerAsteriskStatus::AsteriskAlone(vx.clone())
                            }
                        }
                        (0, 1) => {
                            if let Some(aster) = asters.get(0).and_then(|ast| codes_vx.get(*ast)) {
                                DaggerAsteriskStatus::DaggerWith(vx.clone(), aster.clone())
                            } else {
                                // asterisk's code_vx notfound
                                DaggerAsteriskStatus::Single(vx.clone())
                            }
                        }
                        (_, 0) => DaggerAsteriskStatus::AsteriskWithMultiple(vx.clone(), daggers.into_iter().map(|opt| opt.as_ref().and_then(|dag| codes_vx.get(dag))).flatten().cloned().collect()),
                        (0, _) => DaggerAsteriskStatus::DaggerWithMultiple(vx.clone(), asters.into_iter().filter_map(|ast| codes_vx.get(ast)).cloned().collect()),
                        (_, _) => DaggerAsteriskStatus::Multiple(
                            pairs
                                .iter()
                                .filter_map(|(opt, aster)| {
                                    let dagger = opt.clone().unwrap_or_default();
                                    if dagger.as_str() == code || (!dagger.is_empty() && aster == code) {
                                        if let (Some(dag_vx), Some(ast_vx)) = (codes_vx.get(&dagger), codes_vx.get(aster)) {
                                            Some((dag_vx.clone(), ast_vx.clone()))
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                })
                                .collect(),
                        ),
                    }
                }
            } else {
                DaggerAsteriskStatus::Invalid
            }
        } else {
            DaggerAsteriskStatus::NotFound
        }
    }

    pub fn get_all_pairs(&self) -> Vec<(Option<Arc<I10vx>>, Arc<I10vx>)> {
        let i10vx = self.codes_vx.lock_ref();
        self.pairs
            .lock_ref()
            .iter()
            .filter_map(|(opt, aster)| i10vx.get(aster).map(|asterisk| (opt.as_ref().and_then(|dagger| i10vx.get(dagger)).cloned(), asterisk.clone())))
            .collect()
    }
}

#[derive(Clone, Default)]
pub enum DaggerAsteriskStatus {
    /// ??? or empty
    #[default]
    None,
    NotFound,
    // found but invalid
    Invalid,
    // dagger without pair, not dagger, not asterisk
    Single(Arc<I10vx>),
    /// (Dagger, Asterisk)
    DaggerWith(Arc<I10vx>, Arc<I10vx>),
    /// (Dagger, Asterisks)
    DaggerWithMultiple(Arc<I10vx>, Vec<Arc<I10vx>>),
    /// (Asterisk, Dagger)
    AsteriskWith(Arc<I10vx>, Arc<I10vx>),
    /// (Asterisk, Daggers)
    AsteriskWithMultiple(Arc<I10vx>, Vec<Arc<I10vx>>),
    // Asterisk without dagger : NEED ATTENTION
    AsteriskAlone(Arc<I10vx>),
    /// (Dagger, Asterisk)
    Multiple(Vec<(Arc<I10vx>, Arc<I10vx>)>),
}
