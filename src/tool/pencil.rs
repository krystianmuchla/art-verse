use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::{Clamped, JsCast};
use wasm_bindgen::closure::Closure;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent};

use crate::canvas;
use crate::dom::Dom;
use crate::point::Point;
use crate::tool;

pub fn init(
    dom: Rc<Dom>,
    canvas: Rc<HtmlCanvasElement>,
    context: Rc<CanvasRenderingContext2d>,
    image_vec: Rc<RefCell<Clamped<Vec<u8>>>>,
) {
    let start = start(
        Rc::clone(&dom),
        Rc::clone(&canvas),
        Rc::clone(&context),
        Rc::clone(&image_vec),
    );
    dom.body.set_onmousedown(Some(start.as_ref().unchecked_ref()));
    start.forget();
}

fn start(
    dom: Rc<Dom>,
    canvas: Rc<HtmlCanvasElement>,
    context: Rc<CanvasRenderingContext2d>,
    image_vec: Rc<RefCell<Clamped<Vec<u8>>>>,
) -> Closure<dyn FnMut(MouseEvent)> {
    Closure::wrap(
        Box::new(
            move |mouse_event: MouseEvent| {
                let point_a = canvas::point_on_canvas(Rc::clone(&canvas), &mouse_event);
                let point_a = Rc::new(RefCell::new(point_a));
                dom.body.set_onmousedown(None);
                let advance = advance(
                    Rc::clone(&canvas),
                    Rc::clone(&context),
                    Rc::clone(&image_vec),
                    Rc::clone(&point_a),
                );
                dom.body.set_onmousemove(Some(advance.as_ref().unchecked_ref()));
                advance.forget();
                let end = end(
                    Rc::clone(&dom),
                    Rc::clone(&canvas),
                    Rc::clone(&context),
                    Rc::clone(&image_vec),
                );
                dom.body.set_onmouseup(Some(end.as_ref().unchecked_ref()));
                dom.body.set_onmouseleave(Some(end.as_ref().unchecked_ref()));
                end.forget();
            }
        ) as Box<dyn FnMut(MouseEvent)>
    )
}

fn advance(
    canvas: Rc<HtmlCanvasElement>,
    context: Rc<CanvasRenderingContext2d>,
    image_vec: Rc<RefCell<Clamped<Vec<u8>>>>,
    point_a: Rc<RefCell<Point>>,
) -> Closure<dyn FnMut(MouseEvent)> {
    Closure::wrap(
        Box::new(
            move |mouse_event: MouseEvent| {
                let point_b = canvas::point_on_canvas(Rc::clone(&canvas), &mouse_event);
                let segment = canvas::segment_on_canvas(
                    Rc::clone(&canvas),
                    point_a.borrow().clone(),
                    point_b.clone(),
                );
                if let Some(segment) = segment {
                    tool::line::put(Rc::clone(&image_vec), canvas.width(), segment);
                    canvas::put_image_vec(Rc::clone(&canvas), Rc::clone(&context), Rc::clone(&image_vec));
                }
                *point_a.borrow_mut() = point_b;
            }
        ) as Box<dyn FnMut(MouseEvent)>
    )
}

fn end(
    dom: Rc<Dom>,
    canvas: Rc<HtmlCanvasElement>,
    context: Rc<CanvasRenderingContext2d>,
    image_vec: Rc<RefCell<Clamped<Vec<u8>>>>,
) -> Closure<dyn FnMut()> {
    Closure::<dyn FnMut()>::new(
        move || {
            dom.body.set_onmousemove(None);
            dom.body.set_onmouseup(None);
            dom.body.set_onmouseleave(None);
            let start = start(
                Rc::clone(&dom),
                Rc::clone(&canvas),
                Rc::clone(&context),
                Rc::clone(&image_vec),
            );
            dom.body.set_onmousedown(Some(start.as_ref().unchecked_ref()));
            start.forget();
        }
    )
}
