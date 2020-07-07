use crate::models::request::AppRequest;
use crate::models::device::Device;
use crate::adapters::webrtc_adapter::{create_offer, create_answer, accept_answer};
use crate::repositories::device_repository::local_device;
use crate::adapters::websocket_adapter::send_message_via_websocket;
use crate::models::response::AppResponse;
use regex::Regex;
use uuid::Uuid;
use wasm_bindgen::__rt::core::str::FromStr;
use crate::log;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct AcceptOfferBody {
    pub offer: String,
    pub from_device: Device,
}

#[derive(Debug, Serialize, Deserialize)]
struct AcceptAnswerBody {
    pub answer: String,
    pub from_device: Device,
}

pub async fn create_offer_route(request: AppRequest) -> AppResponse {
    let device_to_send_offer: Device = serde_json::from_str(request.body.as_str()).unwrap();
    let from_device = local_device().await.unwrap();
    let offer = create_offer(device_to_send_offer.clone()).await;
    let path = format!("/webrtc-connection/{}/accept-offer", device_to_send_offer.id.to_string());
    let body = AcceptOfferBody {
        offer,
        from_device,
    };
    let app_request = AppRequest {
        path: path.to_string(),
        method: "POST".to_string(),
        body: json!(body).to_string(),
    };
    send_message_via_websocket(json!(app_request).to_string()).await;

    // TODO some factory for an empty response.  The caller doesn't care about this.
    AppResponse {
        status_code: "".to_string(),
        headers: None,
        body: None,
    }
}



fn accept_offer_route_path_param(request: AppRequest) -> String {
    let captures = accept_offer_route_regex().captures(request.path.as_str()).unwrap();

    captures.name("device_id").unwrap().as_str().to_string()
}
pub fn accept_offer_route_regex() -> Regex {
    Regex::new(r"/webrtc-connection/(?P<device_id>.*)/accept-offer").unwrap()
}

pub async fn accept_offer_route(request: AppRequest) -> AppResponse {
    let accept_offer_body: AcceptOfferBody = serde_json::from_str(request.body.as_str()).unwrap();
    let request_for_local_device_id = accept_offer_route_path_param(request);
    let device_for = Uuid::from_str(request_for_local_device_id.as_str()).unwrap();
    let local_device = local_device().await.unwrap();


    //only accept offers that are meant for the local device. The websocket publishes them to everyone
    if local_device.id.eq(&device_for) {
        let answer = create_answer(accept_offer_body.clone().from_device, accept_offer_body.clone().offer).await;

        let accept_answer_body = AcceptAnswerBody {
            answer: answer,
            from_device: local_device,
        };

        let path = format!("/webrtc-connection/{}/accept-answer", accept_offer_body.from_device.id.to_string());

        let app_request = AppRequest {
            path: path.to_string(),
            method: "POST".to_string(),
            body: json!(accept_answer_body).to_string(),
        };

        send_message_via_websocket(json!(app_request).to_string()).await;
    }

    AppResponse {
        status_code: "".to_string(),
        headers: None,
        body: None,
    }
}


fn accept_answer_route_path_param(request: AppRequest) -> String {
    let captures = accept_answer_route_regex().captures(request.path.as_str()).unwrap();

    captures.name("device_id").unwrap().as_str().to_string()
}

pub fn accept_answer_route_regex() -> Regex {
    Regex::new(r"/webrtc-connection/(?P<device_id>.*)/accept-answer").unwrap()
}

pub async fn accept_answer_route(request: AppRequest) -> AppResponse {
    let accept_answer_body: AcceptAnswerBody = serde_json::from_str(request.body.as_str()).unwrap();
    let request_for_local_device_id = accept_answer_route_path_param(request);
    let device_for = Uuid::from_str(request_for_local_device_id.as_str()).unwrap();
    let local_device = local_device().await.unwrap();

    if local_device.id.eq(&device_for) {
        accept_answer(accept_answer_body.answer.to_string(), accept_answer_body.from_device);

    }

    AppResponse {
        status_code: "".to_string(),
        headers: None,
        body: None
    }
}