use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::closure::Closure;
use wasm_bindgen::{Clamped, JsCast};
use web_sys::{
    CanvasRenderingContext2d, DomRect, Element, HtmlCanvasElement, HtmlElement, MouseEvent,
};

use crate::dom::Dom;
use crate::{canvas, tool};

pub fn entry_point(
    tool_events: Rc<RefCell<tool::Events>>,
    dom: Rc<Dom>,
    canvas: Rc<HtmlCanvasElement>,
    context: Rc<CanvasRenderingContext2d>,
    image_vec: Rc<RefCell<Clamped<Vec<u8>>>>,
) {
    let resizers = dom.document.get_elements_by_class_name("resizer");
    for resizer_index in 0..resizers.length() {
        let resizer = resizers
            .item(resizer_index)
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap();
        let init_canvas_resize = init_canvas_resize(
            Rc::clone(&tool_events),
            Rc::clone(&dom),
            Rc::clone(&canvas),
            Rc::clone(&context),
            Rc::clone(&image_vec),
        );
        resizer.set_onmousedown(Some(init_canvas_resize.as_ref().unchecked_ref()));
        init_canvas_resize.forget();
    }
}

fn init_canvas_resize(
    tool_events: Rc<RefCell<tool::Events>>,
    dom: Rc<Dom>,
    canvas: Rc<HtmlCanvasElement>,
    context: Rc<CanvasRenderingContext2d>,
    image_vec: Rc<RefCell<Clamped<Vec<u8>>>>,
) -> Closure<dyn FnMut(MouseEvent)> {
    Closure::wrap(Box::new(move |mouse_event: MouseEvent| {
        mouse_event.prevent_default();
        tool_events.borrow().pause(&*dom);
        let sketch = dom.document.get_element_by_id("canvas-sketch");
        if let Some(_) = sketch {
            return;
        }
        let resizer = mouse_event.target().unwrap().dyn_into::<Element>().unwrap();
        let resizer_id = Rc::new(resizer.id());
        let canvas_rect = canvas.get_bounding_client_rect();
        let sketch = Rc::new(create_canvas_sketch(Rc::clone(&dom), &canvas_rect));
        dom.body.append_child(&**sketch).unwrap();
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
            Rc::clone(&canvas),
            Rc::clone(&sketch),
            Rc::clone(&resizer_id),
            Rc::clone(&x_offset),
            Rc::clone(&y_offset),
        );
        dom.body
            .set_onmousemove(Some(resize_sketch.as_ref().unchecked_ref()));
        resize_sketch.forget();
        let resize_canvas = resize_canvas(
            Rc::clone(&tool_events),
            Rc::clone(&dom),
            Rc::clone(&canvas),
            Rc::clone(&context),
            Rc::clone(&image_vec),
            Rc::clone(&sketch),
        );
        dom.body
            .set_onmouseup(Some(resize_canvas.as_ref().unchecked_ref()));
        dom.body
            .set_onmouseleave(Some(resize_canvas.as_ref().unchecked_ref()));
        resize_canvas.forget();
    }) as Box<dyn FnMut(MouseEvent)>)
}

fn create_canvas_sketch(dom: Rc<Dom>, rect: &DomRect) -> HtmlElement {
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
    dom: Rc<Dom>,
    canvas: Rc<HtmlCanvasElement>,
    sketch: Rc<HtmlElement>,
    resizer_id: Rc<String>,
    x_offset: Rc<i32>,
    y_offset: Rc<i32>,
) -> Closure<dyn FnMut(MouseEvent)> {
    Closure::wrap(Box::new(move |mouse_event: MouseEvent| {
        let mut height: Option<i32> = None;
        let mut left_diff: Option<i32> = None;
        let mut right_diff: Option<i32> = None;
        let rect = canvas.get_bounding_client_rect();
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
            let window_width = dom.window.inner_width().unwrap().as_f64().unwrap();
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
    tool_events: Rc<RefCell<tool::Events>>,
    dom: Rc<Dom>,
    canvas: Rc<HtmlCanvasElement>,
    context: Rc<CanvasRenderingContext2d>,
    image_vec: Rc<RefCell<Clamped<Vec<u8>>>>,
    sketch: Rc<HtmlElement>,
) -> Closure<dyn FnMut()> {
    Closure::<dyn FnMut()>::new(move || {
        let sketch_rect = sketch.get_bounding_client_rect();
        let new_width = sketch_rect.width() as u32;
        let new_height = sketch_rect.height() as u32;
        canvas.set_width(new_width);
        canvas.set_height(new_height);
        adjust_image_vec(Rc::clone(&image_vec), &new_width, &new_height);
        canvas::put_image_vec(
            Rc::clone(&canvas),
            Rc::clone(&context),
            Rc::clone(&image_vec),
        );
        sketch.remove();
        dom.body.set_onmousemove(None);
        dom.body.set_onmouseup(None);
        dom.body.set_onmouseleave(None);
        tool_events.borrow().resume(&*dom);
    })
}

fn adjust_image_vec(image_vec: Rc<RefCell<Clamped<Vec<u8>>>>, new_width: &u32, new_height: &u32) {
    let blank_image_vec = blank_image_vec(new_width, new_height);
    // todo figure out
    *image_vec.borrow_mut() = blank_image_vec;
}

fn blank_image_vec(width: &u32, height: &u32) -> Clamped<Vec<u8>> {
    let channels = *width * *height * 4;
    Clamped(vec![255; channels as usize])
}
