use web_sys::CanvasRenderingContext2d;

use crate::image::Image;

pub fn apply_image(context: &CanvasRenderingContext2d, image: &Image) {
    context
        .put_image_data(&image.as_image_data(), 0_f64, 0_f64)
        .unwrap();
}
