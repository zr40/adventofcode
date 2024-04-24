use crate::day::Day;
use crate::PuzzleResult;

#[cfg(test)]
const EXAMPLE_A: &str = include_str!("input/1a_example");
#[cfg(test)]
const EXAMPLE_B: &str = include_str!("input/1b_example");
const INPUT: &str = include_str!("input/1");

fn solve_a_for(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut digits = line.chars().filter_map(|digit| digit.to_digit(10));
            let first = digits.next().unwrap();
            let last = digits.last().unwrap_or(first);
            first * 10 + last
        })
        .sum()
}

fn solve_b_for(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut digits = (0..line.len()).filter_map(|index| {
                let substr = &line[index..];
                if substr.starts_with("one") {
                    Some(1)
                } else if substr.starts_with("two") {
                    Some(2)
                } else if substr.starts_with("three") {
                    Some(3)
                } else if substr.starts_with("four") {
                    Some(4)
                } else if substr.starts_with("five") {
                    Some(5)
                } else if substr.starts_with("six") {
                    Some(6)
                } else if substr.starts_with("seven") {
                    Some(7)
                } else if substr.starts_with("eight") {
                    Some(8)
                } else if substr.starts_with("nine") {
                    Some(9)
                } else {
                    substr.chars().next().unwrap().to_digit(10)
                }
            });

            let first = digits.next().unwrap();
            let last = digits.last().unwrap_or(first);
            first * 10 + last
        })
        .sum()
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for(EXAMPLE_A), 142);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(INPUT), 53974);
}

#[test]
fn b_example() {
    assert_eq!(solve_b_for(EXAMPLE_B), 281);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(INPUT), 52840);
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
