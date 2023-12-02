use aoc2023_day2::parse_games;
use aoc2023_utils::read_input;
use std::io;

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

fn main() -> io::Result<()> {
    let input = read_input()?;
    let games = parse_games(&input);

    let valid_games_sum: u32 = games
        .iter()
        .filter(|(_, sets)| {
            sets.iter().all(|set| {
                set.red.unwrap_or(0) <= MAX_RED
                    && set.green.unwrap_or(0) <= MAX_GREEN
                    && set.blue.unwrap_or(0) <= MAX_BLUE
            })
        })
        .map(|(id, _)| *id)
        .sum();

    println!("{}", valid_games_sum);
    Ok(())
}
