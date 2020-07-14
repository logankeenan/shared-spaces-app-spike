use crate::models::app_event::AppEvent;
use crate::models::device::Device;
use crate::repositories::device_repository::{local_device, device_by_id, insert_device};
use crate::adapters::webrtc_adapter::{create_offer, create_answer, accept_answer};
use crate::adapters::websocket_adapter::send_message_via_websocket;
use uuid::Uuid;
use wasm_bindgen::__rt::core::str::FromStr;
use crate::models::device_status::{DeviceStatus, DeviceStatusState};
use wasm_bindgen::__rt::std::time::Instant;
use crate::repositories::device_status_repository::{insert_device_status, by_device_id, update_device_status};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct AcceptOfferBody {
    pub offer: String,
    pub from_device: Device,
    pub to_device: Device,
}

#[derive(Debug, Serialize, Deserialize)]
struct AcceptAnswerBody {
    pub answer: String,
    pub from_device: Device,
    pub to_device: Device,
}


pub async fn websocket_device_connected_listener(event: AppEvent) {
    let device_to_send_offer: Device = serde_json::from_str(event.body.as_str()).unwrap();
    let from_device = local_device().await.unwrap();
    let offer = create_offer(device_to_send_offer.clone()).await;
    let device_to_send_offer_already_exists = device_by_id(device_to_send_offer.id).await;

    match device_to_send_offer_already_exists {
        None => {
            insert_device(device_to_send_offer.clone()).await;
            let device_status = DeviceStatus {
                id: Uuid::new_v4(),
                device_id: device_to_send_offer.id.clone(),
                state: DeviceStatusState::NotConnected,
                device: None
            };
            insert_device_status(device_status).await;
        },
        Some(_) => {},
    }

    let body = AcceptOfferBody {
        offer,
        from_device,
        to_device: device_to_send_offer,
    };
    let app_event = AppEvent {
        event_type: "WEB_SOCKET_ACCEPT_OFFER".to_string(),
        body: json!(body).to_string(),
    };
    send_message_via_websocket(json!(app_event).to_string()).await;
}

pub async fn websocket_device_accept_offer_listener(event: AppEvent) {
    let accept_offer_body: AcceptOfferBody = serde_json::from_str(event.body.as_str()).unwrap();
    let local_device = local_device().await.unwrap();

    if local_device.id.eq(&accept_offer_body.to_device.id) {
        let answer = create_answer(accept_offer_body.clone().from_device, accept_offer_body.clone().offer).await;

        let accept_answer_body = AcceptAnswerBody {
            answer,
            from_device: local_device,
            to_device: accept_offer_body.from_device,
        };

        let app_event = AppEvent {
            event_type: "WEB_SOCKET_ACCEPT_ANSWER".to_string(),
            body: json!(accept_answer_body).to_string(),
        };

        send_message_via_websocket(json!(app_event).to_string()).await;
    }
}

pub async fn web_socket_device_accept_answer_listener(event: AppEvent) {
    let accept_answer_body: AcceptAnswerBody = serde_json::from_str(event.body.as_str()).unwrap();
    let local_device = local_device().await.unwrap();

    if local_device.id.eq(&accept_answer_body.to_device.id) {
        accept_answer(accept_answer_body.answer.to_string(), accept_answer_body.from_device);
    }
}

pub async fn web_socket_device_disconnected_listener(event: AppEvent) {
    let device_that_disconnected: Device = serde_json::from_str(event.body.as_str()).unwrap();
    let device_status_option = by_device_id(device_that_disconnected.id).await;

    match device_status_option {
        None => {

            // this should never happen
        },
        Some(mut device_status) => {
            device_status.state = DeviceStatusState::NotConnected;

            update_device_status(device_status).await;
        },
    }

}