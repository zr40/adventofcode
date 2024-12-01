use itertools::Itertools;

use crate::common::aoc::input_for;
use crate::day::Day;
use crate::puzzle_result::PuzzleResult;

#[cfg(test)]
const EXAMPLE_A: &str = include_str!("example/2a");
#[cfg(test)]
const EXAMPLE_B: &str = include_str!("example/2b");

fn solve_a_for(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let cells: Vec<u32> = line
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            let (min, max) = cells.iter().minmax().into_option().unwrap();

            max - min
        })
        .sum()
}

fn solve_b_for(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let cells: Vec<u32> = line
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();

            for (first, second) in cells.iter().tuple_combinations() {
                let largest = first.max(second);
                let smallest = first.min(second);

                if largest % smallest == 0 {
                    return largest / smallest;
                }
            }
            panic!("no solution found")
        })
        .sum()
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for(EXAMPLE_A), 18);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(&input_for(2017, 2)), 37923);
}

fn solve_a() -> PuzzleResult {
    solve_a_for(&input_for(2017, 2)).into()
}

#[test]
fn b_example() {
    assert_eq!(solve_b_for(EXAMPLE_B), 9);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(&input_for(2017, 2)), 263);
}

fn solve_b() -> PuzzleResult {
    solve_b_for(&input_for(2017, 2)).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
