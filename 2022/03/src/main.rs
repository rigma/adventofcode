use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader},
};

fn item_to_priority(item: char) -> u32 {
    if item.is_alphabetic() && item.is_lowercase() {
        item as u32 - 'a' as u32 + 1
    } else if item.is_alphabetic() && item.is_uppercase() {
        item as u32 - 'A' as u32 + 27
    } else {
        panic!("PANIC!")
    }
}

fn first_part(file: File) {
    let buf_reader = BufReader::new(file).lines();

    println!(
        "Sum of priorities of the duplicates: {}",
        buf_reader
            .map(|line| {
                let line = line.expect("a valid line is expected from the input");
                let (first, second) = line.split_at(line.chars().count() / 2);
                let (first, second) = (first.to_owned(), second.to_owned());

                first
                    .chars()
                    .reduce(|duplicate, item| {
                        if let Some(_) = second.find(item) {
                            item
                        } else {
                            duplicate
                        }
                    })
                    .unwrap()
            })
            .map(item_to_priority)
            .sum::<u32>()
    );
}

fn second_part(file: File) {
    use itertools::Itertools;

    let rucksacks = BufReader::new(file)
        .lines()
        .map(|line| line.expect("Expect a rucksacks line"))
        .collect::<Vec<_>>();

    println!(
        "Sum of badges priorities: {}",
        rucksacks
            .iter()
            .chunks(3)
            .into_iter()
            .map(|mut chunk| {
                let (a, b, c) = (
                    chunk.nth(0).expect("Expect rucksack line"),
                    chunk.nth(0).expect("Expect rucksack line"),
                    chunk.nth(0).expect("Expect rucksack line"),
                );

                a.chars()
                    .reduce(|badge, item| match (b.find(item), c.find(item)) {
                        (Some(_), Some(_)) => item,
                        _ => badge,
                    })
                    .unwrap()
            })
            .map(item_to_priority)
            .sum::<u32>()
    );
}

fn main() -> io::Result<()> {
    let file = env::args().nth(1).expect("Missing input text file");

    first_part(File::open(&file)?);
    second_part(File::open(&file)?);

    Ok(())
}
