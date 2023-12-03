use aoc2023_day3::{get_gear_positions, get_parts, get_symbol_positions, Gear};
use aoc2023_utils::read_input;
use std::io;

fn main() -> io::Result<()> {
    let input = read_input()?;
    let positions = get_symbol_positions(&input);

    let parts = get_parts(&input);
    let parts = parts
        .iter()
        .filter(|part| positions.iter().any(|pos| part.bbox.contains(&pos)))
        .collect::<Vec<_>>();

    let positions = get_gear_positions(&input);
    let gears = positions
        .iter()
        .map(|pos| Gear::validate(&pos, &parts[..]))
        .filter(|gear| gear.is_some())
        .map(|gear| gear.unwrap())
        .collect::<Vec<_>>();

    println!("{}", gears.iter().map(|gear| gear.ratio()).sum::<u32>());
    Ok(())
}
