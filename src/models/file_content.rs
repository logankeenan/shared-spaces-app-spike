use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileContent {
    #[serde(default = "crate::models::default_uuid")]
    pub id: Uuid,
    pub content: String
}