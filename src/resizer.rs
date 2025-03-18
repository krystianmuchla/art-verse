use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{DomRect, Element, HtmlElement, MouseEvent};

use crate::canvas::canvas::Canvas;
use crate::color::Color;
use crate::dom::Dom;
use crate::point::Point;
use crate::util::flat_idx;

pub fn init(dom: Rc<RefCell<Dom>>) {
    let resizers = dom.borrow().document.get_elements_by_class_name("resizer");
    for resizer_index in 0..resizers.length() {
        let resizer = resizers
            .item(resizer_index)
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap();
        let init_canvas_resize = init_canvas_resize(Rc::clone(&dom));
        resizer.set_onmousedown(Some(init_canvas_resize.as_ref().unchecked_ref()));
        init_canvas_resize.forget();
    }
}

fn init_canvas_resize(dom: Rc<RefCell<Dom>>) -> Closure<dyn FnMut(MouseEvent)> {
    Closure::wrap(Box::new(move |mouse_event: MouseEvent| {
        mouse_event.prevent_default();
        dom.borrow().canvas.pause_all_events();
        let sketch = dom.borrow().document.get_element_by_id("canvas-sketch");
        if let Some(_) = sketch {
            return;
        }
        let resizer = mouse_event.target().unwrap().dyn_into::<Element>().unwrap();
        let resizer_id = Rc::new(resizer.id());
        let canvas_rect = dom.borrow().canvas.element.get_bounding_client_rect();
        let sketch = Rc::new(create_canvas_sketch(&dom.borrow(), &canvas_rect));
        dom.borrow().body.append_child(&**sketch).unwrap();
        let x_offset: i32;
        if resizer_id.contains("west") {
            x_offset = canvas_rect.left() as i32 - mouse_event.x();
        } else if resizer_id.contains("east") {
            x_offset = mouse_event.x() - canvas_rect.right() as i32;
        } else {
            x_offset = 0;
        }
        let x_offset = Rc::new(x_offset);
        let y_offset: i32;
        if resizer_id.contains("south") {
            y_offset = mouse_event.y() - canvas_rect.bottom() as i32;
        } else {
            y_offset = 0;
        }
        let y_offset = Rc::new(y_offset);
        let resize_sketch = resize_canvas_sketch(
            Rc::clone(&dom),
            Rc::clone(&sketch),
            Rc::clone(&resizer_id),
            Rc::clone(&x_offset),
            Rc::clone(&y_offset),
        );
        dom.borrow()
            .body
            .set_onmousemove(Some(resize_sketch.as_ref().unchecked_ref()));
        resize_sketch.forget();
        let resize_canvas =
            resize_canvas(Rc::clone(&dom), Rc::clone(&sketch), Rc::clone(&resizer_id));
        dom.borrow()
            .body
            .set_onmouseup(Some(resize_canvas.as_ref().unchecked_ref()));
        dom.borrow()
            .body
            .set_onmouseleave(Some(resize_canvas.as_ref().unchecked_ref()));
        resize_canvas.forget();
    }) as Box<dyn FnMut(MouseEvent)>)
}

fn create_canvas_sketch(dom: &Dom, rect: &DomRect) -> HtmlElement {
    let sketch: HtmlElement = dom
        .document
        .create_element("div")
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap();
    sketch.set_attribute("id", "canvas-sketch").unwrap();
    sketch.style().set_property("position", "fixed").unwrap();
    sketch
        .style()
        .set_property("outline", "black dashed 1px")
        .unwrap();
    sketch
        .style()
        .set_property("width", format!("{}px", rect.width()).as_str())
        .unwrap();
    sketch
        .style()
        .set_property("height", format!("{}px", rect.height()).as_str())
        .unwrap();
    sketch
        .style()
        .set_property("top", format!("{}px", rect.top()).as_str())
        .unwrap();
    sketch
        .style()
        .set_property("left", format!("{}px", rect.left()).as_str())
        .unwrap();
    sketch
}

fn resize_canvas_sketch(
    dom: Rc<RefCell<Dom>>,
    sketch: Rc<HtmlElement>,
    resizer_id: Rc<String>,
    x_offset: Rc<i32>,
    y_offset: Rc<i32>,
) -> Closure<dyn FnMut(MouseEvent)> {
    Closure::wrap(Box::new(move |mouse_event: MouseEvent| {
        let mut height: Option<i32> = None;
        let mut left_diff: Option<i32> = None;
        let mut right_diff: Option<i32> = None;
        let rect = dom.borrow().canvas.element.get_bounding_client_rect();
        if resizer_id.contains("south") {
            height = Some(mouse_event.y() - *y_offset - rect.y() as i32);
        }
        if resizer_id.contains("west") {
            left_diff = Some(rect.left() as i32 - mouse_event.x() - *x_offset);
        } else if resizer_id.contains("east") {
            right_diff = Some(mouse_event.x() - *x_offset - rect.right() as i32);
        }
        let diff = left_diff.or(right_diff);
        let mut width: Option<i32> = None;
        let mut left: Option<f64> = None;
        let mut right: Option<f64> = None;
        if diff.is_some() && diff.unwrap() >= 0 {
            let diff = diff.unwrap();
            width = Some(rect.width() as i32 + 2 * diff);
            left = Some(rect.left() - (width.unwrap() as f64 - rect.width()) / 2_f64);
        } else if left_diff.is_some() {
            let left_diff = left_diff.unwrap();
            let window_width = dom.borrow().window.inner_width().unwrap().as_f64().unwrap();
            right = Some(window_width - rect.right());
            width = Some(rect.width() as i32 + left_diff);
        } else if right_diff.is_some() {
            let right_diff = right_diff.unwrap();
            left = Some(rect.left());
            width = Some(rect.width() as i32 + right_diff);
        }
        if let Some(h) = height {
            sketch
                .style()
                .set_property("height", format!("{}px", h.max(50)).as_str())
                .unwrap();
        }
        if let Some(w) = width {
            sketch
                .style()
                .set_property("width", format!("{}px", w.max(50)).as_str())
                .unwrap();
        }
        if let Some(l) = left {
            sketch.style().remove_property("right").unwrap();
            sketch
                .style()
                .set_property("left", format!("{l}px").as_str())
                .unwrap();
        } else if let Some(r) = right {
            sketch.style().remove_property("left").unwrap();
            sketch
                .style()
                .set_property("right", format!("{r}px").as_str())
                .unwrap();
        }
    }) as Box<dyn FnMut(MouseEvent)>)
}

fn resize_canvas(
    dom: Rc<RefCell<Dom>>,
    sketch: Rc<HtmlElement>,
    resizer_id: Rc<String>,
) -> Closure<dyn FnMut()> {
    Closure::<dyn FnMut()>::new(move || {
        let sketch_rect = sketch.get_bounding_client_rect();
        let sketch_width = sketch_rect.width() as u32;
        let sketch_height = sketch_rect.height() as u32;
        let (src_from, src_to, target_from) = resolve_canvas_points(
            &dom.borrow().canvas,
            &sketch_width,
            &sketch_height,
            &resizer_id,
        );
        let src_width = src_to.x - src_from.x + 1;
        let src_height = src_to.y - src_from.y + 1;
        let src_pixels = dom.borrow().canvas.extract_pixels(&src_from, &src_to);
        let mut target_pixels =
            vec![Rc::new(Color::white()); (sketch_width * sketch_height) as usize];
        for src_y in 0..src_height {
            for src_x in 0..src_width {
                let src_point = Point::new(src_x, src_y);
                let src_idx = flat_idx(&src_point, &(src_width as u32));
                let target_point = Point::new(target_from.x + src_x, target_from.y + src_y);
                let target_idx = flat_idx(&target_point, &sketch_width);
                target_pixels[target_idx] = Rc::clone(&src_pixels[src_idx]);
            }
        }
        dom.borrow_mut()
            .canvas
            .resize(sketch_width, sketch_height, target_pixels);
        sketch.remove();
        dom.borrow().body.set_onmousemove(None);
        dom.borrow().body.set_onmouseup(None);
        dom.borrow().body.set_onmouseleave(None);
        dom.borrow().canvas.resume_all_events();
    })
}

fn resolve_canvas_points(
    canvas: &Canvas,
    new_width: &u32,
    new_height: &u32,
    resizer_id: &String,
) -> (Point, Point, Point) {
    let width = canvas.element.width() as i32;
    let height = canvas.element.height() as i32;
    let x0: i32;
    let y0: i32 = 0;
    let x1: i32;
    let y1: i32;
    let x2: i32;
    let y2: i32 = 0;
    if resizer_id.contains("west") {
        let x_diff = *new_width as i32 - width;
        if x_diff > 0 {
            x0 = 0;
            x2 = x_diff;
        } else {
            x0 = -x_diff;
            x2 = 0;
        }
        x1 = width - 1;
    } else if resizer_id.contains("east") {
        let x_diff = *new_width as i32 - width;
        if x_diff > 0 {
            x1 = width - 1;
        } else {
            x1 = *new_width as i32 - 1;
        }
        x0 = 0;
        x2 = 0;
    } else {
        x0 = 0;
        x1 = width - 1;
        x2 = 0;
    }
    if resizer_id.contains("south") {
        let y_diff = *new_height as i32 - height;
        if y_diff > 0 {
            y1 = height - 1;
        } else {
            y1 = *new_height as i32 - 1;
        }
    } else {
        y1 = height - 1;
    }
    (Point::new(x0, y0), Point::new(x1, y1), Point::new(x2, y2))
}
