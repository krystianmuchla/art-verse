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
) {
    let start = start(
        Rc::clone(&events),
        Rc::clone(&dom),
        Rc::clone(&canvas),
        Rc::clone(&context),
        Rc::clone(&image),
    );
    events.borrow_mut().set_body_on_mouse_down(&*dom, &start);
    start.forget();
}

pub fn put(image: Rc<RefCell<Image>>, segment: Segment) {
    let point_a = segment.a;
    let point_b = segment.b;
    let color = Color::new(0, 0, 0, 255);
    let arg_0: i32;
    let val_0: i32;
    let arg_1: i32;
    let val_1: i32;
    let callback: Box<dyn Fn(i32, i32)>;
    if point_a.x.abs_diff(point_b.x) > point_a.y.abs_diff(point_b.y) {
        if point_a.x < point_b.x {
            if point_a.y < point_b.y {
                arg_0 = point_a.x;
                val_0 = point_a.y;
                arg_1 = point_b.x;
                val_1 = point_b.y;
                callback = Box::new(move |arg: i32, val: i32| {
                    let point = Point::new(arg, val);
                    image.borrow_mut().color_pixel(&point, &color);
                });
            } else {
                arg_0 = point_a.x;
                val_0 = point_b.y;
                arg_1 = point_b.x;
                val_1 = point_a.y;
                callback = Box::new(move |arg: i32, val: i32| {
                    let point = Point::new(arg, point_b.y - val + point_a.y);
                    image.borrow_mut().color_pixel(&point, &color);
                });
            }
        } else {
            if point_a.y < point_b.y {
                arg_0 = point_b.x;
                val_0 = point_a.y;
                arg_1 = point_a.x;
                val_1 = point_b.y;
                callback = Box::new(move |arg: i32, val: i32| {
                    let point = Point::new(arg, point_a.y - val + point_b.y);
                    image.borrow_mut().color_pixel(&point, &color);
                });
            } else {
                arg_0 = point_b.x;
                val_0 = point_b.y;
                arg_1 = point_a.x;
                val_1 = point_a.y;
                callback = Box::new(move |arg: i32, val: i32| {
                    let point = Point::new(arg, val);
                    image.borrow_mut().color_pixel(&point, &color);
                });
            }
        }
    } else {
        if point_a.y < point_b.y {
            if point_a.x < point_b.x {
                arg_0 = point_a.y;
                val_0 = point_a.x;
                arg_1 = point_b.y;
                val_1 = point_b.x;
                callback = Box::new(move |arg: i32, val: i32| {
                    let point = Point::new(val, arg);
                    image.borrow_mut().color_pixel(&point, &color);
                });
            } else {
                arg_0 = point_a.y;
                val_0 = point_b.x;
                arg_1 = point_b.y;
                val_1 = point_a.x;
                callback = Box::new(move |arg: i32, val: i32| {
                    let point = Point::new(point_a.x - val + point_b.x, arg);
                    image.borrow_mut().color_pixel(&point, &color);
                });
            }
        } else {
            if point_a.x < point_b.x {
                arg_0 = point_b.y;
                val_0 = point_a.x;
                arg_1 = point_a.y;
                val_1 = point_b.x;
                callback = Box::new(move |arg: i32, val: i32| {
                    let point = Point::new(point_b.x - val + point_a.x, arg);
                    image.borrow_mut().color_pixel(&point, &color);
                });
            } else {
                arg_0 = point_b.y;
                val_0 = point_b.x;
                arg_1 = point_a.y;
                val_1 = point_a.x;
                callback = Box::new(move |arg: i32, val: i32| {
                    let point = Point::new(val, arg);
                    image.borrow_mut().color_pixel(&point, &color);
                });
            }
        }
    }
    bresenham(arg_0, val_0, arg_1, val_1, callback);
}

fn start(
    events: Rc<RefCell<Events>>,
    dom: Rc<Dom>,
    canvas: Rc<HtmlCanvasElement>,
    context: Rc<CanvasRenderingContext2d>,
    image: Rc<RefCell<Image>>,
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
) -> Closure<dyn FnMut(MouseEvent)> {
    Closure::wrap(Box::new(move |mouse_event: MouseEvent| {
        let image_clone = Rc::new(RefCell::new(image.borrow().clone()));
        let segment = canvas::segment_on_canvas(
            Rc::clone(&canvas),
            (*point_a).clone(),
            canvas::point_on_canvas(&*canvas, &mouse_event),
        );
        if let Some(segment) = segment {
            put(Rc::clone(&image_clone), segment);
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
            put(Rc::clone(&image), segment);
            context::apply_image(&*context, &image.borrow());
        }
        let start = start(
            Rc::clone(&events),
            Rc::clone(&dom),
            Rc::clone(&canvas),
            Rc::clone(&context),
            Rc::clone(&image),
        );
        events.borrow_mut().set_body_on_mouse_down(&*dom, &start);
        start.forget();
    }) as Box<dyn FnMut(MouseEvent)>)
}

fn bresenham(arg_0: i32, val_0: i32, arg_1: i32, val_1: i32, callback: Box<dyn Fn(i32, i32)>) {
    let delta_arg = arg_1 - arg_0;
    let delta_val = val_1 - val_0;
    let mut p = 2 * delta_val - delta_arg;
    let mut val = val_0;
    for arg in arg_0..=arg_1 {
        callback(arg, val);
        if p > 0 {
            val += 1;
            p += 2 * delta_val - 2 * delta_arg;
        } else {
            p += 2 * delta_val;
        }
    }
}
