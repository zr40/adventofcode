use itertools::Itertools;

use crate::PuzzleResult;
use crate::common::aoc::input_for;
use crate::day::Day;

#[cfg(test)]
const EXAMPLE: &str = include_str!("example/7");

#[derive(Clone, Copy)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

enum Mode {
    AddMul,
    Full,
}

fn solve_for(input: &str, mode: Mode) -> usize {
    let ops = match mode {
        Mode::AddMul => vec![Operator::Add, Operator::Multiply],
        Mode::Full => {
            vec![Operator::Add, Operator::Multiply, Operator::Concatenate]
        }
    };

    input
        .lines()
        .filter_map(|line| {
            let (test_value, rest) = line.split_once(": ").unwrap();
            let test_value: usize = test_value.parse().unwrap();
            let numbers = rest
                .split(' ')
                .map(|n| n.parse::<usize>().unwrap())
                .collect_vec();

            for candidate in (0..numbers.len() - 1)
                .map(|_| &ops)
                .multi_cartesian_product()
            {
                let mut value = numbers[0];
                for (op, num) in candidate.into_iter().zip(numbers.iter().skip(1)) {
                    match op {
                        Operator::Add => value += num,
                        Operator::Multiply => value *= num,
                        Operator::Concatenate => {
                            value = value * 10usize.pow(num.ilog10() + 1) + num;
                        }
                    }
                }

                if value == test_value {
                    return Some(test_value);
                }
            }

            None
        })
        .sum()
}

#[test]
fn a_example() {
    assert_eq!(solve_for(EXAMPLE, Mode::AddMul), 3749);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_for(&input_for(2024, 7), Mode::AddMul), 267566105056);
}

fn solve_a() -> PuzzleResult {
    solve_for(&input_for(2024, 7), Mode::AddMul).into()
}

#[test]
fn b_example() {
    assert_eq!(solve_for(EXAMPLE, Mode::Full), 11387);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_for(&input_for(2024, 7), Mode::Full), 116094961956019);
}

fn solve_b() -> PuzzleResult {
    solve_for(&input_for(2024, 7), Mode::Full).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
