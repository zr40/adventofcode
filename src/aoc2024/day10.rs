use std::collections::{BTreeMap, HashSet};

use crate::PuzzleResult;
use crate::common::aoc::input_for;
use crate::common::coordinate::Coordinate;
use crate::day::Day;

#[cfg(test)]
const EXAMPLE_A1: &str = include_str!("example/10a_1");
#[cfg(test)]
const EXAMPLE_A2: &str = include_str!("example/10a_2");
#[cfg(test)]
const EXAMPLE_A3: &str = include_str!("example/10a_3");
#[cfg(test)]
const EXAMPLE_A4: &str = include_str!("example/10a_4");

#[cfg(test)]
const EXAMPLE_LARGER: &str = include_str!("example/10_larger");

#[cfg(test)]
const EXAMPLE_B1: &str = include_str!("example/10b_1");
#[cfg(test)]
const EXAMPLE_B2: &str = include_str!("example/10b_2");
#[cfg(test)]
const EXAMPLE_B3: &str = include_str!("example/10b_3");

enum Mode {
    Score,
    Rating,
}

fn solve_for(input: &str, mode: Mode) -> usize {
    let mut map = BTreeMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '.' {
                continue;
            }

            map.insert(Coordinate { x: x + 1, y: y + 1 }, ch as u16 - '0' as u16);
        }
    }

    map.iter()
        .filter_map(|(coord, height)| {
            let mut reachable = HashSet::new();
            let mut rating = 0;
            if *height == 0 {
                let mut queue = vec![
                    (
                        Coordinate {
                            x: coord.x - 1,
                            y: coord.y,
                        },
                        height + 1,
                    ),
                    (
                        Coordinate {
                            x: coord.x + 1,
                            y: coord.y,
                        },
                        height + 1,
                    ),
                    (
                        Coordinate {
                            x: coord.x,
                            y: coord.y - 1,
                        },
                        height + 1,
                    ),
                    (
                        Coordinate {
                            x: coord.x,
                            y: coord.y + 1,
                        },
                        height + 1,
                    ),
                ];

                while let Some((coord, height)) = queue.pop() {
                    if let Some(map_height) = map.get(&coord) {
                        if *map_height == height {
                            if *map_height == 9 {
                                reachable.insert(coord);
                                rating += 1;
                            } else {
                                queue.push((
                                    Coordinate {
                                        x: coord.x - 1,
                                        y: coord.y,
                                    },
                                    height + 1,
                                ));
                                queue.push((
                                    Coordinate {
                                        x: coord.x + 1,
                                        y: coord.y,
                                    },
                                    height + 1,
                                ));
                                queue.push((
                                    Coordinate {
                                        x: coord.x,
                                        y: coord.y - 1,
                                    },
                                    height + 1,
                                ));
                                queue.push((
                                    Coordinate {
                                        x: coord.x,
                                        y: coord.y + 1,
                                    },
                                    height + 1,
                                ));
                            }
                        }
                    }
                }

                Some(match mode {
                    Mode::Score => reachable.len(),
                    Mode::Rating => rating,
                })
            } else {
                None
            }
        })
        .sum()
}

#[test]
fn a_example() {
    assert_eq!(solve_for(EXAMPLE_A1, Mode::Score), 1);
    assert_eq!(solve_for(EXAMPLE_A2, Mode::Score), 2);
    assert_eq!(solve_for(EXAMPLE_A3, Mode::Score), 4);
    assert_eq!(solve_for(EXAMPLE_A4, Mode::Score), 3);
    assert_eq!(solve_for(EXAMPLE_LARGER, Mode::Score), 36);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_for(&input_for(2024, 10), Mode::Score), 778);
}

fn solve_a() -> PuzzleResult {
    solve_for(&input_for(2024, 10), Mode::Score).into()
}

#[test]
fn b_example() {
    assert_eq!(solve_for(EXAMPLE_B1, Mode::Rating), 3);
    assert_eq!(solve_for(EXAMPLE_B2, Mode::Rating), 13);
    assert_eq!(solve_for(EXAMPLE_B3, Mode::Rating), 227);
    assert_eq!(solve_for(EXAMPLE_LARGER, Mode::Rating), 81);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_for(&input_for(2024, 10), Mode::Rating), 1925);
}

fn solve_b() -> PuzzleResult {
    solve_for(&input_for(2024, 10), Mode::Rating).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
