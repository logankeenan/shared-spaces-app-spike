use uuid::Uuid;

pub mod request;
pub mod file;
pub mod response;

pub fn default_uuid() -> Uuid {
    Uuid::new_v4()
}