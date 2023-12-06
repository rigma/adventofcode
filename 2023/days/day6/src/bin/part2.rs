use aoc2023_day6::RaceSimulator;
use aoc2023_utils::read_input;
use std::io;

fn parse_number(s: &str) -> u64 {
    let (_, digits) = s.split_once(":").expect("invalid number input");
    digits
        .replace(" ", "")
        .parse::<u64>()
        .expect("invalid integer")
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let mut input = input.lines();

    let duration = parse_number(input.next().expect("missing line input"));
    let record_distance = parse_number(input.next().expect("missing line input"));
    let simulator = RaceSimulator::new(duration, record_distance);

    println!("{}", simulator.winning_races());
    Ok(())
}
