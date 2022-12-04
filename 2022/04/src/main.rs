use std::{
    collections::HashSet,
    env,
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() -> io::Result<()> {
    let file = env::args().nth(1).expect("you need the input text file");
    let sections = parse_input(File::open(file)?);

    println!(
        "Number of assignments fully contained in the other pair: {}",
        sections
            .iter()
            .filter(|(left, right)| left.is_superset(right) || right.is_superset(left))
            .collect::<Vec<_>>()
            .len()
    );
    println!(
        "Number of assignments that are overlapped: {}",
        sections
            .iter()
            .filter(|(left, right)| left.intersection(right).collect::<HashSet<_>>().len() > 0)
            .collect::<Vec<_>>()
            .len()
    );

    Ok(())
}

fn parse_input(file: File) -> Vec<(HashSet<u8>, HashSet<u8>)> {
    let buf_reader = BufReader::new(file);

    buf_reader
        .lines()
        .map(|line| {
            let segments = line.expect("unable to parse current line");
            let (left, right) = segments
                .split_once(',')
                .expect("expect to range inputs per line");

            (parse_segment(left), parse_segment(right))
        })
        .collect()
}

fn parse_segment(segment: &str) -> HashSet<u8> {
    let (left, right) = segment.split_once('-').expect("expect range notation");
    let left: u8 = left.parse().expect("expected a number");
    let right: u8 = right.parse().expect("expected a number");

    HashSet::from_iter((left..=right).collect::<Vec<_>>())
}
