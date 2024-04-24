use std::collections::{BTreeMap, BTreeSet};

use itertools::Itertools;

use crate::day::Day;
use crate::PuzzleResult;

#[cfg(test)]
const EXAMPLE: &str = include_str!("input/13_example");
const INPUT: &str = include_str!("input/13");

enum Mode {
    IgnoreSelf,
    AddSelf,
}

fn solve_for(input: &str, mode: Mode) -> i32 {
    let mut people = BTreeSet::new();
    let mut happiness = BTreeMap::new();

    for line in input.lines() {
        let (name, rest) = line.split_once(" would ").unwrap();
        let (gain_lose, rest) = rest.split_once(' ').unwrap();
        let (amount, rest) = rest
            .split_once(" happiness units by sitting next to ")
            .unwrap();
        let (other, _) = rest.split_once('.').unwrap();

        let amount: i32 = amount.parse().unwrap();
        let amount = match gain_lose {
            "gain" => amount,
            "lose" => -amount,
            _ => panic!(),
        };

        people.insert(name);
        happiness
            .entry((name, other))
            .and_modify(|e| *e += amount)
            .or_insert(amount);
        happiness
            .entry((other, name))
            .and_modify(|e| *e += amount)
            .or_insert(amount);
    }

    match mode {
        Mode::IgnoreSelf => {}
        Mode::AddSelf => {
            for person in &people {
                happiness.insert((person, ""), 0);
                happiness.insert(("", person), 0);
            }
            people.insert("");
        }
    }

    let len = people.len();
    people
        .into_iter()
        .permutations(len)
        .map(|permutation| {
            permutation
                .windows(2)
                .map(|pair| happiness[&(pair[0], pair[1])])
                .sum::<i32>()
                + happiness[&(*permutation.first().unwrap(), *permutation.last().unwrap())]
        })
        .max()
        .unwrap()
}

#[test]
fn a_example() {
    assert_eq!(solve_for(EXAMPLE, Mode::IgnoreSelf), 330);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_for(INPUT, Mode::IgnoreSelf), 618);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_for(INPUT, Mode::AddSelf), 601);
}

fn solve_a() -> PuzzleResult {
    solve_for(INPUT, Mode::IgnoreSelf).into()
}

fn solve_b() -> PuzzleResult {
    solve_for(INPUT, Mode::AddSelf).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
