use std::collections::HashMap;

use crate::common::aoc::input_for;
use crate::common::direction::Direction;
use crate::day::Day;
use crate::puzzle_result::PuzzleResult;

#[cfg(test)]
const EXAMPLE: &str = include_str!("example/19");

fn solve_a_for(input: &str) -> String {
    let mut field = HashMap::new();
    let mut start_col = None;

    for (row, line) in input.split('\n').enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c != ' ' {
                field.insert((col, row), c);

                start_col.get_or_insert(col);
            }
        }
    }

    let mut x = start_col.unwrap();
    let mut y = 0;
    let mut direction = Direction::Down;
    let mut chars_seen = String::new();

    while let Some(c) = field.get(&(x, y)) {
        if c.is_alphabetic() {
            chars_seen.push(*c);
        } else if *c == '+' {
            direction = match direction {
                Direction::Down | Direction::Up => match field.get(&(x - 1, y)) {
                    Some(_) => Direction::Left,
                    None => Direction::Right,
                },
                Direction::Left | Direction::Right => match field.get(&(x, y - 1)) {
                    Some(_) => Direction::Up,
                    None => Direction::Down,
                },
            }
        }

        match direction {
            Direction::Down => y += 1,
            Direction::Up => y -= 1,
            Direction::Left => x -= 1,
            Direction::Right => x += 1,
        }
    }

    chars_seen
}

fn solve_b_for(input: &str) -> u32 {
    let mut field = HashMap::new();
    let mut start_col = None;

    for (row, line) in input.split('\n').enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c != ' ' {
                field.insert((col, row), c);

                start_col.get_or_insert(col);
            }
        }
    }

    let mut x = start_col.unwrap();
    let mut y = 0;
    let mut direction = Direction::Down;
    let mut steps = 0;

    while let Some(c) = field.get(&(x, y)) {
        steps += 1;
        if *c == '+' {
            direction = match direction {
                Direction::Down | Direction::Up => match field.get(&(x - 1, y)) {
                    Some(_) => Direction::Left,
                    None => Direction::Right,
                },
                Direction::Left | Direction::Right => match field.get(&(x, y - 1)) {
                    Some(_) => Direction::Up,
                    None => Direction::Down,
                },
            }
        }

        match direction {
            Direction::Down => y += 1,
            Direction::Up => y -= 1,
            Direction::Left => x -= 1,
            Direction::Right => x += 1,
        }
    }

    steps
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for(EXAMPLE), "ABCDEF");
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(&input_for(2017, 19)), "RYLONKEWB");
}

fn solve_a() -> PuzzleResult {
    solve_a_for(&input_for(2017, 19)).into()
}

#[test]
fn b_example() {
    assert_eq!(solve_b_for(EXAMPLE), 38);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(&input_for(2017, 19)), 16016);
}

fn solve_b() -> PuzzleResult {
    solve_b_for(&input_for(2017, 19)).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
