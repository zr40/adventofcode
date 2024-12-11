use crate::PuzzleResult;
use crate::common::aoc::input_for;
use crate::day::Day;

fn blink(stones: Vec<usize>) -> Vec<usize> {
    let mut new_stones = vec![];

    for stone in stones {
        if stone == 0 {
            new_stones.push(1);
        } else {
            let len = stone.ilog10() + 1;
            if len % 2 == 0 {
                new_stones.push(stone / 10usize.pow(len / 2));
                new_stones.push(stone % 10usize.pow(len / 2));
            } else {
                new_stones.push(stone * 2024);
            }
        }
    }

    new_stones
}

fn solve_for(input: &str, blinks: usize) -> usize {
    let mut stones: Vec<usize> = input.split(' ').map(|i| i.parse().unwrap()).collect();

    for _ in 0..blinks {
        stones = blink(stones);
    }

    stones.len()
}

#[test]
fn a_example() {
    assert_eq!(blink(vec![0, 1, 10, 99, 999]), vec![
        1, 2024, 1, 0, 9, 9, 2021976
    ]);

    let stones = vec![125, 17];

    let stones = blink(stones);
    assert_eq!(stones, vec![253000, 1, 7]);

    let stones = blink(stones);
    assert_eq!(stones, vec![253, 0, 2024, 14168]);

    let stones = blink(stones);
    assert_eq!(stones, vec![512072, 1, 20, 24, 28676032]);

    let stones = blink(stones);
    assert_eq!(stones, vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032]);

    let stones = blink(stones);
    assert_eq!(stones, vec![
        1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32
    ]);

    let stones = blink(stones);
    assert_eq!(stones, vec![
        2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6, 0, 3, 2
    ]);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_for(&input_for(2024, 11), 25), 189167);
}

fn solve_a() -> PuzzleResult {
    solve_for(&input_for(2024, 11), 25).into()
}

#[test]
#[ignore]
fn b_puzzle() {
    assert_eq!(solve_for(&input_for(2024, 11), 75), 0);
}

fn solve_b() -> PuzzleResult {
    PuzzleResult::Todo
    // solve_for(&input_for(2024, 11), 75).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
