use crate::models::file::File;
use js_sys::{Promise, try_iter, ArrayIter, IntoIter};
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen::prelude::*;
use std::borrow::Borrow;
use wasm_bindgen::__rt::core::convert::{TryFrom, Infallible};
use serde_json::Error;
use uuid::Uuid;
use crate::repositories::localforage_adapter::{json_entities_by_key_prefix, insert_json_string};

pub async fn insert_file(file: File) {
    let file_as_json = serde_json::to_string(&file).unwrap();
    let key = format!("file:{}", file.id.to_simple());

    insert_json_string(file_as_json.to_string(), key.to_string()).await;
}

pub async fn file_by_id(id: Uuid) -> File {
    let files = select_all_files().await;

    let file = files.into_iter().find(|file| {
        file.id.eq(&id)
    }).unwrap();

    file
}

pub async fn select_all_files() -> Vec<File> {
    let json_entities = json_entities_by_key_prefix("file".to_string()).await;

    let mut vec = Vec::new();
    for json_entity in json_entities {
        let result1: Result<File, Error> = serde_json::from_str(&json_entity);
        let file = result1.unwrap();
        vec.push(file)
    }

    vec
}