use wasm_bindgen::prelude::*;
use crate::models::request::Request;
use crate::controllers::file_controller::{file_list, file_create, file_details};
use crate::models::response::AppResponse;
use crate::controllers::device_controller::{create_device_route, save_device_route};
use crate::adapters::websocket_adapter::{create_web_socket_connection, send_message};
use crate::repositories::device_repository::local_device;
use crate::models::device::Device;
use percent_encoding::{percent_encode, NON_ALPHANUMERIC};

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
    let device_option = local_device().await;

    match device_option {
        None => {},
        Some(device) => {
            let device_name_encoded = percent_encode(device.name.as_bytes(), NON_ALPHANUMERIC);
            let url = format!("ws://localhost:3000/ws?id={}&name={}", device.id.to_simple(), device_name_encoded);

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


    AppResponse {
        status_code: "".to_string(),
        headers: Default::default(),
        body: None,
    }
}

#[wasm_bindgen]
pub fn websocket_on_open()  {
    log("web socket opened")
}

#[wasm_bindgen]
pub fn websocket_on_message(message: String)  {
    log(format!("message: {}", message).as_str());
}

#[wasm_bindgen]
pub fn websocket_on_close()  {
    log("web socket closed")
}