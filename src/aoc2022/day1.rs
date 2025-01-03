use crate::common::aoc::input_for;
use crate::day::Day;
use crate::puzzle_result::PuzzleResult;

#[cfg(test)]
const EXAMPLE: &str = include_str!("example/1");

fn solve_a_for(input: &str) -> u32 {
    let mut max_calories_so_far = 0;
    let mut calories = 0;

    for line in input.lines() {
        if line.is_empty() {
            if calories > max_calories_so_far {
                max_calories_so_far = calories;
            }
            calories = 0;
        } else {
            calories += line.parse::<u32>().unwrap();
        }
    }

    if calories > max_calories_so_far {
        max_calories_so_far = calories;
    }

    max_calories_so_far
}

fn solve_b_for(input: &str) -> u32 {
    let mut elf_calories: Vec<u32> = vec![];
    let mut calories = 0u32;

    for line in input.lines() {
        if line.is_empty() {
            elf_calories.push(calories);
            calories = 0;
        } else {
            calories += line.parse::<u32>().unwrap();
        }
    }

    elf_calories.push(calories);

    elf_calories.sort_unstable();
    elf_calories.into_iter().rev().take(3).sum()
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for(EXAMPLE), 24000);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(&input_for(2022, 1)), 69501);
}

fn solve_a() -> PuzzleResult {
    solve_a_for(&input_for(2022, 1)).into()
}

#[test]
fn b_example() {
    assert_eq!(solve_b_for(EXAMPLE), 45000);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(&input_for(2022, 1)), 202346);
}

fn solve_b() -> PuzzleResult {
    solve_b_for(&input_for(2022, 1)).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
