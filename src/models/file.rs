use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct File {

    #[serde(default = "crate::models::default_uuid")]
    pub id: Uuid,
    pub name: String,
    // TODO Create serde parser so this'll work with NaiveDateTime
    pub last_modified: i64,
    pub size: i32,
    pub file_type: String,
    pub location: String,
    #[serde(default = "crate::models::default_uuid")]
    pub created_by_device_id: Uuid
}