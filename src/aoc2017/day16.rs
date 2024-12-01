use std::collections::{HashMap, VecDeque};

use crate::common::aoc::input_for;
use crate::day::Day;
use crate::puzzle_result::PuzzleResult;

fn solve_a_for(len: usize, input: &str) -> String {
    let mut programs = vec![
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p",
    ];
    programs.truncate(len);

    for dance_move in input.split(',') {
        let (the_move, rest) = dance_move.split_at(1);
        match the_move {
            "s" => {
                for _ in 0..(rest.parse().unwrap()) {
                    let p = programs.pop().unwrap();
                    programs.insert(0, p);
                }
            }
            "x" => {
                let mut iter = rest.split('/');
                let a = iter.next().unwrap().parse().unwrap();
                let b = iter.next().unwrap().parse().unwrap();

                programs.swap(a, b);
            }
            "p" => {
                let mut iter = rest.split('/');
                let a = iter.next().unwrap();
                let b = iter.next().unwrap();

                let a = programs.iter().position(|x| *x == a).unwrap();
                let b = programs.iter().position(|x| *x == b).unwrap();
                programs.swap(a, b);
            }
            x => panic!("Unknown move {x}"),
        }
    }

    programs.concat()
}

enum DanceMove {
    Spin(usize),
    Exchange(usize, usize),
    Partner(usize, usize),
}

fn solve_b_for(len: usize, iterations: usize, input: &str) -> String {
    let program_names = vec![
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p",
    ];
    let mut program_numbers = HashMap::new();
    let mut programs = VecDeque::new();

    let mut formations_seen = HashMap::new();

    for (index, name) in program_names.iter().enumerate() {
        if index == len {
            break;
        }

        program_numbers.insert(*name, index);
        programs.push_back(index);
    }

    let input: Vec<DanceMove> = input
        .split(',')
        .map(|dance_move| {
            let (the_move, rest) = dance_move.split_at(1);

            match the_move {
                "s" => DanceMove::Spin(rest.parse().unwrap()),
                "x" => {
                    let mut iter = rest.split('/');
                    let a = iter.next().unwrap().parse().unwrap();
                    let b = iter.next().unwrap().parse().unwrap();

                    DanceMove::Exchange(a, b)
                }
                "p" => {
                    let mut iter = rest.split('/');
                    let a = iter.next().unwrap();
                    let b = iter.next().unwrap();

                    let a = program_numbers[a];
                    let b = program_numbers[b];

                    DanceMove::Partner(a, b)
                }
                x => panic!("Unknown move {x}"),
            }
        })
        .collect();

    for iteration in 0..iterations {
        for dance_move in &input {
            match *dance_move {
                DanceMove::Spin(size) => {
                    for _ in 0..size {
                        let p = programs.pop_back().unwrap();
                        programs.push_front(p);
                    }
                }
                DanceMove::Exchange(a, b) => {
                    programs.swap(a, b);
                }
                DanceMove::Partner(a, b) => {
                    let a = programs.iter().position(|x| *x == a).unwrap();
                    let b = programs.iter().position(|x| *x == b).unwrap();
                    programs.swap(a, b);
                }
            }
        }

        if let Some(x) = formations_seen.get(&programs) {
            if (iterations - iteration - 1) % (iteration - x) == 0 {
                break;
            }
        }

        formations_seen.insert(programs.clone(), iteration);
    }

    let mut output = String::new();
    for program in programs {
        output.push_str(program_names[program]);
    }

    output
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for(5, "s1"), "eabcd");
    assert_eq!(solve_a_for(5, "s1,x3/4"), "eabdc");
    assert_eq!(solve_a_for(5, "s1,x3/4,pe/b"), "baedc");
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(16, &input_for(2017, 16)), "doeaimlbnpjchfkg");
}

fn solve_a() -> PuzzleResult {
    solve_a_for(16, &input_for(2017, 16)).into()
}

#[test]
fn b_example() {
    assert_eq!(solve_b_for(5, 2, "s1,x3/4,pe/b"), "ceadb");
}

#[test]
fn b_puzzle() {
    assert_eq!(
        solve_b_for(16, 1_000_000_000, &input_for(2017, 16)),
        "agndefjhibklmocp"
    );
}

fn solve_b() -> PuzzleResult {
    solve_b_for(16, 1_000_000_000, &input_for(2017, 16)).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
