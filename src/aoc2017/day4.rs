use std::collections::HashSet;

use crate::common::aoc::input_for;
use crate::day::Day;
use crate::puzzle_result::PuzzleResult;

fn solve_a_for(input: &str) -> usize {
    input
        .lines()
        .filter(|passphrase| {
            let mut words = HashSet::new();

            for word in passphrase.split_whitespace() {
                if words.contains(word) {
                    return false;
                }

                words.insert(word);
            }

            true
        })
        .count()
}

fn solve_b_for(input: &str) -> usize {
    input
        .lines()
        .filter(|passphrase| {
            let mut words: HashSet<String> = HashSet::new();

            for word in passphrase.split_whitespace() {
                let mut chars: Vec<u8> = word.bytes().collect();
                chars.sort_unstable();

                let sorted = String::from_utf8(chars).unwrap();

                if words.contains(&sorted) {
                    return false;
                }

                words.insert(sorted);
            }

            true
        })
        .count()
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for("aa bb cc dd ee"), 1);
    assert_eq!(solve_a_for("aa bb cc dd aa"), 0);
    assert_eq!(solve_a_for("aa bb cc dd aaa"), 1);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(&input_for(2017, 4)), 337);
}

fn solve_a() -> PuzzleResult {
    solve_a_for(&input_for(2017, 4)).into()
}

#[test]
fn b_example() {
    assert_eq!(solve_b_for("abcde fghij"), 1);
    assert_eq!(solve_b_for("abcde xyz ecdab"), 0);
    assert_eq!(solve_b_for("a ab abc abd abf abj"), 1);
    assert_eq!(solve_b_for("iiii oiii ooii oooi oooo"), 1);
    assert_eq!(solve_b_for("oiii ioii iioi iiio"), 0);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(&input_for(2017, 4)), 231);
}

fn solve_b() -> PuzzleResult {
    solve_b_for(&input_for(2017, 4)).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
