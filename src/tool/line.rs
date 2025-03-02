use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::closure::Closure;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent};

use crate::color::Color;
use crate::dom::Dom;
use crate::image::Image;
use crate::point::Point;
use crate::segment::Segment;
use crate::{canvas, context};

use super::Events;

pub fn init(
    events: Rc<RefCell<Events>>,
    dom: Rc<Dom>,
    canvas: Rc<HtmlCanvasElement>,
    context: Rc<CanvasRenderingContext2d>,
    image: Rc<RefCell<Image>>,
    color: Rc<RefCell<Color>>,
) {
    let start = start(
        Rc::clone(&events),
        Rc::clone(&dom),
        Rc::clone(&canvas),
        Rc::clone(&context),
        Rc::clone(&image),
        Rc::clone(&color),
    );
    events.borrow_mut().set_body_on_mouse_down(&*dom, &start);
    start.forget();
}

pub fn put(image: Rc<RefCell<Image>>, segment: &Segment, color: &Color) {
    let point_a = segment.a;
    let point_b = segment.b;
    let kx = if point_a.x <= point_b.x { 1 } else { -1 };
    let ky = if point_a.y <= point_b.y { 1 } else { -1 };
    let dx = (point_a.x - point_b.x).abs();
    let dy = -(point_a.y - point_b.y).abs();
    let mut e = dx + dy;
    let mut e2: i32;
    let mut point = point_a.clone();
    loop {
        image.borrow_mut().color_pixel(&point, &color);
        if point.x == point_b.x && point.y == point_b.y {
            break;
        }
        e2 = 2 * e;
        if e2 >= dy {
            e += dy;
            point.x += kx;
        }
        if e2 <= dx {
            e += dx;
            point.y += ky;
        }
    }
}

fn start(
    events: Rc<RefCell<Events>>,
    dom: Rc<Dom>,
    canvas: Rc<HtmlCanvasElement>,
    context: Rc<CanvasRenderingContext2d>,
    image: Rc<RefCell<Image>>,
    color: Rc<RefCell<Color>>,
) -> Closure<dyn FnMut(MouseEvent)> {
    Closure::wrap(Box::new(move |mouse_event: MouseEvent| {
        events.borrow_mut().remove_body_on_mouse_down(&*dom);
        let point_a = canvas::point_on_canvas(&*canvas, &mouse_event);
        let point_a = Rc::new(point_a);
        let advance = advance(
            Rc::clone(&canvas),
            Rc::clone(&context),
            Rc::clone(&image),
            Rc::clone(&point_a),
            Rc::clone(&color),
        );
        events.borrow_mut().set_body_on_mouse_move(&*dom, &advance);
        advance.forget();
        let end = end(
            Rc::clone(&events),
            Rc::clone(&dom),
            Rc::clone(&canvas),
            Rc::clone(&context),
            Rc::clone(&image),
            Rc::clone(&point_a),
            Rc::clone(&color),
        );
        events.borrow_mut().set_body_on_mouse_up(&*dom, &end);
        events.borrow_mut().set_body_on_mouse_leave(&*dom, &end);
        end.forget();
    }) as Box<dyn FnMut(MouseEvent)>)
}

fn advance(
    canvas: Rc<HtmlCanvasElement>,
    context: Rc<CanvasRenderingContext2d>,
    image: Rc<RefCell<Image>>,
    point_a: Rc<Point>,
    color: Rc<RefCell<Color>>,
) -> Closure<dyn FnMut(MouseEvent)> {
    Closure::wrap(Box::new(move |mouse_event: MouseEvent| {
        let image_clone = Rc::new(RefCell::new(image.borrow().clone()));
        let segment = canvas::segment_on_canvas(
            Rc::clone(&canvas),
            (*point_a).clone(),
            canvas::point_on_canvas(&*canvas, &mouse_event),
        );
        if let Some(segment) = segment {
            put(Rc::clone(&image_clone), &segment, &*color.borrow());
            context::apply_image(&*context, &image_clone.borrow());
        } else {
            context::apply_image(&*context, &image.borrow());
        }
    }) as Box<dyn FnMut(MouseEvent)>)
}

fn end(
    events: Rc<RefCell<Events>>,
    dom: Rc<Dom>,
    canvas: Rc<HtmlCanvasElement>,
    context: Rc<CanvasRenderingContext2d>,
    image: Rc<RefCell<Image>>,
    point_a: Rc<Point>,
    color: Rc<RefCell<Color>>,
) -> Closure<dyn FnMut(MouseEvent)> {
    Closure::wrap(Box::new(move |mouse_event: MouseEvent| {
        events.borrow_mut().remove_body_on_mouse_move(&*dom);
        events.borrow_mut().remove_body_on_mouse_up(&*dom);
        events.borrow_mut().remove_body_on_mouse_leave(&*dom);
        let segment = canvas::segment_on_canvas(
            Rc::clone(&canvas),
            (*point_a).clone(),
            canvas::point_on_canvas(&*canvas, &mouse_event),
        );
        if let Some(segment) = segment {
            put(Rc::clone(&image), &segment, &*color.borrow());
            context::apply_image(&*context, &image.borrow());
        }
        let start = start(
            Rc::clone(&events),
            Rc::clone(&dom),
            Rc::clone(&canvas),
            Rc::clone(&context),
            Rc::clone(&image),
            Rc::clone(&color),
        );
        events.borrow_mut().set_body_on_mouse_down(&*dom, &start);
        start.forget();
    }) as Box<dyn FnMut(MouseEvent)>)
}
