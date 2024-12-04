use itertools::Itertools;

use crate::PuzzleResult;
use crate::common::aoc::input_for;
use crate::day::Day;

#[cfg(test)]
const EXAMPLE: &str = include_str!("example/4");

fn solve_a_for(mut input: &str) -> usize {
    let grid = input.lines().map(|l| l.chars().collect_vec()).collect_vec();

    (0..grid.len() as isize)
        .map(|gy| {
            (0..grid[0].len() as isize)
                .map(|gx| {
                    (-1..=1)
                        .map(|dy| {
                            if dy * 3 + gy < 0 || dy * 3 + gy >= grid.len() as isize {
                                0
                            } else {
                                (-1..=1)
                                    .map(|dx| {
                                        if dx * 3 + gx < 0 || dx * 3 + gx >= grid.len() as isize {
                                            0
                                        } else if grid[gy as usize][gx as usize] == 'X'
                                            && grid[(gy + dy) as usize][(gx + dx) as usize] == 'M'
                                            && grid[(gy + dy * 2) as usize][(gx + dx * 2) as usize]
                                                == 'A'
                                            && grid[(gy + dy * 3) as usize][(gx + dx * 3) as usize]
                                                == 'S'
                                        {
                                            1
                                        } else {
                                            0
                                        }
                                    })
                                    .sum()
                            }
                        })
                        .sum::<usize>()
                })
                .sum::<usize>()
        })
        .sum()
}

fn is_ms_pair(a: char, b: char) -> bool {
    matches!((a, b), ('M', 'S') | ('S', 'M'))
}

fn solve_b_for(mut input: &str) -> usize {
    let grid = input.lines().map(|l| l.chars().collect_vec()).collect_vec();

    (1..grid.len() - 1)
        .map(|gy| {
            (1..grid[0].len() - 1)
                .filter(|gx| {
                    grid[gy][*gx] == 'A'
                        && is_ms_pair(grid[gy - 1][gx - 1], grid[gy + 1][gx + 1])
                        && is_ms_pair(grid[gy - 1][gx + 1], grid[gy + 1][gx - 1])
                })
                .count()
        })
        .sum()
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for(EXAMPLE), 18);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(&input_for(2024, 4)), 2718);
}

fn solve_a() -> PuzzleResult {
    solve_a_for(&input_for(2024, 4)).into()
}

#[test]
fn b_example() {
    assert_eq!(solve_b_for(EXAMPLE), 9);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(&input_for(2024, 4)), 2046);
}

fn solve_b() -> PuzzleResult {
    solve_b_for(&input_for(2024, 4)).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
