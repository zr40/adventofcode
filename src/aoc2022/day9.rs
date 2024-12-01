use std::collections::HashSet;

use crate::common::aoc::input_for;
use crate::day::Day;
use crate::puzzle_result::PuzzleResult;

#[cfg(test)]
const EXAMPLE: &str = include_str!("example/9");
#[cfg(test)]
const EXAMPLE_LARGE: &str = include_str!("example/9_large");

fn solve_for(input: &str, knot_count: usize) -> usize {
    let mut visited = HashSet::new();

    let mut knots: Vec<(i32, i32)> = vec![(0, 0); knot_count];

    for line in input.lines() {
        let (direction, distance) = line.split_once(' ').unwrap();

        let distance = distance.parse().unwrap();

        let step = match direction {
            "U" => (1, 0),
            "D" => (-1, 0),
            "L" => (0, -1),
            "R" => (0, 1),
            unknown => panic!("unknown direction {unknown}"),
        };

        for _ in 0..distance {
            knots[0] = (knots[0].0 + step.0, knots[0].1 + step.1);
            for i in 1..knot_count {
                let head = knots[i - 1];
                let tail = knots[i];
                knots[i] = match (head.0 - tail.0, head.1 - tail.1) {
                    (-1..=1, -1..=1) => tail,
                    (x, y) => (tail.0 + x.signum(), tail.1 + y.signum()),
                };
            }
            visited.insert(knots[knot_count - 1]);
        }
    }
    visited.len()
}

#[test]
fn a_example() {
    assert_eq!(solve_for(EXAMPLE, 2), 13);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_for(&input_for(2022, 9), 2), 6494);
}

#[test]
fn b_example() {
    assert_eq!(solve_for(EXAMPLE, 10), 1);
}

#[test]
fn b_example_large() {
    assert_eq!(solve_for(EXAMPLE_LARGE, 10), 36);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_for(&input_for(2022, 9), 10), 2691);
}

fn solve_a() -> PuzzleResult {
    solve_for(&input_for(2022, 9), 2).into()
}

fn solve_b() -> PuzzleResult {
    solve_for(&input_for(2022, 9), 10).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
