use crate::models::request::AppRequest;
use regex::Regex;
use uuid::Uuid;
use std::str::FromStr;
use crate::repositories::file_part_repository::select_all_file_parts_by_file_id;
use crate::models::file_part::FilePart;
use crate::models::response::AppResponse;


#[derive(Debug, Serialize, Deserialize)]
struct FilePartsViewModel {
    data: Vec<FilePart>
}

fn file_id_path_param(request: AppRequest) -> Uuid {
    let captures = file_parts_api_route_regex().captures(request.path.as_str()).unwrap();

    let file_id_as_string = captures.name("file_parts").unwrap().as_str().to_string();

    Uuid::from_str(&file_id_as_string).unwrap()
}

pub fn file_parts_api_route_regex() -> Regex {
    Regex::new(r"/api/files/(?P<file_id>.*)/file_parts").unwrap()
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