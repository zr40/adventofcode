use itertools::Itertools;

use crate::PuzzleResult;
use crate::common::aoc::input_for;
use crate::day::Day;

#[cfg(test)]
const EXAMPLE: &str = include_str!("example/2");

fn is_safe<T: Iterator<Item = i8>>(levels: T) -> bool {
    levels.tuple_windows().all(|(a, b, c)| {
        ((a < b && b < c) || (a > b && b > c))
            && matches!(a - b, -1 | -2 | -3 | 1 | 2 | 3)
            && matches!(b - c, -1 | -2 | -3 | 1 | 2 | 3)
    })
}

fn solve_a_for(input: &str) -> usize {
    input
        .lines()
        .filter(|line| is_safe(line.split(' ').map(|num| num.parse().unwrap())))
        .count()
}

fn solve_b_for(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let levels: Vec<i8> = line.split(' ').map(|num| num.parse().unwrap()).collect();

            is_safe(levels.iter().copied())
                || (0..(levels.len())).any(|i| {
                    is_safe(
                        levels
                            .iter()
                            .enumerate()
                            .filter_map(|(idx, num)| (idx != i).then_some(*num)),
                    )
                })
        })
        .count()
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for(EXAMPLE), 2);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(&input_for(2024, 2)), 402);
}

fn solve_a() -> PuzzleResult {
    solve_a_for(&input_for(2024, 2)).into()
}

#[test]
fn b_example() {
    assert_eq!(solve_b_for(EXAMPLE), 4);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(&input_for(2024, 2)), 455);
}

fn solve_b() -> PuzzleResult {
    solve_b_for(&input_for(2024, 2)).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
