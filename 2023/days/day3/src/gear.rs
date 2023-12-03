use crate::{part::Part, point::Point};

#[derive(Debug)]
pub struct Gear<'a>(&'a Part, &'a Part);

impl<'a> Gear<'a> {
    pub fn validate(position: &Point, parts: &'a [&'a Part]) -> Option<Self> {
        let adjacent_parts = parts
            .iter()
            .filter(|part| part.bbox.contains(&position))
            .collect::<Vec<_>>();

        if adjacent_parts.len() != 2 {
            return None;
        }

        Some(Gear(
            adjacent_parts.first().unwrap(),
            adjacent_parts.last().unwrap(),
        ))
    }

    pub const fn ratio(&self) -> u32 {
        self.0.number * self.1.number
    }
}
