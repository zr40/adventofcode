use std::collections::BTreeMap;

use itertools::Itertools;

use crate::day::Day;
use crate::PuzzleResult;

#[cfg(test)]
const EXAMPLE: &str = include_str!("input/9_example");
const INPUT: &str = include_str!("input/9");

enum Mode {
    Min,
    Max,
}

fn solve_for(input: &str, mode: Mode) -> u32 {
    let mut routes: BTreeMap<(usize, usize), u32> = BTreeMap::new();
    let mut locations: BTreeMap<String, usize> = BTreeMap::new();

    for line in input.lines() {
        let mut iter = line.split(' ');
        let src = iter.next().unwrap().to_owned();
        iter.next();
        let dest = iter.next().unwrap().to_owned();
        iter.next();
        let distance = iter.next().unwrap().parse().unwrap();

        let len = locations.len();
        let src = *locations.entry(src).or_insert(len);
        let len = locations.len();
        let dest = *locations.entry(dest).or_insert(len);

        routes.insert((src, dest), distance);
        routes.insert((dest, src), distance);
    }

    let distances = (0..locations.len())
        .permutations(locations.len())
        .map(|route| {
            route
                .into_iter()
                .map_windows(|[a, b]| (*a, *b))
                .fold(0, |acc, (a, b)| acc + routes[&(a, b)])
        });

    match mode {
        Mode::Min => distances.min(),
        Mode::Max => distances.max(),
    }
    .unwrap()
}

#[test]
fn a_example() {
    assert_eq!(solve_for(EXAMPLE, Mode::Min), 605);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_for(INPUT, Mode::Min), 141);
}

#[test]
fn b_example() {
    assert_eq!(solve_for(EXAMPLE, Mode::Max), 982);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_for(INPUT, Mode::Max), 736);
}

fn solve_a() -> PuzzleResult {
    solve_for(INPUT, Mode::Min).into()
}

fn solve_b() -> PuzzleResult {
    solve_for(INPUT, Mode::Max).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
