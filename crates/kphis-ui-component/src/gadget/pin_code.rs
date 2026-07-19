use dominator::{Dom, EventOptions, clone, events, html, with_node};
use futures_signals::signal::{Mutable, SignalExt};
use kphis_ui_core::class;
use std::rc::Rc;
use web_sys::HtmlInputElement;

use kphis_util::util::text_to_six_digits;

#[derive(Default)]
pub struct PinCode {
    parent_input: Mutable<String>,
    parent_submit: Mutable<bool>,

    num_1: Mutable<Option<u8>>,
    num_2: Mutable<Option<u8>>,
    num_3: Mutable<Option<u8>>,
    num_4: Mutable<Option<u8>>,
    num_5: Mutable<Option<u8>>,
    num_6: Mutable<Option<u8>>,
    focus_at: Mutable<u8>,

    is_visible: Mutable<bool>,
}

impl PinCode {
    pub fn new(parent_input: Mutable<String>, parent_submit: Mutable<bool>) -> Rc<Self> {
        Rc::new(Self {
            parent_input,
            parent_submit,
            ..Default::default()
        })
    }

    fn get_input(&self, id: u8) -> Mutable<Option<u8>> {
        match id {
            0 => self.num_1.clone(),
            1 => self.num_2.clone(),
            2 => self.num_3.clone(),
            3 => self.num_4.clone(),
            4 => self.num_5.clone(),
            _ => self.num_6.clone(),
        }
    }

    fn set_new(&self) {
        self.num_1.set(None);
        self.num_2.set(None);
        self.num_3.set(None);
        self.num_4.set(None);
        self.num_5.set(None);
        self.num_6.set(None);
        self.parent_input.set_neq(String::new());
        self.focus_at.set(0);
    }

    fn set_all(&self, all: [u8; 6]) {
        self.num_1.set(Some(all[0]));
        self.num_2.set(Some(all[1]));
        self.num_3.set(Some(all[2]));
        self.num_4.set(Some(all[3]));
        self.num_5.set(Some(all[4]));
        self.num_6.set(Some(all[5]));
        self.set_parent();
    }

    fn set_parent(&self) {
        if let (Some(num_1), Some(num_2), Some(num_3), Some(num_4), Some(num_5), Some(num_6)) =
            (self.num_1.get(), self.num_2.get(), self.num_3.get(), self.num_4.get(), self.num_5.get(), self.num_6.get())
        {
            let result = [num_1.to_string(), num_2.to_string(), num_3.to_string(), num_4.to_string(), num_5.to_string(), num_6.to_string()].concat();
            let is_neq = self.parent_input.lock_ref().as_str() != result.as_str();
            if is_neq {
                self.parent_input.set(result);
                self.parent_submit.set(true);
            }
        }
    }

    pub fn render(page: Rc<Self>) -> Dom {
        html!("div", {
            .future(page.parent_input.signal_ref(|v| v.is_empty()).dedupe().for_each(clone!(page => move |is_empty| {
                if is_empty {
                    page.set_new();
                }
                async {}
            })))
            .children([
                html!("div", {
                    .class("pin-container")
                    .children([
                        Self::render_pin_input(0, page.clone()),
                        Self::render_pin_input(1, page.clone()),
                        Self::render_pin_input(2, page.clone()),
                        Self::render_pin_input(3, page.clone()),
                        Self::render_pin_input(4, page.clone()),
                        Self::render_pin_input(5, page.clone()),
                    ])
                }),
                html!("div", {
                    .class("pin-container")
                    .children([
                        html!("button", {
                            .attr("type", "button")
                            .class(class::BTN_BLUEO)
                            .child_signal(page.is_visible.signal().map(|is_visible| {
                                if is_visible {
                                    Some(html!("i", {.class(class::FA_EYE_SLASH)}))
                                } else {
                                    Some(html!("i", {.class(class::FA_EYE)}))
                                }
                            }))
                            .event(clone!(page => move |_:events::Click| {
                                page.is_visible.set(!page.is_visible.get());
                                page.focus_at.set(page.focus_at.get());
                            }))
                        }),
                        html!("button", {
                            .attr("type", "button")
                            .class(class::BTN_REDO)
                            .child(html!("i", {.class(class::FA_X)}))
                            .event(clone!(page => move |_:events::Click| {
                                page.set_new();
                            }))
                        }),
                    ])
                }),
            ])
        })
    }

    fn render_pin_input(id: u8, page: Rc<Self>) -> Dom {
        let input_mutable = page.get_input(id);
        html!("input" => HtmlInputElement, {
            .prop_signal("type", page.is_visible.signal().map(|visible| {
                if visible {
                    "text"
                } else {
                    "password"
                }
            }))
            // .attr("maxlength","1")
            .attr("inputmode","numeric")
            .attr("pattern","[0-9]*")
            .class("pin-input")
            .class_signal("filled", input_mutable.signal().map(|v| v.is_some()))
            .focused_signal(page.focus_at.signal().map(move |focus_id| focus_id == id))
            .prop_signal("value", input_mutable.signal().map(|opt| opt.map(|u| {
                if u < 10 {
                    u.to_string()
                } else {
                    String::new()
                }
            }).unwrap_or_default()))
            .with_node!(element => {
                .event(clone!(page, element, input_mutable => move |_:events::Input| {
                    let input = element.value();
                    // support paste all 6 digit at once
                    if id == 0 && let Some(arr) = text_to_six_digits(&input) {
                        page.set_all(arr);
                    } else {
                        let value = input.parse::<u8>().unwrap_or(10);
                        let current = input_mutable.get().unwrap_or(10);
                        if current != value {
                            let opt = (value < 10).then(|| value);
                            input_mutable.set(opt);
                            if value < 10 {
                                if id < 5 {
                                    page.focus_at.set(id + 1);
                                } else {
                                    page.set_parent();
                                }
                            }
                        } else {
                            input_mutable.set(None);
                        }
                    }
                }))
                .event(clone!(input_mutable => move |_:events::Focus| {
                    input_mutable.set(None);
                }))
                .event_with_options(&EventOptions::preventable(), move |event: events::KeyDown| {
                    if event.key() == "Backspace" && id > 0 {
                        event.prevent_default();
                        let prev_input = page.get_input(id - 1);
                        prev_input.set(None);
                        page.focus_at.set(id - 1);
                    }
                })
            })
        })
    }
}
