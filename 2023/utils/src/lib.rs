use std::{
    env,
    fs::File,
    io::{self, Read},
};

pub fn read_input() -> io::Result<String> {
    let path = env::args().nth(1).unwrap_or("./input.txt".to_owned());
    let mut file = File::open(path)?;
    let mut buffer = String::new();

    file.read_to_string(&mut buffer)?;
    Ok(buffer)
}
