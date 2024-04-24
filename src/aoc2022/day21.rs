use std::collections::HashMap;

use crate::day::Day;
use crate::puzzle_result::PuzzleResult;

#[cfg(test)]
const EXAMPLE: &str = include_str!("input/21_example");
const INPUT: &str = include_str!("input/21");

enum Mode {
    PartA,
    PartB,
}

enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equals,
}

enum Monkey<'a> {
    Constant(i64),
    Operation(&'a str, Operation, &'a str),
    Human,
}

fn resolve(monkeys: &HashMap<&str, Monkey<'_>>, monkey: &str) -> Option<i64> {
    match &monkeys[monkey] {
        Monkey::Constant(c) => Some(*c),
        Monkey::Operation(ml, Operation::Add, mr) => {
            resolve(monkeys, ml).and_then(|l| resolve(monkeys, mr).map(|r| l + r))
        }
        Monkey::Operation(ml, Operation::Subtract, mr) => {
            resolve(monkeys, ml).and_then(|l| resolve(monkeys, mr).map(|r| l - r))
        }
        Monkey::Operation(ml, Operation::Multiply, mr) => {
            resolve(monkeys, ml).and_then(|l| resolve(monkeys, mr).map(|r| l * r))
        }
        Monkey::Operation(ml, Operation::Divide, mr) => {
            resolve(monkeys, ml).and_then(|l| resolve(monkeys, mr).map(|r| l / r))
        }
        Monkey::Operation(ml, Operation::Equals, mr) => {
            let l = resolve(monkeys, ml);
            let r = resolve(monkeys, mr);

            match (l, r) {
                (Some(_), Some(_)) => panic!("both sides are known"),
                (None, None) => panic!("both sides are unknown"),
                (Some(l), None) => Some(backtrace(monkeys, mr, l)),
                (None, Some(r)) => Some(backtrace(monkeys, ml, r)),
            }
        }
        Monkey::Human => None,
    }
}

fn backtrace(monkeys: &HashMap<&str, Monkey<'_>>, monkey: &str, expected: i64) -> i64 {
    match &monkeys[monkey] {
        Monkey::Constant(_) => panic!("backtrace to constant"),
        Monkey::Human => expected,
        Monkey::Operation(ml, op, mr) => {
            let l = resolve(monkeys, ml);
            let r = resolve(monkeys, mr);

            match (l, op, r) {
                (Some(_), _, Some(_)) => panic!("both sides are known"),
                (None, _, None) => panic!("both sides are unknown"),

                (Some(l), Operation::Add, None) => backtrace(monkeys, mr, expected - l),
                (None, Operation::Add, Some(r)) => backtrace(monkeys, ml, expected - r),

                (Some(l), Operation::Subtract, None) => backtrace(monkeys, mr, l - expected),
                (None, Operation::Subtract, Some(r)) => backtrace(monkeys, ml, r + expected),

                (Some(l), Operation::Divide, None) => backtrace(monkeys, mr, l / expected),
                (None, Operation::Divide, Some(r)) => backtrace(monkeys, ml, r * expected),

                (Some(l), Operation::Multiply, None) => backtrace(monkeys, mr, expected / l),
                (None, Operation::Multiply, Some(r)) => backtrace(monkeys, ml, expected / r),

                (_, Operation::Equals, _) => unreachable!(),
            }
        }
    }
}

fn solve_for(input: &str, mode: Mode) -> i64 {
    let mut monkeys = HashMap::new();

    for line in input.lines() {
        let (name, line) = line.split_once(": ").unwrap();
        let parts: Vec<&str> = line.split(' ').collect();

        monkeys.insert(
            name,
            match parts.len() {
                1 => Monkey::Constant(parts[0].parse().unwrap()),
                3 => Monkey::Operation(
                    parts[0],
                    match parts[1] {
                        "+" => Operation::Add,
                        "-" => Operation::Subtract,
                        "*" => Operation::Multiply,
                        "/" => Operation::Divide,
                        _ => panic!(),
                    },
                    parts[2],
                ),
                _ => panic!(),
            },
        );
    }

    match mode {
        Mode::PartA => {}
        Mode::PartB => {
            monkeys.insert("humn", Monkey::Human);

            let Monkey::Operation(l, _, r) = monkeys.remove("root").unwrap() else {
                panic!()
            };
            monkeys.insert("root", Monkey::Operation(l, Operation::Equals, r));
        }
    }

    resolve(&monkeys, "root").unwrap()
}

#[test]
fn a_example() {
    assert_eq!(solve_for(EXAMPLE, Mode::PartA), 152);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_for(INPUT, Mode::PartA), 54703080378102);
}

#[test]
fn b_example() {
    assert_eq!(solve_for(EXAMPLE, Mode::PartB), 301);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_for(INPUT, Mode::PartB), 3952673930912);
}

fn solve_a() -> PuzzleResult {
    solve_for(INPUT, Mode::PartA).into()
}

fn solve_b() -> PuzzleResult {
    solve_for(INPUT, Mode::PartB).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
