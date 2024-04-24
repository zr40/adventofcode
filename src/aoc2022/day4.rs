use crate::day::Day;
use crate::puzzle_result::PuzzleResult;

#[cfg(test)]
const EXAMPLE: &str = include_str!("input/4_example");
const INPUT: &str = include_str!("input/4");

struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn from(input: &str) -> Range {
        let (start, end) = input.split_once('-').unwrap();
        Range {
            start: start.parse().unwrap(),
            end: end.parse().unwrap(),
        }
    }

    fn contains(&self, other: &Range) -> bool {
        self.start <= other.start
            && other.start <= self.end
            && self.start <= other.end
            && other.end <= self.end
    }

    fn overlaps(&self, other: &Range) -> bool {
        (self.start <= other.start && other.start <= self.end)
            || (self.start <= other.end && other.end <= self.end)
    }
}

fn solve_a_for(input: &str) -> u32 {
    let mut contained_ranges = 0;

    for line in input.lines() {
        let (left, right) = line.split_once(',').unwrap();
        let left = Range::from(left);
        let right = Range::from(right);

        if left.contains(&right) || right.contains(&left) {
            contained_ranges += 1;
        }
    }
    contained_ranges
}

fn solve_b_for(input: &str) -> u32 {
    let mut overlapping_ranges = 0;

    for line in input.lines() {
        let (left, right) = line.split_once(',').unwrap();
        let left = Range::from(left);
        let right = Range::from(right);

        if left.overlaps(&right) || right.overlaps(&left) {
            overlapping_ranges += 1;
        }
    }
    overlapping_ranges
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for(EXAMPLE), 2);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(INPUT), 441);
}

#[test]
fn b_example() {
    assert_eq!(solve_b_for(EXAMPLE), 4);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(INPUT), 861);
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
