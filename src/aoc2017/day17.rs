use crate::common::aoc::input_for;
use crate::day::Day;
use crate::puzzle_result::PuzzleResult;

#[cfg(test)]
const EXAMPLE: &str = "3";

fn solve_a_for(input: &str) -> usize {
    let steps: usize = input.parse().unwrap();

    let mut buffer = vec![0];
    let mut position = 0;

    for i in 1..2018 {
        position = (position + steps) % i + 1;

        buffer.insert(position, i);
    }

    buffer[(position + 1) % 2018]
}

fn solve_b_for(input: &str) -> usize {
    let steps: usize = input.parse().unwrap();

    let mut position = 0;
    let mut output = 0;

    for i in 1..50000001 {
        position = (position + steps) % i + 1;

        if position == 1 {
            output = i;
        }
    }

    output
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for(EXAMPLE), 638);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(&input_for(2017, 17)), 1306);
}

fn solve_a() -> PuzzleResult {
    solve_a_for(&input_for(2017, 17)).into()
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(&input_for(2017, 17)), 20430489);
}

fn solve_b() -> PuzzleResult {
    solve_b_for(&input_for(2017, 17)).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
