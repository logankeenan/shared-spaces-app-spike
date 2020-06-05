use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Request {
    pub(crate) path: String,
}

#[wasm_bindgen]
impl Request {
    #[wasm_bindgen(constructor)]
    pub fn new(path: String) -> Request{
        Request {
            path: path.into()
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
}