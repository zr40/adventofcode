use std::collections::HashMap;

use crate::PuzzleResult;
use crate::common::aoc::input_for;
use crate::day::Day;

fn stones_after_blinks(
    stone: usize,
    blinks: usize,
    memo: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if let Some(answer) = memo.get(&(stone, blinks)) {
        return *answer;
    }

    let answer = if blinks == 0 {
        1
    } else if stone == 0 {
        stones_after_blinks(1, blinks - 1, memo)
    } else {
        let len = stone.ilog10() + 1;
        if len % 2 == 0 {
            let left = stone / 10usize.pow(len / 2);
            let right = stone % 10usize.pow(len / 2);

            stones_after_blinks(left, blinks - 1, memo)
                + stones_after_blinks(right, blinks - 1, memo)
        } else {
            stones_after_blinks(stone * 2024, blinks - 1, memo)
        }
    };

    memo.insert((stone, blinks), answer);

    answer
}

fn solve_for(input: &str, blinks: usize) -> usize {
    let mut memo = HashMap::new();
    input
        .split(' ')
        .map(|i| stones_after_blinks(i.parse().unwrap(), blinks, &mut memo))
        .sum()
}

#[test]
fn a_example() {
    assert_eq!(solve_for("125 17", 25), 55312);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_for(&input_for(2024, 11), 25), 189167);
}

fn solve_a() -> PuzzleResult {
    solve_for(&input_for(2024, 11), 25).into()
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_for(&input_for(2024, 11), 75), 225253278506288);
}

fn solve_b() -> PuzzleResult {
    solve_for(&input_for(2024, 11), 75).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
