use dominator::{Dom, clone, events, html, with_node};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, not},
};
use std::{
    rc::Rc,
    sync::atomic::{AtomicU32, Ordering},
};
use web_sys::HtmlButtonElement;

use kphis_model::{app::VisitTypeId, endpoint::EndPoint, fetch::Method, report::TypstReport};
use kphis_ui_app::App;
use kphis_ui_core::class;
use kphis_util::util::{str_some, zero_none};

use crate::modal::{blank_modal, report::preview::ReportPreview};

/// GET `EndPoint::ReportRawTemplateTypeId` (ReportPreview, guarded, invisible)
pub struct PdfButtons<T, C, F>
where
    T: ReportId + std::clone::Clone + 'static,
    C: ReportId + std::clone::Clone + 'static,
    F: Fn() -> String + std::clone::Clone + 'static,
{
    report: TypstReport,
    id: Mutable<T>,
    checker: Mutable<C>,
    // parent changed mutable
    changed: Mutable<bool>,
    json_fn: F,
}

impl<T, C, F> PdfButtons<T, C, F>
where
    T: ReportId + std::clone::Clone + 'static,
    C: ReportId + std::clone::Clone + 'static,
    F: Fn() -> String + std::clone::Clone + 'static,
{
    /// enable when `checker` is true and `changed` is false
    pub fn new(report: TypstReport, id: Mutable<T>, checker: Mutable<C>, changed: Mutable<bool>, json_fn: F) -> Rc<Self> {
        Rc::new(Self {
            report,
            id,
            checker,
            changed,
            json_fn,
        })
    }

    pub fn buttons(handle: Rc<Self>, label: &str, label_all: Option<&str>, app: Rc<App>) -> Vec<Dom> {
        if app.endpoint_is_allow(&Method::GET, &EndPoint::ReportRawTemplateTypeId, false) {
            let template_name = handle.report.template_name();
            let modal_name = ["reportModal-", template_name].concat();
            let modal_tag_name = ["#reportModal-", template_name].concat();
            let report_modal: Mutable<Option<Rc<ReportPreview>>> = Mutable::new(None);
            let can_signed = label_all.is_none();

            let mut doms = vec![
                // unsigned, use loaded data
                html!("button" => HtmlButtonElement, {
                    .attr("type", "button")
                    .class(class::BTN_R)
                    .class_signal("btn-primary", not(handle.changed.signal()))
                    .class_signal("btn-secondary", handle.changed.signal())
                    .attr("data-bs-toggle", "modal")
                    .attr("data-bs-target", &modal_tag_name)
                    .child(html!("i", {.class(class::FA_FILE_PDF_L)}))
                    .text(label)
                    .with_node!(element => {
                        .future(map_ref!{
                            let has_data = handle.checker.signal_cloned().map(|id| id.is_valid()),
                            let changed = handle.changed.signal() =>
                            !has_data || *changed
                        }.for_each(move |disable| {
                            element.set_disabled(disable);
                            async {}
                        }))
                    })
                    .event(clone!(report_modal, handle => move |_: events::Click| {
                        if let Some(id) = handle.id.lock_ref().option_string() {
                            report_modal.set(Some(ReportPreview::new(handle.report.clone(), id, str_some((handle.json_fn)()), can_signed, None)));
                        }
                    }))
                }),
                html!("div", {
                    .class("modal")
                    .attr("id", &modal_name)
                    .attr("role", "dialog")
                    .attr("tabindex", "-1")
                    .child_signal(report_modal.signal_cloned().map(clone!(app => move |opt| {
                        opt.as_ref().map(clone!(app => move |modal| {
                            // GET `EndPoint::ReportRawTemplateTypeId`
                            ReportPreview::render(modal.clone(), app)
                        })).or(Some(blank_modal()))
                    })))
                }),
            ];
            if let Some(label_all) = label_all {
                doms.push(html!("button" => HtmlButtonElement, {
                    .attr("type", "button")
                    .class(class::BTN_R)
                    .class_signal("btn-info", not(handle.changed.signal()))
                    .class_signal("btn-secondary", handle.changed.signal())
                    .attr("data-bs-toggle", "modal")
                    .attr("data-bs-target", &modal_tag_name)
                    .child(html!("i", {.class(class::FA_FILE_PDF_L)}))
                    .text(label_all)
                    // .with_node!(element => {
                    //     .future(map_ref!{
                    //         let has_data = handle.checker.signal_cloned().map(|id| id.is_valid()),
                    //         let changed = handle.changed.signal() =>
                    //         !has_data || *changed
                    //     }.for_each(move |disable| {
                    //         element.set_disabled(disable);
                    //         async {}
                    //     }))
                    // })
                    .event(clone!(report_modal, handle => move |_: events::Click| {
                        if let Some(id) = handle.id.lock_ref().option_string() {
                            report_modal.set(Some(ReportPreview::new(handle.report.clone(), id, None, true, None)));
                        }
                    }))
                }))
            }

            doms
        } else {
            Vec::new()
        }
    }
}

pub trait ReportId {
    fn option_string(&self) -> Option<String>;
    fn is_valid(&self) -> bool;
}

impl ReportId for bool {
    fn option_string(&self) -> Option<String> {
        self.then_some(String::from("Y"))
    }
    fn is_valid(&self) -> bool {
        *self
    }
}

impl ReportId for u32 {
    fn option_string(&self) -> Option<String> {
        zero_none(*self).map(|s| s.to_string())
    }
    fn is_valid(&self) -> bool {
        zero_none(*self).is_some()
    }
}

impl ReportId for String {
    fn option_string(&self) -> Option<String> {
        str_some(self.to_owned())
    }
    fn is_valid(&self) -> bool {
        str_some(self.to_owned()).is_some()
    }
}

impl ReportId for Option<String> {
    fn option_string(&self) -> Option<String> {
        self.clone().and_then(str_some)
    }
    fn is_valid(&self) -> bool {
        self.is_some()
    }
}

impl ReportId for Option<u32> {
    fn option_string(&self) -> Option<String> {
        (*self).and_then(zero_none).map(|s| s.to_string())
    }
    fn is_valid(&self) -> bool {
        self.is_some()
    }
}

impl ReportId for VisitTypeId {
    fn option_string(&self) -> Option<String> {
        match self {
            VisitTypeId::Ipd(an) | VisitTypeId::PreAdmit(an) => str_some(an.to_owned()),
            // report always use vn as key
            VisitTypeId::OpdEr(vn, _) | VisitTypeId::Visit(vn) => str_some(vn.to_owned()),
        }
    }
    fn is_valid(&self) -> bool {
        match self {
            VisitTypeId::Ipd(an) | VisitTypeId::PreAdmit(an) => str_some(an.to_owned()).is_some(),
            // some component use opd_er_order_master_id as checker, none use vn
            VisitTypeId::OpdEr(_vn, id) => zero_none(*id).is_some(),
            VisitTypeId::Visit(_) => false,
        }
    }
}

static STATIC_PDF_NEXT_ID: AtomicU32 = AtomicU32::new(1);

pub fn static_pdf_btn_with_modal(btn_label: &str, pdf_title: &'static str, template: &'static str, data_json: String, app: Rc<App>) -> Dom {
    let modal_id = ["static-pdf-", &STATIC_PDF_NEXT_ID.fetch_add(1, Ordering::SeqCst).to_string()].concat();
    let report_modal = Mutable::new(None);
    html!("div", {
        .apply_if(!app.endpoint_is_allow(&Method::GET, &EndPoint::ReportRawTemplateTypeId, false), |dom| dom.visible(false))
        .children([
            html!("button" => HtmlButtonElement, {
                .attr("type", "button")
                .class(class::BTN_SM_BLUE)
                .attr("data-bs-toggle", "modal")
                .attr("data-bs-target", &["#",&modal_id].concat())
                .child(html!("i", {.class(class::FA_FILE_PDF_L)}))
                .text(btn_label)
                .event(clone!(report_modal => move |_: events::Click| {
                    report_modal.set(Some(ReportPreview::new_static(template, Some(data_json.clone()), pdf_title)));
                }))
            }),
            html!("div", {
                .class("modal")
                .attr("id", &modal_id)
                .attr("role", "dialog")
                .attr("tabindex", "-1")
                .child_signal(report_modal.signal_cloned().map(clone!(app => move |opt| {
                    opt.as_ref().map(clone!(app => move |modal| {
                        // GET `EndPoint::ReportRawTemplateTypeId`
                        ReportPreview::render(modal.clone(), app)
                    })).or(Some(blank_modal()))
                })))
            })
        ])
    })
}
