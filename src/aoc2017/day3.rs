use std::collections::HashMap;

use crate::common::aoc::input_for;
use crate::common::direction::Direction;
use crate::day::Day;
use crate::puzzle_result::PuzzleResult;

fn solve_a_for(input: &str) -> i32 {
    // naive solution

    let target: u32 = input.parse().unwrap();

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

fn solve_b_for(input: &str) -> u32 {
    let target: u32 = input.parse().unwrap();

    let mut cells = HashMap::new();
    cells.insert((0, 0), 1);

    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut border = 0;
    let mut direction = Direction::Right;

    loop {
        let sum: u32 = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 0),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .iter()
        .map(|&(offset_x, offset_y)| cells.get(&(x + offset_x, y + offset_y)).unwrap_or(&0))
        .sum();

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
fn a_example() {
    assert_eq!(solve_a_for("1"), 0);
    assert_eq!(solve_a_for("12"), 3);
    assert_eq!(solve_a_for("23"), 2);
    assert_eq!(solve_a_for("1024"), 31);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(&input_for(2017, 3)), 552);
}

fn solve_a() -> PuzzleResult {
    solve_a_for(&input_for(2017, 3)).into()
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(&input_for(2017, 3)), 330785);
}

fn solve_b() -> PuzzleResult {
    solve_b_for(&input_for(2017, 3)).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
