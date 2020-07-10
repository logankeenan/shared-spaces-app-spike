use uuid::Uuid;

pub mod request;
pub mod file;
pub mod response;
pub mod device;
pub mod app_event;
pub mod device_status;

pub fn default_uuid() -> Uuid {
    Uuid::new_v4()
}
pub fn default_as_false() -> bool {
    false
}