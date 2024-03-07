use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::{prelude::Closure, Clamped, JsCast, JsValue};
use web_sys::{
    CanvasRenderingContext2d, HtmlCanvasElement, HtmlCollection, HtmlElement, MouseEvent,
};

use crate::dom::Dom;

mod line;
mod pencil;

pub struct Events {
    body_on_mouse_down: Option<JsValue>,
    body_on_mouse_move: Option<JsValue>,
    body_on_mouse_up: Option<JsValue>,
    body_on_mouse_leave: Option<JsValue>,
}

impl Events {
    pub fn new() -> Events {
        Events {
            body_on_mouse_down: None,
            body_on_mouse_move: None,
            body_on_mouse_up: None,
            body_on_mouse_leave: None,
        }
    }

    pub fn set_body_on_mouse_down(&mut self, dom: &Dom, event: &Closure<dyn FnMut(MouseEvent)>) {
        let event = event.as_ref().clone();
        dom.body.set_onmousedown(Some(event.unchecked_ref()));
        self.body_on_mouse_down = Some(event);
    }

    pub fn set_body_on_mouse_move(&mut self, dom: &Dom, event: &Closure<dyn FnMut(MouseEvent)>) {
        let event = event.as_ref().clone();
        dom.body.set_onmousemove(Some(event.unchecked_ref()));
        self.body_on_mouse_move = Some(event);
    }

    pub fn set_body_on_mouse_up(&mut self, dom: &Dom, event: &Closure<dyn FnMut(MouseEvent)>) {
        let event = event.as_ref().clone();
        dom.body.set_onmouseup(Some(event.unchecked_ref()));
        self.body_on_mouse_up = Some(event);
    }

    pub fn set_body_on_mouse_leave(&mut self, dom: &Dom, event: &Closure<dyn FnMut(MouseEvent)>) {
        let event = event.as_ref().clone();
        dom.body.set_onmouseup(Some(event.unchecked_ref()));
        self.body_on_mouse_leave = Some(event);
    }

    pub fn remove_body_on_mouse_down(&mut self, dom: &Dom) {
        self.body_on_mouse_down = None;
        dom.body.set_onmousedown(None);
    }

    pub fn remove_body_on_mouse_move(&mut self, dom: &Dom) {
        self.body_on_mouse_move = None;
        dom.body.set_onmousemove(None);
    }

    pub fn remove_body_on_mouse_up(&mut self, dom: &Dom) {
        self.body_on_mouse_up = None;
        dom.body.set_onmouseup(None);
    }

    pub fn remove_body_on_mouse_leave(&mut self, dom: &Dom) {
        self.body_on_mouse_leave = None;
        dom.body.set_onmouseleave(None);
    }

    pub fn remove_all(&mut self, dom: &Dom) {
        self.remove_body_on_mouse_down(dom);
        self.remove_body_on_mouse_move(dom);
        self.remove_body_on_mouse_up(dom);
        self.remove_body_on_mouse_leave(dom);
    }

    pub fn pause(&self, dom: &Dom) {
        dom.body.set_onmousedown(None);
        dom.body.set_onmousemove(None);
        dom.body.set_onmouseup(None);
        dom.body.set_onmouseleave(None);
    }

    pub fn resume(&self, dom: &Dom) {
        if let Some(value) = self.body_on_mouse_down.as_ref() {
            dom.body.set_onmousedown(Some(value.unchecked_ref()));
        }
        if let Some(value) = self.body_on_mouse_move.as_ref() {
            dom.body.set_onmousemove(Some(value.unchecked_ref()));
        }
        if let Some(value) = self.body_on_mouse_up.as_ref() {
            dom.body.set_onmouseup(Some(value.unchecked_ref()));
        }
        if let Some(value) = self.body_on_mouse_leave.as_ref() {
            dom.body.set_onmouseleave(Some(value.unchecked_ref()));
        }
    }
}

pub fn entry_point(
    events: Rc<RefCell<Events>>,
    dom: Rc<Dom>,
    canvas: Rc<HtmlCanvasElement>,
    context: Rc<CanvasRenderingContext2d>,
    image_vec: Rc<RefCell<Clamped<Vec<u8>>>>,
) {
    let tools = Rc::new(dom.document.get_elements_by_class_name("tool"));
    for tool_index in 0..tools.length() {
        let tool = Rc::new(
            tools
                .item(tool_index)
                .unwrap()
                .dyn_into::<HtmlElement>()
                .unwrap(),
        );
        let on_select = on_tool_select(
            Rc::clone(&events),
            Rc::clone(&dom),
            Rc::clone(&tools),
            Rc::clone(&tool),
            Rc::clone(&canvas),
            Rc::clone(&context),
            Rc::clone(&image_vec),
        );
        tool.set_onclick(Some(on_select.as_ref().unchecked_ref()));
        on_select.forget();
    }
}

fn on_tool_select(
    events: Rc<RefCell<Events>>,
    dom: Rc<Dom>,
    tools: Rc<HtmlCollection>,
    tool: Rc<HtmlElement>,
    canvas: Rc<HtmlCanvasElement>,
    context: Rc<CanvasRenderingContext2d>,
    image_vec: Rc<RefCell<Clamped<Vec<u8>>>>,
) -> Closure<dyn FnMut()> {
    Closure::<dyn FnMut()>::new(move || {
        events.borrow_mut().remove_all(&*dom);
        if tool.class_list().contains("selected") {
            tool.class_list().remove_1("selected").unwrap();
            return;
        }
        for tool_index in 0..tools.length() {
            tools
                .item(tool_index)
                .unwrap()
                .class_list()
                .remove_1("selected")
                .unwrap();
        }
        tool.class_list().add_1("selected").unwrap();
        match tool.id().as_str() {
            "pencil" => pencil::init(
                Rc::clone(&events),
                Rc::clone(&dom),
                Rc::clone(&canvas),
                Rc::clone(&context),
                Rc::clone(&image_vec),
            ),
            "line" => line::init(
                Rc::clone(&events),
                Rc::clone(&dom),
                Rc::clone(&canvas),
                Rc::clone(&context),
                Rc::clone(&image_vec),
            ),
            _ => panic!("Unsupported tool"),
        }
    })
}
