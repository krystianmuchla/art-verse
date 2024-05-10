use wasm_bindgen::Clamped;
use web_sys::ImageData;

use crate::{color::Color, point::Point};

#[derive(Clone)]
pub struct Image {
    pub vector: Clamped<Vec<u8>>,
    pub width: u32,
}

impl Image {
    pub fn new(vector: Clamped<Vec<u8>>, width: u32) -> Image {
        Image { vector, width }
    }

    pub fn as_image_data(&self) -> ImageData {
        ImageData::new_with_u8_clamped_array(Clamped(&self.vector.0[..]), self.width).unwrap()
    }

    pub fn vector_index(&self, point: &Point) -> usize {
        (point.y * (self.width as i32 * 4) + point.x * 4) as usize
    }

    pub fn color_pixel(&mut self, point: &Point, color: &Color) {
        let index = self.vector_index(point);
        self.vector[index] = color.r;
        self.vector[index + 1] = color.g;
        self.vector[index + 2] = color.b;
        self.vector[index + 3] = color.a;
    }

    pub fn pixel_color(&self, point: &Point) -> Color {
        let index = self.vector_index(point);
        let r = self.vector[index];
        let g = self.vector[index + 1];
        let b = self.vector[index + 2];
        let a = self.vector[index + 3];
        return Color::new(r, g, b, a);
    }

    pub fn copy(&self, from: &Point, to: &Point, other: &mut Image, other_from: &Point) {
        for (y_index, y) in (from.y..=to.y).enumerate() {
            for (x_index, x) in (from.x..=to.x).enumerate() {
                let point = other_from.add(&Point::new(x_index as i32, y_index as i32));
                let color = self.pixel_color(&Point::new(x, y));
                other.color_pixel(&point, &color);
            }
        }
    }
}
