use dominator::{Dom, EventOptions, body, clone, events, html, with_node};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, or},
};
use std::rc::Rc;
use web_sys::{HtmlButtonElement, HtmlInputElement};

use kphis_model::{
    app::AppAsset,
    route::Route,
    timer::Timeout,
    user::his::{LoginResponse, hash},
};
use kphis_ui_app::App;
use kphis_ui_component::gadget::pin_code::PinCode;
use kphis_ui_core::{class, token::set_user};
use kphis_util::error::CONTACT_ADMIN;

/// - POST `EndPoint::User`
/// - PATCH `EndPoint::User`
#[derive(Clone, Default)]
pub struct IndexPage {
    changed: Mutable<bool>,
    // waiting: Mutable<Option<i32>>,
    reconnecting: Mutable<Option<i32>>,

    username: Mutable<String>,
    password: Mutable<String>,

    token_2fa: Mutable<String>,
    wait_2fa: Mutable<bool>,

    result: Mutable<String>,
}

impl IndexPage {
    pub fn new() -> Rc<Self> {
        Rc::new(Self::default())
    }

    fn reconnecting(&self, app: Rc<App>) {
        log::debug!("Try to set a reconnecting timeout");
        let reconnecting = self.reconnecting.clone();
        // if TimeOut already exists, do nothing
        if reconnecting.get().is_none() {
            let timer = Timeout::new(
                9000,
                clone!(app, reconnecting => move || {
                    log::debug!("Reconnecting EventSource..");
                    app.update_sw();
                    app.sse_end(false);
                    app.sse_ready_state.set(0);
                    App::sse_new_anonymous(app.clone());
                    // Drop TimeOut here
                    reconnecting.set(None);
                }),
            );
            let handle = timer.handle();
            timer.forget();
            reconnecting.set(Some(handle));
            log::debug!("Wait for reconnecting in 9 seconds..");
        }
    }

    fn submit(page: Rc<Self>, app: Rc<App>) {
        let username = page.username.get_cloned();
        let password = page.password.get_cloned();

        if username.is_empty() || password.is_empty() {
            app.user.set(None);
        } else {
            match hash(&password) {
                Ok(pwd) => {
                    app.async_load(
                        false,
                        clone!(app, username, pwd => async move {
                            // get user and token
                            // POST `EndPoint::User`
                            match LoginResponse::call_api_post_access(username.as_str(), &pwd, app.state()).await {
                                Ok(Some(response)) => {
                                    Self::login_success(response, page.clone(), app.clone()).await;
                                }
                                Ok(None) => {
                                    page.password.set_neq(String::new());
                                    page.token_2fa.set_neq(String::new());
                                    page.wait_2fa.set(true);
                                }
                                Err(e) => {
                                    if e.status != 401 {
                                        app.alert_app_error(&e).await;
                                        page.result.set_neq(e.message.clone());
                                    } else {
                                        page.token_2fa.set_neq(String::new());
                                        page.result.set_neq(String::from("ข้อมูลไม่ถูกต้อง กรุณาลองใหม่อีกครั้ง"));
                                    }
                                }
                            }
                        }),
                    );
                }
                Err(e) => {
                    app.alert_error(CONTACT_ADMIN, &["Error: ", &e.to_string()].concat());
                }
            }
        }
    }

    fn submit_2fa(page: Rc<Self>, app: Rc<App>) {
        let username = page.username.get_cloned();
        let token_2fa = page.token_2fa.get_cloned();

        if username.is_empty() || token_2fa.is_empty() {
            app.user.set(None);
        } else {
            app.async_load(
                false,
                clone!(app, username => async move {
                    // get user and token
                    // PATCH `EndPoint::User`
                    match LoginResponse::call_api_patch_access_2fa(false, &username, &token_2fa, app.state()).await {
                        Ok(Some(response)) => {
                            Self::login_success(response, page.clone(), app.clone()).await;
                        }
                        Ok(None) => {
                            page.username.set_neq(String::new());
                            page.password.set_neq(String::new());
                            page.token_2fa.set_neq(String::new());
                            page.wait_2fa.set(false);
                            page.result.set_neq(["ใช้เวลาบันทึก 2FA เกิน ", &app.handshake_2fa_timeout_second().to_string(), " วินาที กรุณาเข้าสู่ระบบใหม่อีกครั้ง"].concat());
                        }
                        Err(e) => {
                            if e.status != 401 {
                                app.alert_app_error(&e).await;
                                page.token_2fa.set_neq(String::new());
                                page.result.set_neq(e.message.clone());
                            } else {
                                page.token_2fa.set_neq(String::new());
                                page.result.set_neq(String::from("Token ไม่ถูกต้อง กรุณาลองใหม่อีกครั้ง"));
                            }
                        }
                    }
                }),
            );
        }
    }

    async fn login_success(response: LoginResponse, page: Rc<Self>, app: Rc<App>) {
        // update user
        if let Err(e) = set_user(Some(response.clone()), app.state()) {
            app.alert_app_error(&e).await;
            page.result.set_neq(e.message.clone());
        } else {
            // load Asset (if is None)
            if app.app_asset.lock_ref().is_none() {
                match AppAsset::get_asset(app.state()).await {
                    Ok(asset) => {
                        app.app_asset.set_neq(Some(Rc::new(asset)));
                        app.no_cache_mode.set(false);
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                        page.result.set_neq(e.message.clone());
                    }
                }
            }

            // clear UI
            page.username.set_neq(String::new());
            page.password.set_neq(String::new());
            page.token_2fa.set_neq(String::new());
            page.wait_2fa.set(false);

            // Clear (if exists) reconnecting TimeOut (prevent reconnect again)
            if let Some(handle) = page.reconnecting.get() {
                Timeout::manual_drop(handle);
                page.reconnecting.set(None);
            }
            // change SSE from anonymous to User specific one
            app.sse_end(false);
            app.sse_ready_state.set(0);
            App::sse_new(app.clone());
            app.get_initial_user_alert().await;

            Route::Info.hard_redirect();
        }
    }

    pub fn render(page: Rc<Self>, app: Rc<App>) -> Dom {
        app.set_title("KPHIS - Login");
        body().set_attribute("data-bs-theme", "light").unwrap();
        body().set_class_name("bg-primary");

        // FIX suspend modal backdrop after `Unauthorized`
        // we use ok() instead of unwrap()
        if let Some(backdrop) = body().query_selector(".modal-backdrop.show").ok().flatten() {
            backdrop.remove();
        }

        html!("main", {
            .future(app.visible.signal().for_each(clone!(app => move |visible| {
                if !visible {
                    app.sse_end(false);
                    app.sse_ready_state.set_neq(2);
                }
                async{}
            })))
            .future(app.sse_ready_state.signal().dedupe().for_each(clone!(app, page => move |state| {
                match state {
                    0 => {}
                    1 => {
                        // Clear (if exists) reconnecting TimeOut (prevent reconnect again)
                        if let Some(handle) = page.reconnecting.get() {
                            Timeout::manual_drop(handle);
                            page.reconnecting.set(None);
                        }
                    }
                    2.. => {
                        page.reconnecting(app.clone());
                    }
                }
                async {}
            })))
            // update SW and start SSE after browser tab gaining focus
            // NOTE: MUST SKIP when logout to prevent renew_token again,
            // so we check app.user (set user==None before calling sse_end())
            .future(map_ref!{
                let busy = app.loader_is_loading(),
                let visible = app.visible.signal(),
                let no_sse = app.sse_ready_state.signal_cloned().map(|state| state == 2) =>
                !busy && *visible && *no_sse
            }.for_each(clone!(app => move |ready| {
                if ready {
                    app.update_sw();
                    app.sse_end(false);
                    app.sse_ready_state.set(0);
                    App::start_sse_by_renew_token(app.clone());
                }
                async{}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let wait_2fa = page.wait_2fa.signal(),
                let changed = page.changed.signal() =>
                (!busy && *changed, *wait_2fa)
            ).for_each(clone!(app, page => move |(ready, wait_2fa)| {
                if ready {
                    if wait_2fa {
                        Self::submit_2fa(page.clone(), app.clone());
                    } else {
                        Self::submit(page.clone(), app.clone());
                    }
                    page.changed.set(false);
                }
                async {}
            })))
            // .class("bg-primary")
            // .style("height","100vh")
            .child(html!("div", {
                .class("container")
                .child(html!("div", {
                    .class(["row","justify-content-center","m-3"])
                    .child(html!("div", {
                        .class(["col","bg-white","py-3","rounded-3"])
                        .style("max-width","555px")
                        .children([
                            html!("h1",{
                                .class(class::TXT_C_BLUE)
                                .text("KPHIS Login")
                            }),
                            html!("hr",{.class("m-0")}),
                            html!("img",{
                                .attr("src","statics/picture/favicon/icon17.svg")
                                .style("width", "100%")
                                .class("img-fluid")
                                .attr("alt","")
                            }),
                        ])
                        .child_signal(app.sse_ready_state.signal().dedupe().map(clone!(app, page => move |ready_state| {
                            if ready_state == 0 {
                                Some(html!("h2", {
                                    .class(class::TXT_C_GREEN)
                                    .child(html!("i",{.class(class::FA_SPIN).style("font-size","32px")}))
                                    .text(" Connecting ...")
                                }))
                            } else if ready_state == 3 {
                                Some(html!("label", {
                                    .class(class::TXT_C_BX_RED)
                                    .style("cursor","pointer")
                                    .child(html!("h2", {
                                        .text("== OFFLINE ==")
                                    }))
                                    .event(clone!(app, page => move |_: events::Click| {
                                        app.update_sw();
                                        app.sse_end(false);
                                        app.sse_ready_state.set(0);
                                        App::sse_new_anonymous(app.clone());
                                    }))
                                }))
                            } else {
                                app.update_sw();
                                Some(html!("form",{
                                    // .class("row")
                                    .child_signal(page.wait_2fa.signal_cloned().map(clone!(app, page => move |wait_2fa| {
                                        (!wait_2fa).then(|| {
                                            html!("div", {
                                                // .class(class::COL_MD12_T)
                                                .class(class::FORM_FLOAT_T)
                                                .children([
                                                    html!("input" => HtmlInputElement, {
                                                        .attr("type", "text")
                                                        .attr("id", "username")
                                                        .class(class::FORM_CTRL_LG)
                                                        .attr("placeholder", "HOSxP Username")
                                                        .attr("autocomplete", "username")
                                                        .focused(true)
                                                        .prop_signal("value", page.username.signal_cloned())
                                                        .with_node!(element => {
                                                            .future(or(app.loader_is_loading(), app.sse_ready_state.signal_cloned().map(|state| state != 1)).for_each(clone!(element => move |disable| {
                                                                element.set_disabled(disable);
                                                                if !disable {
                                                                    element.focus().unwrap();
                                                                }
                                                                async {}
                                                            })))
                                                            .event(clone!(page => move |_: events::Input| {
                                                                page.username.set(element.value());
                                                            }))
                                                        })
                                                    }),
                                                    html!("label", {
                                                        .attr("for", "username")
                                                        .text("Username")
                                                    }),
                                                ])
                                            })
                                        })
                                    })))
                                    .child_signal(page.wait_2fa.signal_cloned().map(clone!(app, page => move |wait_2fa| {
                                        if wait_2fa {
                                            let pincode = PinCode::new(page.token_2fa.clone(), page.changed.clone());
                                            Some(PinCode::render(pincode))
                                        } else {
                                            Some(html!("div",{
                                                // .class("col-md-12")
                                                .class(class::FORM_FLOAT_T)
                                                .children([
                                                    html!("input" => HtmlInputElement,{
                                                        .attr("type", "password")
                                                        .attr("id", "password")
                                                        .class(class::FORM_CTRL_LG)
                                                        .attr("placeholder", "HOSxP Password")
                                                        .attr("autocomplete", "current-password")
                                                        .prop_signal("value", page.password.signal_cloned())
                                                        .with_node!(element => {
                                                            .future(or(app.loader_is_loading(), app.sse_ready_state.signal_cloned().map(|state| state != 1)).for_each(clone!(element => move |disable| {
                                                                element.set_disabled(disable);
                                                                async {}
                                                            })))
                                                            .event(clone!(page => move |_: events::Input| {
                                                                page.password.set(element.value());
                                                            }))
                                                        })
                                                        .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::KeyDown| {
                                                            if event.key() == "Enter" {
                                                                event.prevent_default();
                                                                page.changed.set_neq(true);
                                                            }
                                                        }))
                                                    }),
                                                    html!("label", {
                                                        .attr("for", "password")
                                                        .text("Password")
                                                    }),
                                                ])
                                            }))
                                        }
                                    })))
                                    .children([
                                        html!("div",{
                                            .class(class::COL_MD12_PY2)
                                            .child_signal(app.loader_is_loading().map(clone!(app, page => move |is_loading| {
                                                if is_loading {
                                                    Some(html!("button", {
                                                        .attr("type", "button")
                                                        .class(class::BTN_LG_CTRL_GOLD)
                                                        .child(html!("i",{.class(class::FA_SPIN).style("font-size","24px")}))
                                                        .text(" Cancel")
                                                        .event(clone!(app => move |_: events::Click| {
                                                            app.loader_cancel();
                                                        }))
                                                    }))
                                                } else {
                                                    Some(html!("button" => HtmlButtonElement,{
                                                        .attr("type", "button")
                                                        .class(class::BTN_LG_CTRL_BLUE)
                                                        .child(html!("i",{.class(class::FA_SIGN_IN)}))
                                                        .text_signal(page.wait_2fa.signal_cloned().map(|wait_2fa| if wait_2fa {" ยืนยัน 2FA"} else {" เข้าสู่ระบบ"}))
                                                        .with_node!(element => {
                                                            .future(app.sse_ready_state.signal_cloned().map(|state| state != 1).for_each(move |disable| {
                                                                element.set_disabled(disable);
                                                                async {}
                                                            }))
                                                        })
                                                        .event(clone!(page => move |_: events::Click| {
                                                            page.changed.set_neq(true);
                                                        }))
                                                    }))
                                                }
                                            })))
                                        }),
                                        html!("div", {
                                            .class(class::COL_MD12_C_RED)
                                            .text_signal(page.result.signal_cloned())
                                        }),
                                        html!("div", {
                                            .class("text-center")
                                            .child(html!("a", {
                                                .attr("href","/book/")
                                                .attr("target","_blank")
                                                .attr("title","Go to tutorial")
                                                .attr("rel","noopener noreferrer")
                                                .text("คู่มือการใช้งาน")
                                            }))
                                        }),
                                    ])
                                }))
                            }
                        })))
                        .child(html!("button", {
                            .attr("type", "button")
                            .style("position","fixed")
                            .style("right","19px")
                            .style("bottom","0")
                            .style("border","1px solid #333")
                            .style("border-bottom-width","0px")
                            .style("border-radius","9px 9px 0 0")
                            .style("padding","0 9px")
                            .child(html!("i", {.class(class::FA_SEARCH)}))
                            .text(" \u{00a0}Update")
                            .event(clone!(app => move |_: events::Click| {
                                app.update_sw();
                            }))
                        }))
                    }))
                }))
            }))
        })
    }
}
