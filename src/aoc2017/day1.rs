use crate::day::Day;
use crate::puzzle_result::PuzzleResult;

const INPUT: &str = include_str!("input/1");

fn solve_a_for(input: &str) -> u32 {
    let mut sum = 0;

    let mut iterator = input.chars().peekable();

    while let Some(c) = iterator.next() {
        let next = match iterator.peek() {
            None => input.chars().next().unwrap(),
            Some(x) => *x,
        };

        if c == next {
            sum += c.to_digit(10).unwrap();
        }
    }

    sum
}

fn solve_b_for(input: &str) -> u32 {
    let mut sum = 0;

    let (first, second) = input.split_at(input.len() / 2);

    for (a, b) in first.chars().zip(second.chars()) {
        if a == b {
            sum += a.to_digit(10).unwrap();
        }
    }

    sum * 2
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for("1122"), 3);
    assert_eq!(solve_a_for("1111"), 4);
    assert_eq!(solve_a_for("1234"), 0);
    assert_eq!(solve_a_for("91212129"), 9);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(INPUT), 1102);
}

#[test]
fn b_example() {
    assert_eq!(solve_b_for("1212"), 6);
    assert_eq!(solve_b_for("1221"), 0);
    assert_eq!(solve_b_for("123425"), 4);
    assert_eq!(solve_b_for("123123"), 12);
    assert_eq!(solve_b_for("12131415"), 4);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(INPUT), 1076);
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
