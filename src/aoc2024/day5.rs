use crate::PuzzleResult;
use crate::common::aoc::input_for;
use crate::day::Day;

#[cfg(test)]
const EXAMPLE: &str = include_str!("example/5");

fn parse(input: &str) -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    let mut lines = input.lines();

    let mut rules: Vec<(usize, usize)> = vec![];

    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }

        let (before, after) = line.split_once('|').unwrap();
        rules.push((before.parse().unwrap(), after.parse().unwrap()));
    }

    let mut updates: Vec<Vec<usize>> = vec![];

    for line in lines {
        updates.push(line.split(',').map(|p| p.parse().unwrap()).collect());
    }

    (rules, updates)
}

fn solve_a_for(input: &str) -> usize {
    let (rules, updates) = parse(input);

    let mut sum = 0;

    'update: for update in updates {
        for (left, right) in &rules {
            if let Some(left_index) = update.iter().position(|p| p == left) {
                if let Some(right_index) = update.iter().position(|p| p == right) {
                    if left_index >= right_index {
                        continue 'update;
                    }
                }
            }
        }

        sum += update[update.len() / 2];
    }

    sum
}

fn solve_b_for(input: &str) -> usize {
    let (rules, mut updates) = parse(input);

    let mut sum = 0;
    for update in &mut updates {
        let mut fixed = false;
        'restart: loop {
            for (left, right) in &rules {
                if let Some(left_index) = update.iter().position(|p| p == left) {
                    if let Some(right_index) = update.iter().position(|p| p == right) {
                        if left_index >= right_index {
                            update.swap(left_index, right_index);
                            fixed = true;
                            continue 'restart;
                        }
                    }
                }
            }
            if fixed {
                sum += update[update.len() / 2];
            }
            break;
        }
    }

    sum
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for(EXAMPLE), 143);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(&input_for(2024, 5)), 5275);
}

fn solve_a() -> PuzzleResult {
    solve_a_for(&input_for(2024, 5)).into()
}

#[test]
fn b_example() {
    assert_eq!(solve_b_for(EXAMPLE), 123);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(&input_for(2024, 5)), 6191);
}

fn solve_b() -> PuzzleResult {
    solve_b_for(&input_for(2024, 5)).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
