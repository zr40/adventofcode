use crate::day::Day;
use crate::puzzle_result::PuzzleResult;

#[cfg(test)]
const EXAMPLE: &str = include_str!("input/15_example");
const INPUT: &str = include_str!("input/15");

fn parse(input: &str) -> (u64, u64) {
    let mut iter = input.split('\n');
    let prev_a: u64 = iter
        .next()
        .unwrap()
        .split_once(" starts with ")
        .unwrap()
        .1
        .parse()
        .unwrap();
    let prev_b: u64 = iter
        .next()
        .unwrap()
        .split_once(" starts with ")
        .unwrap()
        .1
        .parse()
        .unwrap();

    (prev_a, prev_b)
}

fn solve_a_for(input: &str) -> u32 {
    let (mut prev_a, mut prev_b) = parse(input);
    let mut count = 0;

    for _ in 0..40_000_000 {
        let a = (prev_a * 16807) % 2147483647;
        let b = (prev_b * 48271) % 2147483647;

        if a & 0b1111_1111_1111_1111 == b & 0b1111_1111_1111_1111 {
            count += 1;
        }

        prev_a = a;
        prev_b = b;
    }

    count
}

fn solve_b_for(input: &str) -> u32 {
    let (mut prev_a, mut prev_b) = parse(input);
    let mut count = 0;

    for _ in 0..5_000_000 {
        let mut a = (prev_a * 16807) % 2147483647;

        while a % 4 != 0 {
            a = (a * 16807) % 2147483647;
        }

        let mut b = (prev_b * 48271) % 2147483647;

        while b % 8 != 0 {
            b = (b * 48271) % 2147483647;
        }

        if a & 0b1111_1111_1111_1111 == b & 0b1111_1111_1111_1111 {
            count += 1;
        }

        prev_a = a;
        prev_b = b;
    }

    count
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for(EXAMPLE), 588);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(INPUT), 569);
}

#[test]
fn b_example() {
    assert_eq!(solve_b_for(EXAMPLE), 309);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(INPUT), 298);
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
