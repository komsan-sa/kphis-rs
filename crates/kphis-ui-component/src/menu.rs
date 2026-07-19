use dominator::{Dom, EventOptions, clone, events, html, is_window_loaded, link, with_node};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt},
    signal_vec::SignalVecExt,
};
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{HtmlButtonElement, HtmlInputElement, HtmlOptionElement, HtmlSelectElement, HtmlTextAreaElement};

use kphis_model::{
    app::{AppState, VisitTypeId},
    endpoint::EndPoint,
    fetch::Method,
    order::{Order, OrderItem, OrderPatch, OrderPatchAction, OrderTypeName},
    route::Route,
    sse::{SseData, SseMenuTab, SsePostMessage},
    tab::Tab,
    user::{
        config::{UserConfig, UserConfigResponse},
        his::LoginResponse,
        permission::Permission,
    },
};
use kphis_ui_app::App;
use kphis_ui_core::{binding::NiceSelect, class, doms, mixins, token::set_user};
use kphis_util::{
    datetime::{date_th, datetime_th, datetime_th_opt, datetime_th_opt_relative, time_hm},
    util::str_some,
};

use crate::{
    gadget::pin_code::PinCode,
    modal::{blank_modal, drug_details::DrugDetailModal},
    order::ORDER_STYLE,
};

/// - POST `EndPoint::UserConfig`
/// - PATCH `EndPoint::User`
/// - GET `EndPoint::IpdOrderOrder` (guarded, remove 'รคส' div)
/// - GET `EndPoint::OpdErOrderOrder` (guarded, remove 'รคส' div)
/// - PATCH `EndPoint::IpdOrderOrder` (guarded, remove 'รคส' div)
/// - PATCH `EndPoint::OpdErOrderOrder` (guarded, remove 'รคส' div)
#[derive(Clone, Default)]
pub struct MenuCpn {
    // check_once: Mutable<bool>,
    clear_cache: Mutable<bool>,
    logout: Mutable<bool>,

    reading_msg_ids: Mutable<Vec<u32>>,
    more_msg: Mutable<Option<SseMenuTab>>,

    active_tab: Mutable<SseMenuTab>,
    msg_ref: Mutable<Option<SseData>>,
    msg_target_user: Mutable<String>,
    msg_target_ward: Mutable<String>,
    msg_target_spclty: Mutable<String>,
    msg_message: Mutable<String>,
    msg_route: Mutable<String>,

    load_ipd_order_as: Mutable<bool>,
    load_opd_er_order_as: Mutable<bool>,
    load_summary_count: Mutable<bool>,

    totp_qr: Mutable<String>,
    token_2fa: Mutable<String>,
    token_2fa_result: Mutable<String>,

    drug_details_modal: Mutable<Option<Rc<DrugDetailModal>>>,
}

impl MenuCpn {
    pub fn new() -> Rc<Self> {
        Rc::new(Self::default())
    }

    fn clear_compose_msg(&self, app: Rc<App>) {
        self.msg_ref.set(None);
        self.msg_message.set(String::new());
        self.msg_target_user.set(String::new());
        self.msg_target_ward.set(String::new());
        self.msg_target_spclty.set(String::new());
        self.msg_route.set(String::new());
        if let Some(elm) = app.get_id("msg-target-user-select") {
            NiceSelect::new_default(&elm);
        }
        if let Some(elm) = app.get_id("msg-target-ward-select") {
            NiceSelect::new_default(&elm);
        }
        if let Some(elm) = app.get_id("msg-target-spclty-select") {
            NiceSelect::new_default(&elm);
        }
    }

    fn read_single_msg(&self, message_id: u32, tab: SseMenuTab, app: Rc<App>) {
        match tab {
            SseMenuTab::Global => app.read_one_global_msg(message_id),
            SseMenuTab::Ward => app.read_one_ward_msg(message_id),
            SseMenuTab::Spclty => app.read_one_spclty_msg(message_id),
            SseMenuTab::Private => app.read_one_private_msg(message_id),
            SseMenuTab::Compose | SseMenuTab::Config => {}
        };
        self.reading_msg_ids.set(vec![message_id]);
    }

    /// order_id = 0 is ALL order
    fn patch_order_ipd(order_id: u32, menu: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app, menu => async move {
                if app.confirm("ยืนยันรายการ").await {
                    let order_patch = OrderPatch {
                        action: OrderPatchAction::DoctorConfirm,
                        order_id,
                        nurse_order_as: app.doctor_code(),
                        order_time: None,
                        medplans: Vec::new(),
                        off_med_plan_numbers: Vec::new(),
                    };
                    // PATCH `EndPoint::IpdOrderOrder`
                    match order_patch.call_api_patch(true, app.state()).await {
                        Ok(responses) => {
                            app.alert_execute_responses(&responses, async move {
                                menu.load_ipd_order_as.set_neq(true);
                            }).await;
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }
            }),
        );
    }

    /// order_id = 0 is ALL order
    fn patch_order_opd_er(order_id: u32, menu: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app, menu => async move {
                if app.confirm("ยืนยันรายการ").await {
                    let order_patch = OrderPatch {
                        action: OrderPatchAction::DoctorConfirm,
                        order_id,
                        nurse_order_as: app.doctor_code(),
                        order_time: None,
                        medplans: Vec::new(),
                        off_med_plan_numbers: Vec::new(),
                    };
                    // PATCH `EndPoint::OpdErOrderOrder`
                    match order_patch.call_api_patch(false, app.state()).await {
                        Ok(responses) => {
                            app.alert_execute_responses(&responses, async move {
                                menu.load_opd_er_order_as.set_neq(true);
                            }).await;
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }
            }),
        );
    }

    fn patch_order_all(menu: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app, menu => async move {
                if app.confirm("ยืนยันรายการ").await {
                    let order_patch = OrderPatch {
                        action: OrderPatchAction::DoctorConfirm,
                        order_id: 0,
                        nurse_order_as: None,
                        order_time: None,
                        medplans: Vec::new(),
                        off_med_plan_numbers: Vec::new(),
                    };
                    // PATCH `EndPoint::IpdOrderOrder`
                    match order_patch.call_api_patch(true, app.state()).await {
                        // maybe 0 row affected
                        Ok(_) => {
                            menu.load_ipd_order_as.set_neq(true);
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                    // PATCH `EndPoint::OpdErOrderOrder`
                    match order_patch.call_api_patch(false, app.state()).await {
                        // maybe 0 row affected
                        Ok(_) => {
                            menu.load_opd_er_order_as.set_neq(true);
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }
            }),
        );
    }

    fn submit_2fa(menu: Rc<Self>, app: Rc<App>) {
        let username = app.token_sub().unwrap_or_default();
        let token_2fa = menu.token_2fa.get_cloned();

        if !username.is_empty() && !token_2fa.is_empty() {
            app.async_load(
                false,
                clone!(app, username => async move {
                    // get user and token
                    // PATCH `EndPoint::User`
                    match LoginResponse::call_api_patch_access_2fa(true, &username, &token_2fa, app.state()).await {
                        Ok(Some(response)) => {
                            if let Err(e) = set_user(Some(response), app.state()) {
                                app.alert_app_error(&e).await;
                                menu.token_2fa.set(String::new());
                                menu.token_2fa_result.set_neq(e.message.clone());
                            } else {
                                menu.totp_qr.set(String::new());
                                menu.token_2fa.set(String::new());
                                menu.token_2fa_result.set(String::new());
                            }
                        }
                        Ok(None) => {
                            post_user_config(Some(true), Some(menu.totp_qr.clone()), app.clone());
                            menu.token_2fa.set_neq(String::new());
                            menu.token_2fa_result.set_neq(["ใช้เวลาบันทึก 2FA เกิน ", &app.handshake_2fa_timeout_second().to_string(), " วินาที กรุณาลบรายการใน Authenticator, ทำการ Scan และยันยัน Token ใหม่อีกครั้ง"].concat());
                        }
                        Err(e) => {
                            if e.status == 418 {
                                menu.token_2fa.set(String::new());
                                menu.token_2fa_result.set_neq(String::from("Token ไม่ถูกต้อง กรุณาลองใหม่อีกครั้ง"));
                            } else {
                                app.alert_app_error(&e).await;
                                menu.token_2fa.set(String::new());
                                menu.token_2fa_result.set_neq(e.message.clone());
                            }
                        }
                    }
                }),
            );
        }
    }

    pub fn render(menu: Rc<Self>, app: Rc<App>) -> Dom {
        let allow_get_ipd_order_as = app.endpoint_is_allow(&Method::GET, &EndPoint::IpdOrderOrder, true) || app.endpoint_is_allow(&Method::GET, &EndPoint::IpdOrderOrder, false);
        let allow_get_opd_er_order_as = app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErOrderOrder, false);
        let is_allow_summary = app.doctor_code().is_some() && app.has_permission(Permission::DataTypeDoctorUse) && app.endpoint_is_allow(&Method::POST, &EndPoint::IpdSummary, false);

        html!("nav", {
            .future(app.visible.signal().for_each(clone!(app => move |visible| {
                if !visible {
                    app.sse_end(false);
                    app.sse_ready_state.set_neq(2);
                }
                async{}
            })))
            // update SW and start SSE after browser tab gaining focus
            .future(map_ref!{
                let busy = app.loader_is_loading(),
                let visible = app.visible.signal(),
                let no_sse = app.sse_ready_state.signal_cloned().map(|state| state == 2),
                let has_user = app.user.signal_cloned().map(|opt| opt.is_some()) =>
                (!busy && *visible && *has_user && *no_sse)
            }.for_each(clone!(app, menu => move |ready| {
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
                let out = menu.logout.signal() =>
                !busy && *out
            ).for_each(clone!(app, menu => move |ready| {
                if ready {
                    menu.logout.set(false);
                    App::logout(app.clone(), false);
                }
                async {}
            })))
            .future(menu.clear_cache.signal().for_each(clone!(app, menu => move |clear| {
                clone!(app, menu => async move {
                    if clear {
                        menu.clear_cache.set(false);
                        App::logout(app.clone(), true);
                    }
                })
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let more_tab_opt = menu.more_msg.signal_cloned() =>
                if !busy && more_tab_opt.is_some() {more_tab_opt.clone()} else {None}
            ).for_each(clone!(app, menu => move |opt| {
                if let Some(tab) = opt {
                    app.get_more_msg(tab);
                    menu.more_msg.set_neq(None);
                }
                async {}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let ids = menu.reading_msg_ids.signal_cloned() =>
                if !busy && !ids.is_empty() {Some(ids.to_owned())} else {None}
            ).for_each(clone!(app, menu => move |opt| {
                if let Some(ids) = opt {
                    app.set_msg_readed(ids);
                    menu.reading_msg_ids.set_neq(Vec::new());
                }
                async {}
            })))
            .apply_if(allow_get_ipd_order_as, |dom| dom
                .future(map_ref!(
                    let busy = app.loader_is_loading(),
                    let load = menu.load_ipd_order_as.signal() =>
                    !busy && *load
                ).for_each(clone!(app, menu => move |ready| {
                    clone!(app, menu => async move {
                        if ready {
                            // GET `EndPoint::IpdOrderOrder`
                            app.get_ipd_order_as().await;
                            menu.load_ipd_order_as.set(false);
                        }
                    })
                })))
            )
            .apply_if(allow_get_opd_er_order_as, |dom| dom
                .future(map_ref!(
                    let busy = app.loader_is_loading(),
                    let load = menu.load_opd_er_order_as.signal() =>
                    !busy && *load
                ).for_each(clone!(app, menu => move |ready| {
                    clone!(app, menu => async move {
                        if ready {
                            // GET `EndPoint::OpdErOrderOrder`
                            app.get_opd_er_order_as().await;
                            menu.load_opd_er_order_as.set(false);
                        }
                    })
                })))
            )
            .apply_if(is_allow_summary, |dom| dom
                .future(map_ref!(
                    let busy = app.loader_is_loading(),
                    let load = menu.load_summary_count.signal() =>
                    !busy && *load
                ).for_each(clone!(app, menu => move |ready| {
                    clone!(app, menu => async move {
                        if ready {
                            // GET `EndPoint::IpdPostAdmitCount`
                            app.get_post_admit_count().await;
                            menu.load_summary_count.set(false);
                        }
                    })
                })))
            )
            // .apply_if(app.is_production(), |dom| dom.style_important("background-color","#e6b728"))
            .class(class::NAV_BAR_BLUE)
            .children([
                link!(Route::Info.string(), {
                    .class(class::NAV_BAR_BRAND_R)
                    .text("KPHIS ")
                    .text(&app.app_status.lock_ref().as_ref().map(|state| state.code_name.clone()).unwrap_or_default())
                    .child(html!("div", {
                        .class(class::RESP_XL_MD)
                        .child(html!("small", {.class("mx-1").text(env!("CARGO_PKG_VERSION"))}))
                    }))
                    .apply_if(app.is_production(), |dom| dom.child(html!("i", {.class(class::FA_USER_SHIELD)})))
                }),
                html!("button", {
                    .class(class::NAV_BAR_TGL)
                    .attr("type", "button")
                    .attr("data-bs-toggle","collapse")
                    .attr("data-bs-target","#collapsibleNavId")
                    .attr("aria-controls","collapsibleNavId")
                    .attr("aria-expanded","false")
                    .attr("aria-label","Toggle navigation")
                    .child(html!("span", {.class("navbar-toggler-icon")}))
                }),
                // Drug information
                html!("div", {
                    .class(class::NAV_BAR_COLLAPSE)
                    .attr("id", "collapsibleNavId")
                    .child(menu_items(app.clone()))
                    .apply_if(
                        app.endpoint_is_allow(&Method::GET, &EndPoint::DrugUseDuration, false)
                        && app.endpoint_is_allow(&Method::GET, &EndPoint::SearchBoxMedHnText, false),
                    |dom| dom
                        .children([
                            html!("div", {
                                .class("me-2")
                                .style("cursor","pointer")
                                .child(html!("i", {.class(class::FA_PILLS).style("font-size","24px")}))
                                .attr("title","Drug information")
                                .attr("data-bs-toggle", "modal")
                                .attr("data-bs-target", "#drugInformationModal")
                                .event(clone!(app, menu => move |_:events::Click| {
                                    menu.drug_details_modal.set(Some(DrugDetailModal::new(false)));
                                }))
                            }),
                            html!("div", {
                                .class("modal")
                                .attr("id", "drugInformationModal")
                                .attr("role", "dialog")
                                .attr("tabindex", "-1")
                                .child_signal(menu.drug_details_modal.signal_cloned().map(clone!(app, menu => move |opt| {
                                    opt.map(|modal| DrugDetailModal::render(modal, menu.drug_details_modal.clone(), None, app.clone())).or(Some(blank_modal()))
                                })))
                            }),
                        ])
                    )
                    // Confirm OrderAs
                    .apply_if(
                        allow_get_ipd_order_as
                        && allow_get_opd_er_order_as
                        && app.has_permission(Permission::IpdOrderCheck)
                        && app.endpoint_is_allow(&Method::PATCH, &EndPoint::IpdOrderOrder, false)
                        && app.has_permission(Permission::OpdErOrderCheck)
                        && app.endpoint_is_allow(&Method::PATCH, &EndPoint::OpdErOrderOrder, false),
                    |dom| {
                        dom.children([
                            html!("div", {
                                .class(class::RELATIVE_L)
                                .style("white-space","nowrap")
                                .child(html!("button", {
                                    .attr("type", "button")
                                    .attr("title","ยืนยันการรับคำสั่งแพทย์ ที่บันทึกโดยพยาบาล")
                                    .class(class::BTN_SM_WHITEO)
                                    .child(html!("i", {.class(class::FA_MARKER)}))
                                    .child(html!("span", {
                                        .class(class::RESP_XL_MD)
                                        .text(" รคส")
                                    }))
                                    .attr("data-bs-toggle", "modal")
                                    .attr("data-bs-target", "#checkOrderAsModal")
                                }))
                                .child_signal(app.count_order_as().map(doms::badge_count_red))
                                .child(html!("span", {
                                    .class(class::BADGE_FIX_RB_GRAY)
                                    .style("cursor","default")
                                    .style("padding","5px")
                                    .style("color","#555")
                                    .child(html!("i", {.class(class::FA_SYNC)}))
                                    .event_with_options(&EventOptions::preventable(), clone!(menu => move |e: events::Click| {
                                        e.prevent_default();
                                        menu.load_ipd_order_as.set(true);
                                        menu.load_opd_er_order_as.set(true);
                                    }))
                                }))
                            }),
                            Self::render_confirm_order_as_modal(menu.clone(), app.clone()),
                        ])
                    })
                    // Summary alert
                    .apply_if(is_allow_summary, |dom| dom
                        .child(html!("div", {
                            .class(class::RELATIVE_L)
                            .class("ms-2")
                            .style("white-space","nowrap")
                            .child(html!("button", {
                                .attr("type", "button")
                                .attr("title","สรุปเวชระเบียนผู้ป่วยใน")
                                .class(class::BTN_SM_WHITEO)
                                .child(html!("i", {.class(class::FA_CLIPBOARD)}))
                                .child(html!("span", {
                                    .class(class::RESP_XL_MD)
                                    .text(" สรุป")
                                }))
                                .event(|_:events::Click| {
                                    Route::IpdPostAdmitList { view_by: String::from("doctor") }.hard_redirect();
                                })
                            }))
                            .child_signal(app.post_admit_count.signal().map(|count| doms::badge_count_red(count as usize)))
                            .child(html!("span", {
                                .class(class::BADGE_FIX_RB_GRAY)
                                .style("cursor","default")
                                .style("padding","5px")
                                .style("color","#555")
                                .child(html!("i", {.class(class::FA_SYNC)}))
                                .event_with_options(&EventOptions::preventable(), clone!(menu => move |e: events::Click| {
                                    e.prevent_default();
                                    menu.load_summary_count.set(true);
                                }))
                            }))
                        }))
                    )
                    // Message panel
                    .child(Self::render_message_panel(menu.clone(), app.clone()))
                    // User panel
                    .child(Self::render_user_panel(menu, app))
                }),
            ])
        })
    }

    fn render_message_panel(menu: Rc<Self>, app: Rc<App>) -> Dom {
        html!("ul", {
            .class("navbar-nav")
            .child(html!("li", {
                .class(class::NAV_ITEM_DROP)
                .children([
                    html!("a", {
                        .attr("href","#")
                        .class(class::NAV_LINK_DROP_TGL_TW_PY)
                        .class(class::FLEX_C)
                        .class("mt-1")
                        .attr("id", "msgDropdownId")
                        .attr("data-bs-toggle","dropdown")
                        .attr("data-bs-auto-close","outside")
                        .attr("aria-haspopup","true")
                        .attr("aria-expanded","false")
                        .attr("title", "Message")
                        .child(html!("div", {
                            .class(class::CIRCLE_L)
                            .class("position-relative")
                            .style("height","40px")
                            .style("width","40px")
                            .style("border-width","3px")
                            .style("border-style","solid")
                            .style_signal("border-color", app.sse_ready_state.signal_cloned().map(|ready_state| {
                                match ready_state {
                                    0 => "gold",
                                    1 => "lime",
                                    2.. => "red",
                                }
                            }))
                            .child(html!("i", {
                                .class(class::FA_ENV)
                                .class("fa-xl")
                                .style("padding-top","17px")
                                .style("padding-left","5px")
                                .style("width","28px")
                            }))
                            .child_signal(app.count_unread_all_msg().map(doms::badge_count_red))
                        }))
                    }),
                    html!("div", {
                        .class(class::DROP_MENU_END)
                        .style("min-width","420px")
                        .attr("aria-labelledby","msgDropdownId")
                        .children([
                            html!("div", {
                                .class("text-center")
                                .children([
                                    html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_REL_L)
                                        .class_signal("btn-primary", menu.active_tab.signal_cloned().map(|tab| tab == SseMenuTab::Global))
                                        .class_signal("btn-outline-primary", menu.active_tab.signal_cloned().map(|tab| tab != SseMenuTab::Global))
                                        .text(SseMenuTab::Global.as_str())
                                        .child_signal(app.count_unread_global_msg().map(doms::badge_count_red))
                                        .event(clone!(menu => move |_: events::Click| {
                                            // menu.read_all_current_tab_msg(app.clone());
                                            menu.active_tab.set_neq(SseMenuTab::Global);
                                        }))
                                    }),
                                    html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_REL_L)
                                        .class_signal("btn-primary", menu.active_tab.signal_cloned().map(|tab| tab == SseMenuTab::Ward))
                                        .class_signal("btn-outline-primary", menu.active_tab.signal_cloned().map(|tab| tab != SseMenuTab::Ward))
                                        .text(SseMenuTab::Ward.as_str())
                                        .child_signal(app.count_unread_ward_msg().map(doms::badge_count_red))
                                        .event(clone!(menu => move |_: events::Click| {
                                            // menu.read_all_current_tab_msg(app.clone());
                                            menu.active_tab.set_neq(SseMenuTab::Ward);
                                        }))
                                    }),
                                    html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_REL_L)
                                        .class_signal("btn-primary", menu.active_tab.signal_cloned().map(|tab| tab == SseMenuTab::Spclty))
                                        .class_signal("btn-outline-primary", menu.active_tab.signal_cloned().map(|tab| tab != SseMenuTab::Spclty))
                                        .text(SseMenuTab::Spclty.as_str())
                                        .child_signal(app.count_unread_spclty_msg().map(doms::badge_count_red))
                                        .event(clone!(menu => move |_: events::Click| {
                                            // menu.read_all_current_tab_msg(app.clone());
                                            menu.active_tab.set_neq(SseMenuTab::Spclty);
                                        }))
                                    }),
                                    html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_REL_L)
                                        .class_signal("btn-primary", menu.active_tab.signal_cloned().map(|tab| tab == SseMenuTab::Private))
                                        .class_signal("btn-outline-primary", menu.active_tab.signal_cloned().map(|tab| tab != SseMenuTab::Private))
                                        .text(SseMenuTab::Private.as_str())
                                        .child_signal(app.count_unread_private_msg().map(doms::badge_count_red))
                                        .event(clone!(menu => move |_: events::Click| {
                                            // menu.read_all_current_tab_msg(app.clone());
                                            menu.active_tab.set_neq(SseMenuTab::Private);
                                        }))
                                    }),
                                    html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_L)
                                        .class_signal("btn-primary", menu.active_tab.signal_cloned().map(|tab| tab == SseMenuTab::Compose))
                                        .class_signal("btn-outline-primary", menu.active_tab.signal_cloned().map(|tab| tab != SseMenuTab::Compose))
                                        .child(html!("i", {.class(class::FA_EDIT)}))
                                        .attr("title", SseMenuTab::Compose.as_str())
                                        .event(clone!(app, menu => move |_: events::Click| {
                                            // menu.read_all_current_tab_msg(app.clone());
                                            menu.active_tab.set_neq(SseMenuTab::Compose);
                                            menu.clear_compose_msg(app.clone());
                                        }))
                                    }),
                                    html!("button", {
                                        .attr("type", "button")
                                        .class("btn")
                                        .class_signal("btn-primary", menu.active_tab.signal_cloned().map(|tab| tab == SseMenuTab::Config))
                                        .class_signal("btn-outline-primary", menu.active_tab.signal_cloned().map(|tab| tab != SseMenuTab::Config))
                                        .child(html!("i", {.class(class::FA_COG)}))
                                        .attr("title", SseMenuTab::Config.as_str())
                                        .event(clone!(menu => move |_: events::Click| {
                                            // menu.read_all_current_tab_msg(app.clone());
                                            menu.active_tab.set_neq(SseMenuTab::Config);
                                        }))
                                    }),
                                ])
                            }),
                            html!("div", {.class("dropdown-divider")}),
                        ])
                        .child_signal(menu.active_tab.signal_cloned().map(clone!(app, menu => move |tab| {
                            Some(Self::render_message(tab.clone(), menu.clone(), app.clone()))
                        })))
                    })
                ])
            }))
        })
    }

    fn render_message_box(msg: SseData, tab: SseMenuTab, menu: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .class(class::BOX_ROUND_BLUES_T)
            .children([
                html!("div", {
                    .class("fw-bold")
                    .style("line-height","28px")
                    .child(html!("i", {
                        .class(class::FA_ENV)
                        .class("me-2")
                        .apply_if(!msg.readed, |dom| dom.class(class::BOLD_RED_L))
                    }))
                    .child(html!("span", {
                        .text(&datetime_th_opt_relative(&msg.message_datetime))
                    }))
                    .apply_if(!msg.readed, |d| { d
                        .child(html!("button", {
                            .attr("type", "button")
                            .class(class::BTN_SM_BLUEO)
                            .class(class::FLOAT_RR)
                            .attr("title","อ่านแล้ว")
                            .child(html!("i", {.class(class::FA_CHECK_CIRCLE)}))
                            .event(clone!(app, menu, tab, msg => move |_: events::Click| {
                                menu.read_single_msg(msg.message_id, tab.clone(), app.clone());
                            }))
                        }))
                    })
                    .child(html!("button", {
                        .attr("type", "button")
                        .class(class::BTN_SM_BLUEO)
                        .class(class::FLOAT_RR)
                        .attr("title","ตอบกลับ")
                        .child(html!("i", {.class(class::FA_REPLY)}))
                        .event(clone!(app, menu, tab, msg => move |_: events::Click| {
                            menu.read_single_msg(msg.message_id, tab.clone(), app.clone());
                            menu.active_tab.set(SseMenuTab::Compose);
                            menu.msg_target_user.set(msg.sender_code.clone());
                            menu.msg_ref.set(Some(msg.clone()));
                        }))
                    }))
                    .apply(|dom| {
                        if let Some(route) = msg.route.as_ref().map(|url| Route::from_url(url, &app.host.lock_ref())) {
                            if let Route::External {path} = route {
                                dom.child(html!("a", {
                                    .attr("href", &path)
                                    .class(class::BTN_SM_BLUEO)
                                    .class(class::FLOAT_RR)
                                    .attr("rel","noopener noreferrer")
                                    .attr("target","_blank")
                                    .attr("title","เปิด URL ที่แนบ")
                                    .child(html!("i", { .class(class::FA_EXT_LINK)}))
                                    .event(clone!(app, menu, tab, msg => move |_: events::Click| {
                                        menu.read_single_msg(msg.message_id, tab.clone(), app.clone());
                                    }))
                                }))
                            } else if !matches!(route, Route::NotFound { path: _ } | Route::UnAuthorized { hash: _ }) && route.has_permission(app.state()) {
                                let href = route.string();
                                dom.child(html!("a", {
                                    .class(class::BTN_SM_BLUEO)
                                    .class(class::FLOAT_RR)
                                    .attr("href", &href)
                                    .attr("title","ไปยังรายการ")
                                    .text("GO")
                                    .event_with_options(&EventOptions::preventable(), clone!(app, menu, tab, msg => move |e: events::Click| {
                                        e.prevent_default();
                                        menu.read_single_msg(msg.message_id, tab.clone(), app.clone());
                                        dominator::routing::go_to_url(&href);
                                    }))
                                }))
                            } else {
                                dom
                            }
                        } else {
                            dom
                        }
                    })
                }),
                html!("div", {
                    .class(class::BORDER_T_T)
                    .text(&msg.message.clone().unwrap_or_default())
                }),
                html!("div", {
                    .class(class::SMALL_R)
                    .text(&msg.sender_name.clone().unwrap_or_default())
                }),
            ])
        })
    }

    fn render_message_ref(msg: SseData, app: Rc<App>) -> Dom {
        html!("div", {
            .class(class::BOX_ROUND_SMALL_R)
            .children([
                html!("div", {
                    .style("line-height","28px")
                    .children([
                        html!("span", {
                            .class("fw-bold")
                            .text(&msg.sender_name.clone().unwrap_or_default())
                        }),
                        html!("span", {
                            .class(class::SMALL_R2)
                            .text(&datetime_th_opt_relative(&msg.message_datetime))
                        }),
                    ])
                    .apply(|dom| {
                        if let Some(route) = msg.route.as_ref().map(|url| Route::from_url(url, &app.host.lock_ref())) {
                            if let Route::External {path} = route {
                                dom.child(html!("a", {
                                    .attr("href", &path)
                                    .class(class::FLOAT_RR)
                                    .attr("rel","noopener noreferrer")
                                    .attr("target","_blank")
                                    .attr("title","เปิด URL ที่แนบ")
                                    .child(html!("i", { .class(class::FA_EXT_LINK)}))
                                }))
                            } else if !matches!(route, Route::NotFound { path: _ } | Route::UnAuthorized { hash: _ }) && route.has_permission(app.state()) {
                                let href = route.string();
                                dom.child(html!("a", {
                                    .class(class::FLOAT_RR)
                                    .attr("href", &href)
                                    .attr("title","ไปยังรายการ")
                                    .text("GO")
                                    .event_with_options(&EventOptions::preventable(), move |e: events::Click| {
                                        e.prevent_default();
                                        dominator::routing::go_to_url(&href);
                                    })
                                }))
                            } else {
                                dom
                            }
                        } else {
                            dom
                        }
                    })
                }),
                html!("div", {
                    .class("ms-2")
                    .text(&msg.message.clone().unwrap_or_default())
                }),
            ])
        })
    }

    fn render_message(tab: SseMenuTab, menu: Rc<Self>, app: Rc<App>) -> Dom {
        match match &tab {
            SseMenuTab::Global => Some(app.msg_global.clone()),
            SseMenuTab::Ward => Some(app.msg_ward.clone()),
            SseMenuTab::Spclty => Some(app.msg_spclty.clone()),
            SseMenuTab::Private => Some(app.msg_private.clone()),
            SseMenuTab::Compose | SseMenuTab::Config => None,
        } {
            Some(messages) => {
                html!("div", {
                    .class(class::ROW_TC)
                    .class("m-0")
                    .children([
                        html!("div", {
                            .class("p-2")
                            .style("overflow-y","auto")
                            .style("height","440px")
                            .children_signal_vec(messages.clone().signal_vec_cloned().map(clone!(app, menu, tab => move |msg| {
                                html!("div", {
                                    .apply(|dom| {
                                        if let Some(ref_msg) = msg.reference.as_ref().and_then(|s| serde_json::from_str::<SseData>(s).ok()) {
                                            dom.child(Self::render_message_ref(ref_msg, app.clone()))
                                        } else {
                                            dom
                                        }
                                    })
                                    .child(Self::render_message_box(msg, tab.clone(), menu.clone(), app.clone()))
                                })
                            })))
                        }),
                        html!("div", {.class("dropdown-divider")}),
                        html!("div", {
                            .class("d-flex")
                            .child(html!("div", {
                                .children([
                                    html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_SM_L_BLUEO)
                                        .child(html!("i", {.class(class::FA_CHECK_CIRCLE)}))
                                        .text(" ทั้งหมด")
                                        .event(clone!(app, menu => move |_: events::Click| {
                                            let all_msg_ids = app.all_msg_ids();
                                            menu.reading_msg_ids.set(all_msg_ids);
                                            app.read_all_global_msg();
                                            app.read_all_ward_msg();
                                            app.read_all_spclty_msg();
                                            app.read_all_private_msg();
                                        }))
                                    }),
                                ])
                            }))
                            .child(html!("div", {
                                .class("ms-auto")
                                .children([
                                    html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_SM_BLUEO)
                                        .child(html!("i", {.class(class::FA_CHECK_CIRCLE_L)}))
                                        .text(tab.as_str())
                                        .event(clone!(app, menu, tab => move |_: events::Click| {
                                            match tab {
                                                SseMenuTab::Global => {
                                                    let msgs = app.msg_global.lock_ref().iter().filter_map(|msg| (!msg.readed).then(|| msg.message_id)).collect();
                                                    menu.reading_msg_ids.set(msgs);
                                                    app.read_all_global_msg();
                                                }
                                                SseMenuTab::Ward => {
                                                    let msgs = app.msg_ward.lock_ref().iter().filter_map(|msg| (!msg.readed).then(|| msg.message_id)).collect();
                                                    menu.reading_msg_ids.set(msgs);
                                                    app.read_all_ward_msg()
                                                }
                                                SseMenuTab::Spclty => {
                                                    let msgs = app.msg_spclty.lock_ref().iter().filter_map(|msg| (!msg.readed).then(|| msg.message_id)).collect();
                                                    menu.reading_msg_ids.set(msgs);
                                                    app.read_all_spclty_msg()
                                                }
                                                SseMenuTab::Private => {
                                                    let msgs = app.msg_private.lock_ref().iter().filter_map(|msg| (!msg.readed).then(|| msg.message_id)).collect();
                                                    menu.reading_msg_ids.set(msgs);
                                                    app.read_all_private_msg();
                                                }
                                                SseMenuTab::Compose
                                                | SseMenuTab::Config => {}
                                            }
                                        }))
                                    }),
                                    html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_SM_BLUEO)
                                        .class("ms-1")
                                        .child(html!("i", {.class(class::FA_SYNC)}))
                                        .text(" ย้อนหลัง..")
                                        .event(clone!(menu, tab => move |_: events::Click| {
                                            menu.more_msg.set(Some(tab.clone()));
                                        }))
                                    }),
                                ])
                            }))
                        }),
                    ])
                })
            }
            None => {
                if matches!(tab, SseMenuTab::Compose) {
                    Self::render_message_compose(menu, app.clone())
                } else {
                    Self::render_message_config(app.clone())
                }
            }
        }
    }

    fn render_message_compose(menu: Rc<Self>, app: Rc<App>) -> Dom {
        let (all_doctor_select_option, ward_select_option, spclty_kphis_select_option) = match app.app_asset.lock_ref().as_ref() {
            Some(assets_arc) => {
                let asset = assets_arc.as_ref().to_owned();
                (asset.all_doctor_select_option, asset.ward_select_option, asset.spclty_kphis_select_option)
            }
            None => (Vec::new(), Vec::new(), Vec::new()),
        };

        html!("div", {
            .future(is_window_loaded().for_each(clone!(app, menu => move |loaded| {
                if loaded {
                    if let Some(elm) = app.get_id("msg-target-user-select") {
                        NiceSelect::new_default_with_value(&elm, &menu.msg_target_user.lock_ref());
                    }
                    if let Some(elm) = app.get_id("msg-target-ward-select") {
                        NiceSelect::new_default(&elm);
                    }
                    if let Some(elm) = app.get_id("msg-target-spclty-select") {
                        NiceSelect::new_default(&elm);
                    }
                    if let Some(elm) = app.get_id("msg-message").and_then(|elm| elm.dyn_into::<HtmlTextAreaElement>().ok()) {
                        elm.focus().unwrap();
                    }
                }
                async {}
            })))
            .class(class::ROW_TC)
            .class("m-0")
            .children([
                html!("div", {
                    .children([
                        html!("div", {
                            //.class(class::INPUT_GROUP_SM_T)
                            .class("mb-1")
                            .child(html!("label", {
                                .class("form-label")
                                //.class("input-group-text")
                                .attr("for", "msg-message")
                                .text("ส่งข้อความ")
                            }))
                            .apply(|dom| {
                                match menu.msg_ref.get_cloned() {
                                    Some(ref_msg) => {
                                        dom.child(Self::render_message_ref(ref_msg, app.clone()))
                                    }
                                    None => {
                                        dom
                                    }
                                }
                            })
                            .child(html!("textarea" => HtmlTextAreaElement, {
                                .class(class::FORM_CTRL_SM)
                                .attr("id", "msg-message")
                                .attr("autocomplete","off")
                                .apply(mixins::textarea_value_auto_expand(menu.msg_message.clone(), Mutable::new(false)))
                            }))
                        }),
                        html!("div", {
                            .class(class::INPUT_GROUP_SM_T)
                            .children([
                                doms::label_group_for("msg-target-user-select","ถึงเจ้าหน้าที่"),
                                html!("div", {
                                    .class(class::FLEX_GROW1)
                                    .child(html!("select" => HtmlSelectElement, {
                                        .class(class::FORM_CTRL_SM)
                                        .attr("id", "msg-target-user-select")
                                        .child(html!("option", {
                                            .attr("value", "")
                                            .text("เลือก")
                                        }))
                                        .children(all_doctor_select_option.iter().map(|option| {
                                            doms::select_option(option, "")
                                        }))
                                        .apply(mixins::string_value_select(menu.msg_target_user.clone(), Mutable::new(false)))
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::INPUT_GROUP_SM_T)
                            .children([
                                doms::label_group_for("msg-target-ward-select","ถึงหอผู้ป่วย"),
                                html!("div", {
                                    .class(class::FLEX_GROW1)
                                    .child(html!("select" => HtmlSelectElement, {
                                        .class(class::FORM_CTRL_SM)
                                        .attr("id", "msg-target-ward-select")
                                        .child(html!("option", {.attr("value", "").text("เลือก")}))
                                        .child(html!("option", {.attr("value", "00").text("ER")}))
                                        .children(ward_select_option.iter().map(|option| {
                                            doms::select_option(option, "")
                                        }))
                                        .apply(mixins::string_value_select(menu.msg_target_ward.clone(), Mutable::new(false)))
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::INPUT_GROUP_SM_T)
                            .children([
                                doms::label_group_for("msg-target-spclty-select","ถึงแผนก"),
                                html!("div", {
                                    .class(class::FLEX_GROW1)
                                    .child(html!("select" => HtmlSelectElement, {
                                        .class(class::FORM_CTRL_SM)
                                        .attr("id", "msg-target-spclty-select")
                                        .child(html!("option", {.attr("value", "").text("เลือก")}))
                                        .child(html!("option", {.attr("value", "0").text("ฝ่ายเภสัชกรรม")}))
                                        .children(spclty_kphis_select_option.iter().map(|option| {
                                            doms::select_option(option, "")
                                        }))
                                        .apply(mixins::string_value_select(menu.msg_target_spclty.clone(), Mutable::new(false)))
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::INPUT_GROUP_SM_T)
                            .children([
                                doms::label_group_for("msg-route","แนบ URL"),
                                html!("input" => HtmlInputElement, {
                                    .attr("type", "text")
                                    .class(class::FORM_CTRL_SM)
                                    .attr("id", "msg-route")
                                    .attr("placeholder","เช่น http://")
                                    .attr("autocomplete","off")
                                    .apply(mixins::string_value_end(menu.msg_route.clone(), Mutable::new(false)))
                                }),
                                html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_GRAY)
                                    .child(html!("i", {.class(class::FA_MAGIC)}))
                                    .event(clone!(app, menu => move |_: events::Click| {
                                        menu.msg_route.set(app.route.lock_ref().string());
                                    }))
                                }),
                                html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_RED)
                                    .child(html!("i", {.class(class::FA_X)}))
                                    .event(clone!(menu => move |_: events::Click| {
                                        menu.msg_route.set(String::new());
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {.class(class::FORM_TEXT_R).text("หากไม่ระบุเจ้าหน้าที่, หอผู้ป่วย หรือแผนก จะเป็นการส่งประกาศ")}),
                        html!("div", {
                            .child(html!("button" => HtmlButtonElement, {
                                .class(class::BTN_SM_FR_T_BLUEO)
                                .attr("type", "button")
                                .child(html!("i", {.class(class::FA_ENV)}))
                                .text(" ส่ง")
                                .text_signal(map_ref!{
                                    let target_user = menu.msg_target_user.signal_cloned(),
                                    let target_ward = menu.msg_target_ward.signal_cloned(),
                                    let target_spclty = menu.msg_target_spclty.signal_cloned() =>
                                    match (target_user.is_empty(), target_ward.is_empty(), target_spclty.is_empty()) {
                                        (false, true, true) => "ส่วนตัว",
                                        (true, false, true) => "หอผู้ป่วย",
                                        (true, true, false) => "แผนก",
                                        (true, true, true) => "ประกาศ",
                                        (_,_,_) => "",
                                    }
                                })
                                .with_node!(element => {
                                    .future(menu.msg_message.signal_cloned().for_each(move |message| {
                                        element.set_disabled(message.is_empty());
                                        async {}
                                    }))
                                })
                                .event(clone!(app, menu => move |_: events::Click| {
                                    let route = Route::from_url(&menu.msg_route.lock_ref(), &app.host.lock_ref());
                                    let message = SsePostMessage {
                                        message: menu.msg_message.get_cloned(),
                                        person: str_some(menu.msg_target_user.get_cloned()),
                                        ward: str_some(menu.msg_target_ward.get_cloned()),
                                        spclty_id: menu.msg_target_spclty.lock_ref().parse::<u32>().ok(),
                                        route: (!matches!(route, Route::NotFound { path: _ } | Route::UnAuthorized { hash: _ })).then_some(route),
                                        reference: menu.msg_ref.get_cloned(),
                                    };
                                    menu.clear_compose_msg(app.clone());
                                    app.send_sse(message);
                                }))
                            }))
                        }),
                    ])
                }),
            ])
        })
    }

    fn render_message_config(app: Rc<App>) -> Dom {
        let (ward_select_option, spclty_kphis_select_option) = match app.app_asset.lock_ref().as_ref() {
            Some(assets_arc) => {
                let asset = assets_arc.as_ref().to_owned();
                (asset.ward_select_option, asset.spclty_kphis_select_option)
            }
            None => (Vec::new(), Vec::new()),
        };

        html!("div", {
            .future(is_window_loaded().for_each(clone!(app => move |loaded| {
                if loaded {
                    if let Some(user) = app.user.lock_ref().as_ref() {
                        if let Some(elm) = app.get_id("msg-ward-select") {
                            NiceSelect::new_default_with_value(&elm, &user.user.wards.lock_ref().join(","));
                        }
                        if let Some(elm) = app.get_id("msg-spclty-select") {
                            NiceSelect::new_default_with_value(&elm, &user.user.spclty_ids.lock_ref().iter().map(|u| u.to_string()).collect::<Vec<String>>().join(","));
                        }
                    }
                }
                async {}
            })))
            .class(class::ROW_TC)
            .class("m-0")
            .apply(|dom| {
                if app.doctor_code().is_some() { dom
                    .children([
                        html!("div", {
                            .children([
                                html!("label", {
                                    .attr("for", "msg-ward-select")
                                    .class("form-label")
                                    .text("รับข่าวสารของหอผู้ป่วย")
                                }),
                                html!("div", {
                                    .class(class::FLEX_GROW1)
                                    .child(html!("select" => HtmlSelectElement, {
                                        .class(class::FORM_CTRL_SM)
                                        .attr("id", "msg-ward-select")
                                        .attr("multiple", "multiple")
                                        .child(html!("option", {.attr("value", "00").text("ER")}))
                                        .children(ward_select_option.iter().map(|option| {
                                            doms::select_option(option, "")
                                        }))
                                        .with_node!(element => {
                                            .event(clone!(app => move |_: events::Change| {
                                                let options = element.selected_options();
                                                let mut values = Vec::new();
                                                for j in 0..options.length() {
                                                    if let Some(item) = options.item(j) {
                                                        if let Ok(option) = item.dyn_into::<HtmlOptionElement>() {
                                                            values.push(option.value());
                                                        }
                                                    }
                                                }
                                                if let Some(user) = app.user.lock_ref().as_ref() {
                                                    user.user.wards.set(values);
                                                }
                                            }))
                                        })
                                    }))
                                }),
                                html!("div", {.class(class::FORM_TEXT_R).text("สามารถเลือกได้หลายหอผู้ป่วย")}),
                                html!("label", {
                                    .attr("for", "msg-splcty-select")
                                    .class("form-label")
                                    .text("รับข่าวสารของแผนก")
                                }),
                                html!("div", {
                                    .class(class::FLEX_GROW1)
                                    .child(html!("select" => HtmlSelectElement, {
                                        .class(class::FORM_CTRL_SM)
                                        .attr("id", "msg-spclty-select")
                                        .attr("multiple", "multiple")
                                        .child(html!("option", {.attr("value", "0").text("ฝ่ายเภสัชกรรม")}))
                                        .children(spclty_kphis_select_option.iter().map(|option| {
                                            doms::select_option(option, "")
                                        }))
                                        .with_node!(element => {
                                            .event(clone!(app => move |_: events::Change| {
                                                let options = element.selected_options();
                                                let mut values = Vec::new();
                                                for j in 0..options.length() {
                                                    if let Some(item) = options.item(j) {
                                                        if let Ok(option) = item.dyn_into::<HtmlOptionElement>() {
                                                            values.push(option.value());
                                                        }
                                                    }
                                                }
                                                if let Some(user) = app.user.lock_ref().as_ref() {
                                                    user.user.spclty_ids.set(values.iter().filter_map(|s| s.parse::<u32>().ok()).collect::<Vec<u32>>());
                                                }
                                            }))
                                        })
                                    }))
                                }),
                                html!("div", {.class(class::FORM_TEXT_R).text("สามารถเลือกได้หลายแผนก")}),
                                html!("div", {
                                    .child(html!("button" => HtmlButtonElement, {
                                        .attr("type", "button")
                                        .class(class::BTN_SM_FR_T_BLUEO)
                                        .child(html!("i", {.class(class::FA_SAVE)}))
                                        .text(" บันทึก")
                                        .apply(mixins::click_with_loader_checked(clone!(app => move || {
                                            app.save_msg_group();
                                        }), app.state()))
                                    }))
                                }),
                            ])
                        }),
                    ])
                } else { dom
                    .child(html!("div", {.text("ท่านไม่สามารถตั้งค่าการรับข้อความได้ กรุณาติดต่อผู้ดูแลระบบ")}))
                }
            })
        })
    }

    fn render_user_panel(menu: Rc<Self>, app: Rc<App>) -> Dom {
        match app.user.lock_ref().as_ref() {
            Some(user) => {
                html!("ul", {
                    .class("navbar-nav")
                    .child(html!("li", {
                        .class(class::NAV_ITEM_DROP)
                        .children([
                            html!("a", {
                                .attr("href","#")
                                .class(class::NAV_LINK_DROP_TGL_TW_PY)
                                .class(class::FLEX_C)
                                .attr("id", "userDropdownId")
                                .attr("data-bs-toggle","dropdown")
                                .attr("data-bs-auto-close","outside")
                                .attr("aria-haspopup","true")
                                .attr("aria-expanded","false")
                                .prop_signal("title", user.user.name.signal_cloned())
                                .child(html!("div", {
                                    .class(class::CIRCLE_L)
                                    .style("height","50px")
                                    .style("width","50px")
                                    .style("background-color","#eee")
                                    .style("overflow","hidden")
                                    .child(html!("img", {
                                        .attr("src", &user.user.image.lock_ref().as_ref().image)
                                        .attr("alt","User Image")
                                        .style("width","50px")
                                    }))
                                }))
                                .child(html!("span", {
                                    .class(class::RESP_XL_MD)
                                    .text_signal(user.user.name.signal_cloned())
                                }))
                            }),
                            html!("div", {
                                .class(class::DROP_MENU_END)
                                .style("min-width","300px")
                                .attr("aria-labelledby","userDropdownId")
                                .child(menu_header("ROLES"))
                                .children_signal_vec(user.roles.signal_vec_cloned().map(|role| {
                                    html!("span", {
                                        .class(class::BADGE_GOLD_RT)
                                        .style("cursor","default")
                                        .text_signal(role.role_desc.signal_cloned())
                                    })
                                }))
                                .children([
                                    html!("div", {.class("dropdown-divider")}),
                                    // theme changer
                                    html!("div", {
                                        .class(class::INPUT_GROUP_JC)
                                        .children([
                                            theme_btn("light", app.clone()),
                                            theme_btn("dark", app.clone()),
                                            theme_btn("auto", app.clone()),
                                        ])
                                    }),
                                    // wide screen changer
                                    html!("div", {
                                        .class(class::INPUT_GROUP_JC)
                                        .class("mt-1")
                                        .children([
                                            doms::span_group_text("Wide Screen"),
                                            wide_screen_btn("table", app.clone()),
                                            wide_screen_btn("card", app.clone()),
                                        ])
                                    }),
                                    // totp
                                    html!("div", {
                                        .class(class::BOX_ROUND_MT1_MX2)
                                        .child(html!("div", {
                                            .class(class::TXT_C_P2)
                                            .child_signal(user.user.totp_done.signal_cloned().map(|totp_done_opt| {
                                                if totp_done_opt.unwrap_or_default() {
                                                    Some(html!("i",{
                                                        .class(class::FA_CHECK_CIRCLE_GREEN)
                                                        .style("vertical-align","middle")
                                                        .style("font-size","32px")
                                                    }))
                                                } else {
                                                    Some(html!("i",{
                                                        .class(class::FA_X_CIRCLE_RED)
                                                        .style("vertical-align","middle")
                                                        .style("font-size","32px")
                                                    }))
                                                }
                                            }))
                                            .children([
                                                html!("span", {
                                                    .class("me-3")
                                                    .style("vertical-align","middle")
                                                    .text("ใช้งาน 2FA")
                                                }),
                                                html!("button" => HtmlButtonElement, {
                                                    .attr("type","button")
                                                    .class(class::BTN_BLUEO)
                                                    .text("สร้างใหม่")
                                                    .apply(mixins::click_with_loader_checked(clone!(app, menu => move || {
                                                        post_user_config(Some(true), Some(menu.totp_qr.clone()), app.clone());
                                                    }), app.state()))
                                                }),
                                            ])
                                            .child_signal(user.user.totp_done.signal_cloned().map(clone!(app, menu => move |totp_done_opt| {
                                                totp_done_opt.unwrap_or_default().then(|| {
                                                    html!("button" => HtmlButtonElement, {
                                                        .attr("type","button")
                                                        .class(class::BTN_R_REDO)
                                                        .attr("title","ยกเลิกการใช้งาน 2FA")
                                                        .child(html!("i", {.class(class::FA_X)}))
                                                        .apply(mixins::click_with_loader_checked(clone!(app, menu => move || {
                                                            post_user_config(Some(false), Some(menu.totp_qr.clone()), app.clone());
                                                        }), app.state()))
                                                    })
                                                })
                                            })))
                                        }))
                                        .child_signal(menu.totp_qr.signal_cloned().map(clone!(app, menu => move |qr_code| {
                                            (!qr_code.is_empty()).then(|| {
                                                let submit = Mutable::new(false);
                                                let pincode = PinCode::new(menu.token_2fa.clone(), submit.clone());
                                                html!("div", {
                                                    .children([
                                                        html!("img", {
                                                            .attr("src", &["data:image/jpeg;base64,", &qr_code].concat())
                                                        }),
                                                        html!("div", {
                                                            .class(class::TXT_CT)
                                                            .text("Scan ด้วย Authenticator Application")
                                                            .child(html!("br"))
                                                            .text("เช่น Google Authenticator เพื่อสร้าง Token")
                                                            .child(html!("br"))
                                                            .text("สำหรับเข้าสู่ระบบทุกครั้ง")
                                                        }),
                                                        html!("div", {
                                                            .future(map_ref!(
                                                                let busy = app.loader_is_loading(),
                                                                let submit = submit.signal() =>
                                                                !busy && *submit
                                                            ).for_each(clone!(menu, app => move |ready| {
                                                                if ready {
                                                                    Self::submit_2fa(menu.clone(), app.clone());
                                                                }
                                                                async {}
                                                            })))
                                                            .child(PinCode::render(pincode))
                                                        }),
                                                    ])
                                                })
                                            })
                                        })))
                                        .child_signal(menu.token_2fa_result.signal_cloned().map(|result| {
                                            (!result.is_empty()).then(|| {
                                                html!("div", {
                                                    .class(class::BOLD_RED_L)
                                                    .class("text-center")
                                                    .text(&result)
                                                })
                                            })
                                        }))
                                    }),
                                    html!("div", {.class("dropdown-divider")}),
                                    // cache control changer
                                    html!("div", {
                                        .class(class::INPUT_GROUP_JC)
                                        .class("mt-1")
                                        .children([
                                            doms::span_group_text("App Data"),
                                            cache_control_btn(true, app.clone()),
                                            cache_control_btn(false, app.clone()),
                                        ])
                                    }),
                                    html!("div", {.class("dropdown-divider")}),
                                    // try update Service Worker
                                    html!("a", {
                                        .class("dropdown-item")
                                        .attr("href","#")
                                        .child(html!("i", {.class(class::FA_SEARCH)}))
                                        .text(" Update")
                                        .event_with_options(&EventOptions::preventable(), clone!(app => move |event: events::Click| {
                                            event.prevent_default();
                                            app.update_sw();
                                        }))
                                    }),
                                    // reload Service Worker
                                    html!("a", {
                                        .class("dropdown-item")
                                        .attr("href","#")
                                        .child(html!("i", {.class(class::FA_UNDO)}))
                                        .text(" Clear Caches and Reload")
                                        .event_with_options(&EventOptions::preventable(), clone!(menu => move |event: events::Click| {
                                            event.prevent_default();
                                            menu.clear_cache.set(true);
                                        }))
                                    }),
                                    html!("div", {.class("dropdown-divider")}),
                                    // logout
                                    html!("a", {
                                        .class("dropdown-item")
                                        .attr("href","#")
                                        .child(html!("i", {.class(class::FA_SIGN_OUT)}))
                                        .text(" ออกจากระบบ")
                                        .event_with_options(&EventOptions::preventable(), clone!(menu => move |event: events::Click| {
                                            event.prevent_default();
                                            menu.logout.set(true);
                                        }))
                                    }),
                                ])
                            })
                        ])
                    }))
                })
            }
            None => Dom::empty(),
        }
    }

    fn render_confirm_order_as_modal(menu: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .class("modal")
            .class("text-dark") // fix menu's `text-light`
            .attr("id", "checkOrderAsModal")
            .attr("tabindex", "-1")
            .children([
                html!("style", { .text(ORDER_STYLE)}),
                html!("div", {
                    .class(class::MODAL_DIALOG_XL)
                    .child(html!("div", {
                        .class("modal-content")
                        .children([
                            html!("div", {
                                .class("modal-header")
                                .children([
                                    html!("h5", {
                                        .class("modal-title")
                                        .text("ยืนยันคำสั่งแพทย์ (รคส)")
                                    }),
                                    doms::close_modal_x_btn(),
                                ])
                            }),
                            html!("div", {
                                .class(class::MODAL_BODY_P2)
                                .child_signal(app.ipd_order_as.signal_vec_cloned().len().map(clone!(app, menu => move |len| {
                                    (len > 0).then(|| {
                                        html!("div", {
                                            .class(class::ROW_COL_RESP2_XL_G2)
                                            .class("mb-2")
                                            .children_signal_vec(app.ipd_order_as.signal_vec_cloned().map(clone!(app, menu => move |order| {
                                                html!("div", {
                                                    .class("col")
                                                    .child(Self::render_order(order, menu.clone(), app.clone()))
                                                })
                                            })))
                                        })
                                    })
                                })))
                                .child_signal(app.opd_er_order_as.signal_vec_cloned().len().map(clone!(app, menu => move |len| {
                                    (len > 0).then(|| {
                                        html!("div", {
                                            .class(class::ROW_COL_RESP2_XL_G2)
                                            .children_signal_vec(app.opd_er_order_as.signal_vec_cloned().map(clone!(app, menu => move |order| {
                                                html!("div", {
                                                    .class("col")
                                                    .child(Self::render_order(order, menu.clone(), app.clone()))
                                                })
                                            })))
                                        })
                                    })
                                })))
                                .child_signal(app.has_any_order_as().map(|any_as| {
                                    (!any_as).then(|| {
                                        html!("div", {
                                            .class(class::BOX_ROUND_DARKS_BOLD_R_PX3)
                                            .class("fw-bold")
                                            .text("ไม่มีคำสั่ง รคส.")
                                        })
                                    })
                                }))
                            }),
                            html!("div", {
                                .class("modal-footer")
                                .child_signal(app.has_any_order_as().map(clone!(app, menu => move |any_as| {
                                    any_as.then(|| {
                                        html!("button", {
                                            .attr("type", "button")
                                            .class(class::BTN_L_BLUE)
                                            .text("ยืนยันทั้งหมด")
                                            .event(clone!(app, menu => move |_: events::Click| {
                                                Self::patch_order_all(menu.clone(), app.clone());
                                            }))
                                        })
                                    })
                                })))
                                .child(html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_GRAY)
                                    .attr("data-bs-dismiss", "modal")
                                    .text("Close")
                                }))
                            })
                        ])
                    }))
                })
            ])
        })
    }

    fn render_order(order: Rc<Order>, menu: Rc<Self>, app: Rc<App>) -> Dom {
        let owner_class = if order.is_by_doctor() { "bg-primary" } else { "text-bg-warning" };
        let type_class = if order.is_oneday() { "bg-info" } else { "bg-primary" };
        let is_ipd = order.visit_type.is_ipd();

        html!("div", {
            .class(class::BOX_ROUND)
            .class("d-flex")
            .children([
                html!("div", {
                    .class(class::FLEX_COL)
                    .class("small")
                    .style("width","90px")
                    .apply(|dom| {
                        if is_ipd {
                            dom.child(html!("span", {
                                .class(class::BADGE_TRUNC_BLUE)
                                .style("cursor","default")
                                .style("font-size","100%")
                                .style("width","90px")
                                .text(&[order.bedno.clone().unwrap_or_default(), order.ward_name.clone().unwrap_or_default()].join(" "))
                            }))
                        } else {
                            dom.child(html!("div", {
                                .class(class::BADGE_TB_C)
                                .style("cursor","default")
                                .style("font-size","100%")
                                .style("width","90px")
                                .apply(|dom| {
                                    if let Some(bed_type_color) = &order.bed_type_color {
                                        dom.style("background-color", bed_type_color)
                                    } else {
                                        dom
                                    }
                                })
                                .text(&[order.bed_type_name.clone().unwrap_or_default(), order.display_bedno.clone().unwrap_or_default()].join(" "))
                            }))
                        }
                    })
                    .children([
                        html!("div", {
                            .child(doms::patient_image(&order.hn, "90px"))
                        }),
                        html!("span", {
                            .class("text-truncate")
                            .text(&order.fullname.clone().unwrap_or_default())
                        }),
                        html!("span", {
                            .class("text-truncate")
                            .text("HN: ")
                            .text(&order.hn.clone().unwrap_or_default())
                        }),
                    ])
                    .apply(|dom| {
                        match &order.visit_type {
                            VisitTypeId::Ipd(an) | VisitTypeId::PreAdmit(an) => {
                                dom.child(html!("span", {
                                    .class("text-truncate")
                                    .text("AN: ")
                                    .text(an)
                                }))
                            }
                            VisitTypeId::OpdEr(vn, _) | VisitTypeId::Visit(vn) => {
                                dom.child(html!("span", {
                                    .class("text-truncate")
                                    .text("VN: ")
                                    .text(vn)
                                }))
                            }
                        }
                    })
                }),
                html!("div", {
                    .class(class::FLEX_ITEM_FILL_R)
                    .children([
                        html!("div", {
                            .class("text-end")
                            .children([
                                html!("span", {
                                    .text(&[date_th(&order.order_date), time_hm(&order.order_time)].join(" "))
                                }),
                                html!("span", {
                                    .class(class::BADGE_R)
                                    .class(owner_class)
                                    .style("cursor","default")
                                    .text(match order.order_owner_type.as_str() {
                                        "doctor" => "Doctor",
                                        "nurse" => "Nurse",
                                        _ => "",
                                    })
                                }),
                                html!("span", {
                                    .class(class::BADGE_R)
                                    .class(type_class)
                                    .style("cursor","default")
                                    .text(match order.order_type.as_str() {
                                        "oneday" => "One Day",
                                        "continuous" => "Continuous",
                                        _ => "",
                                    })
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::BORDER_T_Y)
                            .children({
                                let mut children = Vec::with_capacity(2);
                                order.order_item_types.iter().for_each(|order_item_type| {
                                    // show order item type name
                                    if matches!(order_item_type.order_item_type, OrderTypeName::HomeMedication | OrderTypeName::Discharge) {
                                        children.push(html!("div", {
                                            .class("fw-bold")
                                            .text(order_item_type.order_item_type.string())
                                        }));
                                    }
                                    let lis = order_item_type.order_items.iter().map(clone!(app => move |order_item| {
                                        Self::render_order_item(order_item, app.clone())
                                    })).collect::<Vec<Dom>>();
                                    let ul = html!("ul", {
                                        .class("dash")
                                        .style("white-space","pre-wrap")
                                        .children(lis)
                                    });
                                    children.push(ul);
                                });
                                children
                            })
                        }),
                        // Order by
                        html!("div", {
                            .class(class::SMALL_R)
                            .apply_if(order.order_doctor_is_intern.unwrap_or_default(), |dom| dom.child(html!("span", {.text("(Intern) ")})))
                            .children([
                                html!("span", {.text(&[&order.order_doctor_name.clone().unwrap_or_default(), ", "].concat())}),
                                html!("span", {
                                    .class("text-nowrap")
                                    .text(&[date_th(&order.order_date), time_hm(&order.order_time)].join(" "))
                                })
                            ])
                        }),
                    ])
                    .apply(|dom| {
                        if let Some(nurse_order_as_name) = &order.nurse_order_as_name {
                            dom.child(html!("div", {
                                .class(class::SMALL_R)
                                .children([
                                    html!("span", {.text(&["รคส.", if order.nurse_order_as_is_intern.unwrap_or_default() {"(Intern) "} else {""}, nurse_order_as_name].concat())}),
                                    html!("span", {
                                        .class("text-nowrap")
                                        .apply(|d| {
                                            if let Some(doctor_confirm_time) = &order.doctor_confirm_time {
                                                d.text(&[" (ยืนยัน ", &datetime_th(doctor_confirm_time), ")"].concat())
                                            } else {
                                                d.text(" (รอแพทย์ยืนยัน)")
                                            }
                                        })
                                    }),
                                ])
                            }))
                        } else {
                            dom
                        }
                    })
                    .child(html!("div", {
                        .class(class::BOLD_R)
                        .child(html!("button", {
                            .attr("type", "button")
                            .attr("data-bs-dismiss", "modal")
                            .class(class::BTN_SM_RB_GOLD)
                            .child(html!("i",{.class(class::FA_L_ARROW_CIRCLE)}))
                            .text(" ไปยังคำสั่ง")
                            .event(clone!(app, order => move |_: events::Click| {
                                match &order.visit_type {
                                    VisitTypeId::Ipd(an)
                                    | VisitTypeId::PreAdmit(an) => {
                                        let route = Route::IpdMain {
                                            view_by: String::from("doctor"),
                                            an: an.to_owned(),
                                            tab: Tab::Order.str().to_owned(),
                                            sub: order.order_date.to_string(),
                                            id: order.order_id,
                                        };
                                        if route.has_permission(app.state()) {
                                            dominator::routing::go_to_url(&route.string());
                                        } else {
                                            app.alert_error("ไม่มีสิทธิ์", "ท่านไม่สิทธิ์ใช้งานคำสั่งแพทย์");
                                        }
                                    }
                                    VisitTypeId::OpdEr(_, opd_er_order_master_id) => {
                                        let route = Route::OpdErMain {
                                            view_by: String::from("doctor"),
                                            opd_er_order_master_id: *opd_er_order_master_id,
                                            tab: Tab::Order.str().to_owned(),
                                            id: order.order_id,
                                        };
                                        if route.has_permission(app.state()) {
                                            dominator::routing::go_to_url(&route.string());
                                        } else {
                                            app.alert_error("ไม่มีสิทธิ์", "ท่านไม่สิทธิ์ใช้งานคำสั่งแพทย์");
                                        }
                                    }
                                    VisitTypeId::Visit(_) => {}
                                }
                            }))
                        }))
                        .child(html!("button" => HtmlButtonElement, {
                            .attr("type", "button")
                            .class(class::BTN_SM_RB_BLUE)
                            .text("แพทย์ยืนยัน รคส")
                            .apply(mixins::click_with_loader_checked(clone!(app, menu => move || {
                                if is_ipd {
                                    Self::patch_order_ipd(order.order_id, menu.clone(), app.clone());
                                } else {
                                    Self::patch_order_opd_er(order.order_id, menu.clone(), app.clone());
                                }
                            }), app.state()))
                        }))
                    }))
                }),
            ])
        })
    }

    fn render_order_item(order_item: &OrderItem, app: Rc<App>) -> Dom {
        let will_blue = ["med", "home-medication", "injection", "ivfluid"];

        html!("li", {
            .class("clearfix")
            .apply(|dom| {
                // OFF
                if order_item.order_item_type == Some(String::from("off")) { dom
                    .child(html!("span", {.class(class::BADGE_GOLD_L).text("OFF").style("cursor","default")}))
                    .child(html!("span", {
                        .child(html!("span", {
                            .text(&order_item.off_med_name.clone().unwrap_or_default())
                            .apply_if(order_item.off_icode.is_some(), |d| d.class(class::BOLD_BLUE_EM_L).text("\n"))
                        }))
                        .text(&order_item.off_order_item_detail.clone().unwrap_or_default())
                    }))
                // NOT OFF
                } else { dom
                    .child(html!("span", {
                        .child(html!("span", {
                            .apply_if(order_item.order_item_type.as_ref().map(|ty| will_blue.contains(&ty.as_str())).unwrap_or_default(), |d| d.class(class::BOLD_BLUE_EM_L).style("cursor","default"))
                            .text(&order_item.med_name.clone().unwrap_or_default())
                        }))
                        // Drug allergy badge
                        .apply_if(order_item.allergy_agent_symptom.is_some(), |d| d.child(html!("div", {
                            .class(class::BADGE_WRAP_R_RED)
                            .style("cursor","help")
                            .attr("title", &order_item.allergy_agent_symptom.clone().unwrap_or(String::from("ไม่ระบุอาการ")))
                            .text("แพ้ยา/เฝ้าระวัง")
                        })))
                        // HAD/LASA badge
                        .children(app.drug_alert_badge(order_item.displaycolor))
                        // Med Reconcile badge
                        .apply_if(order_item.med_reconciliation_item_id.is_some(), |d| d.child(html!("div", {
                            .style("cursor","help")
                            .apply(|dd| {
                                match &order_item.used {
                                    Some(used) => {
                                        match used.as_str() {
                                            "N" => dd.class(class::BADGE_WRAP_R_GRAY),
                                            "H" => dd.class(class::BADGE_WRAP_R_CYAN),
                                            "Y" => {
                                                if order_item.is_med_rec_change_usage() {
                                                    dd.class(class::BADGE_WRAP_R_GOLD)
                                                } else {
                                                    dd.class(class::BADGE_WRAP_R_GREEN)
                                                }
                                            }
                                            _ => dd,
                                        }
                                    }
                                    None => dd,
                                }

                            })
                            .attr("title", &order_item.med_rec_info())
                            .text("MR")
                        })))
                        .apply_if(order_item.icode.is_some(), |d| d.child(html!("br")))
                        .text(&order_item.order_item_detail.clone().unwrap_or_default())
                        .apply_if(order_item.off_by_datetime.is_some(), |d| d.style("text-decoration","line-through"))
                    }))
                    .apply_if(order_item.stat == Some(String::from("Y")), |dom| dom.child(html!("span", {
                        .class(class::BADGE_WRAP_R_RED)
                        .style("cursor","default")
                        .text("STAT")
                    })))
                    .apply_if(order_item.off_by_datetime.is_some(), |dom| dom.child(html!("span", {
                        .class(class::BADGE_WRAP_R_GOLD)
                        .style("cursor","default")
                        .text(&["OFF ", &datetime_th_opt(&order_item.off_by_datetime)].concat())
                    })))
                }
            })
        })
    }
}

fn post_user_config(totp: Option<bool>, totp_qr_opt: Option<Mutable<String>>, app: Rc<App>) {
    if let Some(user) = app.user.get_cloned() {
        app.async_load(
            true,
            clone!(app => async move {
                let user_config = UserConfig {
                    theme: str_some(user.user.theme.get_cloned()),
                    wide_screen: str_some(user.user.wide_screen.get_cloned()),
                    totp,
                };
                // POST `EndPoint::UserConfig`
                match UserConfigResponse::call_api_post(&user_config, app.state()).await {
                    Ok(response) => {
                        if let Some(totp_qr) = totp_qr_opt {
                            if let Some(totp) = response.totp {
                                totp_qr.set_neq(totp);
                            } else {
                                totp_qr.set_neq(String::new());
                            }
                            user.user.totp_done.set(None);
                        }
                    }
                    Err(e) => app.alert_app_error(&e).await,
                }
            }),
        )
    }
}

fn menu_items(app: Rc<App>) -> Dom {
    // let is_ipd_doctor = app.has_permission(Permission::IpdDoctorMainProgramAccess);
    // let is_opd_er_doctor = app.has_permission(Permission::OpdErDoctorProgramAccess);
    // let is_ipd_nurse = app.has_permission(Permission::IpdNurseMainProgramAccess);
    // let is_opd_er_nurse = app.has_permission(Permission::OpdErNurseProgramAccess);
    // let is_ipd_pharm = app.has_permission(Permission::IpdPharmacyOrderMainProgramAccess);
    // let is_opd_er_pharm = app.has_permission(Permission::OpdErPharmacyOrderProgramAccess);
    // let is_ipd_other = app.has_permission(Permission::IpdOtherOrderMainProgramAccess);
    // let is_opd_er_other = app.has_permission(Permission::OpdErOtherOrderProgramAccess);

    let ipd_doctor_children = vec![
        link_checked(Route::IpdSearchPatientDr, html!("i", {.class(class::FA_BED).class("ms-2")}), " รายการผู้ป่วยใน", app.state()),
        link_checked(
            Route::IpdPreAdmitList { view_by: String::from("doctor") },
            html!("i", {.class(class::FA_CLOCK).class("ms-2")}),
            " รอ Admit",
            app.state(),
        ),
        link_checked(
            Route::IpdPostAdmitList { view_by: String::from("doctor") },
            html!("i", {.class(class::FA_LIST_CHECK).class("ms-2")}),
            " สรุป Chart",
            app.state(),
        ),
        link_checked(
            Route::IpdPreOrderList { view_by: String::from("doctor") },
            html!("i", {.class(class::FA_PASTE).class("ms-2")}),
            " Order ล่วงหน้า",
            app.state(),
        ),
        link_checked(
            Route::IpdConsultList { view_by: String::from("doctor") },
            html!("i", {.class(class::FA_COMMENTS).class("ms-2")}),
            " รายการผู้ป่วย Consult",
            app.state(),
        ),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<Dom>>();

    let opd_er_doctor_child = link_checked(
        Route::OpdErOrderList { view_by: String::from("doctor") },
        html!("i", {.class(class::FA_USER_INJURED).class("ms-2")}),
        " รายการผู้ป่วย ER",
        app.state(),
    );

    let ipd_nurse_children = vec![
        link_checked(Route::IpdSearchPatientNurse, html!("i", {.class(class::FA_BED).class("ms-2")}), " รายการผู้ป่วยใน", app.state()),
        link_checked(
            Route::IpdPreAdmitList { view_by: String::from("nurse") },
            html!("i", {.class(class::FA_CLOCK).class("ms-2")}),
            " รอ Admit",
            app.state(),
        ),
        link_checked(
            Route::IpdPostAdmitList { view_by: String::from("nurse") },
            html!("i", {.class(class::FA_LIST_CHECK).class("ms-2")}),
            " Audit Chart",
            app.state(),
        ),
        link_checked(Route::IpdVitalSign, html!("i", {.class(class::FA_HEARTBEAT).class("ms-2")}), " IPD Vital Sign", app.state()),
        link_checked(Route::IpdIndexPlan, html!("i", {.class(class::FA_SYRINGE).class("ms-2")}), " IPD Nurse Planning", app.state()),
        link_checked(
            Route::IpdPreOrderList { view_by: String::from("nurse") },
            html!("i", {.class(class::FA_PASTE).class("ms-2")}),
            " Order ล่วงหน้า",
            app.state(),
        ),
        link_checked(
            Route::IpdConsultList { view_by: String::from("nurse") },
            html!("i", {.class(class::FA_COMMENTS).class("ms-2")}),
            " รายการผู้ป่วย Consult",
            app.state(),
        ),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<Dom>>();

    let opd_er_nurse_children = vec![
        link_checked(
            Route::OpdErOrderList { view_by: String::from("nurse") },
            html!("i", {.class(class::FA_USER_INJURED).class("ms-2")}),
            " รายการผู้ป่วย ER",
            app.state(),
        ),
        link_checked(Route::OpdErVitalSign, html!("i", {.class(class::FA_HEARTBEAT).class("ms-2")}), " ER Vital Sign", app.state()),
        link_checked(Route::OpdErIndexPlan, html!("i", {.class(class::FA_SYRINGE).class("ms-2")}), " ER Nurse Planning", app.state()),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<Dom>>();

    let prescription_screen_child = link_checked(
        Route::PrescriptionScreen { hn: String::new() },
        html!("i", {.class(class::FA_FILE_RX).class("ms-2")}),
        " Screen ใบสั่งยา",
        app.state(),
    );

    let ipd_pharm_children = vec![
        link_checked(Route::IpdOrderPharmacy, html!("i", {.class(class::FA_USER_CLOCK).class("ms-2")}), " IPD Order", app.state()),
        link_checked(Route::IpdSearchPatientPharmacist, html!("i", {.class(class::FA_BED).class("ms-2")}), " รายการผู้ป่วยใน", app.state()),
        link_checked(
            Route::IpdPreAdmitList { view_by: String::from("pharmacist") },
            html!("i", {.class(class::FA_CLOCK).class("ms-2")}),
            " รอ Admit",
            app.state(),
        ),
        link_checked(
            Route::IpdPostAdmitList { view_by: String::from("pharmacist") },
            html!("i", {.class(class::FA_LIST_CHECK).class("ms-2")}),
            " Audit Chart",
            app.state(),
        ),
        link_checked(
            Route::IpdPreOrderList { view_by: String::from("pharmacist") },
            html!("i", {.class(class::FA_PASTE).class("ms-2")}),
            " Order ล่วงหน้า",
            app.state(),
        ),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<Dom>>();

    let opd_er_pharm_children = vec![
        link_checked(Route::OpdErOrderPharmacy, html!("i", {.class(class::FA_USER_CLOCK).class("ms-2")}), " ER Order", app.state()),
        link_checked(
            Route::OpdErOrderList { view_by: String::from("pharmacist") },
            html!("i", {.class(class::FA_USER_INJURED).class("ms-2")}),
            " รายการผู้ป่วย ER",
            app.state(),
        ),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<Dom>>();

    let ipd_other_children = vec![
        link_checked(Route::IpdSearchPatientOther, html!("i", {.class(class::FA_BED).class("ms-2")}), " รายการผู้ป่วยใน", app.state()),
        link_checked(
            Route::IpdPreAdmitList { view_by: String::from("other") },
            html!("i", {.class(class::FA_CLOCK).class("ms-2")}),
            " รอ Admit",
            app.state(),
        ),
        link_checked(
            Route::IpdPostAdmitList { view_by: String::from("other") },
            html!("i", {.class(class::FA_LIST_CHECK).class("ms-2")}),
            " Audit Chart",
            app.state(),
        ),
        link_checked(
            Route::IpdPreOrderList { view_by: String::from("other") },
            html!("i", {.class(class::FA_PASTE).class("ms-2")}),
            " Order ล่วงหน้า",
            app.state(),
        ),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<Dom>>();

    let opd_er_other_child = link_checked(
        Route::OpdErOrderList { view_by: String::from("other") },
        html!("i", {.class(class::FA_USER_INJURED).class("ms-2")}),
        " รายการผู้ป่วย ER",
        app.state(),
    );

    let setting_children = vec![
        link_checked(Route::DrugUseDuration, html!("i", {.class(class::FA_PILLS)}), " Drug Information", app.state()),
        link_checked(Route::SettingTemplateNurseNote, html!("i", {.class(class::FA_NOTE_MED)}), " Template Nurse Note", app.state()),
        link_checked(Route::SettingTemplateDcPlan, html!("i", {.class(class::FA_NOTE_MED)}), " Template D/C Plan", app.state()),
        link_checked(Route::UserList, html!("i", {.class(class::FA_USER_LOCK)}), " จัดการผู้ใช้งาน", app.state()),
        link_checked(Route::PermissionList, html!("i", {.class(class::FA_USER_SHIELD)}), " จัดการบทบาท", app.state()),
        link_checked(Route::ReportViewer, html!("i", {.class(class::FA_FILE_PDF)}), " Report Viewer", app.state()),
        link_checked(Route::ReportDesigner, html!("i", {.class(class::FA_EDIT)}), " Report Designer", app.state()),
        link_checked(Route::Image, html!("i", {.class(class::FA_IMAGE)}), " Image Cache", app.state()),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<Dom>>();

    html!("ul", {
        .class(class::NAV_BAR_NAV_LX)
        .apply(|dom| {
            if ipd_doctor_children.is_empty() && opd_er_doctor_child.is_none() {
                dom
            } else {
                dom.child(html!("li", {
                    .class(class::NAV_ITEM_DROP)
                    .children([
                        html!("a", {
                            .attr("href","#")
                            .class(class::NAV_LINK_DROP_TGL_TW_PY)
                            .attr("id", "drDropdownId")
                            .attr("data-bs-toggle","dropdown")
                            .attr("aria-haspopup","true")
                            .attr("aria-expanded","false")
                            .child(html!("em", {.class(class::FA_USER_MD_L)}))
                            .child(html!("span", {
                                .class(class::RESP_LG_SM)
                                .text(" แพทย์")
                            }))
                            .event_with_options(&EventOptions::preventable(), |event: events::Click| {
                                event.prevent_default();
                            })
                        }),
                        html!("div", {
                            .class("dropdown-menu")
                            .attr("aria-labelledby","drDropdownId")
                            .apply_if(!ipd_doctor_children.is_empty(), |d| d
                                .child(menu_header("IPD"))
                                .children(ipd_doctor_children)
                            )
                            .apply(|d| {
                                if let Some(child) = opd_er_doctor_child {
                                    d.child(menu_header("OPD-ER"))
                                    .child(child)
                                } else {
                                    d
                                }
                            })
                        }),
                    ])
                }))
            }
        })
        .apply(|dom| {
            if ipd_nurse_children.is_empty() && opd_er_nurse_children.is_empty() {
                dom
            } else {
                dom.child(html!("li", {
                    .class(class::NAV_ITEM_DROP)
                    .children([
                        html!("a", {
                            .attr("href","#")
                            .class(class::NAV_LINK_DROP_TGL_TW_PY)
                            .attr("id", "nurseDropdownId")
                            .attr("data-bs-toggle","dropdown")
                            .attr("aria-haspopup","true")
                            .attr("aria-expanded","false")
                            .child(html!("em", {.class(class::FA_USER_NURSE_L)}))
                            .child(html!("span", {
                                .class(class::RESP_LG_SM)
                                .text(" พยาบาล")
                            }))
                            .event_with_options(&EventOptions::preventable(), |event: events::Click| {
                                event.prevent_default();
                            })
                        }),
                        html!("div", {
                            .class("dropdown-menu")
                            .attr("aria-labelledby","nurseDropdownId")
                            .apply_if(!ipd_nurse_children.is_empty(), |d| d
                                .child(menu_header("IPD"))
                                .children(ipd_nurse_children)
                            )
                            .apply_if(!opd_er_nurse_children.is_empty(), |d| d
                                .child(menu_header("OPD-ER"))
                                .children(opd_er_nurse_children)
                            )
                        }),
                    ])
                }))
            }
        })
        .apply(|dom| {
            if prescription_screen_child.is_none() && ipd_pharm_children.is_empty() && opd_er_pharm_children.is_empty() {
                dom
            } else {
                dom.child(html!("li", {
                    .class(class::NAV_ITEM_DROP)
                    .children([
                        html!("a", {
                            .attr("href","#")
                            .class(class::NAV_LINK_DROP_TGL_TW_PY)
                            .attr("id", "pharmacyDropdownId")
                            .attr("data-bs-toggle","dropdown")
                            .attr("aria-haspopup","true")
                            .attr("aria-expanded","false")
                            .child(html!("em", {.class(class::FA_RX_L)}))
                            .child(html!("span", {
                                .class(class::RESP_LG_SM)
                                .text(" เภสัชกร")
                            }))
                            .event_with_options(&EventOptions::preventable(), |event: events::Click| {
                                event.prevent_default();
                            })
                        }),
                        html!("div", {
                            .class("dropdown-menu")
                            .attr("aria-labelledby","pharmacyDropdownId")
                            .apply(|d| {
                                if let Some(child) = prescription_screen_child {
                                    d.child(child)
                                } else {
                                    d
                                }
                            })
                            .apply_if(!ipd_pharm_children.is_empty(), |d| d
                                .child(menu_header("IPD"))
                                .children(ipd_pharm_children)
                            )
                            .apply_if(!opd_er_pharm_children.is_empty(), |d| d
                                .child(menu_header("OPD-ER"))
                                .children(opd_er_pharm_children)
                            )
                        }),
                    ])
                }))
            }
        })
        .apply(|dom| {
            if ipd_other_children.is_empty() && opd_er_other_child.is_none() {
                dom
            } else {
                dom.child(html!("li", {
                    .class(class::NAV_ITEM_DROP)
                    .children([
                        html!("a", {
                            .attr("href","#")
                            .class(class::NAV_LINK_DROP_TGL_TW_PY)
                            .attr("id", "otherDropdownId")
                            .attr("data-bs-toggle","dropdown")
                            .attr("aria-haspopup","true")
                            .attr("aria-expanded","false")
                            .child(html!("em", {.class(class::FA_USER_TIE_L)}))
                            .child(html!("span", {
                                .class(class::RESP_LG_SM)
                                .text(" อื่นๆ")
                            }))
                            .event_with_options(&EventOptions::preventable(), |event: events::Click| {
                                event.prevent_default();
                            })
                        }),
                        html!("div", {
                            .class("dropdown-menu")
                            .attr("aria-labelledby","otherDropdownId")
                            .apply_if(!ipd_other_children.is_empty(), |d| d
                                .child(menu_header("IPD"))
                                .children(ipd_other_children)
                            )
                            .apply(|d| {
                                if let Some(child) = opd_er_other_child {
                                    d.child(menu_header("OPD-ER"))
                                    .child(child)
                                } else {
                                    d
                                }
                            })
                        }),
                    ])
                }))
            }
        })
        .apply(|dom| {
            if setting_children.is_empty() {
                dom
            } else {
                dom.child(html!("li", {
                    .class(class::NAV_ITEM_DROP)
                    .children([
                        html!("a", {
                            .attr("href","#")
                            .class(class::NAV_LINK_DROP_TGL_TW_PY)
                            .attr("id", "settingDropdownId")
                            .attr("data-bs-toggle","dropdown")
                            .attr("aria-haspopup","true")
                            .attr("aria-expanded","false")
                            .child(html!("em", {.class(class::FA_COG_L)}))
                            .child(html!("span", {
                                .class(class::RESP_LG_SM)
                                .text(" Setting")
                            }))
                            .event_with_options(&EventOptions::preventable(), |event: events::Click| {
                                event.prevent_default();
                            })
                        }),
                        html!("div", {
                            .class("dropdown-menu")
                            .attr("aria-labelledby","settingDropdownId")
                            .children(setting_children)
                        })
                    ])
                }))
            }
        })
    })
}

fn link_checked(route: Route, icon: Dom, label: &str, state: Rc<AppState>) -> Option<Dom> {
    if route.has_permission(state) {
        Some(link!(route.string(), {
            .class("dropdown-item")
            .child(icon)
            .text(label)
        }))
    } else {
        None
    }
}

fn menu_header(text: &str) -> Dom {
    html!("div", {.class(class::DROP_HEAD_BOLD).text(text)})
}

fn theme_btn(theme: &'static str, app: Rc<App>) -> Dom {
    let (icon, label) = match theme {
        "light" => (class::FA_SUN, " Light"),
        "dark" => (class::FA_MOON, " Dark"),
        _ => (class::FA_MAGIC, " Auto"),
    };

    html!("button" => HtmlButtonElement, {
        .class(class::BTN_BLUEO)
        .apply(|dom| {
            if let Some(user) = app.user.lock_ref().as_ref() {
                dom.class_signal("active", user.user.theme.signal_ref(move |t| t == theme))
            } else {
                dom
            }
        })
        .attr("type", "button")
        .attr("data-bs-toggle", "button")
        .child(html!("i", {.class(icon)}))
        .text(label)
        .apply(mixins::click_with_loader_checked(clone!(app => move || {
            post_user_config(None, None, app.clone());
            app.set_theme(theme);
        }), app.state()))
    })
}

fn wide_screen_btn(wide_screen_mode: &'static str, app: Rc<App>) -> Dom {
    let (icon, label) = match wide_screen_mode {
        "card" => (class::FA_CARD, " Card"),
        _ => (class::FA_TABLE, " Table"),
    };
    html!("button" => HtmlButtonElement, {
        .class(class::BTN_BLUEO)
        .apply(|dom| {
            if let Some(user) = app.user.lock_ref().as_ref() {
                dom.class_signal("active", user.user.wide_screen.signal_ref(move |t| t == wide_screen_mode))
            } else {
                dom
            }
        })
        .attr("type", "button")
        .attr("data-bs-toggle", "button")
        .child(html!("i", {.class(icon)}))
        .text(label)
        .apply(mixins::click_with_loader_checked(clone!(app => move || {
            post_user_config(None, None, app.clone());
            app.set_wide_screen_mode(wide_screen_mode);
        }), app.state()))
    })
}

fn cache_control_btn(no_cache_mode: bool, app: Rc<App>) -> Dom {
    let (icon, label) = if no_cache_mode { (class::FA_NETWORK, " Network") } else { (class::FA_DISPLAY, " Local") };
    html!("button", {
        .class(class::BTN_BLUEO)
        .class_signal("active", app.no_cache_mode.signal_cloned().map(move |b| b == no_cache_mode))
        .attr("type", "button")
        .attr("data-bs-toggle", "button")
        .child(html!("i", {.class(icon)}))
        .text(label)
        .event(move |_: events::Click| {
            app.no_cache_mode.set(no_cache_mode);
         })
    })
}
