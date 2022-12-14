use serde_json::Value;
use std::{
    cmp::Ordering,
    env,
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() -> io::Result<()> {
    let file = env::args().nth(1).expect("puzzle input is expected");
    let pairs = parse_pairs(File::open(&file)?)?;
    let mut packets = parse_packets(File::open(&file)?);

    let sum_of_correct_pairs_idx: usize = pairs
        .iter()
        .enumerate()
        .filter(|(_, pair)| {
            compare_packets(
                pair.0
                    .as_array()
                    .expect("left packet should be a JSON-formatted array"),
                pair.1
                    .as_array()
                    .expect("right packet should be a JSON-formatted array"),
            ) == Ordering::Less
        })
        .map(|(idx, _)| idx + 1)
        .sum();

    println!(
        "Sum of indexes in correct order: {}",
        sum_of_correct_pairs_idx
    );

    packets.sort_by(|a, b| compare_packets(a.as_array().unwrap(), b.as_array().unwrap()));
    let first_divider_idx = packets
        .iter()
        .position(|packet| *packet == serde_json::json!([[2]]))
        .expect("we should have found first divider packet [[2]]");
    let second_divider_idx = packets
        .iter()
        .position(|packet| *packet == serde_json::json!([[6]]))
        .expect("we should have found second divider packet [[6]]");

    println!(
        "Decoding key: {}",
        (first_divider_idx + 1) * (second_divider_idx + 1)
    );
    Ok(())
}

fn parse_pairs(file: File) -> io::Result<Vec<(Value, Value)>> {
    let buffer = BufReader::new(file);
    let mut input = Vec::new();
    let mut left: Option<Value> = None;
    let mut right: Option<Value> = None;

    for line in buffer.lines() {
        let line = line?;

        if line == "" {
            input.push((left.unwrap(), right.unwrap()));
            left = None;
            right = None;
        } else if left.is_none() {
            left =
                Some(serde_json::from_str(&line).expect("input should be a JSON-formatted array"));
        } else if right.is_none() {
            right =
                Some(serde_json::from_str(&line).expect("input should be a JSON-formatted array"));
        } else {
            panic!("PAAAAAAAAANIC!");
        }
    }

    input.push((left.unwrap(), right.unwrap()));
    Ok(input)
}

fn parse_packets(file: File) -> Vec<Value> {
    let buffer = BufReader::new(file);
    let mut packets: Vec<Value> = buffer
        .lines()
        .filter(|line| {
            if let Ok(line) = &line {
                line != ""
            } else {
                panic!("PAAAAAAAAANIC")
            }
        })
        .map(|line| serde_json::from_str(&line.unwrap()).unwrap())
        .collect();

    packets.push(serde_json::json!([[2]]));
    packets.push(serde_json::json!([[6]]));
    packets
}

fn compare_packets(left: &[Value], right: &[Value]) -> Ordering {
    for pair in left.iter().zip(right.iter()) {
        match pair {
            (Value::Number(x), Value::Number(y)) => {
                let x = x.as_u64().expect("should be an integer");
                let y = y.as_u64().expect("should be an integer");

                if x < y {
                    return Ordering::Less;
                } else if x > y {
                    return Ordering::Greater;
                }
            }
            (Value::Array(x), Value::Array(y)) => {
                let cmp = compare_packets(x, y);
                if cmp != Ordering::Equal {
                    return cmp;
                }
            }
            (Value::Array(x), Value::Number(y)) => {
                let cmp = compare_packets(x, &[Value::Number(y.clone())]);
                if cmp != Ordering::Equal {
                    return cmp;
                }
            }
            (Value::Number(x), Value::Array(y)) => {
                let cmp = compare_packets(&[Value::Number(x.clone())], y);
                if cmp != Ordering::Equal {
                    return cmp;
                }
            }
            _ => panic!("PAAAAAAAAANIC"),
        }
    }

    if left.len() < right.len() {
        Ordering::Less
    } else if left.len() > right.len() {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}
