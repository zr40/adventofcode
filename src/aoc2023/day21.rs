use std::collections::{HashMap, VecDeque};

// use indicatif::ParallelProgressIterator;
// use num_integer::Integer;
// use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::common::coordinate::Coordinate;
use crate::common::direction::Direction;
use crate::common::grid::Grid;
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

#[allow(clippy::too_many_lines)]
fn solve_b_for(input: &str, target: isize) -> usize {
    let (map, coord) = parse(input);
    let bounds = map.bounds();

    let mut reachable: HashMap<Coordinate, bool> = HashMap::new();

    let mut queue = VecDeque::new();
    queue.push_back((coord, true));
    reachable.insert(coord, true);

    while let Some((coord, even)) = queue.pop_front() {
        for dir in Direction::ALL {
            if let Some(coord) = dir.step(coord, bounds) {
                if reachable.contains_key(&coord) {
                    continue;
                }
                if let Tile::Garden = map.at(coord) {
                    queue.push_back((coord, !even));

                    reachable.insert(coord, !even);
                }
            }
        }
    }

    let mut interior_even = 0usize;
    let mut interior_odd = 0usize;
    let mut exterior_even = 0usize;
    let mut exterior_odd = 0usize;

    println!("{}", map.len());
    for (coord, even) in reachable {
        let ix = coord.x as isize - 65;
        let iy = coord.y as isize - 65;

        match (ix.abs() + iy.abs() <= 65, even) {
            (true, true) => interior_even += 1,
            (true, false) => interior_odd += 1,
            (false, true) => exterior_even += 1,
            (false, false) => exterior_odd += 1,
        }
    }

    println!(
        "interior: {interior_even} {interior_odd} {}",
        interior_even + interior_odd
    );
    println!(
        "exterior: {exterior_even} {exterior_odd} {}",
        exterior_even + exterior_odd
    );

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
    // Bs = 4 (4 full tiles) :: ttb^2
    // Cs = 6 (1 full tile, 4 half tiles, 4 3/4 tiles) :: (ttb-1)^2 + 2 + (ttb-1 * 3)
    // Ds = 6 (4 full tiles, 8 1/4 tiles) :: (ttb+1)*ttb

    // let interior_even = 1000; // A
    // let interior_odd = 100; // B
    // let exterior_even = 10; // C
    // let exterior_odd = 1; // D

    let tiles = tiles as usize;

    let a = (tiles + 1) * (tiles + 1);
    let b = tiles * tiles;
    let c = (tiles - 1) * (tiles - 1) + 2 + (tiles - 1) * 3;
    let d = tiles * (tiles + 1);

    println!("ttb {tiles} a {a} b {b} c {c} d {d} sum {}", a + b + c + d,);

    a * interior_even + b * interior_odd + c * exterior_even + d * exterior_odd

    // let tiles = tiles as isize;

    // let mut sum = ((-tiles)..=(tiles))
    //     .into_par_iter()
    //     .progress_count(tiles as u64 * 2 + 1)
    //     .map(|y| {
    //         let mut sum = 0usize;
    //         let range = tiles - y.abs();
    //         for x in (-range)..=(range) {
    //             if (x + y).is_even() {
    //                 sum += interior_even + exterior_even;
    //             } else {
    //                 sum += interior_odd + exterior_odd;
    //             }
    //         }
    //         if y.is_negative() {
    //             sum += exterior_odd;
    //         }
    //         sum -= exterior_even;
    //         sum
    //     })
    //     .sum();
    // sum += exterior_even * 2;

    // sum -= exterior_odd * (2 + tiles as usize - 1);

    // As = 9
    // Bs = 16
    // Cs =
    // Ds =
    // n = 3

    //   ^
    //  / \
    // <   >
    //  \ /
    //   V

    //     ^         ^
    //    / \       /|\
    //   /   \     /#|#\
    //  /     \   /##|##\
    // ####    > <---x--->
    //  \tiles/   \##|##/
    //   \   /     \#|#/
    //    \ /       \|/
    //     v         v

    //              < = A+C+E
    //              ^ = A+D+E
    //              > = A+B+D
    //              V = A+B+C
    //     top left / = A+C+D+2E
    //    top right \ = A+B+2D+E
    // bottom right / = A+2B+C+D
    //  bottom left \ = A+B+2C+E
    //       interior = A+B+C+D+E

    // println!(
    //     "{}",
    //     interior_count * interiors + exterior_count * exteriors
    // );

    // sum
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
#[ignore = "todo"]
fn b_puzzle() {
    assert_eq!(solve_b_for(INPUT, 26501365), 0);
}

pub fn solve_a() -> PuzzleResult {
    solve_a_for(INPUT, 64).into()
}

pub fn solve_b() -> PuzzleResult {
    // solve_b_for(INPUT, 26501365).into()
    PuzzleResult::Todo
}
