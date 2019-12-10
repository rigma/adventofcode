use std::env;
use std::fs;
use std::io::Write;

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

fn add(program: &mut [u32], intcode: (usize, usize, usize, usize)) {
    program[intcode.3] = program[intcode.1] + program[intcode.2];
}

fn mul(program: &mut [u32], intcode: (usize, usize, usize, usize)) {
    program[intcode.3] = program[intcode.1] * program[intcode.2];
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let program = fs::read_to_string(get_input_path())?;
    let program: Vec<&str> = program.split(',').collect();

    let mut program: Vec<u32> = program
        .iter()
        .map(|code| code.parse::<u32>().unwrap())
        .collect();

    let mut i = 0;
    loop {
        let intcode: (usize, usize, usize, usize) = (
            if let Some(a) = program.get(i) {
                *a as usize
            } else {
                panic!("Invalid index")
            },
            if let Some(b) = program.get(i + 1) {
                *b as usize
            } else {
                panic!("Invalid index")
            },
            if let Some(c) = program.get(i + 2) {
                *c as usize
            } else {
                panic!("Invalid index")
            },
            if let Some(d) = program.get(i + 3) {
                *d as usize
            } else {
                panic!("Invalid index")
            },
        );

        match intcode.0 {
            1 => add(&mut program, intcode),
            2 => mul(&mut program, intcode),
            99 => break,
            _ => panic!("Unknown opcode found!"),
        };
        i += 4;
    }

    for code in program {
        print!("{},", code);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_operand() {
        let mut program: Vec<u32> = vec![1, 0, 2, 0];

        add(&mut program, (1, 0, 2, 0));
        assert_eq!(3, program[0]);
    }

    #[test]
    fn test_mul_operand() {
        let mut program: Vec<u32> = vec![1, 0, 2, 0];

        mul(&mut program, (1, 0, 2, 0));
        assert_eq!(2, program[0]);
    }
}
