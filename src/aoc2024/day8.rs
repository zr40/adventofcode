use std::collections::HashSet;

use itertools::Itertools;

use crate::PuzzleResult;
use crate::common::aoc::input_for;
use crate::day::Day;

#[cfg(test)]
const EXAMPLE: &str = include_str!("example/8");

enum Mode {
    Single,
    Resonant,
}

fn solve_for(input: &str, mode: Mode) -> usize {
    let map = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut antinodes = HashSet::new();

    for ay in 0..map.len() {
        for ax in 0..map[0].len() {
            let a = map[ay][ax];
            if a == '.' {
                continue;
            }

            for by in 0..map.len() {
                for bx in 0..map.len() {
                    if ax == bx && ay == by {
                        continue;
                    }

                    let b = map[by][bx];

                    let ax = ax as isize;
                    let ay = ay as isize;
                    let bx = bx as isize;
                    let by = by as isize;

                    if a == b {
                        let dx = ax - bx;
                        let dy = ay - by;

                        let mut multiple = match mode {
                            Mode::Single => 1,
                            Mode::Resonant => 0,
                        };

                        loop {
                            let lx = ax + (dx * multiple);
                            let ly = ay + (dy * multiple);

                            if lx < 0
                                || lx >= map[0].len() as isize
                                || ly < 0
                                || ly >= map.len() as isize
                            {
                                break;
                            }

                            antinodes.insert((lx, ly));
                            if let Mode::Single = mode {
                                break;
                            }
                            multiple += 1;
                        }
                    }
                }
            }
        }
    }

    antinodes.len()
}

#[test]
fn a_example() {
    assert_eq!(solve_for(EXAMPLE, Mode::Single), 14);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_for(&input_for(2024, 8), Mode::Single), 354);
}

fn solve_a() -> PuzzleResult {
    solve_for(&input_for(2024, 8), Mode::Single).into()
}

#[test]
fn b_example() {
    assert_eq!(solve_for(EXAMPLE, Mode::Resonant), 34);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_for(&input_for(2024, 8), Mode::Resonant), 1263);
}

fn solve_b() -> PuzzleResult {
    solve_for(&input_for(2024, 8), Mode::Resonant).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
