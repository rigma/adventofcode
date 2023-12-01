fn main() {
    let input = include_str!("../../input.txt");
    let mut sum = 0;

    for line in input.lines() {
        let digits: Vec<u32> = line
            .chars()
            .filter(|c| c.is_ascii_digit())
            .map(|c| c.to_digit(10).unwrap())
            .collect();

        sum += 10 * digits.first().unwrap() + digits.last().unwrap();
    }

    println!("{}", sum);
}
