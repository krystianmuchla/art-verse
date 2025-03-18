use crate::canvas::canvas_events::CanvasEvents;
use crate::color::Color;
use crate::dom::Dom;
use crate::point::Point;
use crate::resizer;
use crate::segment::Segment;
use crate::util::flat_idx;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{Clamped, JsCast};
use web_sys::{
    CanvasRenderingContext2d, Document, HtmlCanvasElement, HtmlElement, ImageData, MouseEvent,
};

pub struct Canvas {
    pub element: HtmlCanvasElement,
    pub context: CanvasRenderingContext2d,
    pub pixels: Vec<Rc<Color>>,
    pub events: CanvasEvents,
    body: HtmlElement,
}

impl Canvas {
    pub fn new(document: &Document) -> Canvas {
        let element = document
            .get_element_by_id("canvas")
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap();
        let context = element
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();
        let channels = context
            .get_image_data(
                0_f64,
                0_f64,
                element.width() as f64,
                element.height() as f64,
            )
            .unwrap()
            .data();
        let mut pixels = Vec::with_capacity(channels.len() / 4);
        for index in (0..channels.len()).step_by(4) {
            let r = channels[index];
            let g = channels[index + 1];
            let b = channels[index + 2];
            let a = channels[index + 3];
            pixels.push(Rc::new(Color::new(r, g, b, a)));
        }
        let events = CanvasEvents::new();
        let body = document.body().unwrap();
        Canvas {
            element,
            context,
            pixels,
            events,
            body,
        }
    }

    pub fn get_point(&self, mouse_event: &MouseEvent) -> Point {
        let rect = self.element.get_bounding_client_rect();
        Point::new(
            mouse_event.x() - rect.left() as i32,
            mouse_event.y() - rect.top() as i32,
        )
    }

    pub fn get_segment(&self, prev: &Point, next: &Point) -> Option<Segment> {
        let segment = Segment::new(prev.clone(), next.clone());
        if self.is_segment_on_canvas(&segment) {
            return Some(segment);
        }
        let min_x = prev.x.min(next.x);
        let max_x = prev.x.max(next.x);
        let min_y = prev.y.min(next.y);
        let max_y = prev.y.max(next.y);
        let width = self.element.width() as i32;
        let height = self.element.height() as i32;
        if max_x < 0 || min_x >= width {
            return None;
        }
        if max_y < 0 || min_y >= height {
            return None;
        }
        if prev.x == next.x {
            let mut from_y = min_y;
            if from_y < 0 {
                from_y = 0;
            }
            let mut to_y = max_y;
            if to_y >= height {
                to_y = height - 1;
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
            if to_x >= width {
                to_x = width - 1;
            }
            return Some(Segment::new(
                Point::new(from_x, prev.y),
                Point::new(to_x, next.y),
            ));
        }
        let line = segment.as_line();
        let top = Point::new(line.x(height - 1), height - 1);
        let right = Point::new(width - 1, line.y(width - 1));
        let bottom = Point::new(line.x(0), 0);
        let left = Point::new(0, line.y(0));
        let potential_points = [prev, next, &top, &right, &bottom, &left];
        let mut points: Vec<&Point> = vec![];
        for potential_point in potential_points {
            if points.len() >= 2 {
                break;
            }
            if self.is_point_on_canvas(potential_point) && segment.is_point_within(potential_point)
            {
                points.push(potential_point);
            }
        }
        if points.len() != 2 {
            return None;
        }
        Some(Segment::new(points[0].clone(), points[1].clone()))
    }

    pub fn is_point_on_canvas(&self, point: &Point) -> bool {
        if point.x < 0 || point.x >= self.element.width() as i32 {
            return false;
        }
        if point.y < 0 || point.y >= self.element.height() as i32 {
            return false;
        }
        true
    }

    pub fn is_segment_on_canvas(&self, segment: &Segment) -> bool {
        self.is_point_on_canvas(&segment.a) && self.is_point_on_canvas(&segment.b)
    }

    pub fn set_on_mouse_down(&mut self, event: Option<&Closure<dyn FnMut(MouseEvent)>>) {
        let event = event.map(|e| e.as_ref().clone());
        self.events.body_on_mouse_down = event;
        self.resume_on_mouse_down();
    }

    pub fn set_on_mouse_move(&mut self, event: Option<&Closure<dyn FnMut(MouseEvent)>>) {
        let event = event.map(|e| e.as_ref().clone());
        self.events.body_on_mouse_move = event;
        self.resume_on_mouse_move();
    }

    pub fn set_on_mouse_up(&mut self, event: Option<&Closure<dyn FnMut(MouseEvent)>>) {
        let event = event.map(|e| e.as_ref().clone());
        self.events.body_on_mouse_up = event;
        self.resume_on_mouse_up();
    }

    pub fn set_on_mouse_leave(&mut self, event: Option<&Closure<dyn FnMut(MouseEvent)>>) {
        let event = event.map(|e| e.as_ref().clone());
        self.events.body_on_mouse_leave = event;
        self.resume_on_mouse_leave();
    }

    pub fn pause_all_events(&self) {
        self.body.set_onmousedown(None);
        self.body.set_onmousemove(None);
        self.body.set_onmouseup(None);
        self.body.set_onmouseleave(None);
    }

    pub fn resume_all_events(&self) {
        self.resume_on_mouse_down();
        self.resume_on_mouse_move();
        self.resume_on_mouse_up();
        self.resume_on_mouse_leave();
    }

    pub fn extract_pixels(&self, from: &Point, to: &Point) -> Vec<Rc<Color>> {
        let target_width = (from.x - to.x) as usize;
        let target_height = (from.y - to.y) as usize;
        let mut pixels = Vec::with_capacity(target_width * target_height);
        for source_y in from.y..=to.y {
            for source_x in from.x..=to.x {
                let point = Point::new(source_x, source_y);
                let idx = flat_idx(&point, &self.element.width());
                pixels.push(Rc::clone(&self.pixels[idx]));
            }
        }
        pixels
    }

    pub fn resize(&mut self, width: u32, height: u32, mut pixels: Vec<Rc<Color>>) {
        pixels.resize_with((width * height) as usize, || Rc::new(Color::white()));
        self.element.set_width(width);
        self.element.set_height(height);
        self.pixels = pixels;
        let image_data = self.create_image_data();
        self.context
            .put_image_data(&image_data, 0_f64, 0_f64)
            .unwrap();
    }

    pub fn refresh(&self) {
        let image_data = self.create_image_data();
        self.context
            .put_image_data(&image_data, 0_f64, 0_f64)
            .unwrap()
    }

    pub fn render_external_pixels(&self, pixels: &Vec<Rc<Color>>) {
        let image_data = self.create_image_data_from_pixels(pixels);
        self.context
            .put_image_data(&image_data, 0_f64, 0_f64)
            .unwrap()
    }

    fn create_image_data(&self) -> ImageData {
        self.create_image_data_from_pixels(&self.pixels)
    }

    fn create_image_data_from_pixels(&self, pixels: &Vec<Rc<Color>>) -> ImageData {
        let width = self.element.width();
        let data: Vec<u8> = pixels
            .iter()
            .flat_map(|pixel| [pixel.r, pixel.g, pixel.b, pixel.a])
            .collect();
        ImageData::new_with_u8_clamped_array(Clamped(&data), width).unwrap()
    }

    fn resume_on_mouse_down(&self) {
        let event = &self.events.body_on_mouse_down;
        self.body
            .set_onmousedown(event.as_ref().map(|e| e.unchecked_ref()));
    }

    fn resume_on_mouse_move(&self) {
        let event = &self.events.body_on_mouse_move;
        self.body
            .set_onmousemove(event.as_ref().map(|e| e.unchecked_ref()));
    }

    fn resume_on_mouse_up(&self) {
        let event = &self.events.body_on_mouse_up;
        self.body
            .set_onmouseup(event.as_ref().map(|e| e.unchecked_ref()));
    }

    fn resume_on_mouse_leave(&self) {
        let event = &self.events.body_on_mouse_leave;
        self.body
            .set_onmouseleave(event.as_ref().map(|e| e.unchecked_ref()));
    }
}

pub fn init(dom: Rc<RefCell<Dom>>) {
    resizer::init(Rc::clone(&dom));
}
