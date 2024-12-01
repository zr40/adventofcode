use md5::{Digest, Md5};

use crate::PuzzleResult;
use crate::common::aoc::input_for;
use crate::day::Day;

fn solve_for(input: &str) -> (u32, u32) {
    let mut five_zero = 0;

    for i in 1.. {
        let mut hasher = Md5::new();
        hasher.update(input);
        hasher.update(i.to_string());
        let hash = hasher.finalize();

        if hash[0] == 0 && hash[1] == 0 {
            if five_zero == 0 && hash[2] < 16 {
                five_zero = i;
            }
            if hash[2] == 0 {
                return (five_zero, i);
            }
        }
    }
    unreachable!();
}

#[test]
#[cfg_attr(debug_assertions, ignore)]
fn a_example() {
    assert_eq!(solve_for("abcdef").0, 609043);
    assert_eq!(solve_for("pqrstuv").0, 1048970);
}

#[test]
#[cfg_attr(debug_assertions, ignore)]
fn puzzle() {
    assert_eq!(solve_for(&input_for(2015, 4)), (254575, 1038736));
}

#[cfg(debug_assertions)]
fn solve_both() -> (PuzzleResult, PuzzleResult) {
    (PuzzleResult::SkipSlow, PuzzleResult::SkipSlow)
}

#[cfg(not(debug_assertions))]
fn solve_both() -> (PuzzleResult, PuzzleResult) {
    let (a, b) = solve_for(&input_for(2015, 4));
    (a.into(), b.into())
}

#[cfg(debug_assertions)]
#[allow(dead_code)]
fn dead_code() {
    solve_for(&input_for(2015, 4));
}

pub(super) static DAY: Day = Day::Pair(solve_both);
