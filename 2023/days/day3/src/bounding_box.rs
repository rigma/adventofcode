use crate::point::Point;

#[derive(Debug)]
pub struct BoundingBox(Point, Point);

impl BoundingBox {
    pub fn new(top_left: Point, bottom_right: Point) -> Self {
        Self(top_left, bottom_right)
    }

    #[inline]
    pub const fn top_left(&self) -> &Point {
        &self.0
    }

    #[inline]
    pub const fn bottom_right(&self) -> &Point {
        &self.1
    }

    #[inline]
    pub const fn contains(&self, point: &Point) -> bool {
        self.top_left().x() <= point.x()
            && point.x() <= self.bottom_right().x()
            && self.top_left().y() <= point.y()
            && point.y() <= self.bottom_right().y()
    }
}
