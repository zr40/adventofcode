use crate::common::aoc::input_for;
use crate::day::Day;
use crate::puzzle_result::PuzzleResult;

#[cfg(test)]
const EXAMPLE: &str = include_str!("example/5");

enum Mode {
    CrateMover9000,
    CrateMover9001,
}

fn solve_for(input: &str, mode: Mode) -> String {
    let mut lines = input.lines();

    let mut stacks: Vec<Vec<char>> = vec![];

    loop {
        let line = lines.next().unwrap();
        let chars: Vec<char> = line.chars().collect();

        if chars[1].is_ascii_digit() {
            lines.next();
            break;
        }

        for i in 0..=chars.len() / 4 {
            if stacks.len() == i {
                stacks.push(vec![]);
            }

            let char = chars[i * 4 + 1];
            if char.is_ascii_uppercase() {
                stacks[i].push(char);
            }
        }
    }

    for stack in &mut stacks {
        stack.reverse();
    }

    for line in lines {
        let mut split = line.split(' ');

        split.next();
        let count = split.next().unwrap().parse::<usize>().unwrap();
        split.next();
        let from = split.next().unwrap().parse::<usize>().unwrap() - 1;
        split.next();
        let to = split.next().unwrap().parse::<usize>().unwrap() - 1;

        match mode {
            Mode::CrateMover9000 => {
                for _ in 0..count {
                    let item = stacks[from].pop().expect("stack is empty");
                    stacks[to].push(item);
                }
            }
            Mode::CrateMover9001 => {
                let index = stacks[from].len() - count;
                let mut substack = stacks[from].split_off(index);
                stacks[to].append(&mut substack);
            }
        }
    }

    stacks.into_iter().map(|mut s| s.pop().unwrap()).collect()
}

#[test]
fn a_example() {
    assert_eq!(solve_for(EXAMPLE, Mode::CrateMover9000), "CMZ");
}

#[test]
fn a_puzzle() {
    assert_eq!(
        solve_for(&input_for(2022, 5), Mode::CrateMover9000),
        "TQRFCBSJJ"
    );
}

fn solve_a() -> PuzzleResult {
    solve_for(&input_for(2022, 5), Mode::CrateMover9000).into()
}

#[test]
fn b_example() {
    assert_eq!(solve_for(EXAMPLE, Mode::CrateMover9001), "MCD");
}

#[test]
fn b_puzzle() {
    assert_eq!(
        solve_for(&input_for(2022, 5), Mode::CrateMover9001),
        "RMHFJNVFP"
    );
}

fn solve_b() -> PuzzleResult {
    solve_for(&input_for(2022, 5), Mode::CrateMover9001).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
