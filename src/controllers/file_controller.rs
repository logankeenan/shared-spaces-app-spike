use crate::models::request::AppRequest;
use handlebars::{Handlebars, TemplateRenderError};
use wasm_bindgen::__rt::std::alloc::handle_alloc_error;
use crate::factories::template_factory::render;
use crate::models::file::{File, FileDownloadStatus};
use serde_json::Error;
use crate::repositories::file_repository::{insert_file, select_all_files, file_by_id, update_file};
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
use regex::Regex;
use crate::controllers::file_part_controller::{FilePartsViewModel, FilePartsContentViewModel};
use crate::adapters::localforage_adapter::insert_by_id;

#[derive(Debug, Serialize, Deserialize)]
struct FileListViewModel {
    files: Vec<File>,
    device_statuses: Vec<DeviceStatus>,
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
            let mut files: Vec<File> = Vec::new();
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
                let result: Result<FileListViewModel, Error> = serde_json::from_str(app_response.body.unwrap().as_str());
                let remote_files = result.unwrap();
                for mut file in remote_files.files {
                    file.created_by_device_id = device_status.device_id;
                    insert_file(file).await
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
    let mut result: FileForm = serde_json::from_str(_request.body.as_str()).unwrap();
    let local_device = local_device().await.unwrap();

    result.file.created_by_device_id = local_device.id;
    result.file.download_status = FileDownloadStatus::Downloaded;

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

fn file_id_path_param(request: AppRequest) -> Uuid {
    let captures = file_download_route_regex().captures(request.path.as_str()).unwrap();

    let file_id_as_string = captures.name("file_part_id").unwrap().as_str().to_string();

    Uuid::from_str(&file_id_as_string).unwrap()
}

pub fn file_download_route_regex() -> Regex {
    Regex::new(r"/api/files/(?P<file_id>.*)/download").unwrap()
}

pub async fn file_download_route(request: AppRequest) -> AppResponse {
    let file_id = file_id_path_param(request);

    let mut file = file_by_id(file_id).await;
    let created_by_device_id = file.created_by_device_id.clone();
    let file_location = file.location.clone();

    let mut file_update_download_status = file.clone();
    file_update_download_status.download_status = FileDownloadStatus::Downloading;

    update_file(file_update_download_status).await;

    let file_parts_app_request = AppRequest {
        path: format!("/api/files/{}/file-parts", file_id.to_string()),
        method: "GET".to_string(),
        body: "".to_string(),
    };

    let app_response = send_webrtc_message(
        file_parts_app_request,
        created_by_device_id,
    ).await;

    let result: Result<FilePartsViewModel, Error> = serde_json::from_str(app_response.body.unwrap().as_str());
    let file_parts_view_model = result.unwrap();

    let mut file_as_string: String = "".to_string();
    for file_part in file_parts_view_model.data {
        let file_part_content_request = AppRequest {
            path: format!("/api/file_parts/{}/content", file_part.id),
            method: "GET".to_string(),
            body: "".to_string(),
        };


        let app_response = send_webrtc_message(
            file_part_content_request,
            created_by_device_id,
        ).await;

        let result: Result<FilePartsContentViewModel, Error> = serde_json::from_str(app_response.body.unwrap().as_str());
        let file_parts_view_model = result.unwrap();

        file_as_string.push_str(file_parts_view_model.data.as_str());
    }

    insert_by_id(file_as_string.to_string(), file_location);

    file.download_status = FileDownloadStatus::Downloaded;
    update_file(file.clone()).await;

    AppResponse {
        status_code: "201".to_string(),
        headers: None,
        body: None,
    }
}

