use std::collections::{HashMap, VecDeque};

use crate::day::Day;
use crate::puzzle_result::PuzzleResult;

#[cfg(test)]
const EXAMPLE_A: &str = include_str!("input/18a_example");
#[cfg(test)]
const EXAMPLE_B: &str = include_str!("input/18b_example");
const INPUT: &str = include_str!("input/18");

fn solve_a_for(input: &str) -> i64 {
    let mut registers = HashMap::new();

    let instructions: Vec<Vec<&str>> = input.split('\n').map(|x| x.split(' ').collect()).collect();

    let mut pos = 0;
    let mut snd = 0;

    loop {
        let instruction = &instructions[pos];
        let left = instruction[1]
            .parse()
            .unwrap_or(*registers.get(instruction[1]).unwrap_or(&0));
        let right = if instruction.len() == 2 {
            0
        } else {
            instruction[2]
                .parse()
                .unwrap_or(*registers.get(instruction[2]).unwrap_or(&0))
        };

        match instruction[0] {
            "set" => {
                registers.insert(instruction[1], right);
            }
            "add" => {
                registers.insert(instruction[1], left + right);
            }
            "mul" => {
                registers.insert(instruction[1], left * right);
            }
            "mod" => {
                registers.insert(instruction[1], left % right);
            }
            "snd" => {
                snd = *registers.get(instruction[1]).unwrap_or(&0);
            }
            "rcv" => {
                if *registers.get(instruction[1]).unwrap_or(&0) > 0 {
                    return snd;
                }
            }
            "jgz" =>
            {
                #[allow(clippy::cast_possible_wrap)]
                if left > 0 {
                    pos = (pos as isize + right as isize - 1) as usize;
                }
            }
            x => panic!("Unknown instruction {x}"),
        }

        pos += 1;
    }
}

fn solve_b_for(input: &str) -> u32 {
    let instructions: Vec<Vec<&str>> = input.split('\n').map(|x| x.split(' ').collect()).collect();

    let mut registers_0 = HashMap::new();
    let mut registers_1 = HashMap::new();

    registers_0.insert("p", 0);
    registers_1.insert("p", 1);

    let mut queue_0 = VecDeque::new();
    let mut queue_1 = VecDeque::new();

    let mut pos_0 = 0;
    let mut pos_1 = 0;

    let mut current_execution_0 = true;

    let mut blocked_0 = false;
    let mut blocked_1 = false;

    let mut send_count = 0;

    loop {
        let current_registers;
        let current_queue;
        let other_queue;
        let current_pos;
        let current_blocked;
        let other_blocked;

        if current_execution_0 {
            current_registers = &mut registers_0;
            current_queue = &mut queue_0;
            other_queue = &mut queue_1;
            current_pos = &mut pos_0;
            current_blocked = &mut blocked_0;
            other_blocked = &mut blocked_1;
        } else {
            current_registers = &mut registers_1;
            current_queue = &mut queue_1;
            other_queue = &mut queue_0;
            current_pos = &mut pos_1;
            current_blocked = &mut blocked_1;
            other_blocked = &mut blocked_0;
        }

        let instruction = &instructions[*current_pos];
        let left: i64 = instruction[1]
            .parse()
            .unwrap_or(*current_registers.get(instruction[1]).unwrap_or(&0));
        let right: i64 = if instruction.len() == 2 {
            0
        } else {
            instruction[2]
                .parse()
                .unwrap_or(*current_registers.get(instruction[2]).unwrap_or(&0))
        };

        match instruction[0] {
            "set" => {
                current_registers.insert(instruction[1], right);
            }
            "add" => {
                current_registers.insert(instruction[1], left + right);
            }
            "mul" => {
                current_registers.insert(instruction[1], left * right);
            }
            "mod" => {
                current_registers.insert(instruction[1], left % right);
            }
            "snd" => {
                other_queue.push_back(left);
                *other_blocked = false;
                if !current_execution_0 {
                    send_count += 1;
                }
            }
            "rcv" => {
                if let Some(x) = current_queue.pop_front() {
                    current_registers.insert(instruction[1], x);
                } else {
                    if *other_blocked {
                        break;
                    }

                    *current_blocked = true;
                    current_execution_0 = !current_execution_0;
                    continue;
                };
            }
            "jgz" =>
            {
                #[allow(clippy::cast_possible_wrap)]
                if left > 0 {
                    *current_pos = (*current_pos as isize + right as isize - 1) as usize;
                }
            }
            x => panic!("Unknown instruction {x}"),
        }

        *current_pos += 1;
    }

    send_count
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for(EXAMPLE_A), 4);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(INPUT), 8600);
}

#[test]
fn b_example() {
    assert_eq!(solve_b_for(EXAMPLE_B), 3);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(INPUT), 7239);
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
