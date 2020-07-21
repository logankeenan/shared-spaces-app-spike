use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct FilePart {
    #[serde(default = "crate::models::default_uuid")]
    pub id: Uuid,
    pub order: i64,
    pub file_id: Uuid,
    pub md5_hash: String
}