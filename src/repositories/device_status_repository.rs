use crate::models::device_status::DeviceStatus;
use serde_json::Error;
use crate::adapters::localforage_adapter::{insert_json_string, json_entities_by_key_prefix};
use uuid::Uuid;
use crate::log;

pub async fn insert_device_status(device_status: DeviceStatus) {
    let device_status_as_json = serde_json::to_string(&device_status).unwrap();
    let key = format!("device_status:{}", device_status.id.to_string());

    insert_json_string(device_status_as_json.to_string(), key.to_string()).await;
}

pub async fn by_device_id(device_id: Uuid) -> Option<DeviceStatus> {
    let statuses = select_all_device_statuses().await;
    let device_status = statuses.into_iter().find(|device_status| {
        device_status.device_id.eq(&device_id)
    });

    device_status
}

pub async fn select_all_device_statuses() -> Vec<DeviceStatus> {
    let json_entities = json_entities_by_key_prefix("device_status".to_string()).await;

    let mut vec = Vec::new();
    for json_entity in json_entities {
        let result1: Result<DeviceStatus, Error> = serde_json::from_str(&json_entity);
        let device = result1.unwrap();
        vec.push(device)
    }

    vec
}

pub async fn update_device_status(device_status: DeviceStatus) {
    insert_device_status(device_status).await;
}