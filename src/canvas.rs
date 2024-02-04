use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::Clamped;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData, MouseEvent};

use crate::color::Color;
use crate::dom::Dom;
use crate::path::Path;
use crate::point::Point;
use crate::resizer;
use crate::tool;

pub fn entry_point(dom: Rc<Dom>) {
    let canvas = dom.document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();
    let canvas = Rc::new(canvas);
    let context = canvas.get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();
    let context = Rc::new(context);
    let image_data = context.get_image_data(
        0_f64,
        0_f64,
        canvas.width() as f64,
        canvas.height() as f64,
    ).unwrap();
    let image_data = Rc::new(RefCell::new(image_data));
    let image_vec = Rc::new(RefCell::new(image_data.borrow().data()));
    resizer::entry_point(
        Rc::clone(&dom),
        Rc::clone(&canvas),
        Rc::clone(&context),
        Rc::clone(&image_data),
    );
    tool::entry_point(
        Rc::clone(&dom),
        Rc::clone(&canvas),
        Rc::clone(&context),
        Rc::clone(&image_vec),
    );
}

pub fn point_on_canvas(canvas: Rc<HtmlCanvasElement>, mouse_event: &MouseEvent) -> Point {
    let rect = canvas.get_bounding_client_rect();
    Point { x: mouse_event.x() - rect.left() as i32, y: mouse_event.y() - rect.top() as i32 }
}

pub fn path_on_canvas(canvas: Rc<HtmlCanvasElement>, prev: Point, next: Point) -> Option<Path> {
    let min_x = prev.x.min(next.x);
    let max_x = prev.x.max(next.x);
    let min_y = prev.y.min(next.y);
    let max_y = prev.y.max(next.y);
    if min_x >= 0 && max_x < canvas.width() as i32 && min_y >= 0 && max_y < canvas.height() as i32 {
        return Some(Path { a: prev, b: next });
    }
    if max_x < 0 || min_x >= canvas.width() as i32 || max_y < 0 || min_y >= canvas.height() as i32 {
        return None;
    }
    if prev.x != next.x && prev.y != prev.y {
        let a = (next.y as f64 - prev.y as f64) / (next.x as f64 - prev.x as f64);
        let b = prev.y as f64 - a * prev.x as f64;
        // todo
        return None;
    }
    // todo
    return None;
}

/*pub fn point_on_canvas_with_prev(canvas: Rc<HtmlCanvasElement>, left: u32, top: u32, prev: Point) -> Point {
    let next = point_on_canvas(Rc::clone(&canvas), left, top);
    let mut x = next.x;
    let mut y = next.y;
    let mut x_limit: Option<i32> = None;
    if x < 0 {
        x_limit = Some(0);
    } else if x >= canvas.width() as i32 {
        x_limit = Some(canvas.width() as i32 - 1);
    }
    if let Some(x_limit) = x_limit {
        y = prev.y + similar_triangle(
            (x - prev.x) as f64,
            (y - prev.y) as f64,
            (x_limit - prev.x) as f64,
        ) as i32;
    }
    let mut y_limit: Option<i32> = None;
    if y < 0 {
        y_limit = Some(0);
    } else if y >= canvas.height() as i32 {
        y_limit = Some(canvas.height() as i32 - 1);
    }
    if let Some(y_limit) = y_limit {
        x = prev.x + similar_triangle(
            (y - prev.y) as f64,
            (x - prev.x) as f64,
            (y_limit - prev.y) as f64,
        ) as i32;
    }
    Point {
        x: x.max(0).min(canvas.width() as i32 - 1),
        y: y.max(0).min(canvas.height() as i32 - 1),
    }
}*/

pub fn color_pixel(image_vec: Rc<RefCell<Clamped<Vec<u8>>>>, image_width: u32, point: &Point, color: &Color) {
    let image_vec = &mut *image_vec.borrow_mut();
    let flat_index = (point.y * (image_width as i32 * 4) + point.x * 4) as usize;
    image_vec[flat_index] = color.r;
    image_vec[flat_index + 1] = color.g;
    image_vec[flat_index + 2] = color.b;
    image_vec[flat_index + 3] = color.a;
}

pub fn put_image_vec(
    canvas: Rc<HtmlCanvasElement>,
    context: Rc<CanvasRenderingContext2d>,
    image_vec: Rc<RefCell<Clamped<Vec<u8>>>>,
) {
    let image_data = ImageData::new_with_u8_clamped_array(
        Clamped(&image_vec.borrow().0[..]),
        canvas.width(),
    ).unwrap();
    context.put_image_data(&image_data, 0_f64, 0_f64).unwrap();
}

fn similar_triangle(a_0: f64, b_0: f64, a_1: f64) -> f64 {
    a_1 * b_0 / a_0
}
