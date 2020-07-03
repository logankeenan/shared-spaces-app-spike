use uuid::Uuid;
use wasm_bindgen::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
    #[serde(default = "crate::models::default_uuid")]
    pub id: Uuid,
    pub name: String,
    #[serde(default="crate::models::default_as_false")]
    pub is_local_device: bool
}
