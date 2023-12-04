use aoc2023_day4::Card;
use aoc2023_utils::read_input;
use std::{io, str::FromStr};

fn main() -> io::Result<()> {
    let input = read_input()?;
    let cards: Vec<Card> = input
        .lines()
        .map(|line| {
            let (_, card_input) = line.split_once(": ").expect("invalid input line");

            Card::from_str(card_input).unwrap()
        })
        .collect();

    println!("{}", cards.iter().map(|card| card.points()).sum::<u32>());
    Ok(())
}
