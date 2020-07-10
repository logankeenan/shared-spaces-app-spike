use crate::models::device::Device;
use uuid::Uuid;
use serde_json::Error;
use crate::log;
use crate::adapters::localforage_adapter::{insert_json_string, json_entities_by_key_prefix};

pub async fn insert_device(device: Device) {
    let device_as_json = serde_json::to_string(&device).unwrap();
    let key = format!("device:{}", device.id.to_string());

    insert_json_string(device_as_json.to_string(), key.to_string()).await;
}

pub async fn device_by_id(id: Uuid) -> Option<Device> {
    let devices = select_all_devices().await;

    let device = devices.into_iter().find(|device| {
        device.id.eq(&id)
    });

    device
}

pub async fn local_device() -> Option<Device> {
    let devices = select_all_devices().await;
    let device = devices.into_iter().find(|device| {
        device.is_local_device.eq(&true)
    });

    device
}

pub async fn select_all_devices() -> Vec<Device> {
    let json_entities = json_entities_by_key_prefix("device".to_string()).await;

    let mut vec = Vec::new();
    for json_entity in json_entities {
        let result1: Result<Device, Error> = serde_json::from_str(&json_entity);
        let device = result1.unwrap();
        vec.push(device)
    }

    vec
}