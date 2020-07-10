use uuid::Uuid;


//TODO it'd be nice to the know the last time a device was connected
// I think it's possible with chrono https://github.com/chronotope/chrono/pull/335
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeviceStatus {
    #[serde(default = "crate::models::default_uuid")]
    pub id: Uuid,
    pub device_id: Uuid,

    #[serde(default="crate::models::default_as_false")]
    pub is_connected: bool
}