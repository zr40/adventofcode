use crate::PuzzleResult;
use crate::common::aoc::input_for;
use crate::day::Day;

#[cfg(test)]
const EXAMPLE: &str = include_str!("example/9");

fn solve_for(input: &str) -> (i32, i32) {
    input
        .lines()
        .map(|line| {
            let mut histories: Vec<Vec<i32>> =
                vec![line.split(' ').map(|num| num.parse().unwrap()).collect()];

            while !histories.last().unwrap().iter().all(|num| *num == 0) {
                histories.push(
                    histories
                        .last()
                        .unwrap()
                        .iter()
                        .map_windows(|[a, b]| *b - *a)
                        .collect(),
                );
            }

            histories
                .into_iter()
                .rev()
                .fold((0, 0), |(acc_a, acc_b), history| {
                    (acc_a + history.last().unwrap(), history[0] - acc_b)
                })
        })
        .fold((0, 0), |(acc_a, acc_b), (a, b)| (acc_a + a, acc_b + b))
}

#[test]
fn example() {
    assert_eq!(solve_for(EXAMPLE), (114, 2));
}

#[test]
fn puzzle() {
    assert_eq!(solve_for(&input_for(2023, 9)), (1647269739, 864));
}

fn solve_both() -> (PuzzleResult, PuzzleResult) {
    let (a, b) = solve_for(&input_for(2023, 9));
    (a.into(), b.into())
}

pub(super) static DAY: Day = Day::Pair(solve_both);
