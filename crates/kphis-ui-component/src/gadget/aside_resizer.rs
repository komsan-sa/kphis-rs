// ipd-dr-main.php

use dominator::{Dom, clone, events, html, with_node};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::rc::Rc;
use strum::IntoEnumIterator;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, HtmlInputElement, HtmlSelectElement};

use kphis_model::{
    A4_HEIGHT,
    A4_WIDTH,
    endpoint::EndPoint,
    fetch::Method,
    image::file_path::DocumentType,
    patient_info::PatientInfo,
    // timer::Timeout,
    report::{SystemReport, TypstRaw, TypstReport, TypstSvg},
};
use kphis_ui_app::App;
use kphis_ui_core::{
    class, mixins,
    pannable::PanState,
    resizable::{Resizable, ResizeState},
};
use kphis_util::{
    error::CONTACT_ADMIN,
    util::{str_some, zoom_step},
};

use crate::{emr::EmrCpn, lab::LabCpn, order::OrderCpn, xray::XrayCpn};

/// - GET `EndPoint::ReportRawTemplateTypeId` (guarded, remove report/document list, tools and btns)
/// - GET `EndPoint::EmrDateHn` (EmrCpn, guarded, remove emr btn)
/// - GET `EndPoint::EmrVisitVn` (EmrCpn, guarded, remove emr btn)
/// - GET `EndPoint::LabHead` (LabCpn, guarded, remove lab btn)
/// - GET `EndPoint::XrayReportHn` (XrayCpn, guarded, remove x-ray btn)
/// - GET `EndPoint::XrayPacsXn` (XrayCpn, guarded, remove x-ray btn)
#[derive(Clone, Default)]
pub struct AsideResizerCpn {
    left_panel_id: &'static str,
    with_report: bool,
    vnan: Mutable<String>,
    hn: Mutable<String>,
    loaded_lab_unread_exists_spinner: Option<Mutable<bool>>,
    loaded_xray_unread_exists_spinner: Option<Mutable<bool>>,

    // Resizable
    resizer: Rc<Resizable>,
    resize_state: Rc<ResizeState>,
    aside_hidden: Mutable<bool>,

    aside_tab: Mutable<String>,

    templates: MutableVec<SystemReport>,
    selected_template: Mutable<Option<SystemReport>>,
    load_and_render_report_svg: Mutable<bool>,

    selected_document: Mutable<Option<DocumentType>>,
    load_and_render_document_svg: Mutable<bool>,

    report_svg: MutableVec<Rc<TypstSvg>>,
    report_width_percent: Mutable<f64>,

    main_height: Mutable<i32>,
    main_client_rect_top: Mutable<f64>,
    viewer_position_percent: Mutable<f64>,
    viewer_offset_top: Mutable<u32>,
    viewer_inner_height: Mutable<u32>,
    viewer_top: Mutable<u32>,
    viewer_top_renew: Mutable<bool>,
    // Pannable
    pan_state: Rc<PanState>,
}

impl AsideResizerCpn {
    pub fn new(
        selected_template: Mutable<Option<SystemReport>>,
        load_and_render_report_svg: Mutable<bool>,
        selected_document: Mutable<Option<DocumentType>>,
        load_and_render_document_svg: Mutable<bool>,
        vnan: Mutable<String>,
        hn: Mutable<String>,
        reports: Vec<SystemReport>,
        left_panel_id: &'static str,
        loaded_lab_unread_exists_spinner: Option<Mutable<bool>>,
        loaded_xray_unread_exists_spinner: Option<Mutable<bool>>,
        app: Rc<App>,
    ) -> Rc<Self> {
        let with_report = !reports.is_empty();
        let report_svg = vec![Rc::new(TypstSvg::default())];
        let templates = if with_report { MutableVec::new_with_values(reports.clone()) } else { MutableVec::new() };
        let has_report = selected_template.lock_ref().as_ref().map(|selected| reports.contains(selected)).unwrap_or_default();
        let is_hide_aside = app.aside_prev_percent.get() == 100.0;
        Rc::new(Self {
            left_panel_id,
            with_report,
            vnan,
            hn,
            loaded_lab_unread_exists_spinner,
            loaded_xray_unread_exists_spinner,
            // set aside width
            resizer: Resizable::new_with_mutable(app.aside_prev_percent.clone(), app.aside_prev_percent_memoize.clone(), false),
            // set aside hidden
            aside_hidden: Mutable::new(is_hide_aside),
            aside_tab: Mutable::new(if has_report { String::from("report") } else { String::new() }),
            templates,
            selected_template,
            load_and_render_report_svg,
            selected_document,
            load_and_render_document_svg,
            report_svg: MutableVec::new_with_values(report_svg),
            report_width_percent: Mutable::new(100.0),
            // viewer_inner_height: Mutable::new(A4_HEIGHT as u32),
            ..Default::default()
        })
    }

    fn has_hn(&self) -> impl Signal<Item = bool> + use<> {
        self.hn.signal_cloned().map(|hn| !hn.is_empty())
    }

    fn load_and_render_template(page: Rc<Self>, app: Rc<App>) {
        if let (Some(template), Some(vnan)) = (
            page.selected_template
                .get_cloned()
                .map(|selected| TypstReport::from_system_with_coercion(selected, &app.state().report_coercions())),
            str_some(page.vnan.get_cloned()),
        ) {
            app.async_load(
                true,
                clone!(app, page => async move {
                    Self::fetch_report(template, &vnan, page.clone(), app.clone()).await;
                }),
            )
        }
    }

    fn load_and_render_document(page: Rc<Self>, app: Rc<App>) {
        if let (Some(doc_type_id), Some(vnan)) = (page.selected_document.get_cloned().map(|selected| selected as u8), str_some(page.vnan.get_cloned())) {
            app.async_load(
                true,
                clone!(app, page => async move {
                    let ids = [&vnan, "|", &doc_type_id.to_string(), "|1"].concat();
                    let template = TypstReport::from_system_with_coercion(SystemReport::DocumentImages, &app.state().report_coercions());
                    Self::fetch_report(template, &ids, page.clone(), app.clone()).await;
                }),
            )
        }
    }

    async fn fetch_report(template: TypstReport, ids: &str, page: Rc<Self>, app: Rc<App>) {
        // GET `EndPoint::ReportRawTemplateTypeId`
        match TypstRaw::call_api_get(template.template_name(), template.report_type(), ids, app.state()).await {
            Ok(response) => {
                let bytes = app.typst_worker().await.svg(response.typ, response.data_json, app.token().unwrap_or_default()).await;
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
                    let mut lock = page.report_svg.lock_mut();
                    lock.clear();
                    lock.extend(reports.into_iter().map(Rc::new));
                }
                page.set_main_position(app.clone());
                page.set_report_wide_percent(app.clone());
                page.viewer_position_percent.set(0.0);
                page.set_viewer_offset_and_inner_height();
            }
            Err(e) => {
                app.alert_app_error(&e).await;
            }
        }
    }

    fn set_main_position(&self, app: Rc<App>) {
        if let Some(main_elm) = app.get_id(self.left_panel_id) {
            let main = main_elm.dyn_into::<HtmlElement>().unwrap();
            let main_height = main.client_height();
            let adjusted = if main_height < 750 { 750 } else { main_height };
            self.main_height.set_neq(adjusted);
            self.main_client_rect_top.set_neq(main.get_bounding_client_rect().top());
        }
    }

    fn set_report_wide_percent(&self, app: Rc<App>) {
        if let Some(aside) = app.get_id("aside-right-panel") {
            let aside_width = aside.client_width() as f64;
            let reports_width = self.report_svg.lock_ref().iter().max_by_key(|i| i.width as u64).map(|i| i.width).unwrap_or(A4_WIDTH);
            let percent = (aside_width * 100.0) / reports_width;
            self.report_width_percent.set(percent);
        }
    }

    fn set_viewer_position_percent(&self, app: Rc<App>) {
        if let Some(gut_elm) = app.get_id("aside-right-gut") {
            let gut = gut_elm.dyn_into::<HtmlElement>().unwrap();
            let height = gut.scroll_height() as u32;
            let gut_scroll_top = gut.scroll_top() as u32;
            let main_client_rect_top = self.main_client_rect_top.get();
            let viewer_offset_top = self.viewer_offset_top.get();
            let content_height = height.saturating_sub(viewer_offset_top * 2);
            let content_offset_top = if main_client_rect_top > 0.0 {
                (gut_scroll_top).saturating_sub(main_client_rect_top.abs() as u32).saturating_sub(viewer_offset_top)
            } else {
                (gut_scroll_top + main_client_rect_top.abs() as u32).saturating_sub(viewer_offset_top)
            };
            let content_position_percent = if content_height > 0 && content_height >= content_offset_top {
                content_offset_top as f64 / content_height as f64 * 100.0
            } else {
                0.0
            };
            self.viewer_position_percent.set(content_position_percent);
        }
    }

    fn set_viewer_offset_and_inner_height(&self) {
        let main_height = self.main_height.get() as u32;
        let main_client_rect_top = self.main_client_rect_top.get();
        let percent = self.report_width_percent.get();
        let viewer_position_percent = self.viewer_position_percent.get();

        let report_count = self.report_svg.lock_ref().len();
        let report_max_page_height = self.report_svg.lock_ref().iter().max_by_key(|i| i.height as u64).map(|i| i.height).unwrap_or(A4_HEIGHT);
        let reports_raw_height = self.report_svg.lock_ref().iter().map(|i| i.height).sum::<f64>();
        let reports_exact_height = if reports_raw_height > 0.0 { reports_raw_height } else { A4_HEIGHT };
        // we add 16px to page height because each page has 8px padding
        let paddings = (report_count * 16) as f64;
        let reports_adjusted_height = (reports_exact_height * percent / 100.0) + paddings;
        let offset = main_height.saturating_sub(report_max_page_height as u32);
        // let height = if report_count > 0 { reports_adjusted_height as u32 + (offset * 2) } else { main_height };
        let height = if report_count > 0 { reports_adjusted_height as u32 + (offset * 2) } else { 0 };
        let viewer_position = offset + (reports_adjusted_height * viewer_position_percent / 100.0) as u32;
        let viewer_top = if main_client_rect_top > 95.0 {
            viewer_position + 150
        } else if main_client_rect_top > 0.0 {
            (viewer_position + main_client_rect_top.abs() as u32) + 55
        } else {
            viewer_position.saturating_sub(main_client_rect_top.abs() as u32) + 55
        };

        self.viewer_inner_height.set(height);
        self.viewer_offset_top.set(offset);
        self.viewer_top.set(viewer_top);
        self.viewer_top_renew.set(true);
    }

    fn is_show_report(&self) -> impl Signal<Item = bool> + use<> {
        map_ref! {
            let is_hidden = self.aside_hidden.signal(),
            let tab = self.aside_tab.signal_cloned() =>
            !is_hidden && tab.as_str() == "report"
        }
    }

    fn is_show_document(&self) -> impl Signal<Item = bool> + use<> {
        map_ref! {
            let is_hidden = self.aside_hidden.signal(),
            let tab = self.aside_tab.signal_cloned() =>
            !is_hidden && tab.as_str() == "document"
        }
    }

    /// with_order is (is_ipd, patient) for OrderCpn
    pub fn render(left_dom: Dom, with_order: Option<(bool, Mutable<Option<Rc<PatientInfo>>>)>, page: Rc<Self>, app: Rc<App>) -> Dom {
        let allow_load_report = app.endpoint_is_allow(&Method::GET, &EndPoint::ReportRawTemplateTypeId, false);
        let can_use_order = (app.endpoint_is_allow(&Method::GET, &EndPoint::IpdOrderOrderDateAn, false)
            && app.endpoint_is_allow(&Method::GET, &EndPoint::IpdOrderOrder, false)
            && app.endpoint_is_allow(&Method::GET, &EndPoint::IpdOrderPrevious, false)
            && app.endpoint_is_allow(&Method::GET, &EndPoint::IpdOrderProgressNote, false)
            && app.endpoint_is_allow(&Method::GET, &EndPoint::IpdMedReconcile, false))
            || (app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErOrderMasterCheckVn, false)
                && app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErOrderOrder, false)
                && app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErOrderProgressNote, false)
                && app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErHisMedVn, false)
                && app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErMedReconcile, false));

        html!("div", {
            .style("overflow-x","hidden")
            .apply_if(allow_load_report, |dom| dom
                .future(map_ref!(
                    let busy = app.loader_is_loading(),
                    let render = page.load_and_render_report_svg.signal() =>
                    !busy && *render
                ).for_each(clone!(app, page => move |ready| {
                    if ready {
                        if page.aside_hidden.get() {
                            page.resizer.set_prev_percent_memoize();
                        }
                        page.aside_tab.set_neq(String::from("report"));
                        Self::load_and_render_template(page.clone(), app.clone());
                        page.load_and_render_report_svg.set(false);
                    }
                    async {}
                })))
                .future(map_ref!(
                    let busy = app.loader_is_loading(),
                    let render = page.load_and_render_document_svg.signal() =>
                    !busy && *render
                ).for_each(clone!(app, page => move |ready| {
                    if ready {
                        if page.aside_hidden.get() {
                            page.resizer.set_prev_percent_memoize();
                        }
                        page.aside_tab.set_neq(String::from("document"));
                        Self::load_and_render_document(page.clone(), app.clone());
                        page.load_and_render_document_svg.set(false);
                    }
                    async {}
                })))
            )
            .future(page.resizer.prev_percent.signal().for_each(clone!(page => move |prev_percent| {
                page.aside_hidden.set_neq(prev_percent >= 100.0);
                async {}
            })))
            .global_event(clone!(page => move |e: events::MouseMove| {
                Resizable::on_mouse_move(&e, page.resize_state.clone());
                if page.with_report {
                    PanState::on_mouse_move(&e, page.pan_state.clone());
                }
            }))
            .global_event(clone!(page => move |_: events::MouseUp| {
                Resizable::on_mouse_up(page.resize_state.clone());
                if page.with_report {
                    PanState::on_mouse_up(page.pan_state.clone());
                }
            }))
            .child(html!("div", {
                // resizer
                .class_signal("resizer-container", page.has_hn())
                // left panel
                .child(html!("div", {
                    .apply(Resizable::prev_mixin(page.resizer.clone()))
                    .child(left_dom)
                }))
                // horizontal resizer
                .child_signal(page.has_hn().map(clone!(page => move |has_hn| {
                    has_hn.then(|| {
                        Resizable::render(page.resizer.clone(), page.resize_state.clone())
                    })
                })))
                // right panel
                .child_signal(page.has_hn().map(clone!(app, page, with_order => move |has_hn| {
                    has_hn.then(|| {
                        html!("div", {
                            .attr("id", "aside-right-panel")
                            .style_signal("height", page.main_height.signal_cloned().map(|h| {
                                if h < 1 {
                                    String::from("100%")
                                } else {
                                    [&h.to_string(),"px"].concat()
                                }
                            }))
                            .apply(Resizable::next_mixin(page.resizer.clone()))
                            .child_signal(page.aside_tab.signal_cloned().map(clone!(app, page, with_order => move |tab| {
                                Some(match tab.as_str() {
                                    "order" => {
                                        if let Some((is_ipd, patient)) = with_order.clone() {
                                            let order = OrderCpn::new(
                                                is_ipd, patient,
                                                Mutable::new(app.guess_view_by().to_owned()),
                                                Mutable::new(String::from("Y")),
                                                Mutable::new(String::new()),
                                                Mutable::new(0),
                                                app.clone(),
                                            );
                                            html!("div", {
                                                .class(class::FULL_P2)
                                                .child(OrderCpn::render("aside", order, app.clone()))
                                            })
                                        } else {
                                            Dom::empty()
                                        }
                                    }
                                    "emr" => {
                                        let emr = EmrCpn::new(page.hn.clone());
                                        html!("div", {
                                            .class(class::FULL_P2)
                                            .child(EmrCpn::render("aside", emr, app.clone()))
                                        })
                                    }
                                    "lab" => {
                                        let lab = LabCpn::new(
                                            Mutable::new(None),
                                            page.hn.clone(),
                                            page.vnan.clone(),
                                            page.loaded_lab_unread_exists_spinner.clone(),
                                        );
                                        html!("div", {
                                            .class(class::FULL_P2)
                                            .child(LabCpn::render("aside", lab, app.clone()))
                                        })
                                    }
                                    "xray" => {
                                        let xray = if page.vnan.lock_ref().len() == app.hosxp_an_len() {
                                            XrayCpn::new_ipd(
                                                page.hn.clone(),
                                                page.vnan.clone(),
                                                page.loaded_xray_unread_exists_spinner.clone(),
                                            )
                                        } else {
                                            XrayCpn::new_opd_er(
                                                page.hn.clone(),
                                                page.vnan.clone(),
                                                page.loaded_xray_unread_exists_spinner.clone(),
                                            )
                                        };
                                        html!("div", {
                                            .class(class::FULL_P2)
                                            .child(XrayCpn::render("aside", xray, app.clone()))
                                        })
                                    }
                                    "report" | "document" => {
                                        html!("div", {
                                            .attr("id", "aside-right-gut")
                                            .style("background-color","#eee")
                                            .apply(PanState::pan_container_mixins(page.pan_state.clone()))
                                            .child(html!("div", {
                                                .class(class::FLEX_COL_VC)
                                                .style_signal("height", page.viewer_inner_height.signal_cloned().map(|height| [&height.to_string(), "px"].concat()))
                                                .apply(mixins::typst_svg_mixins(page.report_width_percent.clone(), page.report_svg.clone()))
                                            }))
                                            .future(page.viewer_top_renew.signal().for_each(clone!(app, page => move |set_top| {
                                                if set_top {
                                                    page.viewer_top_renew.set(false);
                                                    if let Some(gut) = app.get_id("aside-right-gut") {
                                                        gut.set_scroll_top(page.viewer_top.get() as i32);
                                                    }
                                                }
                                                async {}
                                            })))
                                        })
                                    }
                                    _ => {
                                        html!("div", {
                                            .attr("id", "aside-right-gut")
                                            .style("background-color","#eee")
                                            .child(html!("div", {
                                                .class(class::FLEX_COL_VC)
                                                .style_signal("height", page.viewer_inner_height.signal_cloned().map(|height| [&height.to_string(), "px"].concat()))
                                            }))
                                        })
                                    }
                                })
                            })))
                        })
                    })
                })))
            }))
            // viewer tools
            .apply_if(allow_load_report, |dom| dom
                .child_signal(page.has_hn().map(clone!(app, page => move |has_hn| {
                    has_hn.then(|| {
                        html!("div", {
                            .style("position","fixed")
                            .style("bottom","60px")
                            .style("right","15px")
                            .style("z-index","1")
                            .child_signal(page.is_show_report().map(clone!(app, page => move |is_show_report| {
                                is_show_report.then(|| {
                                    html!("div", {
                                        .class("d-flex")
                                        .children([
                                            html!("div", {
                                                .class("me-2")
                                                .style("font-size","24px")
                                                .visible_signal(app.loader_is_loading())
                                                .child(html!("i", {.class(class::FA_SPIN_R)}))
                                            }),
                                            html!("select" => HtmlSelectElement, {
                                                .class(class::FORM_SELECT_X1)
                                                .child(html!("option", {.attr("value","").text("เลือกรายงาน")}))
                                                .children_signal_vec(page.templates.signal_vec_cloned().map(|template| {
                                                    html!("option", {
                                                        .attr("value", template.template_name())
                                                        .text(template.title())
                                                    })
                                                }))
                                                .prop_signal("value", page.selected_template.signal_cloned().map(|opt| opt.as_ref().map(|selected| selected.template_name().to_owned()).unwrap_or_default()))
                                                .with_node!(element => {
                                                    .event(clone!(app, page => move |_: events::Change| {
                                                        let v = element.value();
                                                        page.selected_template.set(SystemReport::new(&v));
                                                        app.report_select.set(v);
                                                        app.to_local_storage();
                                                        page.selected_document.set(None);
                                                        page.load_and_render_report_svg.set(true);
                                                    }))
                                                })
                                            }),
                                            html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_GRAY)
                                                .child(html!("i", {.class(class::FA_SYNC)}))
                                                .event(clone!(page => move |_:events::Click| {
                                                    page.load_and_render_report_svg.set(true);
                                                }))
                                            }),
                                        ])
                                    })
                                })
                            })))
                            .child_signal(page.is_show_document().map(clone!(app, page => move |is_show_document| {
                                is_show_document.then(|| {
                                    html!("div", {
                                        .class("d-flex")
                                        .children([
                                            html!("div", {
                                                .class("me-2")
                                                .style("font-size","24px")
                                                .visible_signal(app.loader_is_loading())
                                                .child(html!("i", {.class(class::FA_SPIN_R)}))
                                            }),
                                            html!("select" => HtmlSelectElement, {
                                                .class(class::FORM_SELECT_X1)
                                                .children(DocumentType::iter().map(|doc| {
                                                    html!("option", {
                                                        .attr("value", doc.as_str())
                                                        .text(doc.label())
                                                    })
                                                }))
                                                .prop_signal("value", page.selected_document.signal_cloned().map(|opt| opt.as_ref().map(|selected| selected.as_str()).unwrap_or_default()))
                                                .with_node!(element => {
                                                    .event(clone!(app, page => move |_: events::Change| {
                                                        let v = element.value();
                                                        page.selected_document.set(Some(DocumentType::new_from_str(&v)));
                                                        page.selected_template.set(None);
                                                        page.load_and_render_document_svg.set(true);
                                                    }))
                                                })
                                            }),
                                            html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_GRAY)
                                                .child(html!("i", {.class(class::FA_SYNC)}))
                                                .event(clone!(page => move |_:events::Click| {
                                                    page.load_and_render_document_svg.set(true);
                                                }))
                                            }),
                                        ])
                                    })
                                })
                            })))
                        })
                    })
                })))
            )
            // aside tool
            .child_signal(page.has_hn().map(clone!(app, page, with_order => move |has_hn| {
                has_hn.then(|| {
                    html!("div", {
                        .style("position","fixed")
                        .style("bottom","15px")
                        .style("right","15px")
                        .style("z-index","1")
                        .child(html!("div", {
                            .class("d-flex")
                            // full screen, resize btns
                            .apply_if(allow_load_report, |dom| dom
                                .children_signal_vec(page.is_show_report().map(clone!(app, page => move |is_show_report| {
                                    is_show_report.then(|| {
                                        vec![
                                            html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_L_GRAY)
                                                .child(html!("i", {.class(class::FA_ARROW_LR)}))
                                                .event(clone!(app, page => move |_:events::Click| {
                                                    page.set_main_position(app.clone());
                                                    page.set_report_wide_percent(app.clone());
                                                    page.set_viewer_position_percent(app.clone());
                                                    page.set_viewer_offset_and_inner_height();
                                                }))
                                            }),
                                            html!("div", {
                                                .class(class::INPUT_GROUP)
                                                .style("max-width","150px")
                                                .children([
                                                    html!("button", {
                                                        .attr("type", "button")
                                                        .class(class::BTN_GRAY)
                                                        .child(html!("i", {.class(class::FA_MINUS)}))
                                                        .event(clone!(app, page => move |_:events::Click| {
                                                            page.set_main_position(app.clone());
                                                            let zoom = page.report_width_percent.get();
                                                            page.report_width_percent.set(zoom_step(zoom, false));
                                                            page.set_viewer_position_percent(app.clone());
                                                            page.set_viewer_offset_and_inner_height();
                                                        }))
                                                    }),
                                                    html!("div", {
                                                        .class(class::FORM_CTRL_C)
                                                        .text_signal(page.report_width_percent.signal_cloned().map(|u| [&u.round().to_string(), "%"].concat()))
                                                    }),
                                                    html!("button", {
                                                        .attr("type", "button")
                                                        .class(class::BTN_GRAY)
                                                        .child(html!("i", {.class(class::FA_PLUS)}))
                                                        .event(clone!(app, page => move |_:events::Click| {
                                                            page.set_main_position(app.clone());
                                                            let zoom = page.report_width_percent.get();
                                                            page.report_width_percent.set(zoom_step(zoom, true));
                                                            page.set_viewer_position_percent(app.clone());
                                                            page.set_viewer_offset_and_inner_height();
                                                        }))
                                                    }),
                                                ])
                                            }),
                                        ]
                                    }).unwrap_or_default()
                                })).to_signal_vec())
                            )
                            // radio btns
                            .child_signal(page.aside_hidden.signal_cloned().map(clone!(app, page, with_order => move |is_hidden| {
                                (!is_hidden).then(|| {
                                    html!("div", {
                                        .class(class::BTN_GROUP_WHITE_R)
                                        .attr("role","group")
                                        .attr("aria-label","Select aside contents")
                                        .apply_if(page.with_report && allow_load_report, |dom| { dom
                                            .children([
                                                html!("input" => HtmlInputElement, {
                                                    .attr("type", "radio")
                                                    .class("btn-check")
                                                    .attr("id", "aside-tab-report")
                                                    .attr("autocomplete","off")
                                                    .attr("checked","")
                                                    .apply(mixins::radio_match(page.aside_tab.clone(), Mutable::new(false), "report"))
                                                }),
                                                html!("label", {
                                                    .class(class::BTN_BLUEO)
                                                    .attr("for", "aside-tab-report")
                                                    .text("รายงาน")
                                                }),
                                            ])
                                        })
                                        .apply_if(allow_load_report, |dom| { dom
                                            .children([
                                                html!("input" => HtmlInputElement, {
                                                    .attr("type", "radio")
                                                    .class("btn-check")
                                                    .attr("id", "aside-tab-image")
                                                    .attr("autocomplete","off")
                                                    .attr("checked","")
                                                    .apply(mixins::radio_match(page.aside_tab.clone(), Mutable::new(false), "document"))
                                                }),
                                                html!("label", {
                                                    .class(class::BTN_BLUEO)
                                                    .attr("for", "aside-tab-image")
                                                    .text("เอกสาร")
                                                }),
                                            ])
                                        })
                                        .apply_if(with_order.is_some() && can_use_order, |dom| dom
                                            .children([
                                                html!("input" => HtmlInputElement, {
                                                    .attr("type", "radio")
                                                    .class("btn-check")
                                                    .attr("id", "aside-tab-order")
                                                    .attr("autocomplete","off")
                                                    .apply(mixins::radio_match(page.aside_tab.clone(), Mutable::new(false), "order"))
                                                }),
                                                html!("label", {
                                                    .class(class::BTN_BLUEO)
                                                    .attr("for", "aside-tab-order")
                                                    .text("Order")
                                                }),
                                            ])
                                        )
                                        .apply_if(app.endpoint_is_allow(&Method::GET, &EndPoint::EmrDateHn, false)
                                            && app.endpoint_is_allow(&Method::GET, &EndPoint::EmrVisitVn, false),
                                        |dom| dom
                                            .children([
                                                html!("input" => HtmlInputElement, {
                                                    .attr("type", "radio")
                                                    .class("btn-check")
                                                    .attr("id", "aside-tab-emr")
                                                    .attr("autocomplete","off")
                                                    .apply(mixins::radio_match(page.aside_tab.clone(), Mutable::new(false), "emr"))
                                                }),
                                                html!("label", {
                                                    .class(class::BTN_BLUEO)
                                                    .attr("for", "aside-tab-emr")
                                                    .text("EMR")
                                                }),
                                            ])
                                        )
                                        .apply_if(app.endpoint_is_allow(&Method::GET, &EndPoint::LabHead, false), |dom| dom
                                            .children([
                                                html!("input" => HtmlInputElement, {
                                                    .attr("type", "radio")
                                                    .class("btn-check")
                                                    .attr("id", "aside-tab-lab")
                                                    .attr("autocomplete","off")
                                                    .apply(mixins::radio_match(page.aside_tab.clone(), Mutable::new(false), "lab"))
                                                }),
                                                html!("label", {
                                                    .class(class::BTN_BLUEO)
                                                    .attr("for", "aside-tab-lab")
                                                    .text("Lab")
                                                }),
                                            ])
                                        )
                                        .apply_if(app.has_pacs_host()
                                            && app.endpoint_is_allow(&Method::GET, &EndPoint::XrayReportHn, false)
                                            && app.endpoint_is_allow(&Method::GET, &EndPoint::XrayPacsXn, false),
                                        |dom| dom
                                            .children([
                                                html!("input" => HtmlInputElement, {
                                                    .attr("type", "radio")
                                                    .class("btn-check")
                                                    .attr("id", "aside-tab-xray")
                                                    .attr("autocomplete","off")
                                                    .apply(mixins::radio_match(page.aside_tab.clone(), Mutable::new(false), "xray"))
                                                }),
                                                html!("label", {
                                                    .class(class::BTN_BLUEO)
                                                    .attr("for", "aside-tab-xray")
                                                    .text("X-Ray")
                                                }),
                                            ])
                                        )
                                    })
                                })
                            })))
                            .child(html!("button", {
                                .attr("type", "button")
                                .class(class::BTN_R_GRAY)
                                .child_signal(page.aside_hidden.signal_cloned().map(|is_hidden| {
                                    if is_hidden {
                                        Some(html!("i", {.class(class::FA_COL2)}))
                                    } else {
                                        Some(html!("i", {.class(class::FA_COL1)}))
                                    }
                                }))
                                .event(clone!(app, page => move |_:events::Click| {
                                    let will_aside_hidden = !page.aside_hidden.get();
                                    page.aside_hidden.set(will_aside_hidden);
                                    if will_aside_hidden {
                                        page.resizer.set_prev_percent_full();
                                    } else {
                                        page.resizer.set_prev_percent_memoize();
                                    };
                                    // if !will_aside_hidden {
                                    //     let fit = Timeout::new(0, clone!(app, page => move || {
                                    //         page.set_main_position(app.clone());
                                    //         page.set_report_wide_percent(app.clone());
                                    //         page.viewer_position_percent.set(0.0);
                                    //         page.set_viewer_offset_and_inner_height();
                                    //     }));
                                    //     fit.forget();
                                    // }
                                }))
                            }))
                        }))
                    })
                })
            })))
        })
    }
}
