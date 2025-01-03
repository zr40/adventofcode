use std::collections::BTreeSet;

use crate::common::aoc::input_for;
use crate::day::Day;
use crate::puzzle_result::PuzzleResult;

fn solve_for(input: &str, marker_size: usize) -> usize {
    let chars: Vec<char> = input.chars().collect();
    for i in 0..=chars.len() - marker_size {
        let mut set = BTreeSet::new();

        for j in 0..marker_size {
            set.insert(chars[i + j]);
        }

        if set.len() == marker_size {
            return i + marker_size;
        }
    }
    unreachable!()
}

#[test]
fn a_example() {
    assert_eq!(solve_for("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4), 7);
    assert_eq!(solve_for("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
    assert_eq!(solve_for("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
    assert_eq!(solve_for("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
    assert_eq!(solve_for("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_for(&input_for(2022, 6), 4), 1655);
}

fn solve_a() -> PuzzleResult {
    solve_for(&input_for(2022, 6), 4).into()
}

#[test]
fn b_example() {
    assert_eq!(solve_for("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
    assert_eq!(solve_for("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
    assert_eq!(solve_for("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
    assert_eq!(solve_for("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14), 29);
    assert_eq!(solve_for("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), 26);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_for(&input_for(2022, 6), 14), 2665);
}

fn solve_b() -> PuzzleResult {
    solve_for(&input_for(2022, 6), 14).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
