use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub(crate) path: String,
    pub(crate) method: String,
    pub(crate) body: String,
}


// TODO add enum with the method types GET POST for now
// TODO add content TYPE for the request application/json for the posting of the file
#[wasm_bindgen]
impl Request {
    #[wasm_bindgen(constructor)]
    pub fn new(path: String, method: String) -> Request{
        Request {
            path: path.into(),
            method: method.into(),
            body: "".to_string()
        }
    }

    #[wasm_bindgen(getter)]
    pub fn path(&self) -> String {
        self.path.to_string()
    }

    #[wasm_bindgen(setter)]
    pub fn set_path(&mut self, path: String) {
        self.path = path;
    }

    #[wasm_bindgen(getter)]
    pub fn method(&self) -> String {
        self.method.to_string()
    }

    #[wasm_bindgen(setter)]
    pub fn set_method(&mut self, method: String) {
        self.method = method;
    }

    #[wasm_bindgen(getter)]
    pub fn body(&self) -> String {
        self.body.to_string()
    }

    #[wasm_bindgen(setter)]
    pub fn set_body(&mut self, body: String) {
        self.body = body;
    }
}