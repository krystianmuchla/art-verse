use std::cell::RefCell;
use std::rc::Rc;

use crate::color::Color;
use crate::dom::Dom;
use crate::image::Image;
use crate::point::Point;
use crate::resizer;
use crate::segment::Segment;
use crate::tool;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent};

pub fn entry_point(dom: Rc<Dom>) {
    let canvas = dom
        .document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();
    let canvas = Rc::new(canvas);
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();
    let context = Rc::new(context);
    let image_data = context
        .get_image_data(0_f64, 0_f64, canvas.width() as f64, canvas.height() as f64)
        .unwrap();
    let image = Image::new(image_data.data(), canvas.width());
    let image = Rc::new(RefCell::new(image));
    let tool_events = Rc::new(RefCell::new(tool::Events::new()));
    let color = Rc::new(RefCell::new(Color::new(0, 0, 0, 255)));
    resizer::entry_point(
        Rc::clone(&tool_events),
        Rc::clone(&dom),
        Rc::clone(&canvas),
        Rc::clone(&context),
        Rc::clone(&image),
    );
    tool::entry_point(
        Rc::clone(&tool_events),
        Rc::clone(&dom),
        Rc::clone(&canvas),
        Rc::clone(&context),
        Rc::clone(&image),
        Rc::clone(&color),
    );
}

pub fn point_on_canvas(canvas: &HtmlCanvasElement, mouse_event: &MouseEvent) -> Point {
    let point = Point::new(mouse_event.x(), mouse_event.y());
    point_relative_to_canvas(canvas, &point)
}

pub fn point_relative_to_canvas(canvas: &HtmlCanvasElement, point: &Point) -> Point {
    let rect = canvas.get_bounding_client_rect();
    Point::new(point.x - rect.left() as i32, point.y - rect.top() as i32)
}

pub fn segment_on_canvas(
    canvas: Rc<HtmlCanvasElement>,
    prev: Point,
    next: Point,
) -> Option<Segment> {
    let segment = Segment::new(prev, next);
    if is_segment_on_canvas(&canvas, &segment) {
        return Some(segment);
    }
    let min_x = prev.x.min(next.x);
    let max_x = prev.x.max(next.x);
    let min_y = prev.y.min(next.y);
    let max_y = prev.y.max(next.y);
    if max_x < 0 || min_x >= canvas.width() as i32 {
        return None;
    }
    if max_y < 0 || min_y >= canvas.height() as i32 {
        return None;
    }
    if prev.x == next.x {
        let mut from_y = min_y;
        if from_y < 0 {
            from_y = 0;
        }
        let mut to_y = max_y;
        if to_y >= canvas.height() as i32 {
            to_y = canvas.height() as i32 - 1;
        }
        return Some(Segment::new(
            Point::new(prev.x, from_y),
            Point::new(next.x, to_y),
        ));
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
        return Some(Segment::new(
            Point::new(from_x, prev.y),
            Point::new(to_x, next.y),
        ));
    }
    let line = segment.as_line();
    let top = Point::new(
        line.x(canvas.height() as i32 - 1),
        canvas.height() as i32 - 1,
    );
    let right = Point::new(canvas.width() as i32 - 1, line.y(canvas.width() as i32 - 1));
    let bottom = Point::new(line.x(0), 0);
    let left = Point::new(0, line.y(0));
    let potential_points = [prev, next, top, right, bottom, left];
    let mut points: Vec<Point> = vec![];
    for potential_point in &potential_points {
        if points.len() >= 2 {
            break;
        }
        if is_point_on_canvas(&canvas, &potential_point)
            && segment.is_point_within(&potential_point)
        {
            points.push(*potential_point);
        }
    }
    if points.len() != 2 {
        return None;
    }
    Some(Segment::new(points[0], points[1]))
}

fn is_point_on_canvas(canvas: &HtmlCanvasElement, point: &Point) -> bool {
    if point.x < 0 || point.x >= canvas.width() as i32 {
        return false;
    }
    if point.y < 0 || point.y >= canvas.height() as i32 {
        return false;
    }
    true
}

fn is_segment_on_canvas(canvas: &HtmlCanvasElement, segment: &Segment) -> bool {
    is_point_on_canvas(&canvas, &segment.a) && is_point_on_canvas(&canvas, &segment.b)
}
