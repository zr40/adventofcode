use crate::common::aoc::input_for;
use crate::day::Day;
use crate::puzzle_result::PuzzleResult;

#[cfg(test)]
const EXAMPLE: &str = include_str!("example/13");

fn solve_a_for(input: &str) -> u32 {
    input
        .split('\n')
        .map(|x| {
            let mut iter = x.split(": ");
            let position: u32 = iter.next().unwrap().parse().unwrap();
            let range: u32 = iter.next().unwrap().parse().unwrap();

            let x = position % (range * 2 - 2);

            if x == 0 { position * range } else { 0 }
        })
        .sum()
}

fn solve_b_for(input: &str) -> u32 {
    let mut delay = 0;

    let input: Vec<(u32, u32)> = input
        .split('\n')
        .map(|x| {
            let mut iter = x.split(": ");
            let position: u32 = iter.next().unwrap().parse().unwrap();
            let range: u32 = iter.next().unwrap().parse().unwrap();

            (position, range)
        })
        .collect();

    loop {
        if input.iter().all(|&(position, range)| {
            let x = (position + delay) % (range * 2 - 2);

            x != 0
        }) {
            break;
        }

        delay += 1;
    }

    delay
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for(EXAMPLE), 24);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(&input_for(2017, 13)), 1704);
}

fn solve_a() -> PuzzleResult {
    solve_a_for(&input_for(2017, 13)).into()
}

#[test]
fn b_example() {
    assert_eq!(solve_b_for(EXAMPLE), 10);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(&input_for(2017, 13)), 3970918);
}

fn solve_b() -> PuzzleResult {
    solve_b_for(&input_for(2017, 13)).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
