use std::collections::BTreeMap;

use num_integer::lcm;

use crate::PuzzleResult;
use crate::common::aoc::input_for;
use crate::day::Day;

#[cfg(test)]
const EXAMPLE_A1: &str = include_str!("example/8a_1");
#[cfg(test)]
const EXAMPLE_A2: &str = include_str!("example/8a_2");
#[cfg(test)]
const EXAMPLE_B: &str = include_str!("example/8b");

enum Direction {
    Left,
    Right,
}

fn parse(input: &str) -> (Vec<Direction>, BTreeMap<String, (String, String)>) {
    let mut lines = input.lines();
    let instructions: Vec<_> = lines
        .next()
        .unwrap()
        .chars()
        .map(|ch| match ch {
            'L' => Direction::Left,
            'R' => Direction::Right,
            ch => panic!("unexpected {ch}"),
        })
        .collect();
    lines.next();

    let directions: BTreeMap<String, (String, String)> = lines
        .map(|line| {
            let (from, line) = line.split_once(" = (").unwrap();
            let (left, right) = line.split_once(", ").unwrap();
            (from.to_owned(), (left.to_owned(), right[0..3].to_owned()))
        })
        .collect();

    (instructions, directions)
}

fn solve_a_for(input: &str) -> usize {
    let (instructions, directions) = parse(input);

    let mut location = "AAA";
    let mut steps = 0;
    'outer: loop {
        for instruction in &instructions {
            let direction = &directions[location];
            location = match instruction {
                Direction::Left => &direction.0,
                Direction::Right => &direction.1,
            };
            steps += 1;
            if location == "ZZZ" {
                break 'outer;
            }
        }
    }

    steps
}

fn solve_b_for(input: &str) -> usize {
    let (instructions, directions) = parse(input);

    let locations: Vec<_> = directions.keys().filter(|k| k.ends_with('A')).collect();

    locations
        .into_iter()
        .map(|mut location| {
            let mut steps = 0;
            'outer: loop {
                for instruction in &instructions {
                    let direction = &directions[location];
                    location = match instruction {
                        Direction::Left => &direction.0,
                        Direction::Right => &direction.1,
                    };
                    steps += 1;
                    if location.ends_with('Z') {
                        break 'outer;
                    }
                }
            }

            steps
        })
        .fold(1, lcm)
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for(EXAMPLE_A1), 2);
    assert_eq!(solve_a_for(EXAMPLE_A2), 6);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(&input_for(2023, 8)), 19783);
}

fn solve_a() -> PuzzleResult {
    solve_a_for(&input_for(2023, 8)).into()
}

#[test]
fn b_example() {
    assert_eq!(solve_b_for(EXAMPLE_B), 6);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(&input_for(2023, 8)), 9177460370549);
}

fn solve_b() -> PuzzleResult {
    solve_b_for(&input_for(2023, 8)).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
