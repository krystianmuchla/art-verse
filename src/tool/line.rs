use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::{Clamped, JsCast};
use wasm_bindgen::closure::Closure;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent};

use crate::canvas;
use crate::color::Color;
use crate::dom::Dom;
use crate::segment::Segment;
use crate::point::Point;

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

pub fn put(
    image_vec: Rc<RefCell<Clamped<Vec<u8>>>>,
    image_width: u32,
    segment: Segment,
) {
    let point_a = segment.a;
    let point_b = segment.b;
    let color = Color { r: 0, g: 0, b: 0, a: 255 };
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
                    let point = Point { x: arg, y: val };
                    canvas::color_pixel(Rc::clone(&image_vec), image_width, &point, &color);
                });
            } else {
                arg_0 = point_a.x;
                val_0 = point_b.y;
                arg_1 = point_b.x;
                val_1 = point_a.y;
                callback = Box::new(move |arg: i32, val: i32| {
                    let point = Point { x: arg, y: point_b.y - val + point_a.y };
                    canvas::color_pixel(Rc::clone(&image_vec), image_width, &point, &color);
                });
            }
        } else {
            if point_a.y < point_b.y {
                arg_0 = point_b.x;
                val_0 = point_a.y;
                arg_1 = point_a.x;
                val_1 = point_b.y;
                callback = Box::new(move |arg: i32, val: i32| {
                    let point = Point { x: arg, y: point_a.y - val + point_b.y };
                    canvas::color_pixel(Rc::clone(&image_vec), image_width, &point, &color);
                });
            } else {
                arg_0 = point_b.x;
                val_0 = point_b.y;
                arg_1 = point_a.x;
                val_1 = point_a.y;
                callback = Box::new(move |arg: i32, val: i32| {
                    let point = Point { x: arg, y: val };
                    canvas::color_pixel(Rc::clone(&image_vec), image_width, &point, &color);
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
                    let point = Point { x: val, y: arg };
                    canvas::color_pixel(Rc::clone(&image_vec), image_width, &point, &color);
                });
            } else {
                arg_0 = point_a.y;
                val_0 = point_b.x;
                arg_1 = point_b.y;
                val_1 = point_a.x;
                callback = Box::new(move |arg: i32, val: i32| {
                    let point = Point { x: point_a.x - val + point_b.x, y: arg };
                    canvas::color_pixel(Rc::clone(&image_vec), image_width, &point, &color);
                });
            }
        } else {
            if point_a.x < point_b.x {
                arg_0 = point_b.y;
                val_0 = point_a.x;
                arg_1 = point_a.y;
                val_1 = point_b.x;
                callback = Box::new(move |arg: i32, val: i32| {
                    let point = Point { x: point_b.x - val + point_a.x, y: arg };
                    canvas::color_pixel(Rc::clone(&image_vec), image_width, &point, &color);
                });
            } else {
                arg_0 = point_b.y;
                val_0 = point_b.x;
                arg_1 = point_a.y;
                val_1 = point_a.x;
                callback = Box::new(move |arg: i32, val: i32| {
                    let point = Point { x: val, y: arg };
                    canvas::color_pixel(Rc::clone(&image_vec), image_width, &point, &color);
                });
            }
        }
    }
    bresenham(arg_0, val_0, arg_1, val_1, callback);
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
                dom.body.set_onmousedown(None);
                let point_a = canvas::point_on_canvas(Rc::clone(&canvas), &mouse_event);
                let point_a = Rc::new(point_a);
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
                    Rc::clone(&point_a),
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
    point_a: Rc<Point>,
) -> Closure<dyn FnMut(MouseEvent)> {
    Closure::wrap(
        Box::new(
            move |mouse_event: MouseEvent| {
                let image_vec_clone = Rc::new(RefCell::new(image_vec.borrow().clone()));
                let segment = canvas::segment_on_canvas(
                    Rc::clone(&canvas),
                    (*point_a).clone(),
                    canvas::point_on_canvas(Rc::clone(&canvas), &mouse_event),
                );
                if let Some(segment) = segment {
                    put(Rc::clone(&image_vec_clone), canvas.width(), segment);
                    canvas::put_image_vec(Rc::clone(&canvas), Rc::clone(&context), Rc::clone(&image_vec_clone));
                } else {
                    canvas::put_image_vec(Rc::clone(&canvas), Rc::clone(&context), Rc::clone(&image_vec));
                }
            }
        ) as Box<dyn FnMut(MouseEvent)>
    )
}

fn end(
    dom: Rc<Dom>,
    canvas: Rc<HtmlCanvasElement>,
    context: Rc<CanvasRenderingContext2d>,
    image_vec: Rc<RefCell<Clamped<Vec<u8>>>>,
    point_a: Rc<Point>,
) -> Closure<dyn FnMut(MouseEvent)> {
    Closure::wrap(
        Box::new(
            move |mouse_event: MouseEvent| {
                dom.body.set_onmousemove(None);
                dom.body.set_onmouseup(None);
                dom.body.set_onmouseleave(None);
                let segment = canvas::segment_on_canvas(
                    Rc::clone(&canvas),
                    (*point_a).clone(),
                    canvas::point_on_canvas(Rc::clone(&canvas), &mouse_event),
                );
                if let Some(segment) = segment {
                    put(Rc::clone(&image_vec), canvas.width(), segment);
                    canvas::put_image_vec(Rc::clone(&canvas), Rc::clone(&context), Rc::clone(&image_vec));
                }
                let start = start(
                    Rc::clone(&dom),
                    Rc::clone(&canvas),
                    Rc::clone(&context),
                    Rc::clone(&image_vec),
                );
                dom.body.set_onmousedown(Some(start.as_ref().unchecked_ref()));
                start.forget();
            }
        ) as Box<dyn FnMut(MouseEvent)>
    )
}

fn bresenham(arg_0: i32, val_0: i32, arg_1: i32, val_1: i32, callback: Box<dyn Fn(i32, i32)>) {
    let delta_arg: i32 = arg_1 - arg_0;
    let delta_val: i32 = val_1 - val_0;
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
