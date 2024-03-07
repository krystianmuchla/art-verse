use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::Clamped;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData, MouseEvent};

use crate::resizer;
use crate::color::Color;
use crate::dom::Dom;
use crate::segment::Segment;
use crate::point::Point;
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
    let image_vec = Rc::new(RefCell::new(image_data.data()));
    let tool_events = Rc::new(RefCell::new(tool::Events::new()));
    resizer::entry_point(
        Rc::clone(&tool_events),
        Rc::clone(&dom),
        Rc::clone(&canvas),
        Rc::clone(&context),
        Rc::clone(&image_vec),
    );
    tool::entry_point(
        Rc::clone(&tool_events),
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

pub fn segment_on_canvas(canvas: Rc<HtmlCanvasElement>, prev: Point, next: Point) -> Option<Segment> {
    let segment = Segment::new(prev, next);
    if is_segment_on_canvas(&canvas, &segment){
        return Some(segment);
    }
    let min_x = prev.x.min(next.x);
    let max_x = prev.x.max(next.x);
    let min_y = prev.y.min(next.y);
    let max_y = prev.y.max(next.y);
    if max_x < 0 || min_x >= canvas.width() as i32 { return None; }
    if max_y < 0 || min_y >= canvas.height() as i32 { return None; }
    if prev.x == next.x {
        let mut from_y = min_y;
        if from_y < 0 {
            from_y = 0;
        }
        let mut to_y = max_y;
        if to_y >= canvas.height() as i32 {
            to_y = canvas.height() as i32 - 1;
        }
        return Some(Segment::new(Point { x: prev.x, y: from_y }, Point { x: next.x, y: to_y } ));
    }
    if prev.y == next.y {
        let mut from_x = min_x;
        if from_x < 0 {
            from_x = 0;
        }
        let mut to_x = max_x;
        if to_x >= canvas.width() as i32 {
            to_x = canvas.width() as i32 - 1;
        }
        return Some(Segment::new(Point { x: from_x, y: prev.y }, Point { x: to_x, y: next.y }));
    }
    let line = segment.as_line();
    let top = Point { x: line.x(canvas.height() as i32 - 1), y: canvas.height() as i32 - 1 };
    let right = Point { x: canvas.width() as i32 - 1, y: line.y(canvas.width() as i32 - 1) };
    let bottom = Point { x: line.x(0), y: 0 };
    let left = Point { x: 0, y: line.y(0) };
    let potential_points = [prev, next, top, right, bottom, left];
    let mut points: Vec<Point> = vec![];
    for potential_point in &potential_points {
        if points.len() >= 2 {
            break;
        }
        if is_point_on_canvas(&canvas, &potential_point) && segment.is_point_within(&potential_point) {
            points.push(*potential_point);
        }
    }
    if points.len() != 2 {
        return None;
    }
    Some(Segment::new(points[0], points[1]))
}

fn is_point_on_canvas(canvas: &HtmlCanvasElement, point: &Point) -> bool {
    if point.x < 0 || point.x >= canvas.width() as i32 { return false; }
    if point.y < 0 || point.y >= canvas.height() as i32 { return false; }
    true
}

fn is_segment_on_canvas(canvas: &HtmlCanvasElement, segment: &Segment) -> bool {
    is_point_on_canvas(&canvas, &segment.a) && is_point_on_canvas(&canvas, &segment.b)
}

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
