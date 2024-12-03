use regex::Regex;

use crate::PuzzleResult;
use crate::common::aoc::input_for;
use crate::day::Day;

#[cfg(test)]
const EXAMPLE_A: &str = include_str!("example/3a");
#[cfg(test)]
const EXAMPLE_B: &str = include_str!("example/3b");

enum Mode {
    Always,
    Toggle,
}

fn solve_for(mut input: &str, mode: Mode) -> usize {
    let mut active = true;

    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)|do\(\)|don't\(\)").unwrap();
    re.captures_iter(input)
        .map(|captures| match captures.get(0).unwrap().as_str() {
            "do()" => {
                if let Mode::Toggle = mode {
                    active = true;
                }
                0
            }
            "don't()" => {
                if let Mode::Toggle = mode {
                    active = false;
                }
                0
            }
            _ => {
                if active {
                    captures.get(1).unwrap().as_str().parse::<usize>().unwrap()
                        * captures.get(2).unwrap().as_str().parse::<usize>().unwrap()
                } else {
                    0
                }
            }
        })
        .sum()
}

#[test]
fn a_example() {
    assert_eq!(solve_for(EXAMPLE_A, Mode::Always), 161);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_for(&input_for(2024, 3), Mode::Always), 183380722);
}

fn solve_a() -> PuzzleResult {
    solve_for(&input_for(2024, 3), Mode::Always).into()
}

#[test]
fn b_example() {
    assert_eq!(solve_for(EXAMPLE_B, Mode::Toggle), 48);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_for(&input_for(2024, 3), Mode::Toggle), 82733683);
}

fn solve_b() -> PuzzleResult {
    solve_for(&input_for(2024, 3), Mode::Toggle).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
