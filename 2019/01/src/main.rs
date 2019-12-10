extern crate rayon;

use std::env;
use std::io::Write;
use std::fs;
use rayon::prelude::*;

#[cfg(target_os = "windows")]
fn get_input_path() -> String {
    let current_dir = env::current_dir().unwrap();
    let current_dir = current_dir.to_str().unwrap();

    let mut path = Vec::new();
    write!(&mut path, "{}\\input.txt", current_dir).unwrap();

    String::from_utf8(path).unwrap()
}

#[cfg(not(target_os = "windows"))]
fn get_input_path() -> String {
    let current_dir = env::current_dir().unwrap();
    let current_dir = current_dir.to_str().unwrap();

    let mut path = Vec::new();
    write!(&mut path, "{}/input.txt", current_dir).unwrap();

    String::from_utf8(path).unwrap()
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let masses: String = fs::read_to_string(get_input_path())?;
    let masses: Vec<&str> = masses.split("\n").collect();
    let result: f64 = masses.par_iter()
        .map(|&mass| {
            let mass: f64 = mass.parse::<f64>().unwrap();

            (mass / 3.).floor() - 2.
        })
        .sum();

    println!("{}", result);
    Ok(())
}
