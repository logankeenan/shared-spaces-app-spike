use crate::models::file::File;
use js_sys::{Promise, try_iter, ArrayIter, IntoIter};
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen::prelude::*;
use crate::log;
use std::borrow::Borrow;
use wasm_bindgen::__rt::core::convert::{TryFrom, Infallible};
use serde_json::Error;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = localforage)]
    fn getItem(key: &str) -> Promise;

    #[wasm_bindgen(js_namespace = localforage)]
    fn setItem(key: &str, value: &str) -> Promise;

    #[wasm_bindgen(js_namespace = localforage)]
    fn keys() -> Promise;
}

pub async fn insert_file(file: File) {
    let file_as_json = serde_json::to_string(&file).unwrap();

    let key = format!("file:{}", file.id.to_simple());

    let promise: Promise = setItem(key.as_str(), file_as_json.as_str());
    let promise_as_future = JsFuture::from(promise);
    promise_as_future.await;
}

pub async fn keys_from_local_forage() -> Vec<String> {
    let promise: Promise = keys();
    let promise_as_future = JsFuture::from(promise);
    let future_data = promise_as_future.await.unwrap();
    let future_data_as_iter = js_sys::try_iter(&future_data).unwrap().unwrap();
    let mut keys: Vec<String> = Vec::new();

    for iter_result in future_data_as_iter {
        let actual_value = iter_result.unwrap().as_string().unwrap();
        keys.push(actual_value)
    }

    keys
}

pub async fn select_all_files() -> Vec<File> {
    let keys = keys_from_local_forage().await;

    let keys_for_file_records: Vec<String> = keys.into_iter()
        .filter(|key| key.starts_with("file"))
        .collect();
    let mut vec = Vec::new();

    for key in keys_for_file_records {
        let promise = getItem(&key);
        let promise_as_future = JsFuture::from(promise);
        let result = promise_as_future.await.unwrap();

        let json = result.as_string().unwrap();
        let result1: Result<File, Error> = serde_json::from_str(&json);
        let file = result1.unwrap();
        vec.push(file)
    }

    vec
}