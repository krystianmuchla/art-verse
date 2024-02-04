extern crate console_error_panic_hook;

use std::rc::Rc;

use wasm_bindgen::prelude::*;
use web_sys::window;

use crate::dom::Dom;

mod canvas;
mod dom;
mod point;
mod tool;
mod color;
mod path;
mod resizer;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn entry_point() {
    console_error_panic_hook::set_once();
    let window = window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();
    let dom = Rc::new(Dom {
        window,
        document,
        body,
    });
    canvas::entry_point(Rc::clone(&dom));
}
