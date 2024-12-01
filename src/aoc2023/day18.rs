use std::collections::BTreeSet;

use crate::PuzzleResult;
use crate::common::aoc::input_for;
use crate::common::direction::Direction;
use crate::day::Day;

#[cfg(test)]
const EXAMPLE: &str = include_str!("example/18");

enum Mode {
    PartA,
    PartB,
}

struct Instruction {
    direction: Direction,
    length: isize,
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
            let swapped_length = isize::from_str_radix(&color[0..5], 16).unwrap();

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

fn solve_for(input: &str, mode: Mode) -> isize {
    let instructions = parse(input, mode);
    let mut map = BTreeSet::new();
    map.insert((0, 0));

    let mut x = 0;
    let mut y = 0;

    let mut vertices = vec![];
    for instruction in &instructions {
        (x, y) = instruction.direction.move_for(x, y, instruction.length);
        vertices.push((x, y));
    }

    vertices
        .into_iter()
        .map_windows(|[i, j]| i.0 * j.1 - i.1 * j.0)
        .sum::<isize>()
        / 2
        + instructions.into_iter().map(|i| i.length).sum::<isize>() / 2
        + 1
}

#[test]
fn a_example() {
    assert_eq!(solve_for(EXAMPLE, Mode::PartA), 62);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_for(&input_for(2023, 18), Mode::PartA), 35244);
}

fn solve_a() -> PuzzleResult {
    solve_for(&input_for(2023, 18), Mode::PartA).into()
}

#[test]
fn b_example() {
    assert_eq!(solve_for(EXAMPLE, Mode::PartB), 952408144115);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_for(&input_for(2023, 18), Mode::PartB), 85070763635666);
}

fn solve_b() -> PuzzleResult {
    solve_for(&input_for(2023, 18), Mode::PartB).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
