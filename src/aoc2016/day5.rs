use md5::{Digest, Md5};

use crate::common::aoc::input_for;
use crate::day::Day;
use crate::puzzle_result::PuzzleResult;

#[cfg(test)]
const EXAMPLE: &str = "abc";

fn solve_a_for(input: &str) -> String {
    let door_id = input;

    let mut password = String::new();
    let mut index = 0;

    for _ in 0..8 {
        loop {
            let mut hasher = Md5::new();
            hasher.update(door_id);
            hasher.update(index.to_string());
            let hash = hasher.finalize();

            index += 1;

            if hash[0] == 0 && hash[1] == 0 && hash[2] < 16 {
                password.push(char::from_digit(hash[2] as u32, 16).unwrap());
                break;
            }
        }
    }

    password
}

fn solve_b_for(input: &str) -> String {
    let door_id = input;

    let mut password = [None, None, None, None, None, None, None, None];
    let mut index = 0;

    for _ in 0..8 {
        loop {
            let mut hasher = Md5::new();
            hasher.update(door_id);
            hasher.update(index.to_string());
            let hash = hasher.finalize();

            index += 1;

            if hash[0] == 0 && hash[1] == 0 && hash[2] < 16 {
                let index = hash[2] as usize;
                if index > 7 {
                    continue;
                }
                if password[index].is_none() {
                    password[index] = char::from_digit((hash[3] / 16) as u32, 16);
                } else {
                    continue;
                }
                break;
            }
        }
    }

    password.iter().map(|c| c.unwrap()).collect()
}

#[test]
#[cfg_attr(debug_assertions, ignore)]
fn a_example() {
    assert_eq!(solve_a_for(EXAMPLE), "18f47a30");
}

#[test]
#[cfg_attr(debug_assertions, ignore)]
fn a_puzzle() {
    assert_eq!(solve_a_for(&input_for(2016, 5)), "801b56a7");
}

#[cfg(debug_assertions)]
fn solve_a() -> PuzzleResult {
    PuzzleResult::SkipSlow
}

#[cfg(not(debug_assertions))]
fn solve_a() -> PuzzleResult {
    solve_a_for(&input_for(2016, 5)).into()
}

#[test]
#[cfg_attr(debug_assertions, ignore)]
fn b_example() {
    assert_eq!(solve_b_for(EXAMPLE), "05ace8e3");
}

#[test]
#[cfg_attr(debug_assertions, ignore)]
fn b_puzzle() {
    assert_eq!(solve_b_for(&input_for(2016, 5)), "424a0197");
}
#[cfg(debug_assertions)]
fn solve_b() -> PuzzleResult {
    PuzzleResult::SkipSlow
}

#[cfg(not(debug_assertions))]
fn solve_b() -> PuzzleResult {
    solve_b_for(&input_for(2016, 5)).into()
}

#[cfg(debug_assertions)]
#[allow(dead_code)]
fn dead_code() {
    solve_a_for(&input_for(2016, 5));
    solve_b_for(&input_for(2016, 5));
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
