pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }

    pub fn as_css_value(&self) -> String {
        format!("rgba({},{},{},{})", self.r, self.g, self.b, (self.a as f64) / 255f64)
    }
}
