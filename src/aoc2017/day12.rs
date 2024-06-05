use std::collections::{HashMap, HashSet};

use crate::day::Day;
use crate::puzzle_result::PuzzleResult;

#[cfg(test)]
const EXAMPLE: &str = include_str!("input/12_example");
const INPUT: &str = include_str!("input/12");

fn parse(input: &str) -> HashMap<u32, Vec<u32>> {
    let mut connections = HashMap::new();

    for line in input.lines() {
        let mut line = line.split(" <-> ");
        let program = line.next().unwrap().parse().unwrap();
        let connected_programs: Vec<u32> = line
            .next()
            .unwrap()
            .split(", ")
            .map(|x| x.parse().unwrap())
            .collect();

        connections.insert(program, connected_programs);
    }

    connections
}

fn solve_a_for(input: &str) -> usize {
    let connections = parse(input);

    let mut queue = vec![0];
    let mut visited = HashSet::new();

    while let Some(program) = queue.pop() {
        if visited.contains(&program) {
            continue;
        }

        visited.insert(program);

        for connected_program in &connections[&program] {
            queue.push(*connected_program);
        }
    }

    visited.len()
}

fn solve_b_for(input: &str) -> usize {
    let connections = parse(input);

    let mut visited: HashSet<u32> = HashSet::new();
    let mut groups = 0;

    for program in connections.keys() {
        if visited.contains(program) {
            continue;
        }

        let mut queue = vec![program];

        groups += 1;

        while let Some(program) = queue.pop() {
            if visited.contains(program) {
                continue;
            }

            visited.insert(*program);

            for connected_program in &connections[program] {
                queue.push(connected_program);
            }
        }
    }

    groups
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for(EXAMPLE), 6);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(INPUT), 239);
}

#[test]
fn b_example() {
    assert_eq!(solve_b_for(EXAMPLE), 2);
}
#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(INPUT), 215);
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
