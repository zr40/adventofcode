use std::collections::{HashMap, HashSet};

use crate::PuzzleResult;
use crate::common::aoc::input_for;
use crate::day::Day;

#[cfg(test)]
const EXAMPLE: &str = include_str!("example/3");

struct Part {
    number: u32,
    row: usize,
    start_col: usize,
    end_col: usize,
}

fn solve_a_for(input: &str) -> u32 {
    let mut parts: Vec<Part> = vec![];
    let mut adjacent_to_symbol: HashSet<(usize, usize)> = HashSet::new();

    for (row, line) in input.lines().enumerate() {
        let row = row + 1;

        let mut char_iter = line.chars().enumerate().map(|(col, char)| (col + 1, char));
        let mut number = 0;
        let mut start_col = 0;
        let mut end_col = 0;

        loop {
            match char_iter.next() {
                Some((col, ch)) if ch.is_ascii_digit() => {
                    if number == 0 {
                        start_col = col;
                    }
                    end_col = col;
                    number = number * 10 + ch.to_digit(10).unwrap();
                }
                Some((col, ch)) => {
                    if number != 0 {
                        parts.push(Part {
                            number,
                            row,
                            start_col,
                            end_col,
                        });
                        number = 0;
                    }
                    if ch != '.' {
                        for row in (row - 1)..=(row + 1) {
                            for col in (col - 1)..=(col + 1) {
                                adjacent_to_symbol.insert((row, col));
                            }
                        }
                    }
                }
                None => {
                    if number != 0 {
                        parts.push(Part {
                            number,
                            row,
                            start_col,
                            end_col,
                        });
                    }
                    break;
                }
            }
        }
    }
    parts
        .iter()
        .filter_map(|part| {
            for col in part.start_col..=part.end_col {
                if adjacent_to_symbol.contains(&(part.row, col)) {
                    return Some(part.number);
                }
            }
            None
        })
        .sum()
}

fn solve_b_for(input: &str) -> u32 {
    let mut parts: Vec<Part> = vec![];
    let mut adjacent_to_gear: HashMap<(usize, usize), usize> = HashMap::new();
    let mut gears: Vec<Vec<Part>> = vec![];

    for (row, line) in input.lines().enumerate() {
        let row = row + 1;

        let mut char_iter = line.chars().enumerate().map(|(col, char)| (col + 1, char));
        let mut number = 0;
        let mut start_col = 0;
        let mut end_col = 0;

        loop {
            match char_iter.next() {
                Some((col, ch)) if ch.is_ascii_digit() => {
                    if number == 0 {
                        start_col = col;
                    }
                    end_col = col;
                    number = number * 10 + ch.to_digit(10).unwrap();
                }
                Some((col, sym)) => {
                    if number != 0 {
                        parts.push(Part {
                            number,
                            row,
                            start_col,
                            end_col,
                        });
                        number = 0;
                    }
                    if sym == '*' {
                        let index = gears.len();
                        gears.push(vec![]);
                        for row in (row - 1)..=(row + 1) {
                            for col in (col - 1)..=(col + 1) {
                                adjacent_to_gear.insert((row, col), index);
                            }
                        }
                    }
                }
                None => {
                    if number != 0 {
                        parts.push(Part {
                            number,
                            row,
                            start_col,
                            end_col,
                        });
                    }
                    break;
                }
            }
        }
    }
    for part in parts {
        for col in part.start_col..=part.end_col {
            if adjacent_to_gear.contains_key(&(part.row, col)) {
                gears[adjacent_to_gear[&(part.row, col)]].push(part);
                break;
            }
        }
    }

    gears
        .into_iter()
        .filter_map(|gear| match gear.len() {
            1 => None,
            2 => Some(gear[0].number * gear[1].number),
            other => {
                panic!("unexpected adjacency count {other}")
            }
        })
        .sum()
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for(EXAMPLE), 4361);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(&input_for(2023, 3)), 546563);
}

fn solve_a() -> PuzzleResult {
    solve_a_for(&input_for(2023, 3)).into()
}

#[test]
fn b_example() {
    assert_eq!(solve_b_for(EXAMPLE), 467835);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(&input_for(2023, 3)), 91031374);
}

fn solve_b() -> PuzzleResult {
    solve_b_for(&input_for(2023, 3)).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
