use wasm_bindgen::prelude::*;
use js_sys::{Promise, try_iter, ArrayIter, IntoIter};
use wasm_bindgen_futures::JsFuture;
use crate::models::device::Device;
use crate::log;
use uuid::Uuid;
use wasm_bindgen::__rt::core::str::FromStr;
use crate::repositories::device_status_repository::{by_device_id, update_device_status};
use crate::models::device_status::DeviceStatus;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = simplePeerAdapter)]
    fn createSimplePeer(initiator: &str, device_id: &str, offer: &str) -> Promise;

    #[wasm_bindgen(js_namespace = simplePeerAdapter)]
    fn sendSimplePeerMessage(message: &str, device_id: &str) -> Promise;

    #[wasm_bindgen(js_namespace = simplePeerAdapter)]
    fn signalToSimplePeer(data: &str, device_id: &str);
}

pub async fn create_offer(device: Device) -> String {
    let promise: Promise = createSimplePeer(
        "true".to_string().as_str(),
        device.id.to_string().as_str(),
        "".to_string().as_str(),
    );

    let promise_as_future = JsFuture::from(promise);
    let result = promise_as_future.await.unwrap();
    let offer = result.as_string().unwrap();

    offer
}

pub async fn create_answer(device: Device, offer: String) -> String {
    let promise: Promise = createSimplePeer(
        "false".to_string().as_str(),
        device.id.to_string().as_str(),
        offer.as_str()
    );
    let promise_as_future = JsFuture::from(promise);
    let result = promise_as_future.await.unwrap();
    let answer = result.as_string().unwrap();

    answer
}

pub fn accept_answer(answer: String, device: Device) {
    signalToSimplePeer(answer.as_str(), device.id.to_string().as_str());
}

#[wasm_bindgen]
pub fn webrtc_on_signal(message: String)  {
    log("webrtc on signal")
}

#[wasm_bindgen]
pub async fn webrtc_on_connect(device_id_string: String)  {
    //TODO this should probably go in some sort of webrtc listener
    let device_id = Uuid::from_str(device_id_string.as_str()).unwrap();

    let device_option = by_device_id(device_id).await;

    match device_option {
        None => {
            // This should never occur
        },
        Some(mut device_status) => {
            device_status.is_connected = true;
            update_device_status(device_status);
        },
    }
}

#[wasm_bindgen]
pub fn webrtc_on_message(message: String)  {
    log("webrtc_on_message")
}