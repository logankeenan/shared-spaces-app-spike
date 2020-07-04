use wasm_bindgen::prelude::*;
use js_sys::{Promise, try_iter, ArrayIter, IntoIter};
use wasm_bindgen_futures::JsFuture;
use crate::{log, app};
use crate::models::request::Request;
use serde_json::Error;


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = wsAdapter)]
    fn create(url: &str);

    #[wasm_bindgen(js_namespace = wsAdapter)]
    fn sendMessage(message: &str) -> Promise;
}

#[wasm_bindgen]
pub fn websocket_on_open() {
    log("web socket opened")
}

#[wasm_bindgen]
pub async fn websocket_on_message(message: String) {
    let result: Result<Request, Error> = serde_json::from_str(message.as_str());

    match result {
        Ok(request) => {
            app(request).await;
        },
        Err(_) => {
            let string = format!("An error occurred websocket_on_message: {}", message);
            log(string.as_str());
        },
    }
}

#[wasm_bindgen]
pub fn websocket_on_close() {
    log("web socket closed")
}

pub fn create_web_socket_connection(url: String) {
    create(url.as_str());
}

pub async fn send_message_via_websocket(message: String) {
    let promise: Promise = sendMessage(message.as_str());

    let promise_as_future = JsFuture::from(promise);
    promise_as_future.await;
}

