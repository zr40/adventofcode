use std::collections::HashMap;

use crate::day::Day;
use crate::puzzle_result::PuzzleResult;

#[cfg(test)]
const EXAMPLE: &str = include_str!("input/22_example");
const INPUT: &str = include_str!("input/22");

enum Mode {
    PartA,
    PartB,
}

enum Tile {
    Open,
    Wall,
}

enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn facing(self) -> usize {
        match self {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }
    }

    fn step(
        &self,
        map: &HashMap<(usize, usize), Tile>,
        (row, col): (usize, usize),
        rows: usize,
        cols: usize,
    ) -> (usize, usize) {
        let candidate = match self {
            Direction::Right => (row, col + 1),
            Direction::Down => (row + 1, col),
            Direction::Left => (row, col - 1),
            Direction::Up => (row - 1, col),
        };

        if map.contains_key(&candidate) {
            candidate
        } else {
            let mut candidate = match self {
                Direction::Right => (candidate.0, 0),
                Direction::Down => (0, candidate.1),
                Direction::Left => (candidate.0, cols),
                Direction::Up => (rows, candidate.1),
            };

            loop {
                if map.contains_key(&candidate) {
                    return candidate;
                }

                candidate = match self {
                    Direction::Right => (candidate.0, candidate.1 + 1),
                    Direction::Down => (candidate.0 + 1, candidate.1),
                    Direction::Left => (candidate.0, candidate.1 - 1),
                    Direction::Up => (candidate.0 - 1, candidate.1),
                }
            }
        }
    }

    fn left(self) -> Direction {
        match self {
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Up => Direction::Left,
        }
    }

    fn right(self) -> Direction {
        match self {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
        }
    }
}

enum Instruction {
    Move(usize),
    Left,
    Right,
}

fn solve_for(input: &str, mode: Mode) -> usize {
    let mut map = HashMap::new();
    let mut lines = input.lines();

    for (row, line) in lines.by_ref().enumerate() {
        if line.is_empty() {
            break;
        }

        for (col, char) in line.chars().enumerate() {
            match char {
                '.' => {
                    map.insert((row + 1, col + 1), Tile::Open);
                }
                '#' => {
                    map.insert((row + 1, col + 1), Tile::Wall);
                }
                ' ' => {}
                _ => panic!(),
            }
        }
    }

    let mut instructions = vec![];

    let mut number = String::new();

    for char in lines.next().unwrap().chars() {
        match char {
            'L' => {
                instructions.push(Instruction::Move(number.parse().unwrap()));
                number.clear();
                instructions.push(Instruction::Left);
            }
            'R' => {
                instructions.push(Instruction::Move(number.parse().unwrap()));
                number.clear();
                instructions.push(Instruction::Right);
            }
            d => number.push(d),
        }
    }

    instructions.push(Instruction::Move(number.parse().unwrap()));

    // TODO directions

    let rows = map.keys().map(|(row, _)| *row).max().unwrap();
    let cols = map.keys().map(|(_, col)| *col).max().unwrap();

    let mut pos = *map.keys().min().unwrap();
    let mut direction = Direction::Right;

    for instruction in instructions {
        (pos, direction) = match instruction {
            Instruction::Left => (pos, direction.left()),
            Instruction::Right => (pos, direction.right()),
            Instruction::Move(steps) => {
                let mut pos = pos;
                for _ in 0..steps {
                    let candidate_pos = direction.step(&map, pos, rows, cols);

                    if let Tile::Wall = map[&candidate_pos] {
                        break;
                    }
                    pos = candidate_pos;
                }

                (pos, direction)
            }
        }
    }

    pos.0 * 1000 + pos.1 * 4 + direction.facing()
}

#[test]
fn a_example() {
    assert_eq!(solve_for(EXAMPLE, Mode::PartA), 6032);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_for(INPUT, Mode::PartA), 29408);
}

/*
#[test]
fn b_example() {
    assert_eq!(solve_for(EXAMPLE, Mode::PartB), 5031);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_for(INPUT, Mode::PartB), 0);
}
*/

fn solve_a() -> PuzzleResult {
    solve_for(INPUT, Mode::PartA).into()
}

fn solve_b() -> PuzzleResult {
    solve_for(INPUT, Mode::PartB).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
