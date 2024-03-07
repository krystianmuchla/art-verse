use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::closure::Closure;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent};

use crate::canvas;
use crate::dom::Dom;
use crate::point::Point;
use crate::tool;

use super::Events;

pub fn init(
    events: Rc<RefCell<Events>>,
    dom: Rc<Dom>,
    canvas: Rc<HtmlCanvasElement>,
    context: Rc<CanvasRenderingContext2d>,
    image_vec: Rc<RefCell<Clamped<Vec<u8>>>>,
) {
    let start = start(
        Rc::clone(&events),
        Rc::clone(&dom),
        Rc::clone(&canvas),
        Rc::clone(&context),
        Rc::clone(&image_vec),
    );
    events.borrow_mut().set_body_on_mouse_down(&*dom, &start);
    start.forget();
}

fn start(
    events: Rc<RefCell<Events>>,
    dom: Rc<Dom>,
    canvas: Rc<HtmlCanvasElement>,
    context: Rc<CanvasRenderingContext2d>,
    image_vec: Rc<RefCell<Clamped<Vec<u8>>>>,
) -> Closure<dyn FnMut(MouseEvent)> {
    Closure::wrap(Box::new(move |mouse_event: MouseEvent| {
        events.borrow_mut().remove_body_on_mouse_down(&*dom);
        let point_a = canvas::point_on_canvas(Rc::clone(&canvas), &mouse_event);
        let point_a = Rc::new(RefCell::new(point_a));
        let advance = advance(
            Rc::clone(&canvas),
            Rc::clone(&context),
            Rc::clone(&image_vec),
            Rc::clone(&point_a),
        );
        events.borrow_mut().set_body_on_mouse_move(&*dom, &advance);
        advance.forget();
        let end = end(
            Rc::clone(&events),
            Rc::clone(&dom),
            Rc::clone(&canvas),
            Rc::clone(&context),
            Rc::clone(&image_vec),
        );
        events.borrow_mut().set_body_on_mouse_up(&*dom, &end);
        events.borrow_mut().set_body_on_mouse_leave(&*dom, &end);
        end.forget();
    }) as Box<dyn FnMut(MouseEvent)>)
}

fn advance(
    canvas: Rc<HtmlCanvasElement>,
    context: Rc<CanvasRenderingContext2d>,
    image_vec: Rc<RefCell<Clamped<Vec<u8>>>>,
    point_a: Rc<RefCell<Point>>,
) -> Closure<dyn FnMut(MouseEvent)> {
    Closure::wrap(Box::new(move |mouse_event: MouseEvent| {
        let point_b = canvas::point_on_canvas(Rc::clone(&canvas), &mouse_event);
        let segment = canvas::segment_on_canvas(
            Rc::clone(&canvas),
            point_a.borrow().clone(),
            point_b.clone(),
        );
        if let Some(segment) = segment {
            tool::line::put(Rc::clone(&image_vec), canvas.width(), segment);
            canvas::put_image_vec(
                Rc::clone(&canvas),
                Rc::clone(&context),
                Rc::clone(&image_vec),
            );
        }
        *point_a.borrow_mut() = point_b;
    }) as Box<dyn FnMut(MouseEvent)>)
}

fn end(
    events: Rc<RefCell<Events>>,
    dom: Rc<Dom>,
    canvas: Rc<HtmlCanvasElement>,
    context: Rc<CanvasRenderingContext2d>,
    image_vec: Rc<RefCell<Clamped<Vec<u8>>>>,
) -> Closure<dyn FnMut(MouseEvent)> {
    Closure::wrap(Box::new(move |_: MouseEvent| {
        events.borrow_mut().remove_body_on_mouse_move(&*dom);
        events.borrow_mut().remove_body_on_mouse_up(&*dom);
        events.borrow_mut().remove_body_on_mouse_leave(&*dom);
        let start = start(
            Rc::clone(&events),
            Rc::clone(&dom),
            Rc::clone(&canvas),
            Rc::clone(&context),
            Rc::clone(&image_vec),
        );
        events.borrow_mut().set_body_on_mouse_down(&*dom, &start);
        start.forget();
    }) as Box<dyn FnMut(MouseEvent)>)
}
