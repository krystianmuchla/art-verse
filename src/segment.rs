use crate::{line::Line, point::Point};

#[derive(Clone)]
pub struct Segment {
    pub a: Point,
    pub b: Point,
}

impl Segment {
    pub fn new(a: Point, b: Point) -> Self {
        Segment { a, b }
    }

    pub fn as_line(&self) -> Line {
        Line::from_points(&self.a, &self.b)
    }

    pub fn is_point_within(&self, point: &Point) -> bool {
        let min_x = self.a.x.min(self.b.x);
        let max_x = self.a.x.max(self.b.x);
        if point.x < min_x || point.x > max_x {
            return false;
        }
        let min_y = self.a.y.min(self.b.y);
        let max_y = self.a.y.max(self.b.y);
        if point.y < min_y || point.y > max_y {
            return false;
        }
        true
    }
}
