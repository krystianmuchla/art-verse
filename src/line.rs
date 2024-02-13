use crate::point::Point;

pub struct Line {
    a: f64,
    b: f64,
}

impl Line {
    pub fn from_points(point_a: &Point, point_b: &Point) -> Self {
        let a = (point_b.y as f64 - point_a.y as f64) / (point_b.x as f64 - point_a.x as f64);
        let b = point_a.y as f64 - a * point_a.x as f64;
        Line { a, b }
    }

    pub fn x(&self, y: i32) -> i32 {
        ((y as f64 - self.b) / self.a) as i32
    }

    pub fn y(&self, x: i32) -> i32 {
        (self.a * x as f64 + self.b) as i32
    }
}
