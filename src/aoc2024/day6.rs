use std::collections::HashSet;

use itertools::Itertools;

use crate::PuzzleResult;
use crate::common::aoc::input_for;
use crate::common::coordinate::Coordinate;
use crate::common::direction::Direction;
use crate::day::Day;

#[cfg(test)]
const EXAMPLE: &str = include_str!("example/6");

fn solve_a_for(input: &str) -> usize {
    let mut guard = Coordinate { x: 0, y: 0 };
    let mut visited = HashSet::new();
    let map = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, ch)| {
                    if ch == '^' {
                        guard = Coordinate { x, y };
                    }
                    ch == '#'
                })
                .collect_vec()
        })
        .collect_vec();

    let mut direction = Direction::Up;

    let bounds = Coordinate {
        x: map[0].len(),
        y: map.len(),
    };

    loop {
        visited.insert(guard);
        if let Some(next) = direction.step(guard, bounds) {
            if map[next.y][next.x] {
                direction = direction.right();
            } else {
                guard = next;
            }
        } else {
            break;
        }
    }
    visited.len()
}

fn solve_b_for(input: &str) -> usize {
    let mut init_guard = Coordinate { x: 0, y: 0 };
    let mut map = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, ch)| {
                    if ch == '^' {
                        init_guard = Coordinate { x, y };
                    }
                    ch == '#'
                })
                .collect_vec()
        })
        .collect_vec();

    let bounds = Coordinate {
        x: map[0].len(),
        y: map.len(),
    };

    let mut obstacle_candidates = HashSet::new();
    let mut guard = init_guard;
    let mut direction = Direction::Up;
    loop {
        obstacle_candidates.insert(guard);
        if let Some(next) = direction.step(guard, bounds) {
            if map[next.y][next.x] {
                direction = direction.right();
            } else {
                guard = next;
            }
        } else {
            break;
        }
    }

    obstacle_candidates
        .into_iter()
        .filter(|Coordinate { x, y }| {
            if map[*y][*x] {
                return false;
            }

            map[*y][*x] = true;

            let mut visited = HashSet::new();
            let mut guard = init_guard;
            let mut direction = Direction::Up;

            let result = loop {
                if !(visited.insert((guard, direction))) {
                    break true;
                }
                if let Some(next) = direction.step(guard, bounds) {
                    if map[next.y][next.x] {
                        direction = direction.right();
                    } else {
                        guard = next;
                    }
                } else {
                    break false;
                }
            };

            map[*y][*x] = false;
            result
        })
        .count()
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for(EXAMPLE), 41);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(&input_for(2024, 6)), 4696);
}

fn solve_a() -> PuzzleResult {
    solve_a_for(&input_for(2024, 6)).into()
}

#[test]
fn b_example() {
    assert_eq!(solve_b_for(EXAMPLE), 6);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(&input_for(2024, 6)), 1443);
}

fn solve_b() -> PuzzleResult {
    solve_b_for(&input_for(2024, 6)).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
