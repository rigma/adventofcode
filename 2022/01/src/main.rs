use std::{
    env, fs,
    io::{self, BufRead, BufReader},
};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1])?;
    let file = BufReader::new(file);

    let mut stocks = Vec::new();
    let mut sum = 0;

    for line in file.lines() {
        match line?.parse::<i32>() {
            Ok(n) => sum += n,
            Err(_) => {
                stocks.push(sum);
                sum = 0;
            }
        }
    }

    stocks.push(sum);
    stocks.sort_by(|a, b| b.cmp(a));

    println!("{}", stocks[..3].iter().sum::<i32>());
    Ok(())
}
