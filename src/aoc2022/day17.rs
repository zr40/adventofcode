use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

use crate::puzzle_result::PuzzleResult;

#[cfg(test)]
const EXAMPLE: &str = include_str!("input/17_example");
const INPUT: &str = include_str!("input/17");

fn movement_blocked(
    chamber: &HashSet<(i64, i64)>,
    rock: &Vec<(i64, i64)>,
    candidate_pos: (i64, i64),
) -> bool {
    for (x, y) in rock {
        let pos = (x + candidate_pos.0, y + candidate_pos.1);
        if pos.0 < 0 || pos.0 > 6 || pos.1 < 0 || chamber.contains(&pos) {
            return true;
        }
    }
    false
}

fn solve_for(input: &str, iterations: usize) -> i64 {
    let mut jet_pattern = input
        .bytes()
        .map(|b| match b {
            b'<' => -1,
            b'>' => 1,
            _ => panic!(),
        })
        .enumerate()
        .cycle();
    let rocks = [
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 0), (0, 1), (1, 0), (1, 1)],
    ];
    let mut rocks = rocks.iter().enumerate().cycle();

    let mut highest = -1;
    let mut chamber = HashSet::new();

    let mut highest_at_index = HashMap::new();

    let mut i = 0;
    let mut increase_highest = 0;

    while i < iterations {
        let mut pos = (2, highest + 4);
        let (rock_index, rock) = rocks.next().unwrap();

        loop {
            let (jet_index, jet) = jet_pattern.next().unwrap();
            let candidate_pos = (pos.0 + jet, pos.1);
            if !movement_blocked(&chamber, rock, candidate_pos) {
                pos = candidate_pos;
            }

            let candidate_pos = (pos.0, pos.1 - 1);
            if movement_blocked(&chamber, rock, candidate_pos) {
                for (x, y) in rock {
                    let (x, y) = (pos.0 + x, pos.1 + y);

                    chamber.insert((x, y));
                    highest = highest.max(y);
                }

                if increase_highest == 0 && i > 1000 {
                    if let Entry::Vacant(e) = highest_at_index.entry((rock_index, jet_index)) {
                        e.insert((highest, i));
                    } else {
                        let (highest_there, highest_i) = highest_at_index[&(rock_index, jet_index)];
                        let increase = highest - highest_there;
                        let period = i - highest_i;

                        let remaining = iterations - i;
                        let periods_remaining = remaining / period;

                        increase_highest = periods_remaining * increase as usize;
                        i += periods_remaining * period;
                    }
                }
                break;
            }
            pos = candidate_pos;
        }

        i += 1;
    }

    #[allow(clippy::cast_possible_wrap)]
    return highest + 1 + increase_highest as i64;
}

#[test]
fn a_example() {
    assert_eq!(solve_for(EXAMPLE, 2022), 3068);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_for(INPUT, 2022), 3081);
}

#[test]
fn b_example() {
    assert_eq!(solve_for(EXAMPLE, 1000000000000), 1514285714288);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_for(INPUT, 1000000000000), 1524637681145);
}

pub fn solve_a() -> PuzzleResult {
    solve_for(INPUT, 2022).into()
}

pub fn solve_b() -> PuzzleResult {
    solve_for(INPUT, 1000000000000).into()
}
