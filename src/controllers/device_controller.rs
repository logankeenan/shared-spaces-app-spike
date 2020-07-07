use crate::models::request::AppRequest;
use crate::models::device::Device;
use crate::factories::template_factory::render;
use crate::models::response::AppResponse;
use crate::repositories::device_repository::insert_device;
use crate::factories::app_response_factory::redirect_app_response;
use crate::log;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct CreateDeviceViewModel {
    form: Device
}

pub async fn create_device_route(_request: AppRequest) -> AppResponse {
    let create_device_view_model = CreateDeviceViewModel {
        form: Device {
            id: Uuid::new_v4(),
            name: "".to_string(),
            is_local_device: true
        }
    };

    let markup = render("device/create".to_string(), json!(create_device_view_model));
    AppResponse {
        status_code: "200".to_string(),
        headers: None,
        body: Some(markup),
    }
}

pub async fn save_device_route(request: AppRequest) -> AppResponse {
    let mut device: Device = serde_json::from_str(request.body.as_str()).unwrap();

    device.is_local_device = true;

    insert_device(device).await;
    redirect_app_response("/files".to_string())
}

