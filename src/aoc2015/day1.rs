use crate::PuzzleResult;
use crate::common::aoc::input_for;
use crate::day::Day;

fn solve_a_for(input: &str) -> i32 {
    input
        .chars()
        .map(|ch| match ch {
            '(' => 1,
            ')' => -1,
            c => panic!("unexpected char {c}"),
        })
        .sum()
}

fn solve_b_for(input: &str) -> u32 {
    let mut level: i32 = 0;
    let mut index = 1;
    for ch in input.chars() {
        level += match ch {
            '(' => 1,
            ')' => -1,
            c => panic!("unexpected char {c}"),
        };
        if level.is_negative() {
            return index;
        }
        index += 1;
    }
    panic!("did not reach basement")
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for("(())"), 0);
    assert_eq!(solve_a_for("()()"), 0);
    assert_eq!(solve_a_for("((("), 3);
    assert_eq!(solve_a_for("(()(()("), 3);
    assert_eq!(solve_a_for("))((((("), 3);
    assert_eq!(solve_a_for("())"), -1);
    assert_eq!(solve_a_for("))("), -1);
    assert_eq!(solve_a_for(")))"), -3);
    assert_eq!(solve_a_for(")())())"), -3);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(&input_for(2015, 1)), 232);
}

fn solve_a() -> PuzzleResult {
    solve_a_for(&input_for(2015, 1)).into()
}

#[test]
fn b_example() {
    assert_eq!(solve_b_for(")"), 1);
    assert_eq!(solve_b_for("()())"), 5);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(&input_for(2015, 1)), 1783);
}

fn solve_b() -> PuzzleResult {
    solve_b_for(&input_for(2015, 1)).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
