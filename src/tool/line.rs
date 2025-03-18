use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use web_sys::MouseEvent;

use crate::color::Color;
use crate::dom::Dom;
use crate::point::Point;
use crate::segment::Segment;
use crate::util::flat_idx;

pub fn init(dom: Rc<RefCell<Dom>>, color: Rc<RefCell<Color>>) {
    let tools = dom.borrow().document.get_elements_by_class_name("tool");
    for tool_idx in 0..tools.length() {
        let tool = tools.item(tool_idx).unwrap();
        tool.class_list().remove_1("selected").unwrap();
    }
    dom.borrow()
        .tool_bar
        .line
        .class_list()
        .add_1("selected")
        .unwrap();
    let start = start(Rc::clone(&dom), Rc::clone(&color));
    dom.borrow_mut().canvas.set_on_mouse_down(Some(&start));
    start.forget();
}

pub fn put(pixels: &mut Vec<Rc<Color>>, width: &u32, segment: &Segment, color: &Color) {
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
        pixels[flat_idx(&point, width)] = Rc::new(color.clone());
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

fn start(dom: Rc<RefCell<Dom>>, color: Rc<RefCell<Color>>) -> Closure<dyn FnMut(MouseEvent)> {
    Closure::wrap(Box::new(move |mouse_event: MouseEvent| {
        dom.borrow_mut().canvas.set_on_mouse_down(None);
        let point_a = dom.borrow().canvas.get_point(&mouse_event);
        let point_a = Rc::new(point_a);
        let advance = advance(Rc::clone(&dom), Rc::clone(&point_a), Rc::clone(&color));
        dom.borrow_mut().canvas.set_on_mouse_move(Some(&advance));
        advance.forget();
        let end = end(Rc::clone(&dom), Rc::clone(&point_a), Rc::clone(&color));
        dom.borrow_mut().canvas.set_on_mouse_up(Some(&end));
        dom.borrow_mut().canvas.set_on_mouse_leave(Some(&end));
        end.forget();
    }) as Box<dyn FnMut(MouseEvent)>)
}

fn advance(
    dom: Rc<RefCell<Dom>>,
    point_a: Rc<Point>,
    color: Rc<RefCell<Color>>,
) -> Closure<dyn FnMut(MouseEvent)> {
    Closure::wrap(Box::new(move |mouse_event: MouseEvent| {
        let point_b = dom.borrow().canvas.get_point(&mouse_event);
        let segment = dom.borrow().canvas.get_segment(&point_a, &point_b);
        if let Some(segment) = segment {
            let mut pixels: Vec<Rc<Color>> = dom
                .borrow()
                .canvas
                .pixels
                .iter()
                .map(|color| Rc::clone(color))
                .collect();
            put(
                &mut pixels,
                &dom.borrow().canvas.element.width(),
                &segment,
                &color.borrow(),
            );
            dom.borrow().canvas.render_external_pixels(&pixels);
        } else {
            dom.borrow().canvas.refresh();
        }
    }) as Box<dyn FnMut(MouseEvent)>)
}

fn end(
    dom: Rc<RefCell<Dom>>,
    point_a: Rc<Point>,
    color: Rc<RefCell<Color>>,
) -> Closure<dyn FnMut(MouseEvent)> {
    Closure::wrap(Box::new(move |mouse_event: MouseEvent| {
        dom.borrow_mut().canvas.set_on_mouse_move(None);
        dom.borrow_mut().canvas.set_on_mouse_up(None);
        dom.borrow_mut().canvas.set_on_mouse_leave(None);
        let point_b = dom.borrow().canvas.get_point(&mouse_event);
        let segment = dom.borrow().canvas.get_segment(&point_a, &point_b);
        if let Some(segment) = segment {
            let width = dom.borrow().canvas.element.width();
            put(
                &mut dom.borrow_mut().canvas.pixels,
                &width,
                &segment,
                &color.borrow(),
            );
            dom.borrow().canvas.refresh();
        }
        let start = start(Rc::clone(&dom), Rc::clone(&color));
        dom.borrow_mut().canvas.set_on_mouse_down(Some(&start));
        start.forget();
    }) as Box<dyn FnMut(MouseEvent)>)
}
