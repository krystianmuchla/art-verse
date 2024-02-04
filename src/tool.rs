use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::{Clamped, JsCast, prelude::Closure};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlCollection, HtmlElement};

use crate::dom::Dom;

mod line;
mod pencil;

pub fn entry_point(
    dom: Rc<Dom>,
    canvas: Rc<HtmlCanvasElement>,
    context: Rc<CanvasRenderingContext2d>,
    image_vec: Rc<RefCell<Clamped<Vec<u8>>>>,
) {
    let tools = Rc::new(dom.document.get_elements_by_class_name("tool"));
    for tool_index in 0..tools.length() {
        let tool = Rc::new(tools.item(tool_index).unwrap().dyn_into::<HtmlElement>().unwrap());
        let on_select = on_tool_select(
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
    dom: Rc<Dom>,
    tools: Rc<HtmlCollection>,
    tool: Rc<HtmlElement>,
    canvas: Rc<HtmlCanvasElement>,
    context: Rc<CanvasRenderingContext2d>,
    image_vec: Rc<RefCell<Clamped<Vec<u8>>>>,
) -> Closure<dyn FnMut()> {
    Closure::<dyn FnMut()>::new(
        move || {
            remove_listeners(Rc::clone(&dom), Rc::clone(&canvas));
            if tool.class_list().contains("selected") {
                tool.class_list().remove_1("selected").unwrap();
                return;
            }
            for tool_index in 0..tools.length() {
                tools.item(tool_index).unwrap().class_list().remove_1("selected").unwrap();
            }
            tool.class_list().add_1("selected").unwrap();
            match tool.id().as_str() {
                "pencil" => pencil::init(
                    Rc::clone(&dom),
                    Rc::clone(&canvas),
                    Rc::clone(&context),
                    Rc::clone(&image_vec),
                ),
                "line" => line::init(
                    Rc::clone(&dom),
                    Rc::clone(&canvas),
                    Rc::clone(&context),
                    Rc::clone(&image_vec),
                ),
                _ => panic!("Unsupported tool"),
            }
        }
    )
}

fn remove_listeners(dom: Rc<Dom>, canvas: Rc<HtmlCanvasElement>) {
    canvas.set_onmousemove(None);
    canvas.set_onmouseleave(None);
    canvas.set_onmouseenter(None);
    canvas.set_onmousedown(None);
    dom.body.set_onmousemove(None);
    dom.body.set_onmouseup(None);
    dom.body.set_onmouseleave(None);
}
