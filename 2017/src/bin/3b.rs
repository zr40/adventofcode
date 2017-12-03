use std::io::Read;
use std::fs::File;
use std::collections::HashMap;

enum Direction {
    Up,
    Left,
    Down,
    Right,
}

fn solve(input: &str) -> u32 {
    let target = u32::from_str_radix(input, 10).unwrap();

    let mut cells = HashMap::new();
    cells.insert((0, 0), 1);

    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut border = 0;
    let mut direction = Direction::Right;

    loop {
        let sum: u32 = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 0), (0, 1), (1, -1), (1, 0), (1, 1)].iter().map(
            |&(offset_x, offset_y)| cells.get(&(x + offset_x, y + offset_y)).unwrap_or(&0)
        ).sum();

        if sum > target {
            return sum;
        }
        cells.insert((x, y), sum);

        match direction {
            Direction::Right => {
                if x == border {
                    direction = Direction::Up;
                    border += 1;
                }

                x += 1;
            }
            Direction::Up => {
                if y == -border {
                    direction = Direction::Left;
                    x -= 1;
                } else {
                    y -= 1;
                }
            }
            Direction::Left => {
                if x == -border {
                    direction = Direction::Down;
                    y += 1;
                } else {
                    x -= 1;
                }
            }
            Direction::Down => {
                if y == border {
                    direction = Direction::Right;
                    x += 1;
                } else {
                    y += 1;
                }
            }
        }
    }
}

#[test]
fn test() {
    assert_eq!(solve("1"), 2);
    assert_eq!(solve("2"), 4);
    assert_eq!(solve("3"), 4);
    assert_eq!(solve("4"), 5);
}

fn main() {
    let mut f = File::open("input/3").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    println!("{}", solve(input.trim()));
}
