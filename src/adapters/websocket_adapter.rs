use wasm_bindgen::prelude::*;
use js_sys::{Promise, try_iter, ArrayIter, IntoIter};
use wasm_bindgen_futures::JsFuture;
use crate::{log, app, app_event};
use serde_json::Error;
use crate::models::app_event::AppEvent;


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = wsAdapter)]
    fn create(path: &str);

    #[wasm_bindgen(js_namespace = wsAdapter)]
    fn sendMessage(message: &str) -> Promise;
}

#[wasm_bindgen]
pub fn websocket_on_open() {

}

#[wasm_bindgen]
pub async fn websocket_on_message(message: String) {
    let result: Result<AppEvent, Error> = serde_json::from_str(message.as_str());

    match result {
        Ok(event) => {
            app_event(event).await;
        }
        Err(_) => {
            let string = format!("An error occurred websocket_on_message: {}", message);
        },
    }
}

#[wasm_bindgen]
pub fn websocket_on_close() {
}

pub fn create_web_socket_connection(path: String) {
    create(path.as_str());
}

pub async fn send_message_via_websocket(message: String) {
    let promise: Promise = sendMessage(message.as_str());

    let promise_as_future = JsFuture::from(promise);
    promise_as_future.await;
}

