use std::collections::{HashMap, VecDeque};

use crate::common::coordinate::Coordinate;
use crate::common::direction::Direction;
use crate::common::grid::Grid;
use crate::day::Day;
use crate::PuzzleResult;

#[cfg(test)]
const EXAMPLE: &str = include_str!("input/21_example");
const INPUT: &str = include_str!("input/21");

enum Tile {
    Garden,
    Rock,
}

fn parse(input: &str) -> (Vec<Vec<Tile>>, Coordinate) {
    let mut start = Coordinate { x: 0, y: 0 };
    let map = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, ch)| match ch {
                    '#' => Tile::Rock,
                    '.' => Tile::Garden,
                    'S' => {
                        start = Coordinate { x: col, y: row };
                        Tile::Garden
                    }
                    ch => panic!("unknown tile '{ch}'"),
                })
                .collect()
        })
        .collect();
    (map, start)
}

fn solve_a_for(input: &str, target: usize) -> usize {
    let (map, coord) = parse(input);

    let mut distances: HashMap<Coordinate, usize> = HashMap::new();
    distances.insert(coord, 0);

    let mut queue = VecDeque::new();
    queue.push_back((coord, 0));

    let bounds = map.bounds();

    while let Some((coord, distance)) = queue.pop_front() {
        if distance != target {
            for dir in [
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
            ] {
                if let Some(coord) = dir.step(coord, bounds) {
                    if distances.contains_key(&coord) {
                        continue;
                    }
                    if let Tile::Garden = map.at(coord) {
                        queue.push_back((coord, distance + 1));
                        distances.insert(coord, distance + 1);
                    }
                }
            }
        }
    }

    distances.values().filter(|f| *f % 2 == 0).count()
}

fn solve_b_for(input: &str, target: isize) -> usize {
    let (map, coord) = parse(input);
    let bounds = map.bounds();

    let mut reachable: HashMap<Coordinate, bool> = HashMap::new();

    let mut queue = VecDeque::new();
    queue.push_back((coord, true));
    reachable.insert(coord, false);

    while let Some((coord, even)) = queue.pop_front() {
        for dir in Direction::ALL {
            if let Some(coord) = dir.step(coord, bounds) {
                if reachable.contains_key(&coord) {
                    continue;
                }
                if let Tile::Garden = map.at(coord) {
                    queue.push_back((coord, !even));

                    reachable.insert(coord, even);
                }
            }
        }
    }

    let mut interior_even = 0usize;
    let mut interior_odd = 0usize;
    let mut exterior = 0usize;

    for (coord, even) in reachable {
        #[allow(clippy::cast_possible_wrap)]
        let ix = coord.x as isize - 65;
        #[allow(clippy::cast_possible_wrap)]
        let iy = coord.y as isize - 65;

        match (ix.abs() + iy.abs() <= 65, even) {
            (true, true) => interior_even += 1,
            (true, false) => interior_odd += 1,
            (false, _) => exterior += 1,
        }
    }

    let step_target = target;
    let tiles = (step_target - 65) / 131;
    assert_eq!(step_target, tiles * 131 + 65);

    //   -2   -1    0    1    2
    // +----+----+----+----+----+
    // |    |    | /\ |    |    |
    // |    |    |/AA\|    |    | -2
    // |    |   /|\AA/|\   |    |
    // |    |  /D|C\/C|D\  |    |
    // +----+----+----+----+----+
    // |    | /\C|D/\D|C/\ |    |
    // |    |/AA\|/BB\|/AA\|    | -1
    // |   /|\AA/|\BB/|\AA/|\   |
    // |  /D|C\/C|D\/D|C\/C|D\  |
    // +----+----+----+----+----+
    // | /\C|D/\D|C/\C|D/\D|C/\ |
    // |/AA\|/BB\|/AA\|/BB\|/AA\| 0
    // |\AA/|\BB/|\AA/|\BB/|\AA/|
    // | \/C|D\/D|C\/C|D\/D|C\/ |
    // +----+----+----+----+----+
    // |  \D|C/\C|D/\D|C/\C|D/  |
    // |   \|/AA\|/BB\|/AA\|/   | 1
    // |    |\AA/|\BB/|\AA/|    |
    // |    | \/C|D\/D|C\/ |    |
    // +----+----+----+----+----+
    // |    |  \D|C/\C|D/  |    |
    // |    |   \|/AA\|/   |    | 2
    // |    |    |\AA/|    |    |
    // |    |    | \/ |    |    |
    // +----+----+----+----+----+
    //
    // tiles to border = 2
    // As = 9 :: 9 full tiles :: (ttb+1)^2
    // Bs = 4 :: 4 full tiles :: ttb^2
    // Cs = 6 :: 1 full tile, 4 half tiles, 4 3/4 tiles :: (ttb-1)^2 + 2 + (ttb-1) * 3
    // Ds = 6 :: 4 full tiles, 8 1/4 tiles :: (ttb+1)*ttb

    // C = D so even/odd is not distinguished for exterior

    let tiles = tiles as usize;

    let a = (tiles + 1) * (tiles + 1);
    let b = tiles * tiles;
    // let c = (tiles - 1) * (tiles - 1) + 2 + (tiles - 1) * 3;
    let d = tiles * (tiles + 1);

    a * interior_even + b * interior_odd + d * exterior
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for(EXAMPLE, 6), 16);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(INPUT, 64), 3776);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(INPUT, 26501365), 625587097150084);
}

fn solve_a() -> PuzzleResult {
    solve_a_for(INPUT, 64).into()
}

fn solve_b() -> PuzzleResult {
    solve_b_for(INPUT, 26501365).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
