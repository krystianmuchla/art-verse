use crate::color::Color;
use crate::dom::Dom;
use crate::tool::{color_picker, line, pencil};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlElement};

pub struct ToolBar {
    pub pencil: HtmlElement,
    pub line: HtmlElement,
    pub color: HtmlElement,
}

impl ToolBar {
    pub fn new(document: &Document) -> ToolBar {
        let pencil = document
            .get_element_by_id("pencil")
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap();
        let line = document
            .get_element_by_id("line")
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap();
        let color = document
            .get_element_by_id("color")
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap();
        ToolBar {
            pencil,
            line,
            color,
        }
    }
}

pub fn init(dom: Rc<RefCell<Dom>>) {
    let color = Rc::new(RefCell::new(Color::black()));
    let tools = dom.borrow().document.get_elements_by_class_name("tool");
    for tool_idx in 0..tools.length() {
        let tool = tools.item(tool_idx).unwrap();
        match tool.id().as_str() {
            "pencil" => {
                let on_click = init_pencil(Rc::clone(&dom), Rc::clone(&color));
                dom.borrow()
                    .tool_bar
                    .pencil
                    .set_onclick(Some(on_click.as_ref().unchecked_ref()));
                on_click.forget();
            }
            "line" => {
                let on_click = init_line(Rc::clone(&dom), Rc::clone(&color));
                dom.borrow()
                    .tool_bar
                    .line
                    .set_onclick(Some(on_click.as_ref().unchecked_ref()));
                on_click.forget();
            }
            "color" => {
                let on_click = init_color_picker(Rc::clone(&dom), Rc::clone(&color));
                dom.borrow()
                    .tool_bar
                    .color
                    .set_onclick(Some(on_click.as_ref().unchecked_ref()));
                on_click.forget();
            }
            _ => panic!("Unsupported tool"),
        }
    }
}

fn init_pencil(dom: Rc<RefCell<Dom>>, color: Rc<RefCell<Color>>) -> Closure<dyn FnMut()> {
    Closure::<dyn FnMut()>::new(move || {
        pencil::init(Rc::clone(&dom), Rc::clone(&color));
    })
}

fn init_line(dom: Rc<RefCell<Dom>>, color: Rc<RefCell<Color>>) -> Closure<dyn FnMut()> {
    Closure::<dyn FnMut()>::new(move || {
        line::init(Rc::clone(&dom), Rc::clone(&color));
    })
}

fn init_color_picker(dom: Rc<RefCell<Dom>>, color: Rc<RefCell<Color>>) -> Closure<dyn FnMut()> {
    Closure::<dyn FnMut()>::new(move || {
        color_picker::init(Rc::clone(&dom), Rc::clone(&color));
    })
}
