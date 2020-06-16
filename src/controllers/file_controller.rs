use crate::models::request::Request;
use handlebars::{Handlebars, TemplateRenderError};
use wasm_bindgen::__rt::std::alloc::handle_alloc_error;
use crate::factories::template_factory::render;
use crate::log;

pub fn file_list(_request: Request) -> String {
    let model = json!({"dynamic_text": "Rust is pretty nice!"});

    render("file/list".to_string(), model)
}

// TODO add controller method to handle the file
//  This should take in the file and save the file to storage
pub fn file_create(_request: Request) -> String {
    log(_request.body.as_str());
    return "".to_string();
}

