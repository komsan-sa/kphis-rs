use dominator::{Dom, clone, html, routing};
use futures_signals::{map_ref, signal::SignalExt, signal_vec::SignalVecExt};
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::js_sys;

use kphis_model::{app::AppAsset, route::Route, tab::Tab};
use kphis_ui_app::App;
use kphis_ui_component::menu::MenuCpn;
use kphis_ui_core::token;
use kphis_ui_page::{
    drug_use_duration::DrugUseDurationPage, image::ImagePage, index::IndexPage, index_plan::IndexPlanPage, info::InfoPage, ipd_admission_note_dr::IpdAdmissionNoteDrPage,
    ipd_admission_note_nurse::IpdAdmissionNoteNursePage, ipd_consult_list::IpdConsultListPage, ipd_main::IpdMainPage, ipd_mra::IpdMraPage, ipd_order_pharmacy::IpdOrderPharmacyPage,
    ipd_post_admit_list::IpdPostAdmitListPage, ipd_pre_admit_list::IpdPreAdmitListPage, ipd_pre_order_list::IpdPreOrderListPage, ipd_pre_order_main::IpdPreOrderPage,
    ipd_search_patient_dr::IpdSearchPatientDrPage, ipd_search_patient_nurse::IpdSearchPatientNursePage, ipd_search_patient_other::IpdSearchPatientOtherPage,
    ipd_search_patient_pharmacist::IpdSearchPatientPharmacistPage, ipd_summary_audit::IpdSummaryAuditPage, not_found::NotFoundPage, opd_er_main::OpdErMainPage, opd_er_order_list::OpdErOrderListPage,
    opd_er_order_pharmacy::OpdErOrderPharmacyPage, permission_list::PermissionListPage, prescription_screen::PrescriptionScreenPage, report_designer::ReportDesignerPage,
    report_viewer::ReportViewerPage, setting_template_dc_plan::SettingTemplateDcPlanPage, setting_template_nurse_note::SettingTemplateNurseNotePage, summary::SummaryPage,
    unauthorized::UnAuthorizedPage, user_list::UserListPage, vital_sign::VitalSignPage,
};

pub fn render(app: Rc<App>) -> Dom {
    html!("div", {
        // sse messages sending queue
        .future(map_ref!{
            let busy = app.loader_is_loading(),
            let msg_len = app.messages.signal_vec_cloned().len() =>
            !busy && *msg_len > 0
        }.for_each(clone!(app => move |ready| {
            if ready {
                app.async_load(true, clone!(app => async move {
                    if let Some(message) = app.messages.lock_mut().pop() {
                        // POST `EndPoint::SseMessage`
                        if let Err(e) = message.call_api_post(app.state()).await {
                            app.alert_app_error(&e).await;
                        }
                    }
                }));
            }
            async{}
        })))
        .class_signal("app-is-loading", app.loader_is_loading())
        .child_signal(map_ref!{
            let app_route = app.route.signal_cloned(),
            let host = app.host.signal_cloned(),
            let url = routing::url().signal_cloned() => {
                let url_route = Route::from_url(url, host);
                if *app_route == url_route {app_route.clone()} else {url_route}
            }
        }.dedupe_cloned().map_future(move |route| render_inner(route.clone(), app.clone())))
    })
}

async fn render_inner(route: Route, app: Rc<App>) -> Dom {
    // log::debug!("Render {}", route.string());

    // clear Bootstrap `modal-backdrop` if exists by
    // 1. remove <div class="modal-backdrop"></div>
    // 2. remove `class` and `style` from <body data-bs-theme="light" class="modal-open" style="overflow: hidden; padding-right: 15px;">
    app.window.with(|w| {
        if let Some(backdrop) = w.document().unwrap().get_elements_by_class_name("modal-backdrop").item(0) {
            if let Some(backdrop_parent) = backdrop.parent_node() {
                backdrop_parent.remove_child(&backdrop).unwrap();
            }
            let body = w.document().unwrap().body().unwrap();
            let classes = js_sys::Array::new();
            classes.push(&JsValue::from_str("modal-open"));
            body.class_list().remove(&classes).unwrap();
            body.style().remove_property("overflow").unwrap();
            body.style().remove_property("padding-right").unwrap();
        }
    });

    if let Route::NotFound { path } = &route {
        // app.sse_end(true);
        app.route.set_neq(route.clone());
        return NotFoundPage { path: path.to_owned() }.render();
    } else if let Route::UnAuthorized { hash } = &route {
        // app.sse_end(true);
        app.route.set_neq(route.clone());
        return UnAuthorizedPage { hash: hash.to_owned() }.render();
    } else if token::update_token(app.state()).await {
        // redirect authorized user out of root and index
        if matches!(route, Route::Root | Route::Index) {
            Route::Info.hard_redirect();
            Dom::empty()
        } else {
            app.route.set_neq(route.clone());

            match app.user.get_cloned() {
                Some(user) => {
                    // app_assets not in local-storage, we use CACHE_CONTROL header and cache in service-worker
                    if app.app_asset.lock_ref().is_none() {
                        match AppAsset::get_asset(app.state()).await {
                            Ok(asset) => {
                                app.app_asset.set_neq(Some(Rc::new(asset)));
                                app.no_cache_mode.set(false);
                            }
                            Err(e) => {
                                app.alert_app_error(&e).await;
                            }
                        }
                    }
                    // for refresh brower, EventSource will throw Error => sse_ready_state > 1
                    if app.sse_ready_state.get() > 1 {
                        app.sse_end(false);
                        app.sse_ready_state.set(0);
                        App::sse_new(app.clone());
                        app.get_initial_user_alert().await;
                    }

                    app.load_theme();
                    let mut is_fixed_height = false;

                    let content = if route.has_permission(app.state()) {
                        match &route {
                            Route::Image => ImagePage::render(app.clone()),
                            Route::Info => InfoPage::render(InfoPage::new(), user.clone(), app.clone()),
                            Route::IpdAdmissionNoteDr { an } => {
                                let page = IpdAdmissionNoteDrPage::new(an.clone());
                                IpdAdmissionNoteDrPage::render(page, app.clone())
                            }
                            Route::IpdAdmissionNoteNurse { an } => {
                                let page = IpdAdmissionNoteNursePage::new(an.clone(), app.clone());
                                IpdAdmissionNoteNursePage::render(page, app.clone())
                            }
                            Route::Summary { view_by, an } => SummaryPage::render(SummaryPage::new(view_by.clone(), an.clone()), app.clone()),
                            Route::IpdConsultList { view_by } => {
                                let page = IpdConsultListPage::new(view_by.clone());
                                IpdConsultListPage::render(page, app.clone())
                            }
                            Route::IpdIndexPlan => {
                                is_fixed_height = true;
                                let page = IndexPlanPage::new_ipd();
                                IndexPlanPage::render(page, app.clone())
                            }
                            Route::IpdMain { view_by, an, tab, sub, id } => {
                                let page = IpdMainPage::new(view_by.clone(), an.clone(), Tab::from_string(&tab), sub.to_owned(), *id);
                                IpdMainPage::render(page, app.clone())
                            }
                            Route::IpdMra { an } => {
                                let page = IpdMraPage::new(an.clone());
                                IpdMraPage::render(page, app.clone())
                            }
                            Route::IpdOrderPharmacy => {
                                let page = IpdOrderPharmacyPage::new();
                                IpdOrderPharmacyPage::render(page, app.clone())
                            }
                            Route::IpdPostAdmitList { view_by } => {
                                let page = IpdPostAdmitListPage::new(&view_by, app.clone());
                                IpdPostAdmitListPage::render(page, app.clone())
                            }
                            Route::IpdPreAdmitList { view_by } => {
                                let page = IpdPreAdmitListPage::new(&view_by);
                                IpdPreAdmitListPage::render(page, app.clone())
                            }
                            Route::IpdPreOrder { view_by, pre_order_master_id } => {
                                let page = IpdPreOrderPage::new(view_by, *pre_order_master_id);
                                IpdPreOrderPage::render(page, app.clone())
                            }
                            Route::IpdPreOrderList { view_by } => {
                                let page = IpdPreOrderListPage::new(&view_by, app.clone());
                                IpdPreOrderListPage::render(page, app.clone())
                            }
                            Route::IpdSearchPatientDr => {
                                let page = IpdSearchPatientDrPage::new();
                                IpdSearchPatientDrPage::render(page, app.clone())
                            }
                            Route::IpdSearchPatientNurse => {
                                let page = IpdSearchPatientNursePage::new();
                                IpdSearchPatientNursePage::render(page, app.clone())
                            }
                            Route::IpdSearchPatientOther => {
                                let page = IpdSearchPatientOtherPage::new();
                                IpdSearchPatientOtherPage::render(page, app.clone())
                            }
                            Route::IpdSearchPatientPharmacist => {
                                let page = IpdSearchPatientPharmacistPage::new();
                                IpdSearchPatientPharmacistPage::render(page, app.clone())
                            }
                            Route::IpdSummaryAudit { an } => {
                                let page = IpdSummaryAuditPage::new(an.clone());
                                IpdSummaryAuditPage::render(page, app.clone())
                            }
                            Route::IpdVitalSign => {
                                is_fixed_height = true;
                                let page = VitalSignPage::new(true, app.clone());
                                VitalSignPage::render(page, app.clone())
                            }
                            Route::OpdErIndexPlan => {
                                is_fixed_height = true;
                                let page = IndexPlanPage::new_opd_er();
                                IndexPlanPage::render(page, app.clone())
                            }
                            Route::OpdErMain {
                                view_by,
                                opd_er_order_master_id,
                                tab,
                                id,
                            } => {
                                let page = OpdErMainPage::new(view_by.clone(), *opd_er_order_master_id, Tab::from_string(&tab), *id);
                                OpdErMainPage::render(page, app.clone())
                            }
                            Route::OpdErOrderList { view_by } => {
                                let page = OpdErOrderListPage::new(view_by.clone());
                                OpdErOrderListPage::render(page, app.clone())
                            }
                            Route::OpdErOrderPharmacy => {
                                let page = OpdErOrderPharmacyPage::new();
                                OpdErOrderPharmacyPage::render(page, app.clone())
                            }
                            Route::OpdErVitalSign => {
                                is_fixed_height = true;
                                let page = VitalSignPage::new(false, app.clone());
                                VitalSignPage::render(page, app.clone())
                            }

                            Route::PrescriptionScreen { hn } => {
                                is_fixed_height = true;
                                PrescriptionScreenPage::render(PrescriptionScreenPage::new(hn.clone()), app.clone())
                            }
                            Route::DrugUseDuration => DrugUseDurationPage::render(DrugUseDurationPage::new(), app.clone()),

                            Route::SettingTemplateDcPlan => SettingTemplateDcPlanPage::render(SettingTemplateDcPlanPage::new(), app.clone()),
                            Route::SettingTemplateNurseNote => SettingTemplateNurseNotePage::render(SettingTemplateNurseNotePage::new(), app.clone()),
                            Route::ReportViewer => {
                                is_fixed_height = true;
                                let page = ReportViewerPage::new(app.clone());
                                ReportViewerPage::render(page, app.clone())
                            }
                            Route::ReportDesigner => {
                                is_fixed_height = true;
                                ReportDesignerPage::render(ReportDesignerPage::new(), app.clone())
                            }
                            Route::UserList => UserListPage::render(UserListPage::new(), app.clone()),
                            Route::PermissionList => PermissionListPage::render(PermissionListPage::new(), app.clone()),
                            Route::Root | Route::NotFound { .. } | Route::UnAuthorized { .. } | Route::Index | Route::External { .. } => NotFoundPage { path: route.string() }.render(),
                        }
                    } else {
                        UnAuthorizedPage { hash: route.string() }.render()
                    };

                    html!("main", {
                        .apply_if(!is_fixed_height, |dom| dom.style("padding-bottom","88px"))
                        .children([
                            // GET `EndPoint::UserConfig`
                            // PATCH `EndPoint::User`
                            MenuCpn::render(MenuCpn::new(), app.clone()),
                            content,
                        ])
                    })
                }
                None => {
                    // no user, may not happened because token::update_token() will return false
                    // log::debug!("No User, redirect to index page");
                    app.sse_end(true);
                    Route::Index.hard_redirect();
                    Dom::empty()
                }
            }
        }
    // no refresh token, but route to Index
    } else if matches!(route, Route::Index) {
        if app.user.lock_ref().is_some() {
            // log::debug!("Go back to index page, clear user if exists");
            app.user.set(None);
            app.to_local_storage();
        }
        app.sse_end(true);
        app.route.set_neq(Route::Index);
        IndexPage::render(IndexPage::new(), app.clone())
    // no refresh token, may not happened
    } else {
        // log::debug!("Token invalid, remove user and redirect to index page");
        app.sse_end(true);
        app.route.set_neq(Route::Index);
        app.remove_user_and_go_index();
        Dom::empty()
    }
}
