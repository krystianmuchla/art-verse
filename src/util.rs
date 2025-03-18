use crate::point::Point;

pub fn flat_idx(point: &Point, width: &u32) -> usize {
    (point.y * *width as i32 + point.x) as usize
}
