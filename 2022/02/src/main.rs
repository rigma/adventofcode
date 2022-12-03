use std::{
    env, fs,
    io::{self, BufRead, BufReader},
};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1])?;
    let file = BufReader::new(file);

    let total_score: i32 = file
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

    println!("Total score: {}", total_score);
    Ok(())
}
