use crate::models::request::Request;
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

pub async fn create_offer_route(request: Request) -> AppResponse {
    log("1");
    let device_to_send_offer: Device = serde_json::from_str(request.body.as_str()).unwrap();
    log("2");

    let from_device = local_device().await.unwrap();
    log("3");
    let offer = create_offer(device_to_send_offer.clone()).await;
    log("4");
    let path = format!("/webrtc-connection/{}/accept-offer", device_to_send_offer.id.to_string());
    log("5");
    let body = AcceptOfferBody {
        offer,
        from_device,
    };
    log("6");
    let app_request = Request {
        path: path.to_string(),
        method: "POST".to_string(),
        body: json!(body).to_string(),
    };

    log("before send_message_via_websocket");
    send_message_via_websocket(json!(app_request).to_string()).await;
    log("after send_message_via_websocket");
    // TODO some factory for an empty response.  The caller doesn't care about this.
    AppResponse {
        status_code: "".to_string(),
        headers: None,
        body: None,
    }
}


// TODO this does not work as expected
fn accept_offer_route_path_param(request: Request) -> String {
    let re = Regex::new(r#"/webrtc-connection/(.*)/accept-offer"#).unwrap();
    let captures = re.captures(request.path.as_str()).unwrap();

    captures.get(0).unwrap().as_str().to_string()
}

pub async fn accept_offer_route(request: Request) -> AppResponse {
    log("accept_offer_route 1");
    let accept_offer_body: AcceptOfferBody = serde_json::from_str(request.body.as_str()).unwrap();
    log("accept_offer_route 2");
    let request_for_local_device_id = accept_offer_route_path_param(request);
    log("accept_offer_route 3");
    log(request_for_local_device_id.as_str());
    let device_for = Uuid::from_str(request_for_local_device_id.as_str()).unwrap();
    log("accept_offer_route 4");
    let local_device = local_device().await.unwrap();

    log("accept_offer_route 5");
    //only accept offers that are meant for the local device. The websocket publishes them to everyone
    if local_device.id.eq(&device_for) {
        log("accept_offer_route 6");
        let answer = create_answer(accept_offer_body.clone().from_device, accept_offer_body.clone().offer).await;

        let accept_answer_body = AcceptAnswerBody {
            answer: answer,
            from_device: local_device,
        };

        let path = format!("/webrtc-connection/{}/accept-answer", accept_offer_body.from_device.id.to_string());

        let app_request = Request {
            path: path.to_string(),
            method: "POST".to_string(),
            body: json!(accept_answer_body).to_string(),
        };

        log("accept_offer_route");
        send_message_via_websocket(json!(app_request).to_string()).await;
    }

    AppResponse {
        status_code: "".to_string(),
        headers: None,
        body: None,
    }
}


fn accept_answer_route_path_param(request: Request) -> String {
    let re = Regex::new(r#"/webrtc-connection/(.*)/accept-answer"#).unwrap();
    let captures = re.captures(request.path.as_str()).unwrap();

    captures.get(0).unwrap().as_str().to_string()
}

pub async fn accept_answer_route(request: Request) -> AppResponse {
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