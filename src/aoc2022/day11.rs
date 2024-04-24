use std::collections::VecDeque;
use std::str::Lines;

use crate::day::Day;
use crate::puzzle_result::PuzzleResult;

#[cfg(test)]
const EXAMPLE: &str = include_str!("input/11_example");
const INPUT: &str = include_str!("input/11");

enum Operation {
    Plus(u64),
    Times(u64),
    TimesOld,
}

struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    test: u64,
    if_true: usize,
    if_false: usize,
    items_inspected: usize,
}

impl Monkey {
    fn from_lines(lines: &mut Lines<'_>) -> Monkey {
        let items = lines
            .next()
            .unwrap()
            .split_once("Starting items: ")
            .unwrap()
            .1
            .split(", ")
            .map(|item| item.parse().unwrap())
            .collect();

        let operation = match lines
            .next()
            .unwrap()
            .split_once("Operation: new = old ")
            .unwrap()
            .1
            .split_once(' ')
            .unwrap()
        {
            ("+", num) => Operation::Plus(num.parse().unwrap()),
            ("*", "old") => Operation::TimesOld,
            ("*", num) => Operation::Times(num.parse().unwrap()),
            (unknown, _) => panic!("unknown operation {unknown}"),
        };

        let test = lines
            .next()
            .unwrap()
            .split_once("Test: divisible by ")
            .unwrap()
            .1
            .parse()
            .unwrap();

        let if_true = lines
            .next()
            .unwrap()
            .split_once("If true: throw to monkey ")
            .unwrap()
            .1
            .parse()
            .unwrap();

        let if_false = lines
            .next()
            .unwrap()
            .split_once("If false: throw to monkey ")
            .unwrap()
            .1
            .parse()
            .unwrap();

        lines.next();

        Monkey {
            items,
            operation,
            test,
            if_true,
            if_false,
            items_inspected: 0,
        }
    }
}

fn solve_for(input: &str, divide: bool, rounds: usize) -> usize {
    let mut lines = input.lines();

    let mut monkeys = vec![];
    while let Some(_) = lines.next() {
        monkeys.push(Monkey::from_lines(&mut lines));
    }

    let multiple: u64 = monkeys.iter().map(|m| m.test).product();

    for _ in 0..rounds {
        for monkey in 0..monkeys.len() {
            loop {
                let monkey = &mut monkeys[monkey];
                if let Some(mut item) = monkey.items.pop_front() {
                    match monkey.operation {
                        Operation::Plus(n) => {
                            item += n;
                        }
                        Operation::Times(n) => {
                            item *= n;
                        }
                        Operation::TimesOld => {
                            item *= item;
                        }
                    }

                    if divide {
                        item /= 3;
                    }

                    item %= multiple;

                    let target = if item % monkey.test == 0 {
                        monkey.if_true
                    } else {
                        monkey.if_false
                    };

                    monkey.items_inspected += 1;

                    monkeys[target].items.push_back(item);
                } else {
                    break;
                }
            }
        }
    }

    monkeys.sort_unstable_by(|a, b| b.items_inspected.partial_cmp(&a.items_inspected).unwrap());
    monkeys[0].items_inspected * monkeys[1].items_inspected
}

#[test]
fn a_example() {
    assert_eq!(solve_for(EXAMPLE, true, 20), 10605);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_for(INPUT, true, 20), 78960);
}

#[test]
fn b_example() {
    assert_eq!(solve_for(EXAMPLE, false, 10000), 2713310158);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_for(INPUT, false, 10000), 14561971968);
}

fn solve_a() -> PuzzleResult {
    solve_for(INPUT, true, 20).into()
}

fn solve_b() -> PuzzleResult {
    solve_for(INPUT, false, 10000).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
