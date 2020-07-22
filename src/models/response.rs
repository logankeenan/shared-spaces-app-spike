use wasm_bindgen::prelude::*;
use wasm_bindgen::__rt::std::collections::HashMap;

#[wasm_bindgen]
#[derive(Deserialize, Serialize, Clone)]
pub struct AppResponse {
    pub(crate) status_code: String,
    pub(crate) headers: Option<String>,
    pub(crate) body: Option<String>,
}

#[wasm_bindgen]
impl AppResponse {
    #[wasm_bindgen(constructor)]
    pub fn new() -> AppResponse {
        AppResponse {
            status_code: "".to_string(),
            headers: None,
            body: None
        }
    }

    #[wasm_bindgen(method)]
    pub fn as_json_string(&self) -> String {
        let response = AppResponse {
            status_code: self.status_code.clone(),
            headers: self.headers.clone(),
            body: self.body.clone()
        };

        json!(response).to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn status_code(&self) -> String {
        self.status_code.to_string()
    }

    #[wasm_bindgen(setter)]
    pub fn set_status_code(&mut self, status_code: String) {
        self.status_code = status_code;
    }

    #[wasm_bindgen(getter)]
    pub fn body(&self) -> Option<String> {
        self.body.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_body(&mut self, body: String) {
        self.body = Some(body);
    }

    #[wasm_bindgen(getter)]
    pub fn headers(&self) -> Option<String> {
        self.headers.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_headers(&mut self, headers: Option<String>) {
        self.headers = headers;
    }
}