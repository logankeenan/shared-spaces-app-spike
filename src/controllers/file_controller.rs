use crate::models::request::Request;
use handlebars::{Handlebars, TemplateRenderError};
use wasm_bindgen::__rt::std::alloc::handle_alloc_error;
use crate::factories::template_factory::render;
use crate::log;
use crate::models::file::File;
use serde_json::Error;
use crate::repositories::file_repository::{insert_file, select_all_files};
use uuid::Uuid;
use crate::models::response::AppResponse;
use wasm_bindgen::__rt::std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct FileListViewModel {
    files: Vec<File>
}

pub async fn file_list(_request: Request) -> AppResponse {
    let files = select_all_files().await;
    let view_model = FileListViewModel { files };

    let model = json!(view_model);

    let string = render("file/list".to_string(), model);
    let mut map = HashMap::new();
    map.insert("hello".to_string(), "world".to_string());
    let response = AppResponse {
        status_code: 200.to_string(),
        headers: Some(json!(map).to_string()),
        body: Some(string.clone())
    };

    response
}
#[derive(Debug, Serialize, Deserialize)]
struct FileForm {
    file: File
}

// TODO add controller method to handle the file
//  This should take in the file and save the file to storage
pub async fn file_create(_request: Request) -> AppResponse {
    let result: FileForm = serde_json::from_str(_request.body.as_str()).unwrap();

    insert_file(result.file).await;

    let mut headers: HashMap<String, String> = HashMap::new();
    headers.insert("Location".to_string(), "/files".to_string());

    AppResponse {
        status_code: 303.to_string(),
        headers: Some(json!(headers).to_string()),
        body: None
    }
}

