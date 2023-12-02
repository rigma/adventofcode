use aoc2023_day2::parse_games;
use aoc2023_utils::read_input;
use std::io;

fn main() -> io::Result<()> {
    let input = read_input()?;
    let games = parse_games(&input);

    let power_sum: u32 = games
        .iter()
        .map(|(_, sets)| {
            if let Some(first_set) = sets.first() {
                let mut min_red = first_set.red.unwrap_or(1);
                let mut min_green = first_set.green.unwrap_or(1);
                let mut min_blue = first_set.blue.unwrap_or(1);

                sets.iter().skip(1).for_each(|set| {
                    if let Some(red) = set.red {
                        if red > min_red {
                            min_red = red;
                        }
                    }
                    if let Some(green) = set.green {
                        if green > min_green {
                            min_green = green;
                        }
                    }
                    if let Some(blue) = set.blue {
                        if blue > min_blue {
                            min_blue = blue;
                        }
                    }
                });

                (min_red, min_green, min_blue)
            } else {
                (1, 1, 1)
            }
        })
        .map(|(min_red, min_green, min_blue)| min_red * min_blue * min_green)
        .sum();

    println!("{}", power_sum);
    Ok(())
}
