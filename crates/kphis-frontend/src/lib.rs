mod router;

use dominator::{clone, routing};
use std::rc::Rc;
use wasm_bindgen::{JsCast, closure::Closure, prelude::wasm_bindgen};
use web_sys::{Event, HtmlDivElement};

use kphis_model::app::AppState;
use kphis_ui_app::App;

#[wasm_bindgen(start)]
pub fn main_js() {
    // wasm_logger::init(wasm_logger::Config::default());
    // std::panic::set_hook(Box::new(on_panic));

    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();
    // log::info!("wasm logging enabled");

    // let typst_worker = Rc::new(typst_worker::spawn("/typst_worker_init.js").await);
    let app_state = AppState::new_from_local_storage(&routing::url().lock_ref());
    let app = App::new(app_state);

    if let Some(app_elm) = app.get_id("app") {
        dominator::append_dom(&app_elm, router::render(app.clone()));
        init_visibility_checker(app.clone());
    }
    if let Some(splash_elm) = app.get_id("splash").and_then(|elm| elm.dyn_into::<HtmlDivElement>().ok()) {
        splash_elm.set_hidden(true);
    }
}

fn init_visibility_checker(app: Rc<App>) {
    app.window.with(|w| {
        let document = w.document().unwrap();
        let hidden_cs = Closure::<dyn FnMut(_)>::new(clone!(app, document => move |_: Event| {
            // hidden() will true when tab was closed or switched to another app
            app.visible.set_neq(!document.hidden());
        }));
        document.set_onvisibilitychange(Some(hidden_cs.as_ref().unchecked_ref()));
        hidden_cs.forget();
    });
}
