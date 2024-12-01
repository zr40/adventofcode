use crate::PuzzleResult;
use crate::common::aoc::input_for;
use crate::day::Day;

fn solve_a_for(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut dims: Vec<u32> = line.split('x').map(|n| n.parse().unwrap()).collect();
            dims.sort_unstable();

            (dims[0] * dims[1] + dims[1] * dims[2] + dims[2] * dims[0]) * 2 + dims[0] * dims[1]
        })
        .sum()
}

fn solve_b_for(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut dims: Vec<u32> = line.split('x').map(|n| n.parse().unwrap()).collect();
            dims.sort_unstable();

            (dims[0] + dims[1]) * 2 + dims.iter().product::<u32>()
        })
        .sum()
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for("2x3x4"), 58);
    assert_eq!(solve_a_for("1x1x10"), 43);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(&input_for(2015, 2)), 1606483);
}

fn solve_a() -> PuzzleResult {
    solve_a_for(&input_for(2015, 2)).into()
}

#[test]
fn b_example() {
    assert_eq!(solve_b_for("2x3x4"), 34);
    assert_eq!(solve_b_for("1x1x10"), 14);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(&input_for(2015, 2)), 3842356);
}

fn solve_b() -> PuzzleResult {
    solve_b_for(&input_for(2015, 2)).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
