use std::collections::HashSet;

use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::PuzzleResult;

#[cfg(test)]
const EXAMPLE: &str = include_str!("input/16_example");
const INPUT: &str = include_str!("input/16");

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn step(self, x: usize, y: usize, bound_x: usize, bound_y: usize) -> Option<(usize, usize)> {
        match self {
            Direction::North if y == 0 => None,
            Direction::East if x + 1 == bound_x => None,
            Direction::South if y + 1 == bound_y => None,
            Direction::West if x == 0 => None,
            Direction::North => Some((x, y - 1)),
            Direction::East => Some((x + 1, y)),
            Direction::South => Some((x, y + 1)),
            Direction::West => Some((x - 1, y)),
        }
    }
}

enum Tile {
    Empty,
    MirrorNorthEast,
    MirrorNorthWest,
    SplitterNorthSouth,
    SplitterEastWest,
}

impl Tile {
    fn output(&self, input_beam: Direction) -> (Direction, Option<Direction>) {
        match (self, input_beam) {
            (Tile::Empty, input_beam) => (input_beam, None),
            (Tile::MirrorNorthEast, Direction::North) => (Direction::East, None),
            (Tile::MirrorNorthEast, Direction::East) => (Direction::North, None),
            (Tile::MirrorNorthEast, Direction::South) => (Direction::West, None),
            (Tile::MirrorNorthEast, Direction::West) => (Direction::South, None),
            (Tile::MirrorNorthWest, Direction::North) => (Direction::West, None),
            (Tile::MirrorNorthWest, Direction::East) => (Direction::South, None),
            (Tile::MirrorNorthWest, Direction::South) => (Direction::East, None),
            (Tile::MirrorNorthWest, Direction::West) => (Direction::North, None),
            (Tile::SplitterNorthSouth, Direction::North) => (Direction::North, None),
            (Tile::SplitterNorthSouth, Direction::South) => (Direction::South, None),
            (Tile::SplitterNorthSouth, Direction::East | Direction::West) => {
                (Direction::North, Some(Direction::South))
            }
            (Tile::SplitterEastWest, Direction::East) => (Direction::East, None),
            (Tile::SplitterEastWest, Direction::West) => (Direction::West, None),
            (Tile::SplitterEastWest, Direction::North | Direction::South) => {
                (Direction::East, Some(Direction::West))
            }
        }
    }
}

fn parse(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    '.' => Tile::Empty,
                    '|' => Tile::SplitterNorthSouth,
                    '-' => Tile::SplitterEastWest,
                    '/' => Tile::MirrorNorthEast,
                    '\\' => Tile::MirrorNorthWest,
                    ch => panic!("unknown tile '{ch}'"),
                })
                .collect()
        })
        .collect()
}

fn solve_for(
    grid: &Vec<Vec<Tile>>,
    start_x: usize,
    start_y: usize,
    start_direction: Direction,
) -> usize {
    let mut beams = vec![(start_x, start_y, start_direction)];
    let mut energized = HashSet::new();
    let mut visited = HashSet::new();

    let len = grid.len();

    while let Some((x, y, direction)) = beams.pop() {
        if !visited.insert((x, y, direction)) {
            continue;
        }
        energized.insert((x, y));

        let (a, b) = grid[y][x].output(direction);
        if let Some((x, y)) = a.step(x, y, len, len) {
            beams.push((x, y, a));
        }
        if let Some(b) = b {
            if let Some((x, y)) = b.step(x, y, len, len) {
                beams.push((x, y, b));
            }
        }
    }
    energized.len()
}

fn solve_a_for(input: &str) -> usize {
    solve_for(&parse(input), 0, 0, Direction::East)
}

fn solve_b_for(input: &str) -> usize {
    let grid = parse(input);
    let edge = grid.len() - 1;

    (0..grid.len())
        .into_par_iter()
        .map(|start| {
            solve_for(&grid, start, edge, Direction::North)
                .max(solve_for(&grid, 0, start, Direction::East))
                .max(solve_for(&grid, start, 0, Direction::South))
                .max(solve_for(&grid, edge, start, Direction::West))
        })
        .max()
        .unwrap()
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for(EXAMPLE), 46);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(INPUT), 8389);
}

#[test]
fn b_example() {
    assert_eq!(solve_b_for(EXAMPLE), 51);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(INPUT), 8564);
}

pub fn solve_a() -> PuzzleResult {
    solve_a_for(INPUT).into()
}

pub fn solve_b() -> PuzzleResult {
    solve_b_for(INPUT).into()
}
