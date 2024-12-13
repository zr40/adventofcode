use std::collections::{BTreeMap, BTreeSet};

use crate::PuzzleResult;
use crate::common::aoc::input_for;
use crate::common::direction::Direction;
use crate::day::Day;

#[cfg(test)]
const EXAMPLE_1: &str = include_str!("example/12_1");
#[cfg(test)]
const EXAMPLE_2: &str = include_str!("example/12_2");
#[cfg(test)]
const EXAMPLE_3: &str = include_str!("example/12_3");

#[cfg(test)]
const EXAMPLE_B1: &str = include_str!("example/12b_1");
#[cfg(test)]
const EXAMPLE_B2: &str = include_str!("example/12b_2");

fn solve_a_for(input: &str) -> usize {
    let mut plots = BTreeMap::new();
    for (row, line) in input.lines().enumerate() {
        for (col, plant) in line.chars().enumerate() {
            plots.insert((row, col), plant);
        }
    }

    let mut visited = BTreeSet::new();

    let mut sum = 0;

    for ((row, col), plant) in &plots {
        let mut area = 0;
        let mut perimeter = 0;

        let mut queue = vec![(*row, *col)];

        while let Some((row, col)) = queue.pop() {
            if visited.insert((row, col)) {
                area += 1;

                if let Some(other) = plots.get(&(row.wrapping_sub(1), col)) {
                    if other == plant {
                        queue.push((row.wrapping_sub(1), col));
                    } else {
                        perimeter += 1;
                    }
                } else {
                    perimeter += 1;
                }

                if let Some(other) = plots.get(&(row + 1, col)) {
                    if other == plant {
                        queue.push((row + 1, col));
                    } else {
                        perimeter += 1;
                    }
                } else {
                    perimeter += 1;
                }

                if let Some(other) = plots.get(&(row, col.wrapping_sub(1))) {
                    if other == plant {
                        queue.push((row, col.wrapping_sub(1)));
                    } else {
                        perimeter += 1;
                    }
                } else {
                    perimeter += 1;
                }

                if let Some(other) = plots.get(&(row, col + 1)) {
                    if other == plant {
                        queue.push((row, col + 1));
                    } else {
                        perimeter += 1;
                    }
                } else {
                    perimeter += 1;
                }
            }
        }
        sum += perimeter * area;
    }
    sum
}

fn solve_b_for(input: &str) -> usize {
    let mut plots = BTreeMap::new();
    for (row, line) in input.lines().enumerate() {
        for (col, plant) in line.chars().enumerate() {
            plots.insert((row, col), plant);
        }
    }

    let mut visited = BTreeSet::new();

    let mut sum = 0;

    for ((row, col), plant) in &plots {
        let mut area = 0;
        let mut borders = BTreeSet::new();

        let mut queue = vec![(*row, *col)];

        while let Some((row, col)) = queue.pop() {
            if visited.insert((row, col)) {
                area += 1;

                if let Some(other) = plots.get(&(row.wrapping_sub(1), col)) {
                    if other == plant {
                        queue.push((row.wrapping_sub(1), col));
                    } else {
                        borders.insert((Direction::Up, row, col));
                    }
                } else {
                    borders.insert((Direction::Up, row, col));
                }

                if let Some(other) = plots.get(&(row + 1, col)) {
                    if other == plant {
                        queue.push((row + 1, col));
                    } else {
                        borders.insert((Direction::Down, row + 1, col));
                    }
                } else {
                    borders.insert((Direction::Down, row + 1, col));
                }

                if let Some(other) = plots.get(&(row, col.wrapping_sub(1))) {
                    if other == plant {
                        queue.push((row, col.wrapping_sub(1)));
                    } else {
                        borders.insert((Direction::Left, row, col));
                    }
                } else {
                    borders.insert((Direction::Left, row, col));
                }

                if let Some(other) = plots.get(&(row, col + 1)) {
                    if other == plant {
                        queue.push((row, col + 1));
                    } else {
                        borders.insert((Direction::Right, row, col + 1));
                    }
                } else {
                    borders.insert((Direction::Right, row, col + 1));
                }
            }
        }

        let mut perimeters = 0;

        while let Some((border, mut top, mut left)) = borders.pop_first() {
            perimeters += 1;

            print!("Border {border:?} from {top}x{left}");

            loop {
                match border {
                    Direction::Left | Direction::Right => top += 1,
                    Direction::Up | Direction::Down => left += 1,
                };

                if !borders.remove(&(border, top, left)) {
                    break;
                }
            }
        }

        sum += perimeters * area;
    }
    sum
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for(EXAMPLE_1), 140);
    assert_eq!(solve_a_for(EXAMPLE_2), 772);
    assert_eq!(solve_a_for(EXAMPLE_3), 1930);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(&input_for(2024, 12)), 1370100);
}

fn solve_a() -> PuzzleResult {
    solve_a_for(&input_for(2024, 12)).into()
}

#[test]
fn b_example_1() {
    assert_eq!(solve_b_for(EXAMPLE_1), 80);
}
#[test]
fn b_example_2() {
    assert_eq!(solve_b_for(EXAMPLE_2), 436);
}
#[test]
fn b_example_3() {
    assert_eq!(solve_b_for(EXAMPLE_B1), 236);
}
#[test]
fn b_example_4() {
    assert_eq!(solve_b_for(EXAMPLE_B2), 368);
}
#[test]
fn b_example_5() {
    assert_eq!(solve_b_for(EXAMPLE_3), 1206);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(&input_for(2024, 12)), 818286);
}

fn solve_b() -> PuzzleResult {
    solve_b_for(&input_for(2024, 12)).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
