use std::io::Read;
use std::fs::File;

enum Direction {
    Up,
    Left,
    Down,
    Right,
}

fn solve(input: &str) -> i32 {
    // naive solution

    let target = u32::from_str_radix(input, 10).unwrap();

    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut border = 0;
    let mut direction = Direction::Right;

    for _ in 1..target {
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

    x.abs() + y.abs()
}

#[test]
fn test() {
    assert_eq!(solve("1"), 0);
    assert_eq!(solve("12"), 3);
    assert_eq!(solve("23"), 2);
    assert_eq!(solve("1024"), 31);
}

fn main() {
    let mut f = File::open("input/3").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    println!("{}", solve(input.trim()));
}
