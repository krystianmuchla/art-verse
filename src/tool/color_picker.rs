use crate::color::Color;
use crate::dom::Dom;
use std::cell::RefCell;
use std::collections::LinkedList;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCollection, HtmlElement, HtmlInputElement, InputEvent};

pub fn init(dom: Rc<RefCell<Dom>>, color: Rc<RefCell<Color>>) {
    dom.borrow()
        .tool_bar
        .color
        .class_list()
        .add_1("selected")
        .unwrap();
    show_color_picker(Rc::clone(&dom));
    let color_picker_color = dom
        .borrow()
        .document
        .get_element_by_id("color-picker-color")
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap();
    let color_picker_color = Rc::new(color_picker_color);
    let color_inputs = dom
        .borrow()
        .document
        .get_elements_by_class_name("color-picker-color");
    let color_inputs = Rc::new(color_inputs);
    for color_input_index in 0..color_inputs.length() {
        let color_input = color_inputs
            .item(color_input_index)
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap();
        let on_input = on_input(
            Rc::clone(&color_picker_color),
            Rc::clone(&color_inputs),
            Rc::clone(&color),
        );
        color_input.set_oninput(Some(on_input.as_ref().unchecked_ref()));
        on_input.forget();
    }
    read_color(&*color_picker_color, &*color_inputs, &*color.borrow_mut());
}

fn on_input(
    color_picker_color: Rc<HtmlElement>,
    color_inputs: Rc<HtmlCollection>,
    color: Rc<RefCell<Color>>,
) -> Closure<dyn FnMut(InputEvent)> {
    Closure::wrap(Box::new(move |event: InputEvent| {
        let input = event
            .current_target()
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap();
        let mut digits: LinkedList<u32> = input
            .value()
            .chars()
            .filter_map(|char| char.to_digit(10))
            .collect();
        loop {
            let digit = digits.pop_front();
            match digit {
                Some(digit) => {
                    if digit == 0 {
                        continue;
                    } else {
                        digits.push_front(digit);
                    }
                }
                None => digits.push_front(0),
            }
            break;
        }
        let mut value = digits.iter().fold(0, |acc, digit| acc * 10 + digit);
        if value > 255 {
            value = 255;
        }
        input.set_value(&*value.to_string());
        write_color(&color_inputs, &color_picker_color, &mut *color.borrow_mut());
    }) as Box<dyn FnMut(InputEvent)>)
}

fn read_color(color_picker_color: &HtmlElement, color_inputs: &HtmlCollection, color: &Color) {
    color_picker_color
        .style()
        .set_property("background-color", &*color.as_css_value())
        .unwrap();
    for color_input_index in 0..color_inputs.length() {
        let color_input = color_inputs
            .item(color_input_index)
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap();
        let color_value: u8;
        match color_input.id().as_str() {
            "color-picker-red" => color_value = color.r,
            "color-picker-green" => color_value = color.g,
            "color-picker-blue" => color_value = color.b,
            id => panic!("Unknown color input {}", id),
        }
        color_input.set_value(&*color_value.to_string());
    }
}

fn write_color(color_inputs: &HtmlCollection, color_picker_color: &HtmlElement, color: &mut Color) {
    for color_input_index in 0..color_inputs.length() {
        let color_input = color_inputs
            .item(color_input_index)
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap();
        let color_value = color_input.value().parse::<u8>().unwrap_or(255);
        match color_input.id().as_str() {
            "color-picker-red" => color.r = color_value,
            "color-picker-green" => color.g = color_value,
            "color-picker-blue" => color.b = color_value,
            id => panic!("Unknown color input {}", id),
        }
    }
    color_picker_color
        .style()
        .set_property("background-color", &*color.as_css_value())
        .unwrap();
}

fn show_color_picker(dom: Rc<RefCell<Dom>>) {
    dom.borrow()
        .document
        .get_element_by_id("color-picker")
        .unwrap()
        .parent_element()
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap()
        .style()
        .set_property("visibility", "visible")
        .unwrap();
}
