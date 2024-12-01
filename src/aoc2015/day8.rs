use crate::PuzzleResult;
use crate::common::aoc::input_for;
use crate::day::Day;

#[cfg(test)]
const EXAMPLE: &str = include_str!("example/8");

fn solve_a_for(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut waste = 2;
            let mut iter = line.chars();
            while let Some(ch) = iter.next() {
                if ch == '\\' {
                    match iter.next() {
                        Some('x') => {
                            waste += 3;
                            iter.next();
                            iter.next();
                        }
                        Some('\\' | '"') => waste += 1,
                        other => panic!("unknown {other:?}"),
                    }
                }
            }
            waste
        })
        .sum()
}

fn solve_b_for(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.chars().filter(|ch| *ch == '"' || *ch == '\\').count() + 2)
        .sum()
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for(EXAMPLE), 12);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(&input_for(2015, 8)), 1350);
}

fn solve_a() -> PuzzleResult {
    solve_a_for(&input_for(2015, 8)).into()
}

#[test]
fn b_example() {
    assert_eq!(solve_b_for(EXAMPLE), 19);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(&input_for(2015, 8)), 2085);
}

fn solve_b() -> PuzzleResult {
    solve_b_for(&input_for(2015, 8)).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
