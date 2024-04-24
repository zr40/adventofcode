use std::collections::VecDeque;

use crate::day::Day;
use crate::puzzle_result::PuzzleResult;

#[cfg(test)]
const EXAMPLE: &str = include_str!("input/20_example");
const INPUT: &str = include_str!("input/20");

enum Mode {
    PartOne,
    PartTwo,
}

struct Number {
    number: i64,
    original_order: usize,
}

fn solve_for(input: &str, mode: Mode) -> i64 {
    let mut numbers = VecDeque::new();

    for (index, number) in input.lines().enumerate() {
        numbers.push_back(Number {
            number: number.parse::<i64>().unwrap()
                * match mode {
                    Mode::PartOne => 1,
                    Mode::PartTwo => 811589153,
                },
            original_order: index,
        });
    }

    let size = numbers.len();

    for _ in 0..match mode {
        Mode::PartOne => 1,
        Mode::PartTwo => 10,
    } {
        for index in 0..size {
            for current_index in 0..size {
                #[allow(clippy::cast_possible_wrap)]
                if numbers[current_index].original_order == index {
                    let item = numbers.remove(current_index).unwrap();
                    let mut new_index = current_index as i64 + item.number;

                    new_index %= size as i64 - 1;
                    if new_index < 0 {
                        new_index += size as i64 - 1;
                    } else if new_index >= size as i64 {
                        new_index -= size as i64 - 1;
                    }

                    numbers.insert(new_index as usize, item);

                    break;
                }
            }
        }
    }

    for zero_index in 0..size {
        if numbers[zero_index].number == 0 {
            return numbers[(zero_index + 1000) % size].number
                + numbers[(zero_index + 2000) % size].number
                + numbers[(zero_index + 3000) % size].number;
        }
    }

    panic!("0 not found");
}

#[test]
fn a_example() {
    assert_eq!(solve_for(EXAMPLE, Mode::PartOne), 3);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_for(INPUT, Mode::PartOne), 4224);
}

#[test]
fn b_example() {
    assert_eq!(solve_for(EXAMPLE, Mode::PartTwo), 1623178306);
}

#[test]
#[cfg_attr(debug_assertions, ignore)]
fn b_puzzle() {
    assert_eq!(solve_for(INPUT, Mode::PartTwo), 861907680486);
}

fn solve_a() -> PuzzleResult {
    solve_for(INPUT, Mode::PartOne).into()
}

#[cfg(debug_assertions)]
fn solve_b() -> PuzzleResult {
    PuzzleResult::SkipSlow
}

#[cfg(not(debug_assertions))]
fn solve_b() -> PuzzleResult {
    solve_for(INPUT, Mode::PartTwo).into()
}

#[cfg(debug_assertions)]
#[allow(dead_code)]
fn dead_code() {
    solve_for(INPUT, Mode::PartTwo);
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
