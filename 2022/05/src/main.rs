#[macro_use]
extern crate lazy_static;

use std::{
    collections::VecDeque,
    env,
    fs::File,
    io::{self, BufRead, BufReader},
};

lazy_static! {
    static ref STACK_BASE: String = String::from(" 1   2   3   4   5   6   7   8   9 ");
}

fn main() -> io::Result<()> {
    let file = env::args()
        .nth(1)
        .expect("you must provide input text file");

    let (skip_lines, initial_state) = parse_initial_state(File::open(&file)?);
    let hanoi_final_state = simulate_hanoi_crane(File::open(&file)?, skip_lines, &initial_state);
    let final_state = simulate_crane(File::open(&file)?, skip_lines, &initial_state);

    print!("Front crates after Hanoï strategy: ");
    for mut stack in hanoi_final_state {
        print!("{}", stack.pop_front().unwrap());
    }
    println!("");

    print!("Front crates after Real-Life © strategy: ");
    for mut stack in final_state {
        print!("{}", stack.pop_front().unwrap());
    }
    println!("");

    Ok(())
}

fn parse_initial_state(file: File) -> (usize, Vec<VecDeque<char>>) {
    let buf_reader = BufReader::new(file);
    let mut cargo_crane = vec![VecDeque::<char>::new(); 9];

    for line in buf_reader.lines() {
        match line {
            Ok(line) => {
                if line == *STACK_BASE {
                    break;
                }

                for (idx, char) in line.char_indices() {
                    if char != ' ' && char != '[' && char != ']' {
                        cargo_crane[idx / 4].push_back(char);
                    }
                }
            }
            Err(_) => panic!("PAAAAAANIC!"),
        }
    }

    (
        2 + cargo_crane.iter().fold(0, |len, stack| {
            let current_len = stack.len();
            if current_len > len {
                current_len
            } else {
                len
            }
        }),
        cargo_crane,
    )
}

fn simulate_hanoi_crane(
    file: File,
    skip_lines: usize,
    cargo_crane: &Vec<VecDeque<char>>,
) -> Vec<VecDeque<char>> {
    let mut universe = cargo_crane.clone();
    let buf_reader = BufReader::new(file);

    for line in buf_reader.lines().skip(skip_lines) {
        let (nb, from, to) = {
            let instruction = line.expect("crane instruction is expected here");
            let (nb, coordinates) = instruction[5..]
                .split_once(" from ")
                .expect("instructions must have a number of crates");
            let (from, to) = coordinates
                .split_once(" to ")
                .expect("instructions must have coordinates");

            (
                nb.parse::<usize>().expect("expect a usize interger"),
                from.parse::<usize>().expect("expect a usize integer") - 1,
                to.parse::<usize>().expect("expect a usize integer") - 1,
            )
        };

        for _ in 0..nb {
            let crate_ = universe[from]
                .pop_front()
                .expect("crates should be remaining!");
            universe[to].push_front(crate_);
        }
    }

    universe
}

fn simulate_crane(
    file: File,
    skip_lines: usize,
    cargo_crane: &Vec<VecDeque<char>>,
) -> Vec<VecDeque<char>> {
    let mut universe = cargo_crane.clone();
    let buf_reader = BufReader::new(file);

    for line in buf_reader.lines().skip(skip_lines) {
        let (nb, from, to) = {
            let instruction = line.expect("crane instruction is expected here");
            let (nb, coordinates) = instruction[5..]
                .split_once(" from ")
                .expect("instructions must have a number of crates");
            let (from, to) = coordinates
                .split_once(" to ")
                .expect("instructions must have coordinates");

            (
                nb.parse::<usize>().expect("expect a usize interger"),
                from.parse::<usize>().expect("expect a usize integer") - 1,
                to.parse::<usize>().expect("expect a usize integer") - 1,
            )
        };

        let mut stack = VecDeque::new();
        for _ in 0..nb {
            let crate_ = universe[from]
                .pop_front()
                .expect("crates should be remaining!");
            stack.push_back(crate_);
        }

        while let Some(crate_) = stack.pop_back() {
            universe[to].push_front(crate_);
        }
    }

    universe
}
