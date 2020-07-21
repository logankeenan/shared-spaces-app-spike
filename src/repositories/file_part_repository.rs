use crate::adapters::localforage_adapter::{insert_json_string, json_entities_by_key_prefix};
use crate::models::file_part::FilePart;
use uuid::Uuid;
use serde_json::Error;

pub async fn insert_file_part(file_part: FilePart) {
    let file_part_as_json = serde_json::to_string(&file_part).unwrap();
    let key = format!("file_part:{}", file_part.id.to_string());

    insert_json_string(file_part_as_json.to_string(), key.to_string()).await;
}

pub async fn select_all_file_parts_by_file_id(file_id: Uuid) -> Vec<FilePart> {
    let json_entities = json_entities_by_key_prefix("file_part".to_string()).await;

    let mut file_parts = Vec::new();
    for json_entity in json_entities {
        let result1: Result<FilePart, Error> = serde_json::from_str(&json_entity);
        let device = result1.unwrap();
        file_parts.push(device)
    }

    file_parts.into_iter().filter(|file_part| {
        file_part.file_id.eq(&file_id)
    }).collect()
}