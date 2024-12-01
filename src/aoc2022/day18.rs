use std::collections::HashSet;

use crate::common::aoc::input_for;
use crate::day::Day;
use crate::puzzle_result::PuzzleResult;

#[cfg(test)]
const EXAMPLE: &str = include_str!("example/18");

enum Mode {
    Adjacent,
    Exterior,
}

const DIRECTIONS: [(i32, i32, i32); 6] = [
    (-1, 0, 0),
    (1, 0, 0),
    (0, -1, 0),
    (0, 1, 0),
    (0, 0, -1),
    (0, 0, 1),
];

fn solve_for(input: &str, mode: Mode) -> usize {
    let cubes: HashSet<(i32, i32, i32)> = input
        .lines()
        .map(|line| {
            let mut coords = line.split(',').map(|i| i.parse().unwrap());
            (
                coords.next().unwrap(),
                coords.next().unwrap(),
                coords.next().unwrap(),
            )
        })
        .collect();

    let lower_x = cubes.iter().map(|cube| cube.0).min().unwrap();
    let upper_x = cubes.iter().map(|cube| cube.0).max().unwrap();
    let lower_y = cubes.iter().map(|cube| cube.1).min().unwrap();
    let upper_y = cubes.iter().map(|cube| cube.1).max().unwrap();
    let lower_z = cubes.iter().map(|cube| cube.2).min().unwrap();
    let upper_z = cubes.iter().map(|cube| cube.2).max().unwrap();

    let mut enclosed: HashSet<(i32, i32, i32)> = HashSet::new();

    cubes
        .iter()
        .map(|(x, y, z)| {
            DIRECTIONS
                .iter()
                .filter(|(dx, dy, dz)| match mode {
                    Mode::Adjacent => !cubes.contains(&(x + dx, y + dy, z + dz)),
                    Mode::Exterior => {
                        if enclosed.contains(&(x + dx, y + dy, z + dz)) {
                            return false;
                        }

                        let mut seen = HashSet::new();
                        let mut visit = vec![(x + dx, y + dy, z + dz)];

                        let mut exterior = false;

                        while let Some((x, y, z)) = visit.pop() {
                            if x < lower_x
                                || x > upper_x
                                || y < lower_y
                                || y > upper_y
                                || z < lower_z
                                || z > upper_z
                            {
                                exterior = true;
                                break;
                            }

                            if !cubes.contains(&(x, y, z)) && seen.insert((x, y, z)) {
                                for (dx, dy, dz) in DIRECTIONS {
                                    visit.push((x + dx, y + dy, z + dz));
                                }
                            }
                        }

                        if exterior {
                            true
                        } else {
                            enclosed.extend(seen);
                            false
                        }
                    }
                })
                .count()
        })
        .sum()
}

#[test]
fn a_example_small() {
    assert_eq!(solve_for("1,1,1\n2,1,1", Mode::Adjacent), 10);
}

#[test]
fn a_example() {
    assert_eq!(solve_for(EXAMPLE, Mode::Adjacent), 64);
}

fn solve_a() -> PuzzleResult {
    solve_for(&input_for(2022, 18), Mode::Adjacent).into()
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_for(&input_for(2022, 18), Mode::Adjacent), 3454);
}

#[test]
fn b_example() {
    assert_eq!(solve_for(EXAMPLE, Mode::Exterior), 58);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_for(&input_for(2022, 18), Mode::Exterior), 2014);
}

fn solve_b() -> PuzzleResult {
    solve_for(&input_for(2022, 18), Mode::Exterior).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
