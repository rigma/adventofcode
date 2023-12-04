use std::{collections::BTreeSet, str::FromStr};

#[inline]
fn parse_numbers(s: &str) -> Vec<u8> {
    s.split(" ")
        .filter(|s| s.len() > 0)
        .map(|s| s.parse::<u8>().expect("expected integer"))
        .collect()
}

#[derive(Clone, Debug)]
pub struct Card {
    winning_numbers: BTreeSet<u8>,
    scratched_numbers: Vec<u8>,
}

impl Card {
    pub fn nb_matches(&self) -> u8 {
        let mut nb_matches: u8 = 0;

        for number in &self.scratched_numbers {
            if self.winning_numbers.contains(number) {
                nb_matches += 1;
            }
        }

        nb_matches
    }

    pub fn points(&self) -> u32 {
        let nb_matches = self.nb_matches() as i8 - 1;
        if nb_matches >= 0 {
            2u32.pow(nb_matches.try_into().unwrap())
        } else {
            0
        }
    }
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (winning_numbers, scratched_numbers) =
            s.split_once(" | ").expect("expected valid card input");

        let winning_numbers = {
            let mut set = BTreeSet::new();
            let numbers = parse_numbers(winning_numbers);

            for number in numbers {
                set.insert(number);
            }

            set
        };
        let scratched_numbers = parse_numbers(scratched_numbers);

        Ok(Self {
            winning_numbers,
            scratched_numbers,
        })
    }
}
