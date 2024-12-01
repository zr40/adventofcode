use std::collections::{HashSet, VecDeque};

use crate::common::aoc::input_for;
use crate::day::Day;
use crate::puzzle_result::PuzzleResult;

#[cfg(test)]
const EXAMPLE: &str = include_str!("example/12");

enum Mode {
    SinglePath,
    HikingTrail,
}

type Position = (usize, usize);

fn parse_input(input: &str) -> (Vec<Vec<u8>>, Position, Position) {
    let mut heightmap = vec![];
    let mut start = (0, 0);
    let mut end = (0, 0);

    heightmap.push(vec![]);

    for line in input.lines() {
        let mut map_line = vec![255];

        for char in line.chars() {
            map_line.push(
                match char {
                    'S' => {
                        start = (heightmap.len(), map_line.len());
                        'a'
                    }
                    'E' => {
                        end = (heightmap.len(), map_line.len());
                        'z'
                    }
                    c => c,
                } as u8
                    - b'a',
            );
        }

        heightmap.push(map_line);
    }

    (heightmap, start, end)
}

fn solve_for(input: &str, mode: Mode) -> usize {
    let (heightmap, start, end) = parse_input(input);

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(match mode {
        Mode::SinglePath => (start, 0, 0),
        Mode::HikingTrail => (end, 0, 25),
    });

    let bounds = (heightmap.len(), heightmap[1].len());

    while let Some((position, distance, last_height)) = queue.pop_front() {
        if position.0 == 0 || position.0 == bounds.0 || position.1 == 0 || position.1 == bounds.1 {
            continue;
        }

        if visited.contains(&position) {
            continue;
        }

        let current_height = heightmap[position.0][position.1];

        match mode {
            Mode::SinglePath if current_height > last_height + 1 => {
                continue;
            }
            Mode::SinglePath if position == end => {
                return distance;
            }

            Mode::HikingTrail if current_height < last_height - 1 => {
                continue;
            }
            Mode::HikingTrail if current_height == 0 => {
                return distance;
            }
            _ => {}
        }

        visited.insert(position);

        queue.push_back(((position.0 - 1, position.1), distance + 1, current_height));
        queue.push_back(((position.0 + 1, position.1), distance + 1, current_height));
        queue.push_back(((position.0, position.1 - 1), distance + 1, current_height));
        queue.push_back(((position.0, position.1 + 1), distance + 1, current_height));
    }
    panic!("solution not found");
}

#[test]
fn a_example() {
    assert_eq!(solve_for(EXAMPLE, Mode::SinglePath), 31);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_for(&input_for(2022, 12), Mode::SinglePath), 437);
}

fn solve_a() -> PuzzleResult {
    solve_for(&input_for(2022, 12), Mode::SinglePath).into()
}

#[test]
fn b_example() {
    assert_eq!(solve_for(EXAMPLE, Mode::HikingTrail), 29);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_for(&input_for(2022, 12), Mode::HikingTrail), 430);
}

fn solve_b() -> PuzzleResult {
    solve_for(&input_for(2022, 12), Mode::HikingTrail).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
