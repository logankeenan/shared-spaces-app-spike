use wasm_bindgen::prelude::*;
use crate::models::request::AppRequest;
use crate::controllers::file_controller::{files_route, file_create_route, file_details_route, files_api_route};
use crate::models::response::AppResponse;
use crate::controllers::device_controller::{create_device_route, save_device_route};
use crate::adapters::websocket_adapter::{create_web_socket_connection, send_message_via_websocket};
use crate::repositories::device_repository::local_device;
use crate::models::device::Device;
use percent_encoding::{percent_encode, NON_ALPHANUMERIC};
use crate::adapters::webrtc_adapter::create_answer;
use regex::Regex;
use crate::models::app_event::AppEvent;
use crate::listeners::websocket_listener::{websocket_device_accept_offer_listener, web_socket_device_accept_answer_listener, websocket_device_connected_listener};
use crate::repositories::device_status_repository::{select_all_device_statuses, update_device_status};
use crate::models::device_status::DeviceStatusState;

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

mod controllers;
mod models;
mod factories;
mod repositories;
mod adapters;
mod services;
mod listeners;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub async fn app_event(event: AppEvent) {

    if event.event_type.eq("WEB_SOCKET_DEVICE_CONNECTED") {
        websocket_device_connected_listener(event.clone()).await;
    }

    if event.event_type.eq("WEB_SOCKET_ACCEPT_OFFER") {
        websocket_device_accept_offer_listener(event.clone()).await;
    }


    if event.event_type.eq("WEB_SOCKET_ACCEPT_ANSWER") {
        web_socket_device_accept_answer_listener(event).await;
    }
}

#[wasm_bindgen]
pub async fn app_start() {
    let device_statuses = select_all_device_statuses().await;

    for mut device_status in device_statuses {
        device_status.state = DeviceStatusState::NotConnected;

        update_device_status(device_status).await
    }
}

#[wasm_bindgen]
pub async fn app(request: AppRequest) -> AppResponse {
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

    if request.path == "/api/files" {
        let response = files_api_route(request).await;
        return response;
    }

    if request.path == "/files" {
        if request.method == "GET" {
            let response = files_route(request).await;

            return response;
        } else if request.method == "POST" {
            let response = file_create_route(request).await;
            return response;
        }
    }

    if request.path.starts_with("/files/") {
        let response = file_details_route(request).await;
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

    AppResponse {
        status_code: "".to_string(),
        headers: Default::default(),
        body: None,
    }
}