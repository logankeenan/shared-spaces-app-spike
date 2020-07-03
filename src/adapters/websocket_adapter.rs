use wasm_bindgen::prelude::*;
use js_sys::{Promise, try_iter, ArrayIter, IntoIter};
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = wsAdapter)]
    fn create(url: &str);

    #[wasm_bindgen(js_namespace = wsAdapter)]
    fn sendMessage(message: &str) -> Promise;
}

pub fn create_web_socket_connection(url: String) {
    create(url.as_str());
}

pub async fn send_message(message: String) {
    let promise: Promise = sendMessage(message.as_str());

    let promise_as_future = JsFuture::from(promise);
    promise_as_future.await;
}

