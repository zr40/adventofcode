use crate::common::aoc::input_for;
use crate::day::Day;
use crate::puzzle_result::PuzzleResult;

#[cfg(test)]
const EXAMPLE: &str = include_str!("example/5");

enum Mode {
    Normal,
    Strange,
}

fn solve_for(input: &str, mode: Mode) -> u32 {
    let mut pos = 0;
    let mut jumps = 0;

    let mut jump_offsets: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    #[allow(clippy::cast_possible_wrap)]
    while pos >= 0 && pos < jump_offsets.len() as i32 {
        let offset = jump_offsets[pos as usize];
        match (&mode, offset >= 3) {
            (Mode::Strange, true) => jump_offsets[pos as usize] -= 1,
            _ => jump_offsets[pos as usize] += 1,
        }
        pos += offset;
        jumps += 1;
    }

    jumps
}

#[test]
fn a_example() {
    assert_eq!(solve_for(EXAMPLE, Mode::Normal), 5);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_for(&input_for(2017, 5), Mode::Normal), 372671);
}

fn solve_a() -> PuzzleResult {
    solve_for(&input_for(2017, 5), Mode::Normal).into()
}

#[test]
fn b_example() {
    assert_eq!(solve_for(EXAMPLE, Mode::Strange), 10);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_for(&input_for(2017, 5), Mode::Strange), 25608480);
}

fn solve_b() -> PuzzleResult {
    solve_for(&input_for(2017, 5), Mode::Strange).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
