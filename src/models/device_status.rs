use uuid::Uuid;
use crate::models::device::Device;


//TODO it'd be nice to the know the last time a device was connected
// I think it's possible with chrono https://github.com/chronotope/chrono/pull/335
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeviceStatus {
    #[serde(default = "crate::models::default_uuid")]
    pub id: Uuid,
    pub device_id: Uuid,

    #[serde(default="crate::models::device_status::default_device_status_state")]
    pub state: DeviceStatusState,

    pub device: Option<Device>

}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum  DeviceStatusState {
    NotConnected,
    Connected,
    Connecting
}

pub fn default_device_status_state() -> DeviceStatusState {
    DeviceStatusState::NotConnected
}