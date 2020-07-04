use crate::models::file::File;
use js_sys::{Promise, try_iter, ArrayIter, IntoIter};
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen::prelude::*;
use std::borrow::Borrow;
use wasm_bindgen::__rt::core::convert::{TryFrom, Infallible};


//TODO this is duplicated code and should be extracted somewhere
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = localforage)]
    fn getItem(key: &str) -> Promise;

    #[wasm_bindgen(js_namespace = localforage)]
    fn setItem(key: &str, value: &str) -> Promise;

    #[wasm_bindgen(js_namespace = localforage)]
    fn keys() -> Promise;
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


// TODO this should probably leveerage generics, but I don't want o figure that out right now
pub async fn json_entities_by_key_prefix(prefix: String) -> Vec<String> {
    let keys = keys_from_local_forage().await;

    let keys_for_file_records: Vec<String> = keys.into_iter()
        .filter(|key| key.starts_with(prefix.as_str()))
        .collect();
    let mut vec = Vec::new();

    for key in keys_for_file_records {
        let promise = getItem(&key);
        let promise_as_future = JsFuture::from(promise);
        let result = promise_as_future.await.unwrap();

        let json = result.as_string().unwrap();

        vec.push(json)
    }

    vec
}

pub async fn insert_json_string(json_string: String, key: String) {
    let promise: Promise = setItem(key.as_str(), json_string.as_str());
    let promise_as_future = JsFuture::from(promise);
    promise_as_future.await;
}

