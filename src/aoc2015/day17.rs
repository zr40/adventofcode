use itertools::Itertools;

use crate::PuzzleResult;
use crate::day::Day;

#[cfg(test)]
const EXAMPLE: &str = include_str!("input/17_example");
const INPUT: &str = include_str!("input/17");

fn solve_a_for(input: &str, eggnog: u16) -> usize {
    let containers: Vec<u16> = input.lines().map(|l| l.parse().unwrap()).collect();

    containers
        .into_iter()
        .powerset()
        .filter(|c| c.iter().sum::<u16>() == eggnog)
        .count()
}
fn solve_b_for(input: &str, eggnog: u16) -> usize {
    let containers: Vec<u16> = input.lines().map(|l| l.parse().unwrap()).collect();

    containers
        .into_iter()
        .powerset()
        .filter_map(|c| (c.iter().sum::<u16>() == eggnog).then_some(c.len()))
        .min_set()
        .len()
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for(EXAMPLE, 25), 4);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(INPUT, 150), 1304);
}

#[test]
fn b_example() {
    assert_eq!(solve_b_for(EXAMPLE, 25), 3);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(INPUT, 150), 18);
}

fn solve_a() -> PuzzleResult {
    solve_a_for(INPUT, 150).into()
}

fn solve_b() -> PuzzleResult {
    solve_b_for(INPUT, 150).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
