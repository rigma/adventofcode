pub mod bounding_box;
pub mod gear;
pub mod part;
pub mod point;

pub use bounding_box::BoundingBox;
pub use gear::Gear;
pub use part::Part;
pub use point::Point;

const GEAR: char = '*';
const RADIX: u32 = 10;
const PERIOD: char = '.';

pub fn get_gear_positions(input: &str) -> Vec<Point> {
    let mut positions = Vec::new();

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c == GEAR {
                positions.push((x, y).into());
            }
        });
    });

    positions
}

pub fn get_symbol_positions(input: &str) -> Vec<Point> {
    let mut positions = Vec::new();

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if !c.is_digit(RADIX) && c != PERIOD {
                positions.push((x, y).into());
            }
        });
    });

    positions
}

pub fn get_parts(input: &str) -> Vec<Part> {
    let mut parts = Vec::new();

    input.lines().enumerate().for_each(|(y, line)| {
        let mut number = None;
        let mut start_pos = 0;

        line.chars().enumerate().for_each(|(x, c)| {
            if c.is_digit(RADIX) {
                let digit = c.to_digit(RADIX).unwrap();
                if let Some(n) = number {
                    number = Some(10 * n + digit);
                } else {
                    number = Some(digit);
                    start_pos = x;
                }
            } else {
                if let Some(n) = number {
                    let pos: i32 = start_pos.try_into().unwrap();
                    let current_x: i32 = x.try_into().unwrap();
                    let current_y: i32 = y.try_into().unwrap();

                    parts.push(Part {
                        number: n,
                        bbox: BoundingBox::new(
                            (pos - 1, current_y - 1).into(),
                            (current_x, current_y + 1).into(),
                        ),
                    });

                    number = None;
                    start_pos = 0;
                }
            }
        });

        if let Some(number) = number {
            let start_pos: i32 = start_pos.try_into().unwrap();
            let current_x: i32 = line.len().try_into().unwrap();
            let current_x = current_x - 1;
            let current_y: i32 = y.try_into().unwrap();

            parts.push(Part {
                number,
                bbox: BoundingBox::new(
                    (start_pos - 1, current_y - 1).into(),
                    (current_x, current_y + 1).into(),
                ),
            });
        }
    });

    parts
}
