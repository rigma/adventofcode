use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() -> io::Result<()> {
    let file = env::args().nth(1).expect("an input file is required");

    first_part(File::open(&file)?);
    second_part(File::open(&file)?);
    Ok(())
}

fn first_part(file: File) {
    let buf_reader = BufReader::new(file);

    let total_score: u32 = buf_reader
        .lines()
        .map(|line| match &line.expect("unparsable strategy line")[..] {
            "A X" => 4,
            "A Y" => 8,
            "A Z" => 3,
            "B X" => 1,
            "B Y" => 5,
            "B Z" => 9,
            "C X" => 7,
            "C Y" => 2,
            "C Z" => 6,
            _ => 0,
        })
        .sum();

    println!("Naive strategy total score: {}", total_score);
}

fn second_part(file: File) {
    let buf_reader = BufReader::new(file);

    let total_score: u32 = buf_reader
        .lines()
        .map(|line| match &line.unwrap()[..] {
            "A X" => 3,
            "A Y" => 4,
            "A Z" => 8,
            "B X" => 1,
            "B Y" => 5,
            "B Z" => 9,
            "C X" => 2,
            "C Y" => 6,
            "C Z" => 7,
            _ => 0,
        })
        .sum();

    println!("Smart Elve strategy © total score: {}", total_score);
}
