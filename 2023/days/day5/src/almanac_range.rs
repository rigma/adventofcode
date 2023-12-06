use std::str::FromStr;

use crate::AlmanacError;

#[derive(Clone, Eq, Debug, PartialEq)]
pub struct AlmanacRange {
    destination: u64,
    source: u64,
    length: u64,
}

impl AlmanacRange {
    #[inline]
    pub fn new(destination: u64, source: u64, length: u64) -> Self {
        Self {
            destination,
            source,
            length,
        }
    }

    #[inline]
    pub fn contains(&self, number: &u64) -> bool {
        (self.source..self.source + self.length).contains(&number)
    }

    #[inline]
    pub fn transform(&self, number: u64) -> u64 {
        if self.contains(&number) {
            self.destination + number - self.source
        } else {
            number
        }
    }
}

impl FromStr for AlmanacRange {
    type Err = AlmanacError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [destination, source, length] = s.splitn(3, " ").collect::<Vec<_>>()[..] else {
            return Err(AlmanacError::Unpack);
        };

        let destination = destination
            .parse()
            .map_err(|err| AlmanacError::IntError(err))?;
        let source = source.parse().map_err(|err| AlmanacError::IntError(err))?;
        let length = length.parse().map_err(|err| AlmanacError::IntError(err))?;

        Ok(Self::new(destination, source, length))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_transforms_a_number() {
        let range = AlmanacRange {
            destination: 50,
            source: 98,
            length: 2,
        };

        assert_eq!(range.transform(98), 50);
        assert_eq!(range.transform(99), 51);
        assert_eq!(range.transform(100), 100);
    }

    #[test]
    fn it_parses_a_string_into_a_range() {
        assert_eq!(
            Ok(AlmanacRange {
                destination: 50,
                source: 98,
                length: 2
            }),
            AlmanacRange::from_str("50 98 2"),
        );
        assert_eq!(Err(AlmanacError::Unpack), AlmanacRange::from_str(""));
        assert_eq!(Err(AlmanacError::Unpack), AlmanacRange::from_str("1"));
        assert_eq!(Err(AlmanacError::Unpack), AlmanacRange::from_str("1 2"));
        assert_eq!(
            Err(AlmanacError::IntError("foo".parse::<u32>().unwrap_err())),
            AlmanacRange::from_str("50 foo 2")
        );
    }
}
