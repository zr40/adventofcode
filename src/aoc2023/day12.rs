use std::collections::HashMap;

use crate::PuzzleResult;

#[cfg(test)]
const EXAMPLE: &str = include_str!("input/12_example");
const INPUT: &str = include_str!("input/12");

enum Mode {
    PartA,
    PartB,
}

#[derive(Clone, Copy)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

fn recurse(
    conditions: &Vec<Condition>,
    counts: &Vec<usize>,
    arrangement: &mut Vec<usize>,
    moving: usize,
    mut max_movement: usize,
    memo: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if moving == arrangement.len() {
        for condition in &conditions[*arrangement.last().unwrap() + counts.last().unwrap()..] {
            if let Condition::Damaged = condition {
                return 0;
            }
        }

        return 1;
    }

    let min = if moving == 0 {
        0
    } else {
        arrangement[moving - 1] + counts[moving - 1] + 1
    };

    let mut count = 0;

    for i in min..=conditions.len() - counts[moving] {
        arrangement[moving] = i;

        let mut valid = true;
        for x in 0..counts[moving] {
            if let Condition::Operational = conditions[i + x] {
                valid = false;
            }
        }
        if let Some(Condition::Damaged) = conditions.get(i + counts[moving]) {
            valid = false;
        }
        if i != 0 {
            if let Some(Condition::Damaged) = conditions.get(i - 1) {
                break;
            }
        }

        if valid {
            if let Some(c) = memo.get(&(moving, arrangement[moving])) {
                count += c;
            } else {
                let c = recurse(
                    conditions,
                    counts,
                    arrangement,
                    moving + 1,
                    max_movement,
                    memo,
                );
                memo.insert((moving, arrangement[moving]), c);
                count += c;
            }
        }

        if max_movement == 0 {
            break;
        }
        max_movement -= 1;
    }

    count
}

fn solve_for(input: &str, mode: Mode) -> usize {
    input
        .lines()
        .map(|line| {
            let (conditions, counts) = line.split_once(' ').unwrap();
            let conditions: Vec<Condition> = conditions
                .chars()
                .map(|ch| match ch {
                    '.' => Condition::Operational,
                    '#' => Condition::Damaged,
                    '?' => Condition::Unknown,
                    _ => panic!("unknown condition '{ch}'"),
                })
                .collect();
            let counts: Vec<usize> = counts.split(',').map(|n| n.parse().unwrap()).collect();

            let (conditions, counts) = match mode {
                Mode::PartA => (conditions, counts),
                Mode::PartB => {
                    let mut conditions = conditions;
                    conditions.push(Condition::Unknown);
                    let mut conditions = conditions.repeat(5);
                    conditions.pop();
                    (conditions, counts.repeat(5))
                }
            };

            let mut arrangement = vec![0; counts.len()];
            let max_movement = conditions.len() + 1 - counts.iter().map(|c| c + 1).sum::<usize>();
            recurse(
                &conditions,
                &counts,
                &mut arrangement,
                0,
                max_movement,
                &mut HashMap::new(),
            )
        })
        .sum()
}

#[test]
fn a_example() {
    assert_eq!(solve_for(EXAMPLE, Mode::PartA), 21);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_for(INPUT, Mode::PartA), 7084);
}

#[test]
fn b_example() {
    assert_eq!(solve_for(EXAMPLE, Mode::PartB), 525152);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_for(INPUT, Mode::PartB), 8414003326821);
}

pub fn solve_a() -> PuzzleResult {
    solve_for(INPUT, Mode::PartA).into()
}

pub fn solve_b() -> PuzzleResult {
    solve_for(INPUT, Mode::PartB).into()
}
