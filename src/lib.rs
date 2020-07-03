use wasm_bindgen::prelude::*;
use crate::models::request::Request;
use crate::controllers::file_controller::{file_list, file_create, file_details};
use crate::models::response::AppResponse;
use crate::controllers::device_controller::{create_device_route, save_device_route};

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

    log(format!("request.path: {}", request.path).as_str());
    log(format!("request.method: {}", request.method).as_str());

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