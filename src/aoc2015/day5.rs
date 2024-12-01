use crate::PuzzleResult;
use crate::common::aoc::input_for;
use crate::day::Day;

fn solve_a_for(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            if line
                .chars()
                .filter(|ch| *ch == 'a' || *ch == 'e' || *ch == 'i' || *ch == 'o' || *ch == 'u')
                .count()
                < 3
            {
                return false;
            }
            if !line.chars().map_windows(|[a, b]| a == b).any(|t| t) {
                return false;
            }

            !(line.contains("ab")
                || line.contains("cd")
                || line.contains("pq")
                || line.contains("xy"))
        })
        .count()
}

fn solve_b_for(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            if !(0..line.len() - 1).any(|index| line[index + 2..].contains(&line[index..index + 2]))
            {
                return false;
            }

            let line = line.as_bytes();
            (0..line.len() - 2).any(|index| line[index] == line[index + 2])
        })
        .count()
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for("ugknbfddgicrmopn"), 1);
    assert_eq!(solve_a_for("aaa"), 1);
    assert_eq!(solve_a_for("jchzalrnumimnmhp"), 0);
    assert_eq!(solve_a_for("haegwjzuvuyypxyu"), 0);
    assert_eq!(solve_a_for("dvszwmarrgswjxmb"), 0);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(&input_for(2015, 5)), 236);
}

fn solve_a() -> PuzzleResult {
    solve_a_for(&input_for(2015, 5)).into()
}

#[test]
fn b_example() {
    assert_eq!(solve_b_for("qjhvhtzxzqqjkmpb"), 1);
    assert_eq!(solve_b_for("xxyxx"), 1);
    assert_eq!(solve_b_for("uurcxstgmygtbstg"), 0);
    assert_eq!(solve_b_for("ieodomkazucvgmuy"), 0);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(&input_for(2015, 5)), 51);
}

fn solve_b() -> PuzzleResult {
    solve_b_for(&input_for(2015, 5)).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
