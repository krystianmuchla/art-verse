use wasm_bindgen::JsValue;

pub struct CanvasEvents {
    pub body_on_mouse_down: Option<JsValue>,
    pub body_on_mouse_move: Option<JsValue>,
    pub body_on_mouse_up: Option<JsValue>,
    pub body_on_mouse_leave: Option<JsValue>,
}

impl CanvasEvents {
    pub fn new() -> CanvasEvents {
        CanvasEvents {
            body_on_mouse_down: None,
            body_on_mouse_move: None,
            body_on_mouse_up: None,
            body_on_mouse_leave: None,
        }
    }
}
