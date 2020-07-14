use crate::models::device_status::DeviceStatus;
use crate::repositories::device_status_repository::select_all_device_statuses;
use crate::repositories::device_repository::device_by_id;
use crate::log;

pub async fn all_device_statuses_include_device() -> Vec<DeviceStatus> {
    let mut device_statuses_with_device = Vec::new();
    let mut device_statuses = select_all_device_statuses().await;

    for mut device_status in device_statuses {
        let device = device_by_id(device_status.device_id).await.unwrap();

        let mut device_status_with_device = device_status.clone();
        device_status_with_device.device = Option::from(device);
        device_statuses_with_device.push(device_status_with_device)
    }

    device_statuses_with_device
}