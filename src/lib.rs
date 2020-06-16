use wasm_bindgen::prelude::*;
use crate::models::request::Request;
use crate::controllers::file_controller::{file_list, file_create};

#[macro_use]
extern crate serde_json;

mod controllers;
mod models;
mod factories;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn app(request: Request) -> String {
    if request.path == "/files" {
        if request.method == "GET" {
            let markup = file_list(request);

            return markup;
        } else if request.method == "POST" {
            file_create(request);
        }
    }


    return "".to_string();
}