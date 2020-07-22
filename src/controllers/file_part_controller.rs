use crate::models::request::AppRequest;
use regex::Regex;
use uuid::Uuid;
use std::str::FromStr;
use crate::repositories::file_part_repository::{select_all_file_parts_by_file_id, file_part_by_id};
use crate::models::file_part::FilePart;
use crate::models::response::AppResponse;
use crate::repositories::file_repository::file_by_id;
use crate::services::file_location_service::read_file_contents;
use crate::log;


#[derive(Debug, Serialize, Deserialize)]
pub struct FilePartsViewModel {
    pub data: Vec<FilePart>
}

fn file_id_path_param(request: AppRequest) -> Uuid {
    let captures = file_parts_api_route_regex().captures(request.path.as_str()).unwrap();

    let file_id_as_string = captures.name("file_id").unwrap().as_str().to_string();

    Uuid::from_str(&file_id_as_string).unwrap()
}

pub fn file_parts_api_route_regex() -> Regex {
    Regex::new(r"/api/files/(?P<file_id>.*)/file-parts").unwrap()
}

pub async fn file_parts_api_route(request: AppRequest) -> AppResponse {
    let file_id = file_id_path_param(request);
    let file_parts = select_all_file_parts_by_file_id(file_id).await;

    let model = FilePartsViewModel {
        data: file_parts
    };

    AppResponse {
        status_code: "200".to_string(),
        headers: None,
        body: Some(json!(model).to_string()),
    }
}

fn file_part_id_path_param(request: AppRequest) -> Uuid {
    let captures = file_part_content_route_regex().captures(request.path.as_str()).unwrap();

    let file_id_as_string = captures.name("file_part_id").unwrap().as_str().to_string();

    Uuid::from_str(&file_id_as_string).unwrap()
}

pub fn file_part_content_route_regex() -> Regex {
    Regex::new(r"/api/file-parts/(?P<file_part_id>.*)/content").unwrap()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilePartsContentViewModel {
    pub data: String
}

pub async fn file_part_content_route(request: AppRequest) -> AppResponse {
    let file_part_id = file_part_id_path_param(request);

    let file_part = file_part_by_id(file_part_id).await;
    let file = file_by_id(file_part.file_id).await;
    let file_contents = read_file_contents(file).await;
    let offset = 31999;
    let starting_position = offset * file_part.order as usize;

    let mut file_part_content = "";
    if starting_position + offset > file_contents.len() {
        file_part_content = &file_contents[starting_position..];
    } else {
        file_part_content = &file_contents[starting_position..starting_position + offset]
    }

    let model = FilePartsContentViewModel {
        data: file_part_content.to_string()
    };

    AppResponse {
        status_code: "200".to_string(),
        headers: None,
        body: Some(json!(model).to_string())
    }
}