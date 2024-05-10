use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{
    CanvasRenderingContext2d, DomRect, Element, HtmlCanvasElement, HtmlElement, MouseEvent,
};

use crate::dom::Dom;
use crate::image::Image;
use crate::point::Point;
use crate::{context, image_vec, tool};

pub fn entry_point(
    tool_events: Rc<RefCell<tool::Events>>,
    dom: Rc<Dom>,
    canvas: Rc<HtmlCanvasElement>,
    context: Rc<CanvasRenderingContext2d>,
    image: Rc<RefCell<Image>>,
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
            Rc::clone(&image),
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
    image: Rc<RefCell<Image>>,
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
            Rc::clone(&image),
            Rc::clone(&sketch),
            Rc::clone(&resizer_id),
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
    image: Rc<RefCell<Image>>,
    sketch: Rc<HtmlElement>,
    resizer_id: Rc<String>,
) -> Closure<dyn FnMut()> {
    Closure::<dyn FnMut()>::new(move || {
        let sketch_rect = sketch.get_bounding_client_rect();
        let sketch_width = sketch_rect.width() as u32;
        let sketch_height = sketch_rect.height() as u32;
        adjust_image(
            &*canvas,
            &mut *image.borrow_mut(),
            &sketch_width,
            &sketch_height,
            &*resizer_id,
        );
        canvas.set_width(sketch_width);
        canvas.set_height(sketch_height);
        context::apply_image(&*context, &image.borrow());
        sketch.remove();
        dom.body.set_onmousemove(None);
        dom.body.set_onmouseup(None);
        dom.body.set_onmouseleave(None);
        tool_events.borrow().resume(&*dom);
    })
}

fn adjust_image(
    canvas: &HtmlCanvasElement,
    image: &mut Image,
    sketch_width: &u32,
    sketch_height: &u32,
    resizer_id: &String,
) {
    let image_width = canvas.width() as i32;
    let image_height = canvas.height() as i32;
    let x0: i32;
    let y0: i32 = 0;
    let x1: i32;
    let y1: i32;
    let x2: i32;
    let y2: i32 = 0;
    let new_image_vec = image_vec::blank(sketch_width * sketch_height);
    let new_image_width = *sketch_width;
    let mut new_image = Image::new(new_image_vec, new_image_width);
    if resizer_id.contains("west") {
        let x_diff = *sketch_width as i32 - image_width;
        if x_diff > 0 {
            x0 = 0;
            x2 = x_diff;
        } else {
            x0 = -x_diff;
            x2 = 0;
        }
        x1 = image_width - 1;
    } else if resizer_id.contains("east") {
        let x_diff = *sketch_width as i32 - image_width;
        if x_diff > 0 {
            x1 = image_width - 1;
        } else {
            x1 = *sketch_width as i32 - 1;
        }
        x0 = 0;
        x2 = 0;
    } else {
        x0 = 0;
        x1 = image_width - 1;
        x2 = 0;
    }
    if resizer_id.contains("south") {
        let y_diff = *sketch_height as i32 - image_height;
        if y_diff > 0 {
            y1 = image_height - 1;
        } else {
            y1 = *sketch_height as i32 - 1;
        }
    } else {
        y1 = image_height - 1;
    }
    image.copy(
        &Point::new(x0, y0),
        &Point::new(x1, y1),
        &mut new_image,
        &Point::new(x2, y2),
    );
    *image = new_image;
}
