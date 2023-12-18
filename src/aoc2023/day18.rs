use std::collections::BTreeSet;

use crate::common::direction::Direction;
use crate::PuzzleResult;

#[cfg(test)]
const EXAMPLE: &str = include_str!("input/18_example");
const INPUT: &str = include_str!("input/18");

enum Mode {
    PartA,
    PartB,
}

struct Instruction {
    direction: Direction,
    length: usize,
}

fn parse(input: &str, mode: Mode) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let (direction, rest) = line.split_once(' ').unwrap();
            let direction = match direction {
                "U" => Direction::North,
                "R" => Direction::East,
                "D" => Direction::South,
                "L" => Direction::West,
                ch => panic!("unknown direction '{ch}'"),
            };

            let (length, rest) = rest.split_once(' ').unwrap();
            let length = length.parse().unwrap();

            let (_, rest) = rest.split_once('#').unwrap();
            let color = &rest[0..6];

            let swapped_direction = match &color[5..6] {
                "0" => Direction::East,
                "1" => Direction::South,
                "2" => Direction::West,
                "3" => Direction::North,
                ch => panic!("unknown direction '{ch}'"),
            };
            let swapped_length = usize::from_str_radix(&color[0..5], 16).unwrap();

            match mode {
                Mode::PartA => Instruction { direction, length },
                Mode::PartB => Instruction {
                    direction: swapped_direction,
                    length: swapped_length,
                },
            }
        })
        .collect()
}

fn solve_for(input: &str, mode: Mode) -> usize {
    let instructions = parse(input, mode);
    let mut map = BTreeSet::new();
    map.insert((0, 0));

    let mut x = 0;
    let mut y = 0;

    for instruction in instructions {
        for _ in 0..instruction.length {
            (x, y) = instruction.direction.step_unbounded(x, y);
            map.insert((x, y));
        }
    }

    let mut queue = vec![(1, 1)];

    while let Some((x, y)) = queue.pop() {
        if map.contains(&(x, y)) {
            continue;
        }
        map.insert((x, y));

        queue.push((x + 1, y));
        queue.push((x - 1, y));
        queue.push((x, y + 1));
        queue.push((x, y - 1));
    }

    map.len()
}

#[test]
fn a_example() {
    assert_eq!(solve_for(EXAMPLE, Mode::PartA), 62);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_for(INPUT, Mode::PartA), 35244);
}

#[test]
#[ignore = "part two not implemented"]
fn b_example() {
    assert_eq!(solve_for(EXAMPLE, Mode::PartB), 952408144115);
}

#[test]
#[ignore = "part two not implemented"]
fn b_puzzle() {
    assert_eq!(solve_for(EXAMPLE, Mode::PartB), todo!());
}

pub fn solve_a() -> PuzzleResult {
    solve_for(INPUT, Mode::PartA).into()
}

pub fn solve_b() -> PuzzleResult {
    PuzzleResult::Todo
}
