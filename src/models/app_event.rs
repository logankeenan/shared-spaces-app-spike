use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppEvent {
    pub(crate) event_type: String,
    pub(crate) body: String,
}

// TODO add enum with the method types GET POST for now
// TODO add content TYPE for the request application/json for the posting of the file
#[wasm_bindgen]
impl AppEvent {
    #[wasm_bindgen(constructor)]
    pub fn new(event_type: String) -> AppEvent {
        AppEvent {
            event_type: event_type.into(),
            body: "".to_string()
        }
    }

    #[wasm_bindgen(getter)]
    pub fn event_type(&self) -> String {
        self.event_type.to_string()
    }

    #[wasm_bindgen(setter)]
    pub fn set_event_type(&mut self, event_type: String) {
        self.event_type = event_type;
    }

    #[wasm_bindgen(getter)]
    pub fn body(&self) -> String {
        self.body.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_body(&mut self, body: String) {
        self.body = body;
    }
}
