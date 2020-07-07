use crate::models::response::AppResponse;
use crate::models::request::AppRequest;


/*
    This will send a request to another device using the following url schema.
    The service will be responsible for determining the schema to be used. Currently webrtc is the
    only way to make requests to other devices.

    device_id/path_name?query=params

 */

pub async fn send_request(request: AppRequest) -> AppResponse {

    AppResponse {
        status_code: "".to_string(),
        headers: None,
        body: None
    }
}