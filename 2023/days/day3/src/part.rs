use crate::bounding_box::BoundingBox;

#[derive(Debug)]
pub struct Part {
    pub number: u32,
    pub bbox: BoundingBox,
}
