use wasm_bindgen::prelude::*;
use crate::models::request::Request;
use crate::controllers::file_controller::{file_list, file_create};
use crate::models::response::AppResponse;

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

mod controllers;
mod models;
mod factories;
mod repositories;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub async fn app(request: Request) -> AppResponse {
    if request.path == "/files" {
        if request.method == "GET" {
            let response = file_list(request).await;

            return response;
        } else if request.method == "POST" {
            let response = file_create(request).await;
            return response;
        }
    }

    AppResponse {
        status_code: "".to_string(),
        headers: Default::default(),
        body: None
    }
}