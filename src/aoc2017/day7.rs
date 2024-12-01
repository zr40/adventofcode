use std::collections::HashMap;

use itertools::Itertools;

use crate::common::aoc::input_for;
use crate::day::Day;
use crate::puzzle_result::PuzzleResult;

#[cfg(test)]
const EXAMPLE: &str = include_str!("example/7");

fn solve_a_for(input: &str) -> &str {
    let mut held_by = HashMap::new();

    for line in input.lines() {
        let mut line = line.split(" -> ");

        let program = line.next().unwrap().split(' ').next().unwrap();

        if let Some(others) = line.next() {
            for other in others.split(", ") {
                held_by.insert(other, program);
            }
        }
    }

    let mut program = held_by.keys().next().unwrap();

    while held_by.contains_key(program) {
        program = &held_by[program];
    }

    program
}

fn solve_b_for(input: &str) -> u32 {
    let mut holding = HashMap::new();
    let mut weights = HashMap::new();

    for line in input.lines() {
        let mut line = line.split(" -> ");

        let program = line.next().unwrap();
        let mut parts = program.split(" (");
        let name = parts.next().unwrap();
        let weight = parts.next().unwrap().trim_matches(')').parse().unwrap();

        weights.insert(name, weight);

        if let Some(others) = line.next() {
            let others: Vec<&str> = others.split(", ").collect();
            holding.insert(name, others);
        }
    }

    holding
        .keys()
        .filter_map(|program| {
            let (min, max) = holding[program]
                .iter()
                .minmax_by_key(|x| weight_of(x, &holding, &weights))
                .into_option()
                .unwrap();

            let max_weight = weight_of(max, &holding, &weights);
            let min_weight = weight_of(min, &holding, &weights);

            if max_weight == min_weight {
                None
            } else {
                let mut min_count = 0;
                let mut min_program = "";
                let mut max_program = "";

                for held_program in &holding[program] {
                    let w = weight_of(held_program, &holding, &weights);
                    if w == min_weight {
                        min_count += 1;
                        min_program = held_program;
                    } else {
                        max_program = held_program;
                    }
                }

                if min_count == 1 {
                    Some((max_weight, weights[min_program] + max_weight - min_weight))
                } else {
                    Some((min_weight, weights[max_program] + min_weight - max_weight))
                }
            }
        })
        .min_by_key(|(weight, _)| *weight)
        .unwrap()
        .1
}

fn weight_of(
    program: &str,
    holding: &HashMap<&str, Vec<&str>>,
    weights: &HashMap<&str, u32>,
) -> u32 {
    let mut weight = *weights.get(program).unwrap();
    for held_program in holding.get(program).unwrap_or(&vec![]) {
        weight += weight_of(held_program, holding, weights);
    }
    weight
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for(EXAMPLE), "tknk");
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(&input_for(2017, 7)), "mwzaxaj");
}

fn solve_a() -> PuzzleResult {
    solve_a_for(&input_for(2017, 7)).into()
}

#[test]
fn b_example() {
    assert_eq!(solve_b_for(EXAMPLE), 60);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(&input_for(2017, 7)), 1219);
}

fn solve_b() -> PuzzleResult {
    solve_b_for(&input_for(2017, 7)).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
