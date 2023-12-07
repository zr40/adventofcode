use std::collections::HashSet;

use crate::PuzzleResult;

const INPUT: &str = include_str!("input/3");

enum Mode {
    PartA,
    PartB,
}

fn solve_for(input: &str, mode: Mode) -> usize {
    let mut santa_position = (0, 0);
    let mut robosanta_position = (0, 0);

    let mut robosanta = false;
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0, 0));

    let mut position = &mut santa_position;

    for ch in input.chars() {
        let direction = match ch {
            '^' => (0, -1),
            '>' => (1, 0),
            'v' => (0, 1),
            '<' => (-1, 0),
            ch => panic!("unknown direction '{ch}'"),
        };

        (position.0, position.1) = (position.0 + direction.0, position.1 + direction.1);
        visited.insert(*position);

        if let Mode::PartB = mode {
            robosanta = !robosanta;
            position = match robosanta {
                false => &mut santa_position,
                true => &mut robosanta_position,
            };
        }
    }
    visited.len()
}

#[test]
fn a_example() {
    assert_eq!(solve_for(">", Mode::PartA), 2);
    assert_eq!(solve_for("^>v<", Mode::PartA), 4);
    assert_eq!(solve_for("^v^v^v^v^v", Mode::PartA), 2);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_for(INPUT, Mode::PartA), 2081);
}

#[test]
fn b_example() {
    assert_eq!(solve_for("^v", Mode::PartB), 3);
    assert_eq!(solve_for("^>v<", Mode::PartB), 3);
    assert_eq!(solve_for("^v^v^v^v^v", Mode::PartB), 11);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_for(INPUT, Mode::PartB), 2341);
}

pub fn solve_a() -> PuzzleResult {
    solve_for(INPUT, Mode::PartA).into()
}

pub fn solve_b() -> PuzzleResult {
    solve_for(INPUT, Mode::PartB).into()
}