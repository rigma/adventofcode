use aoc2023_day6::RaceSimulator;
use aoc2023_utils::read_input;
use std::io;

fn parse_numbers(s: &str) -> Vec<u64> {
    let (_, numbers) = s.split_once(":").expect("invalid numbers input");
    numbers
        .trim()
        .split(" ")
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().expect("invalid integer"))
        .collect()
}

fn main() -> io::Result<()> {
    let input = read_input()?;
    let mut input = input.lines();

    let timings = parse_numbers(input.next().expect("expected input line"));
    let distances = parse_numbers(input.next().expect("expected input line"));

    let answer: usize = timings
        .iter()
        .zip(distances)
        .map(|(duration, record_distance)| {
            let simulator = RaceSimulator::new(*duration, record_distance);
            simulator.winning_races()
        })
        .product();

    println!("{}", answer);
    Ok(())
}
