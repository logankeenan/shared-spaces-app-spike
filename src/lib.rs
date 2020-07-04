use wasm_bindgen::prelude::*;
use crate::models::request::Request;
use crate::controllers::file_controller::{file_list, file_create, file_details};
use crate::models::response::AppResponse;
use crate::controllers::device_controller::{create_device_route, save_device_route};
use crate::adapters::websocket_adapter::{create_web_socket_connection, send_message_via_websocket};
use crate::repositories::device_repository::local_device;
use crate::models::device::Device;
use percent_encoding::{percent_encode, NON_ALPHANUMERIC};
use crate::controllers::webrtc_connection_controller::{create_offer_route, accept_offer_route, accept_answer_route, accept_offer_route_regex, accept_answer_route_regex};
use crate::adapters::webrtc_adapter::create_answer;
use regex::Regex;

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

mod controllers;
mod models;
mod factories;
mod repositories;
mod adapters;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub async fn app(request: Request) -> AppResponse {
    log(format!("request path: {}", request.path).as_str());

    let device_option = local_device().await;

    match device_option {
        None => {},
        Some(device) => {
            let device_name_encoded = percent_encode(device.name.as_bytes(), NON_ALPHANUMERIC);
            // TODO this url really needs to use the correct host/port/protocol
            let url = format!("ws://localhost:3000/ws?id={}&name={}", device.id.to_string(), device_name_encoded);

            create_web_socket_connection(url);
        },
    }

    if request.path == "/files" {
        if request.method == "GET" {
            let response = file_list(request).await;

            return response;
        } else if request.method == "POST" {
            let response = file_create(request).await;
            return response;
        }
    }

    if request.path.starts_with("/files/") {
        let response = file_details(request).await;
        return response;
    }

    if request.path.starts_with("/devices") {
        if request.method == "POST" {
            return save_device_route(request).await;
        }

        if request.path.starts_with("/devices/create") {
            return create_device_route(request).await;
        }
    }

    if request.path.eq("/webrtc-connection/create-offer") {
        return create_offer_route(request).await;
    }

    if accept_offer_route_regex().is_match(request.path.as_str()) {
        return accept_offer_route(request).await;
    }

    if accept_answer_route_regex().is_match(request.path.as_str()) {
        return accept_answer_route(request).await;
    }


    AppResponse {
        status_code: "".to_string(),
        headers: Default::default(),
        body: None,
    }
}

#[wasm_bindgen]
pub fn webrtc_on_signal(message: String)  {
    log("webrtc on signal")
}

#[wasm_bindgen]
pub fn webrtc_on_connect(message: String)  {
    log("webrtc_on_connect")
}

#[wasm_bindgen]
pub fn webrtc_on_message(message: String)  {
    log("webrtc_on_message")
}