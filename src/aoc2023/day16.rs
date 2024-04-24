use std::collections::HashSet;

use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::common::coordinate::Coordinate;
use crate::common::direction::Direction;
use crate::common::grid::Grid;
use crate::day::Day;
use crate::PuzzleResult;

#[cfg(test)]
const EXAMPLE: &str = include_str!("input/16_example");
const INPUT: &str = include_str!("input/16");

enum Tile {
    Empty,
    MirrorNorthEast,
    MirrorNorthWest,
    SplitterNorthSouth,
    SplitterEastWest,
}

impl Tile {
    fn output(&self, input_beam: Direction) -> (Direction, Option<Direction>) {
        #[allow(clippy::match_same_arms)]
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

fn solve_for(grid: &Vec<Vec<Tile>>, start: Coordinate, start_direction: Direction) -> usize {
    let mut beams = vec![(start, start_direction)];
    let mut energized = HashSet::new();
    let mut visited = HashSet::new();

    let bounds = Coordinate {
        x: grid[0].len(),
        y: grid.len(),
    };

    while let Some((coord, direction)) = beams.pop() {
        if !visited.insert((coord, direction)) {
            continue;
        }
        energized.insert(coord);

        let (a, b) = grid.at(coord).output(direction);
        if let Some(coord) = a.step(coord, bounds) {
            beams.push((coord, a));
        }
        if let Some(b) = b {
            if let Some(coord) = b.step(coord, bounds) {
                beams.push((coord, b));
            }
        }
    }
    energized.len()
}

fn solve_a_for(input: &str) -> usize {
    solve_for(&parse(input), Coordinate { x: 0, y: 0 }, Direction::East)
}

fn solve_b_for(input: &str) -> usize {
    let grid = parse(input);
    let edge = grid.len() - 1;

    (0..grid.len())
        .into_par_iter()
        .map(|start| {
            solve_for(&grid, Coordinate { x: start, y: edge }, Direction::North)
                .max(solve_for(
                    &grid,
                    Coordinate { x: 0, y: start },
                    Direction::East,
                ))
                .max(solve_for(
                    &grid,
                    Coordinate { x: start, y: 0 },
                    Direction::South,
                ))
                .max(solve_for(
                    &grid,
                    Coordinate { x: edge, y: start },
                    Direction::West,
                ))
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

fn solve_a() -> PuzzleResult {
    solve_a_for(INPUT).into()
}

fn solve_b() -> PuzzleResult {
    solve_b_for(INPUT).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
