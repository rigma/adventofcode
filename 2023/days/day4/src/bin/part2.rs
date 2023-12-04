use aoc2023_day4::Card;
use aoc2023_utils::read_input;
use std::{collections::BTreeMap, io, str::FromStr};

fn read_cards(input: &str) -> BTreeMap<u8, Vec<Card>> {
    let mut mapping: BTreeMap<u8, Vec<Card>> = BTreeMap::new();

    input.lines().enumerate().for_each(|(idx, line)| {
        let (_, card) = line.split_once(": ").expect("invalid input line");
        let card = Card::from_str(card).expect("invalid card format");
        let number = idx as u8 + 1;

        if let Some(cards) = mapping.get_mut(&number) {
            cards.push(card);
        } else {
            mapping.insert(number, vec![card]);
        }
    });

    mapping
}

fn main() -> io::Result<()> {
    let input = read_input()?;

    let original_mapping = read_cards(&input);
    let mut earnings: BTreeMap<u8, usize> =
        BTreeMap::from_iter(original_mapping.iter().map(|(n, _)| (*n, 1)));

    for (number, cards) in original_mapping {
        let card = cards.first().unwrap();
        let nb_matches = card.nb_matches();

        if nb_matches > 0 {
            let current_card_earning = earnings.get(&number).unwrap().clone();

            for n in number + 1..=number + nb_matches {
                if let Some(count) = earnings.get_mut(&n) {
                    *count += current_card_earning;
                }
            }
        }
    }

    println!("{}", earnings.iter().map(|(_, count)| count).sum::<usize>());
    Ok(())
}
