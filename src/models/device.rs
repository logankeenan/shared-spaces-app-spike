use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Device {
    #[serde(default = "crate::models::default_uuid")]
    pub id: Uuid,
    pub name: String,
    #[serde(default="crate::models::default_as_false")]
    pub is_local_device: bool
}
