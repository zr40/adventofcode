use std::collections::HashMap;

use crate::day::Day;
use crate::puzzle_result::PuzzleResult;

#[cfg(test)]
const EXAMPLE: &str = include_str!("input/8_example");
const INPUT: &str = include_str!("input/8");

fn solve_for(input: &str) -> (i32, i32) {
    let mut registers = HashMap::new();
    let mut max = 0;

    for line in input.lines() {
        let mut tokens = line.split_whitespace();

        let register = tokens.next().unwrap();
        let instruction = tokens.next().unwrap();
        let amount: i32 = tokens.next().unwrap().parse().unwrap();
        tokens.next(); // "if"
        let condition_register = tokens.next().unwrap();
        let comparison = tokens.next().unwrap();
        let comparison_value: i32 = tokens.next().unwrap().parse().unwrap();

        let if_register_value = *registers.get(condition_register).unwrap_or(&0);
        if match comparison {
            "!=" => if_register_value != comparison_value,
            "<" => if_register_value < comparison_value,
            "<=" => if_register_value <= comparison_value,
            "==" => if_register_value == comparison_value,
            ">" => if_register_value > comparison_value,
            ">=" => if_register_value >= comparison_value,
            unknown => panic!("Unknown comparison {unknown}"),
        } {
            let register_value = *registers.get(register).unwrap_or(&0);
            let new_value = match instruction {
                "inc" => register_value + amount,
                "dec" => register_value - amount,
                unknown => panic!("Unknown instruction {unknown}"),
            };
            registers.insert(register, new_value);

            max = max.max(new_value);
        }
    }

    (*registers.values().max().unwrap(), max)
}

#[test]
fn example() {
    assert_eq!(solve_for(EXAMPLE), (1, 10));
}

#[test]
fn puzzle() {
    assert_eq!(solve_for(INPUT), (3880, 5035));
}

fn solve_both() -> (PuzzleResult, PuzzleResult) {
    let (a, b) = solve_for(INPUT);
    (a.into(), b.into())
}

pub(super) static DAY: Day = Day::Pair(solve_both);
