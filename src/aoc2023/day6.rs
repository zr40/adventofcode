use crate::PuzzleResult;
use crate::day::Day;

#[cfg(test)]
const EXAMPLE: &str = include_str!("input/6_example");
const INPUT: &str = include_str!("input/6");

fn solve_with_quadratic_equation(time: u64, distance: u64) -> u64 {
    let time = time as f64;
    let distance = distance as f64;

    let lowest_speed = (-time + (time.powi(2) - (4.0 * distance)).sqrt()) / -2.0;
    let highest_speed = (-time - (time.powi(2) - (4.0 * distance)).sqrt()) / -2.0;

    highest_speed.next_down() as u64 - lowest_speed.next_up() as u64
}

fn solve_a_for(input: &str) -> u64 {
    let mut lines = input.lines();
    let times = lines
        .next()
        .unwrap()
        .split(' ')
        .filter_map(|t| t.parse::<u64>().ok());
    let distances = lines
        .next()
        .unwrap()
        .split(' ')
        .filter_map(|t| t.parse::<u64>().ok());

    times
        .zip(distances)
        .map(|(time, distance)| solve_with_quadratic_equation(time, distance))
        .product()
}

fn solve_b_for(input: &str) -> u64 {
    let mut lines = input.lines();

    let time = lines
        .next()
        .unwrap()
        .split_once(' ')
        .unwrap()
        .1
        .chars()
        .fold(0, |acc, t| match t.to_digit(10) {
            None => acc,
            Some(d) => acc * 10 + d as u64,
        });
    let distance = lines
        .next()
        .unwrap()
        .split_once(' ')
        .unwrap()
        .1
        .chars()
        .fold(0, |acc, t| match t.to_digit(10) {
            None => acc,
            Some(d) => acc * 10 + d as u64,
        });

    solve_with_quadratic_equation(time, distance)
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for(EXAMPLE), 288);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(INPUT), 1413720);
}

#[test]
fn b_example() {
    assert_eq!(solve_b_for(EXAMPLE), 71503);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(INPUT), 30565288);
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
