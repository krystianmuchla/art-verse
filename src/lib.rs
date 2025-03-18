extern crate console_error_panic_hook;

use std::cell::RefCell;
use std::rc::Rc;

use crate::dom::Dom;
use wasm_bindgen::prelude::*;

mod canvas;
mod color;
mod dom;
mod line;
mod point;
mod resizer;
mod segment;
mod tool;
mod util;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn entry_point() {
    console_error_panic_hook::set_once();
    let dom = Rc::new(RefCell::new(Dom::new()));

    tool::tool_bar::init(Rc::clone(&dom));
    canvas::canvas::init(Rc::clone(&dom));
}
