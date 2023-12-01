#[macro_use]
extern crate lazy_static;

use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{self, Read},
};

lazy_static! {
    static ref PATTERNS: HashMap<&'static str, &'static str> = {
        let mut mapping = HashMap::new();
        mapping.insert("one", "o1e");
        mapping.insert("two", "t2o");
        mapping.insert("three", "t3e");
        mapping.insert("four", "f4r");
        mapping.insert("five", "f5e");
        mapping.insert("six", "s6x");
        mapping.insert("seven", "s7n");
        mapping.insert("eight", "e8t");
        mapping.insert("nine", "n9e");
        mapping
    };
}

fn mangled_line(line: &str) -> bool {
    PATTERNS.keys().any(|pattern| line.contains(pattern))
}

fn decode_line(line: &str) -> String {
    let mut decoded_line = line.to_string();

    while mangled_line(&decoded_line) {
        let mut selected_pattern: Option<&str> = None;
        let mut min_idx: Option<usize> = None;

        for pattern in PATTERNS.keys() {
            if let Some(idx) = decoded_line.find(pattern) {
                if let Some(current) = min_idx {
                    if current > idx {
                        selected_pattern = Some(pattern);
                        min_idx = Some(idx);
                    }
                } else {
                    selected_pattern = Some(pattern);
                    min_idx = Some(idx);
                }
            }
        }

        if let Some(pattern) = selected_pattern {
            decoded_line = decoded_line.replace(pattern, PATTERNS.get(pattern).unwrap());
        }
    }

    decoded_line
}

fn main() -> io::Result<()> {
    let path = env::args().nth(1).unwrap_or("./input.txt".to_owned());
    let mut file = File::open(path)?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let mut sum = 0;

    for line in input.lines() {
        let decoded_line = decode_line(line);
        let digits: Vec<u32> = decoded_line
            .chars()
            .filter(|c| c.is_ascii_digit())
            .map(|c| c.to_digit(10).unwrap())
            .collect();

        let calibration_value = 10 * digits.first().unwrap() + digits.last().unwrap();
        println!("{} => {} => {}", line, decoded_line, calibration_value);
        sum += calibration_value;
    }

    println!("{}", sum);
    Ok(())
}
