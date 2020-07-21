use crate::models::request::AppRequest;
use handlebars::{Handlebars, TemplateRenderError};
use wasm_bindgen::__rt::std::alloc::handle_alloc_error;
use crate::factories::template_factory::render;
use crate::models::file::File;
use serde_json::Error;
use crate::repositories::file_repository::{insert_file, select_all_files, file_by_id};
use uuid::Uuid;
use crate::models::response::AppResponse;
use wasm_bindgen::__rt::std::collections::HashMap;
use std::str::FromStr;
use crate::repositories::device_repository::local_device;
use crate::models::device::Device;
use crate::factories::app_response_factory::redirect_app_response;
use crate::adapters::webrtc_adapter::send_webrtc_message;
use crate::repositories::device_status_repository::{select_all_connected_device_statuses, select_all_device_statuses};
use crate::log;
use crate::models::device_status::DeviceStatus;
use crate::services::device_status_service::all_device_statuses_include_device;
use crate::services::file_service::save_file;

#[derive(Debug, Serialize, Deserialize)]
struct FileListViewModel {
    files: Vec<File>,
    device_statuses: Vec<DeviceStatus>
}

pub async fn files_api_route(request: AppRequest) -> AppResponse {
    let files = select_all_files().await;

    let view_model = FileListViewModel { files, device_statuses: vec![] };
    AppResponse {
        status_code: 200.to_string(),
        headers: None,
        body: Some(json!(view_model).to_string()),
    }
}

pub async fn files_route(_request: AppRequest) -> AppResponse {
    let device_option = local_device().await;

    match device_option {
        None => {
            redirect_app_response("/devices/create".to_string())
        }
        Some(_) => {
            let mut files:Vec<File> = Vec::new();
            let connected_device_statuses = select_all_connected_device_statuses().await;

            for device_status in connected_device_statuses.clone() {
                let request = AppRequest {
                    path: "/api/files".to_string(),
                    method: "GET".to_string(),
                    body: "".to_string(),
                };

                let app_response = send_webrtc_message(
                    request,
                    device_status.device_id,
                ).await;
                let result : Result<FileListViewModel, Error>  = serde_json::from_str(app_response.body.unwrap().as_str());
                let remote_files = result.unwrap();
                for file in remote_files.files {
                    files.push(file)
                }
            }

            let local_files = select_all_files().await;
            for file in local_files {
                files.push(file)
            }

            let device_statuses = all_device_statuses_include_device().await;
            let view_model = FileListViewModel { files, device_statuses };

            let model = json!(view_model);

            let string = render("file/list".to_string(), model);
            let response = AppResponse {
                status_code: 200.to_string(),
                headers: None,
                body: Some(string.clone()),
            };

            response
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct FileForm {
    file: File
}

// TODO add controller method to handle the file
//  This should take in the file and save the file to storage
pub async fn file_create_route(_request: AppRequest) -> AppResponse {
    let result: FileForm = serde_json::from_str(_request.body.as_str()).unwrap();


    save_file(result.file).await;

    redirect_app_response("/files".to_string())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileDetailsViewModel {
    file: File
}

pub async fn file_details_route(request: AppRequest) -> AppResponse {
    let uuid_as_string = request.path.replace("/files/", "");
    let file_id = Uuid::from_str(uuid_as_string.as_str()).unwrap();
    let file = file_by_id(file_id).await;

    let model = FileDetailsViewModel { file };
    let markup = render("file/details".to_string(), json!(model));

    AppResponse {
        status_code: 200.to_string(),
        headers: None,
        body: Some(markup),
    }
}

