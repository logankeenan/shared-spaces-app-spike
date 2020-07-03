use crate::models::response::AppResponse;
use wasm_bindgen::__rt::std::collections::HashMap;

pub fn redirect_app_response(location: String) -> AppResponse {
    let mut headers: HashMap<String, String> = HashMap::new();
    headers.insert("Location".to_string(), location);

    AppResponse {
        // TODO I'm not specific on this status code, but probably should be
        //  303 has the least implications about a request/redirect compared to the others
        status_code: "303".to_string(),
        headers: Some(json!(headers).to_string()),
        body: None,
    }
}