use std::str::FromStr;

use crate::almanac_range::AlmanacRange;

#[derive(Debug)]
pub struct AlmanacMapping {
    inner: Vec<AlmanacRange>,
}

impl AlmanacMapping {
    pub fn new(ranges: Vec<AlmanacRange>) -> Self {
        Self { inner: ranges }
    }

    pub fn transform(&self, number: u64) -> u64 {
        for range in &self.inner {
            if range.contains(&number) {
                return range.transform(number);
            }
        }

        number
    }
}

impl PartialEq for AlmanacMapping {
    fn eq(&self, other: &Self) -> bool {
        if self.inner.len() != other.inner.len() {
            return false;
        }

        self.inner
            .iter()
            .zip(other.inner.iter())
            .all(|(left, right)| left == right)
    }
}

impl<'a> FromIterator<&'a str> for AlmanacMapping {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        let mut ranges = Vec::new();

        for raw in iter {
            if let Ok(range) = AlmanacRange::from_str(raw) {
                ranges.push(range);
            }
        }

        AlmanacMapping::new(ranges)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_parses_multiple_lines_into_a_mapping() {
        let mapping = r"50 98 2
52 50 48";
        let mapping = AlmanacMapping::from_iter(mapping.lines());

        assert_eq!(
            AlmanacMapping::new(vec![
                AlmanacRange::new(50, 98, 2),
                AlmanacRange::new(52, 50, 48)
            ]),
            mapping
        );
    }
}
