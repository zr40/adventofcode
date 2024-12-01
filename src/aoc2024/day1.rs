use std::collections::BTreeMap;

use crate::PuzzleResult;
use crate::common::aoc::input_for;
use crate::day::Day;

#[cfg(test)]
const EXAMPLE: &str = include_str!("example/1");

fn solve_a_for(input: &str) -> i32 {
    let mut numbers_left: Vec<i32> = vec![];
    let mut numbers_right: Vec<i32> = vec![];

    for line in input.lines() {
        let (left, right) = line.split_once("   ").unwrap();
        numbers_left.push(left.parse().unwrap());
        numbers_right.push(right.parse().unwrap());
    }

    numbers_left.sort_unstable();
    numbers_right.sort_unstable();

    numbers_left
        .into_iter()
        .zip(numbers_right)
        .map(|(l, r)| (l - r).abs())
        .sum()
}

fn solve_b_for(input: &str) -> u32 {
    let mut numbers_left: Vec<u32> = vec![];
    let mut appearances_right: BTreeMap<u32, u32> = BTreeMap::new();

    for line in input.lines() {
        let (left, right) = line.split_once("   ").unwrap();

        numbers_left.push(left.parse().unwrap());
        let right: u32 = right.parse().unwrap();

        match appearances_right.get(&right) {
            Some(count) => appearances_right.insert(right, count + 1),
            None => appearances_right.insert(right, 1),
        };
    }

    numbers_left
        .into_iter()
        .map(|number| number * appearances_right.get(&number).unwrap_or(&0))
        .sum()
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for(EXAMPLE), 11);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(&input_for(2024, 1)), 2057374);
}

fn solve_a() -> PuzzleResult {
    solve_a_for(&input_for(2024, 1)).into()
}

#[test]
fn b_example() {
    assert_eq!(solve_b_for(EXAMPLE), 31);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(&input_for(2024, 1)), 23177084);
}

fn solve_b() -> PuzzleResult {
    solve_b_for(&input_for(2024, 1)).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
