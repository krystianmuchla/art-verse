use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::closure::Closure;
use web_sys::MouseEvent;

use crate::color::Color;
use crate::dom::Dom;
use crate::point::Point;
use crate::tool;

pub fn init(dom: Rc<RefCell<Dom>>, color: Rc<RefCell<Color>>) {
    let tools = dom.borrow().document.get_elements_by_class_name("tool");
    for tool_idx in 0..tools.length() {
        let tool = tools.item(tool_idx).unwrap();
        tool.class_list().remove_1("selected").unwrap();
    }
    dom.borrow()
        .tool_bar
        .pencil
        .class_list()
        .add_1("selected")
        .unwrap();
    let start = start(Rc::clone(&dom), Rc::clone(&color));
    dom.borrow_mut().canvas.set_on_mouse_down(Some(&start));
    start.forget();
}

fn start(dom: Rc<RefCell<Dom>>, color: Rc<RefCell<Color>>) -> Closure<dyn FnMut(MouseEvent)> {
    Closure::wrap(Box::new(move |mouse_event: MouseEvent| {
        dom.borrow_mut().canvas.set_on_mouse_down(None);
        let point_a = dom.borrow().canvas.get_point(&mouse_event);
        let point_a = Rc::new(RefCell::new(point_a));
        let advance = advance(Rc::clone(&dom), Rc::clone(&point_a), Rc::clone(&color));
        dom.borrow_mut().canvas.set_on_mouse_move(Some(&advance));
        advance.forget();
        let end = end(Rc::clone(&dom), Rc::clone(&color));
        dom.borrow_mut().canvas.set_on_mouse_up(Some(&end));
        dom.borrow_mut().canvas.set_on_mouse_leave(Some(&end));
        end.forget();
    }) as Box<dyn FnMut(MouseEvent)>)
}

fn advance(
    dom: Rc<RefCell<Dom>>,
    point_a: Rc<RefCell<Point>>,
    color: Rc<RefCell<Color>>,
) -> Closure<dyn FnMut(MouseEvent)> {
    Closure::wrap(Box::new(move |mouse_event: MouseEvent| {
        let point_b = dom.borrow().canvas.get_point(&mouse_event);
        let segment = dom.borrow().canvas.get_segment(&point_a.borrow(), &point_b);
        if let Some(segment) = segment {
            let width = dom.borrow().canvas.element.width();
            tool::line::put(
                &mut dom.borrow_mut().canvas.pixels,
                &width,
                &segment,
                &color.borrow(),
            );
            dom.borrow().canvas.refresh();
        }
        *point_a.borrow_mut() = point_b;
    }) as Box<dyn FnMut(MouseEvent)>)
}

fn end(dom: Rc<RefCell<Dom>>, color: Rc<RefCell<Color>>) -> Closure<dyn FnMut(MouseEvent)> {
    Closure::wrap(Box::new(move |_: MouseEvent| {
        dom.borrow_mut().canvas.set_on_mouse_move(None);
        dom.borrow_mut().canvas.set_on_mouse_up(None);
        dom.borrow_mut().canvas.set_on_mouse_leave(None);
        let start = start(Rc::clone(&dom), Rc::clone(&color));
        dom.borrow_mut().canvas.set_on_mouse_down(Some(&start));
        start.forget();
    }) as Box<dyn FnMut(MouseEvent)>)
}
