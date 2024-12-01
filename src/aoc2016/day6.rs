use std::collections::BTreeMap;

use itertools::Itertools;

use crate::common::aoc::input_for;
use crate::day::Day;
use crate::puzzle_result::PuzzleResult;

#[cfg(test)]
const EXAMPLE: &str = include_str!("example/6");

enum Mode {
    MostCommon,
    LeastCommon,
}

fn solve_for(input: &str, mode: Mode) -> String {
    let mut positions: Vec<BTreeMap<char, usize>> = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|_| BTreeMap::new())
        .collect();

    for line in input.lines() {
        for (chars, ch) in positions.iter_mut().zip(line.chars()) {
            chars.entry(ch).and_modify(|count| *count += 1).or_insert(1);
        }
    }

    positions
        .into_iter()
        .map(|chars| {
            let mut sorted = chars.into_iter().sorted_by_key(|(_, count)| *count);
            match mode {
                Mode::MostCommon => sorted.next_back(),
                Mode::LeastCommon => sorted.next(),
            }
            .unwrap()
            .0
        })
        .collect()
}

#[test]
fn a_example() {
    assert_eq!(solve_for(EXAMPLE, Mode::MostCommon), "easter");
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_for(&input_for(2016, 6), Mode::MostCommon), "mlncjgdg");
}

fn solve_a() -> PuzzleResult {
    solve_for(&input_for(2016, 6), Mode::MostCommon).into()
}

#[test]
fn b_example() {
    assert_eq!(solve_for(EXAMPLE, Mode::LeastCommon), "advent");
}

#[test]
fn b_puzzle() {
    assert_eq!(
        solve_for(&input_for(2016, 6), Mode::LeastCommon),
        "bipjaytb"
    );
}

fn solve_b() -> PuzzleResult {
    solve_for(&input_for(2016, 6), Mode::LeastCommon).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
