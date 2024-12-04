use itertools::{Itertools, iproduct};

use crate::PuzzleResult;
use crate::common::aoc::input_for;
use crate::day::Day;

#[cfg(test)]
const EXAMPLE: &str = include_str!("example/4");

fn is_xmas(grid: &Vec<Vec<char>>, gx: usize, gy: usize, dx: isize, dy: isize) -> bool {
    let gx = gx as isize;
    let gy = gy as isize;

    if dy * 3 + gy < 0 || dy * 3 + gy >= grid.len() as isize {
        return false;
    }
    if dx * 3 + gx < 0 || dx * 3 + gx >= grid[0].len() as isize {
        return false;
    }

    let x = grid[gy as usize][gx as usize];
    let m = grid[(gy + dy) as usize][(gx + dx) as usize];
    let a = grid[(gy + dy * 2) as usize][(gx + dx * 2) as usize];
    let s = grid[(gy + dy * 3) as usize][(gx + dx * 3) as usize];

    matches!((x, m, a, s), ('X', 'M', 'A', 'S'))
}

fn solve_a_for(input: &str) -> usize {
    let grid = input.lines().map(|l| l.chars().collect_vec()).collect_vec();

    iproduct!(0..grid.len(), 0..grid[0].len(), -1..=1, -1..=1)
        .filter(|(gy, gx, dy, dx)| is_xmas(&grid, *gx, *gy, *dx, *dy))
        .count()
}

fn is_ms_pair(a: char, b: char) -> bool {
    matches!((a, b), ('M', 'S') | ('S', 'M'))
}

fn is_x_mas(grid: &Vec<Vec<char>>, gx: usize, gy: usize) -> bool {
    grid[gy][gx] == 'A'
        && is_ms_pair(grid[gy - 1][gx - 1], grid[gy + 1][gx + 1])
        && is_ms_pair(grid[gy - 1][gx + 1], grid[gy + 1][gx - 1])
}

fn solve_b_for(input: &str) -> usize {
    let grid = input.lines().map(|l| l.chars().collect_vec()).collect_vec();

    iproduct!(1..grid.len() - 1, 1..grid[0].len() - 1)
        .filter(|(gy, gx)| is_x_mas(&grid, *gx, *gy))
        .count()
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
