use itertools::Itertools;

use crate::day::Day;
use crate::puzzle_result::PuzzleResult;

#[cfg(test)]
const EXAMPLE_B: &str = include_str!("input/3b_example");
const INPUT: &str = include_str!("input/3");

fn solve_a_for(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let mut max = 0;
            let sum: u32 = line
                .split_whitespace()
                .map(|num| {
                    let num: u32 = num.parse().unwrap();
                    max = max.max(num);
                    num
                })
                .sum();

            sum > max * 2
        })
        .count()
}

fn solve_b_for(input: &str) -> usize {
    input
        .split_whitespace()
        .map(|num| num.parse::<u32>().unwrap())
        .chunks(9)
        .into_iter()
        .map(|mut chunk| {
            let a1 = chunk.next().unwrap();
            let b1 = chunk.next().unwrap();
            let c1 = chunk.next().unwrap();
            let a2 = chunk.next().unwrap();
            let b2 = chunk.next().unwrap();
            let c2 = chunk.next().unwrap();
            let a3 = chunk.next().unwrap();
            let b3 = chunk.next().unwrap();
            let c3 = chunk.next().unwrap();

            [(a1, a2, a3), (b1, b2, b3), (c1, c2, c3)]
                .iter()
                .filter(|(a, b, c)| {
                    let max = a.max(b).max(c);
                    a + b + c > max * 2
                })
                .count()
        })
        .sum()
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for("5 10 25"), 0);
    assert_eq!(solve_a_for("5 10 15"), 0);
    assert_eq!(solve_a_for("5 10 14"), 1);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(INPUT), 1032);
}

#[test]
fn b_example() {
    assert_eq!(solve_b_for(EXAMPLE_B), 6);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(INPUT), 1838);
}

fn solve_a() -> PuzzleResult {
    solve_a_for(INPUT).into()
}

fn solve_b() -> PuzzleResult {
    solve_b_for(INPUT).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
