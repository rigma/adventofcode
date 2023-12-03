use aoc2023_day3::{get_parts, get_symbol_positions};
use aoc2023_utils::read_input;
use std::io;

fn main() -> io::Result<()> {
    let input = read_input()?;
    let positions = get_symbol_positions(&input);

    let parts = get_parts(&input);
    let correct_parts = parts
        .iter()
        .filter(|part| positions.iter().any(|pos| part.bbox.contains(&pos)))
        .collect::<Vec<_>>();

    println!(
        "{}",
        correct_parts.iter().map(|part| part.number).sum::<u32>()
    );
    Ok(())
}
