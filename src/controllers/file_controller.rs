use crate::models::request::Request;
use handlebars::{Handlebars, TemplateRenderError};
use wasm_bindgen::__rt::std::alloc::handle_alloc_error;
use crate::factories::template_factory::render;

pub fn file_list(_request: Request) -> String {
    let model = json!({"dynamic_text": "Rust is pretty nice!"});

    render("file/list".to_string(), model)
}