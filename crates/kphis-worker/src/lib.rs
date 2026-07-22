pub use transfer::{MessageReturn, MessageSend};

use discard::Discard;
use futures_channel::mpsc::{UnboundedReceiver, unbounded};
use futures_core::stream::Stream;
use futures_signals::signal::SignalExt;
use futures_util::stream::StreamExt;
use gloo_events::EventListener;
use js_sys::{Array, JsString, Object, global};
use std::{
    fmt::Debug,
    future::Future,
    pin::Pin,
    sync::atomic::{AtomicUsize, Ordering},
    task::{Context, Poll},
};
use wasm_bindgen::{JsCast, prelude::*};
use wasm_bindgen_futures::JsFuture;
use web_sys::{DedicatedWorkerGlobalScope, MessageEvent, WorkerOptions, WorkerType, window};

use kphis_ui_core::popups::with_close::WithClosePopup;

mod arguments;
mod macros;
mod transfer;
use crate::arguments::Arguments;

// JsMessage, is a new-type for 'event.data' passed by PostMessage
// { loaded: bool, call: string, id: number, value: Object }
// JsMessage.id(): 0 is log, 1.. is calling
// JsMessage.value(): to worker = args; to main = log is text message or value return from worker
#[wasm_bindgen]
extern "C" {
    #[derive(Debug, Clone)]
    pub type JsMessage;

    // loaded getter/setter
    #[wasm_bindgen(method, getter)]
    fn loaded(this: &JsMessage) -> Option<bool>;

    #[wasm_bindgen(method, setter)]
    fn set_loaded(this: &JsMessage, value: bool);

    // call getter/setter
    #[wasm_bindgen(method, getter)]
    fn call(this: &JsMessage) -> String;

    #[wasm_bindgen(method, setter)]
    fn set_call(this: &JsMessage, value: &str);

    // args for caller name (is JsMessage.value() only for args when sending message to worker)
    #[wasm_bindgen(method, getter, js_name = value)]
    fn args(this: &JsMessage) -> Vec<JsValue>;

    // id getter/setter
    #[wasm_bindgen(method, getter)]
    fn id(this: &JsMessage) -> usize;

    #[wasm_bindgen(method, setter)]
    pub fn set_id(this: &JsMessage, value: usize);

    // value getter/setter
    #[wasm_bindgen(method, getter)]
    fn value(this: &JsMessage) -> JsValue;

    #[wasm_bindgen(method, setter)]
    pub fn set_value(this: &JsMessage, value: &JsValue);
}

// message system used by both 'to worker' and 'from worker'
#[derive(Debug)]
struct WorkerMessages {
    receiver: UnboundedReceiver<JsMessage>,
    _listener: EventListener,
}

impl Stream for WorkerMessages {
    type Item = JsMessage;

    #[inline]
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.receiver).poll_next(cx)
    }
}

#[derive(Debug)]
pub struct Worker {
    worker: web_sys::Worker,
}

impl Worker {
    pub fn new(url: &str) -> Self {
        let options = WorkerOptions::new();
        options.set_type(WorkerType::Module);
        Self {
            worker: web_sys::Worker::new_with_options(url, &options).unwrap(),
        }
    }

    fn messages(&self) -> WorkerMessages {
        let (sender, receiver) = unbounded();

        WorkerMessages {
            _listener: EventListener::new(&self.worker, "message", move |e| {
                let e: &MessageEvent = e.unchecked_ref();
                sender.unbounded_send(e.data().unchecked_into()).unwrap();
            }),
            receiver,
        }
    }

    fn send_message(&self, message: &JsValue, transfer: &Array) {
        self.worker.post_message_with_transfer(message, transfer).unwrap()
    }

    // call api!'s function by function name
    pub fn call(&self, name: &'static str) -> WorkerCall<'_> {
        WorkerCall::new(self, name)
    }

    pub fn wait_loaded(&self) -> impl Future<Output = ()> + use<> {
        // start event listener
        let mut messages = self.messages();

        async move {
            while let Some(message) = messages.next().await {
                if message.loaded().is_some() {
                    return;
                }
            }

            unreachable!();
        }
    }
}

// represent 'message of function call'
#[derive(Debug)]
pub struct WorkerCall<'a> {
    worker: &'a Worker,
    name: &'static str, // function name
    transfer: Array,
    args: Array, // function arguments
}

impl<'a> WorkerCall<'a> {
    fn new(worker: &'a Worker, name: &'static str) -> Self {
        Self {
            worker,
            name,
            transfer: Array::new(),
            args: Array::new(),
        }
    }
    // add args to message
    pub fn arg<A>(self, value: A) -> Self
    where
        A: MessageSend,
    {
        self.args.push(&A::into_js(value, &self.transfer));
        self
    }
    // send JsMessage as a calling function
    pub fn send<A>(self) -> impl Future<Output = A> + use<A>
    where
        A: MessageReturn + Debug,
    {
        static ID: AtomicUsize = AtomicUsize::new(1);

        let id = ID.fetch_add(1, Ordering::SeqCst);
        let name = self.name;

        let message: JsMessage = Object::new().unchecked_into();
        message.set_call(name);
        message.set_id(id);
        message.set_value(&self.args);

        self.worker.send_message(&message, &self.transfer);

        // add event-listener every call
        // TODO use a single message dispatcher instead
        let mut messages = self.worker.messages();

        // future for intercepting return-message
        async move {
            while let Some(message) = messages.next().await {
                // id == 0 is a log
                if message.id() == 0 {
                    let message = String::from_js(&message.value());
                    let window = window().unwrap();
                    // save to clipboard
                    if let Err(e) = JsFuture::from(window.navigator().clipboard().write_text(&message)).await {
                        log::error!("{:?}", e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("Cannot save to Clipboard")));
                    }

                    // red alert box

                    // if let Some(elm) = window.document().unwrap().get_element_by_id("alert") {
                    //     if let Some(title_elm) = elm.first_element_child() {
                    //         title_elm.set_text_content(Some("Typst Error"));
                    //     }
                    //     if let Some(message_elm) = elm.last_element_child() {
                    //         message_elm.set_text_content(Some(&message));
                    //     }
                    //     elm.class_list().add_2("show", "danger").unwrap();
                    //     let show = Timeout::new(7000, move || {
                    //         elm.class_list().remove_2("show", "danger").unwrap();
                    //     });
                    //     show.forget();
                    // }

                    let document = window.document().unwrap();
                    let popup = WithClosePopup::new("Typst Error", &message, true);
                    // bootstrap modal will lock focus only within .modal-content
                    // so we need to append to '.modal.show .modal-body' if exist
                    match document.query_selector(".modal.show .modal-body").ok().flatten().or(document.get_element_by_id("popup")) {
                        Some(parent) => {
                            let handle = dominator::append_dom(&parent, WithClosePopup::render(popup.clone()));
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
                } else if message.id() == id {
                    return A::from_js(&message.value());
                }
            }

            panic!("Message call {} failed: stream closed", name);
        }
    }
}

#[derive(Debug)]
pub struct SpawnedWorker {
    worker: DedicatedWorkerGlobalScope,
}

impl Default for SpawnedWorker {
    fn default() -> Self {
        Self::new()
    }
}

impl SpawnedWorker {
    pub fn new() -> Self {
        Self { worker: global().unchecked_into() }
    }

    fn messages(&self) -> WorkerMessages {
        let (sender, receiver) = unbounded();

        WorkerMessages {
            _listener: EventListener::new(&self.worker, "message", move |e| {
                let e: &MessageEvent = e.unchecked_ref();
                sender.unbounded_send(e.data().unchecked_into()).unwrap();
            }),
            receiver,
        }
    }

    // worker's listener for compute calling message
    pub fn listen<A, F>(&self, mut f: F) -> impl Future<Output = ()> + use<A, F>
    where
        A: Future<Output = ()>,
        F: FnMut(Message) -> A,
    {
        // This must be outside of the async block so that way
        // it adds the event listener immediately instead of on the next tick
        let messages = self.messages();

        let worker = self.worker.clone();

        async move {
            // poll data from unbounded channel receiver
            messages.for_each_concurrent(None, move |message| f(Message { worker: worker.clone(), js: message })).await;
        }
    }

    pub fn set_loaded(&self) {
        let message: JsMessage = Object::new().unchecked_into();
        message.set_loaded(true);
        self.worker.post_message(&message).unwrap();
    }
}

// for sending to main js
#[derive(Debug)]
pub struct Message {
    worker: DedicatedWorkerGlobalScope,
    js: JsMessage,
}

impl Message {
    pub fn name(&self) -> String {
        self.js.call()
    }

    pub fn with_args<A, R, F>(&self, f: F) -> R
    where
        A: Arguments,
        F: FnOnce(A) -> R,
    {
        f(A::from_vec(self.js.args()))
    }

    pub fn send<A>(&self, value: A)
    where
        A: MessageSend,
    {
        let id = self.js.id();

        let transfer = Array::new();

        let value = A::into_js(value, &transfer);

        let output: JsMessage = Object::new().unchecked_into();
        output.set_id(id);
        output.set_value(&value);

        self.worker.post_message_with_transfer(&output, &transfer).unwrap();
    }
}
