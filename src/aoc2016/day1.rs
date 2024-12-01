use std::collections::HashSet;

use crate::common::aoc::input_for;
use crate::common::direction::Direction;
use crate::day::Day;
use crate::puzzle_result::PuzzleResult;

fn solve_a_for(input: &str) -> isize {
    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut orientation = Direction::North;

    for instruction in input.split(", ") {
        let (turn, distance) = instruction.split_at(1);
        let distance: isize = distance.parse().unwrap();

        orientation = match turn {
            "L" => orientation.left(),
            "R" => orientation.right(),
            x => panic!("Unknown direction {x}"),
        };

        (x, y) = orientation.move_for(x, y, distance);
    }

    x.abs() + y.abs()
}

fn solve_b_for(input: &str) -> isize {
    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut orientation = Direction::North;
    let mut visited = HashSet::new();

    visited.insert((0, 0));

    'outer: for instruction in input.split(", ") {
        let (turn, distance) = instruction.split_at(1);
        let distance: i32 = distance.parse().unwrap();

        orientation = match turn {
            "L" => orientation.left(),
            "R" => orientation.right(),
            x => panic!("Unknown direction {x}"),
        };

        for _ in 0..distance {
            (x, y) = orientation.move_for(x, y, 1);

            if visited.contains(&(x, y)) {
                break 'outer;
            }

            visited.insert((x, y));
        }
    }

    x.abs() + y.abs()
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for("R2, L3"), 5);
    assert_eq!(solve_a_for("R2, R2, R2"), 2);
    assert_eq!(solve_a_for("R5, L5, R5, R3"), 12);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(&input_for(2016, 1)), 301);
}

fn solve_a() -> PuzzleResult {
    solve_a_for(&input_for(2016, 1)).into()
}

#[test]
fn b_example() {
    assert_eq!(solve_b_for("R8, R4, R4, R8"), 4);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(&input_for(2016, 1)), 130);
}

fn solve_b() -> PuzzleResult {
    solve_b_for(&input_for(2016, 1)).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
