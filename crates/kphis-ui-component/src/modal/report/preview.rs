use dominator::{Dom, clone, events, html, with_node};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, not},
    signal_vec::MutableVec,
};
use std::{
    rc::Rc,
    sync::atomic::{AtomicU32, Ordering},
};
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, HtmlInputElement};

use kphis_model::{
    A4_HEIGHT, A4_WIDTH,
    app::AppState,
    endpoint::EndPoint,
    fetch::{Method, get_text_from_url},
    report::{TypstRaw, TypstReport, TypstSvg},
    timer::Timeout,
};
use kphis_ui_app::App;
use kphis_ui_core::{class, doms, mixins, pannable::PanState};
use kphis_util::{
    error::CONTACT_ADMIN,
    util::{str_some, zoom_step},
};

static NEXT_ID: AtomicU32 = AtomicU32::new(1);

/// - GET `EndPoint::ReportRawTemplateTypeId`
/// - GET `EndPoint::ReportTemplateTypeId` (guarded, remove 'Signed PDF' btn)
#[derive(Clone, Default)]
pub struct ReportPreview {
    preview_id: u32,
    can_signed: bool,
    report: Option<TypstReport>,
    template: Mutable<Option<String>>,
    data_json: Mutable<Option<String>>,
    ids: String,
    title: Option<String>,

    // Pannable
    pan_state: Rc<PanState>,

    loaded: Mutable<bool>,
    render_svg: Mutable<bool>,
    render_pdf: Mutable<bool>,
    reload_signed_pdf: Mutable<bool>,

    report_svg: MutableVec<Rc<TypstSvg>>,
    report_fullscreen: Mutable<bool>,

    report_width_percent: Mutable<f64>,
    viewer_position_percent: Mutable<f64>,
    viewer_top: Mutable<u32>,
    viewer_top_renew: Mutable<bool>,
}

impl ReportPreview {
    pub fn new(report: TypstReport, ids: String, data_json: Option<String>, can_signed: bool, custom_title: Option<String>) -> Rc<Self> {
        Rc::new(Self {
            preview_id: NEXT_ID.fetch_add(1, Ordering::SeqCst),
            can_signed,
            report: Some(report),
            data_json: Mutable::new(data_json),
            title: custom_title,
            ids,
            report_svg: MutableVec::new_with_values(vec![Rc::new(TypstSvg::default())]),
            report_width_percent: Mutable::new(100.0),
            ..Default::default()
        })
    }

    pub fn new_static(template: &str, data_json: Option<String>, title: &str) -> Rc<Self> {
        Rc::new(Self {
            preview_id: NEXT_ID.fetch_add(1, Ordering::SeqCst),
            template: Mutable::new(Some(template.to_owned())),
            data_json: Mutable::new(data_json),
            title: Some(title.to_owned()),
            loaded: Mutable::new(true),
            render_svg: Mutable::new(true),
            report_svg: MutableVec::new_with_values(vec![Rc::new(TypstSvg::default())]),
            report_width_percent: Mutable::new(100.0),
            ..Default::default()
        })
    }

    fn load(modal: Rc<Self>, app: Rc<App>) {
        if let Some(report) = modal.report.clone() {
            app.async_load(
                true,
                clone!(app, modal => async move {
                    if modal.data_json.lock_ref().is_some() {
                        match get_text_from_url(&report.typ_path_client(), app.state()).await {
                            Ok(response) => {
                                if let Some(template) = response {
                                    modal.template.set(Some(template));
                                    modal.render_svg.set(true);
                                } else {
                                    app.alert_error_with_closed("ไม่พบ Template เอกสาร", "").await;
                                }
                            }
                            Err(e) => {
                                app.alert_app_error(&e).await;
                            }
                        }
                    } else if let Some(ids) = str_some(modal.ids.clone()) {
                        // GET `EndPoint::ReportRawTemplateTypeId`
                        match TypstRaw::call_api_get(report.template_name(), report.report_type(), &ids, app.state()).await {
                            Ok(response) => {
                                modal.template.set(Some(response.typ));
                                modal.data_json.set(Some(response.data_json));
                                modal.render_svg.set(true);
                            }
                            Err(e) => {
                                app.alert_app_error(&e).await;
                            }
                        }
                    }
                }),
            );
        }
    }

    async fn render_svg(&self, app: Rc<App>) {
        if let (Some(template), Some(data_json)) = (self.template.get_cloned(), self.data_json.get_cloned()) {
            let bytes = app.typst_worker().await.svg(template, data_json, app.token().unwrap_or_default()).await;
            let reports = if bytes.is_empty() {
                vec![TypstSvg::default()]
            } else {
                match bitcode::decode(&bytes) {
                    Ok(pages) => pages,
                    Err(e) => {
                        app.alert_error_with_clipboard(CONTACT_ADMIN, &["BitcodeError: ", &e.to_string()].concat()).await;
                        vec![TypstSvg::default()]
                    }
                }
            };
            {
                let mut lock = self.report_svg.lock_mut();
                lock.clear();
                lock.extend(reports.into_iter().map(Rc::new));
            }
            self.set_fit(app);
            self.viewer_position_percent.set(0.0);
            self.set_viewer_offset_and_inner_height();
        }
    }

    fn set_fit(&self, app: Rc<App>) {
        if let Some(viewer) = app.get_id(&["preview-viewer-", &self.preview_id.to_string()].concat()) {
            let viewer_width = viewer.client_width() as f64;
            let reports_width = self.report_svg.lock_ref().iter().max_by_key(|i| i.width as u64).map(|i| i.width).unwrap_or(A4_WIDTH);
            let percent = (viewer_width * 100.0) / reports_width;
            self.report_width_percent.set(percent);
        }
    }

    fn set_viewer_offset_and_inner_height(&self) {
        let percent = self.report_width_percent.get();
        let viewer_position_percent = self.viewer_position_percent.get();

        let report_count = self.report_svg.lock_ref().len();
        let reports_raw_height = self.report_svg.lock_ref().iter().map(|i| i.height).sum::<f64>();
        let reports_exact_height = if reports_raw_height > 0.0 { reports_raw_height } else { A4_HEIGHT };
        let gaps = ((report_count - 1) * 32) as f64;
        let reports_adjusted_height = (reports_exact_height * percent / 100.0) - gaps;

        self.viewer_top.set((reports_adjusted_height * viewer_position_percent / 100.0) as u32);
        self.viewer_top_renew.set(true);
    }

    fn set_viewer_position_percent(&self, app: Rc<App>) {
        if let Some(elm) = app.get_id("report-right-gut") {
            let gut = elm.dyn_into::<HtmlElement>().unwrap();
            let content_height = gut.scroll_height() as u32;
            let scroll_top = gut.scroll_top() as u32;
            let content_position_percent = if content_height > 0 { scroll_top as f64 / content_height as f64 * 100.0 } else { 0.0 };
            self.viewer_position_percent.set(content_position_percent);
        }
    }

    async fn render_pdf(&self, app: Rc<App>) {
        if let (Some(template), Some(data_json)) = (self.template.get_cloned(), self.data_json.get_cloned()) {
            let ids = self.ids.clone();
            let file_name = if let Some(report) = self.report.clone() {
                report.download_file_name(&ids)
            } else {
                String::from("CUSTOM")
            };

            let author = app.app_status.lock_ref().as_ref().map(|app_status| app_status.hospital_name.clone()).unwrap_or_default();
            let user = app.user.lock_ref().as_ref().map(|user| user.user.name.get_cloned()).unwrap_or_default();

            let bytes = app.typst_worker().await.pdf(template, data_json, self.title(), author, user, app.token().unwrap_or_default()).await;

            if !bytes.is_empty() {
                app.open_file_with_mime(&bytes, &file_name, "application/pdf");
            }
        }
    }

    fn title(&self) -> String {
        let title_with_id_opt = self.report.as_ref().map(|report| report.title_with_ids(&self.ids));
        match (&title_with_id_opt, &self.title) {
            (Some(title_with_id), Some(title)) => [title, " (", title_with_id, ")"].concat(),
            (Some(title_with_id), None) => title_with_id.to_owned(),
            (None, Some(title)) => {
                if self.ids.is_empty() {
                    title.to_owned()
                } else {
                    [title, " (", &self.ids, ")"].concat()
                }
            }
            (None, None) => self.ids.clone(),
        }
    }

    fn load_signed_pdf(modal: Rc<Self>, app: Rc<App>) {
        if let (Some(ids), Some(report)) = (str_some(modal.ids.clone()), modal.report.clone()) {
            app.async_load(
                true,
                clone!(app => async move {
                    // GET `EndPoint::ReportTemplateTypeId`
                    match AppState::call_api_get_pdf_report(report.template_name(), report.report_type(), &ids, app.state()).await {
                        Ok(blob) => {
                            app.open_response_blob(blob, &report.download_file_name(&ids));
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }),
            );
        }
    }

    pub fn render(modal: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let loaded = modal.loaded.signal() =>
                !busy && !loaded
            ).for_each(clone!(app, modal => move |ready| {
                if ready {
                    Self::load(modal.clone(), app.clone());
                    modal.loaded.set_neq(true);
                }
                async {}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let render = modal.render_svg.signal() =>
                !busy && *render
            ).for_each(clone!(app, modal => move |ready| {
                if ready {
                    modal.render_svg.set(false);
                    app.async_load(
                        true,
                        clone!(app, modal => async move {
                            modal.render_svg(app).await;
                        })
                    )
                }
                async {}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let render = modal.render_pdf.signal() =>
                !busy && *render
            ).for_each(clone!(app, modal => move |ready| {
                if ready {
                    modal.render_pdf.set(false);
                    app.async_load(
                        true,
                        clone!(app, modal => async move {
                            modal.render_pdf(app.clone()).await;
                        })
                    )
                }
                async {}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let reload = modal.reload_signed_pdf.signal() =>
                !busy && *reload
            ).for_each(clone!(app, modal => move |ready| {
                if ready {
                    Self::load_signed_pdf(modal.clone(), app.clone());
                    modal.reload_signed_pdf.set(false);
                }
                async {}
            })))
            .global_event(clone!(modal => move |e: events::MouseMove| {
                PanState::on_mouse_move(&e, modal.pan_state.clone());
            }))
            .global_event(clone!(modal => move |_: events::MouseUp| {
                PanState::on_mouse_up(modal.pan_state.clone());
            }))
            .class("modal-dialog")
            .class_signal("modal-xl", not(modal.report_fullscreen.signal()))
            .class_signal("modal-fullscreen", modal.report_fullscreen.signal())
            .attr("role", "document")
            .child(html!("div", {
                .class("modal-content")
                .children([
                    html!("div", {
                        .class("modal-header")
                        .children([
                            html!("h5", {
                                .class("modal-title")
                                .text(&modal.title())
                            }),
                            doms::close_modal_x_btn(),
                        ])
                    }),
                    html!("div", {
                        .class("modal-body")
                        //.style("height","400px")
                        .style("width", "100%")
                        .children([
                            html!("div", {
                                .class(class::FLEX_JCR)
                                .child(html!("div", {
                                    .class(class::FLEX_T)
                                    .style("white-space","nowrap")
                                    .children([
                                        html!("div", {
                                            .class("me-2")
                                            .style("font-size","24px")
                                            .visible_signal(app.loader_is_loading())
                                            .child(html!("i", {.class(class::FA_SPIN_R)}))
                                        }),
                                        html!("button", {
                                            .attr("type", "button")
                                            .class(class::BTN_L_GRAY)
                                            .child_signal(modal.report_fullscreen.signal_cloned().map(|is_full| {
                                                if is_full {
                                                    Some(html!("i", {.class(class::FA_MIN)}))
                                                } else {
                                                    Some(html!("i", {.class(class::FA_MAX)}))
                                                }
                                            }))
                                            .event(clone!(app, modal => move |_:events::Click| {
                                                modal.report_fullscreen.set(!modal.report_fullscreen.get());
                                                let fit = Timeout::new(0, clone!(app, modal => move || {
                                                    modal.set_fit(app.clone());
                                                    modal.set_viewer_position_percent(app.clone());
                                                    modal.set_viewer_offset_and_inner_height();
                                                }));
                                                fit.forget();
                                            }))
                                        }),
                                        html!("button", {
                                            .attr("type", "button")
                                            .class(class::BTN_L_GRAY)
                                            .child(html!("i", {.class(class::FA_ARROW_LR)}))
                                            .event(clone!(app, modal => move |_:events::Click| {
                                                modal.set_fit(app.clone());
                                                modal.set_viewer_position_percent(app.clone());
                                                modal.set_viewer_offset_and_inner_height();
                                            }))
                                        }),
                                        html!("div", {
                                            .class(class::INPUT_GROUP)
                                            .style("max-width","170px")
                                            .children([
                                                html!("button", {
                                                    .attr("type", "button")
                                                    .class(class::BTN_GRAY)
                                                    .child(html!("i", {.class(class::FA_MINUS)}))
                                                    .event(clone!(app, modal => move |_:events::Click| {
                                                        let zoom = modal.report_width_percent.get();
                                                        modal.report_width_percent.set(zoom_step(zoom, false));
                                                        modal.set_viewer_position_percent(app.clone());
                                                        modal.set_viewer_offset_and_inner_height();
                                                    }))
                                                }),
                                                html!("input" => HtmlInputElement, {
                                                    .class(class::FORM_CTRL_C)
                                                    .prop_signal("value", modal.report_width_percent.signal_cloned().map(|u| [&u.round().to_string(), "%"].concat()))
                                                    .with_node!(element => {
                                                        .event(clone!(app, modal => move |_:events::Change| {
                                                            if let Ok(value) = element.value().trim_end_matches('%').parse::<f64>() {
                                                                modal.report_width_percent.set(value);
                                                                modal.set_viewer_position_percent(app.clone());
                                                                modal.set_viewer_offset_and_inner_height();
                                                            }
                                                        }))
                                                    })
                                                    // .text_signal(modal.report_width_percent.signal_cloned().map(|u| [&u.round().to_string(), "%"].concat()))
                                                }),
                                                html!("button", {
                                                    .attr("type", "button")
                                                    .class(class::BTN_GRAY)
                                                    .child(html!("i", {.class(class::FA_PLUS)}))
                                                    .event(clone!(app, modal => move |_:events::Click| {
                                                        let zoom = modal.report_width_percent.get();
                                                        modal.report_width_percent.set(zoom_step(zoom, true));
                                                        modal.set_viewer_position_percent(app.clone());
                                                        modal.set_viewer_offset_and_inner_height();
                                                    }))
                                                }),
                                            ])
                                        }),
                                        html!("button", {
                                            .attr("type", "button")
                                            .class(class::BTN_R_BLUE)
                                            .child(html!("i", {
                                                .class(class::FA_FILE_PDF_L)
                                            }))
                                            .text("PDF")
                                            .event(clone!(modal => move |_:events::Click| {
                                                modal.render_pdf.set_neq(true);
                                            }))
                                        }),
                                    ])
                                    .apply_if(
                                        app.endpoint_is_allow(&Method::GET, &EndPoint::ReportTemplateTypeId, false)
                                        && app.can_sign_pdf()
                                        && modal.can_signed,
                                    |dom| {
                                        dom.child(html!("button", {
                                            .attr("type", "button")
                                            .class(class::BTN_R_CYAN)
                                            .child(html!("i", {
                                                .class(class::FA_FILE_PDF_L)
                                            }))
                                            .text("Signed PDF")
                                            .event(clone!(modal => move |_:events::Click| {
                                                modal.reload_signed_pdf.set_neq(true);
                                            }))
                                        }))
                                    })
                                }))
                            }),
                            html!("div", {
                                .class("border")
                                .attr("id",&["preview-viewer-", &modal.preview_id.to_string()].concat())
                                .style_signal("height", modal.report_fullscreen.signal_cloned().map(|is_full| {
                                    if is_full {
                                        "calc(100vh - 210px)"
                                    } else {
                                        "calc(100vh - 270px)"
                                    }
                                }))
                                .child(html!("div", {
                                    .attr("id",&["preview-viewer-", &modal.preview_id.to_string(), "-gut"].concat())
                                    .style("background-color","#eee")
                                    .apply(PanState::pan_container_mixins(modal.pan_state.clone()))
                                    .child(html!("div", {
                                        .apply(mixins::typst_svg_mixins(modal.report_width_percent.clone(), modal.report_svg.clone()))
                                    }))
                                    .future(modal.viewer_top_renew.signal().for_each(clone!(app, modal => move |set_top| {
                                        if set_top {
                                            modal.viewer_top_renew.set(false);
                                            if let Some(gut) = app.get_id(&["preview-viewer-", &modal.preview_id.to_string(), "-gut"].concat()) {
                                                gut.set_scroll_top(modal.viewer_top.get() as i32);
                                            }
                                        }
                                        async {}
                                    })))
                                }))
                            }),
                        ])
                    }),
                    html!("div", {
                        .class("modal-footer")
                        .child(html!("button", {
                            .attr("type", "button")
                            .class(class::BTN_GRAY)
                            .attr("data-bs-dismiss", "modal")
                            .text("ปิด")
                        }))
                    }),
                ])
            }))
        })
    }
}
