use std::collections::HashMap;

use crate::common::aoc::input_for;
use crate::day::Day;
use crate::puzzle_result::PuzzleResult;

enum Mode {
    CyclesToDuplicate,
    InfiniteLoopCycles,
}

fn solve_for(input: &str, mode: Mode) -> usize {
    let mut banks: Vec<usize> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let mut cycles = 0;

    let mut configurations_seen: HashMap<Vec<usize>, usize> = HashMap::new();

    while !configurations_seen.contains_key(&banks) {
        configurations_seen.insert(banks.clone(), cycles);
        cycles += 1;

        let mut blocks = *banks.iter().max().unwrap();
        let redistribution_position = banks.iter().position(|x| *x == blocks).unwrap();

        banks[redistribution_position] = 0;

        let mut remaining = banks.len();
        let mut index = redistribution_position;
        while remaining != 0 {
            let redistributed = blocks / remaining;
            blocks -= redistributed;
            banks[index] += redistributed;

            if index == 0 {
                index = banks.len() - 1;
            } else {
                index -= 1;
            }
            remaining -= 1;
        }
    }

    match mode {
        Mode::CyclesToDuplicate => cycles,
        Mode::InfiniteLoopCycles => cycles - configurations_seen[&banks],
    }
}

#[test]
fn a_example() {
    assert_eq!(solve_for("0 2 7 0", Mode::CyclesToDuplicate), 5);
}

#[test]
fn a_puzzle() {
    assert_eq!(
        solve_for(&input_for(2017, 6), Mode::CyclesToDuplicate),
        6681
    );
}

fn solve_a() -> PuzzleResult {
    solve_for(&input_for(2017, 6), Mode::CyclesToDuplicate).into()
}

#[test]
fn b_example() {
    assert_eq!(solve_for("0 2 7 0", Mode::InfiniteLoopCycles), 4);
}

#[test]
fn b_puzzle() {
    assert_eq!(
        solve_for(&input_for(2017, 6), Mode::InfiniteLoopCycles),
        2392
    );
}

fn solve_b() -> PuzzleResult {
    solve_for(&input_for(2017, 6), Mode::InfiniteLoopCycles).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
