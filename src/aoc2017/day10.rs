use super::knot_hash::{knot_hash, knot_rounds};
use crate::day::Day;
use crate::puzzle_result::PuzzleResult;

const INPUT: &str = include_str!("input/10");

fn solve_a_for(input: &str) -> usize {
    solve_a_size(256, input)
}

fn solve_a_size(list_size: usize, input: &str) -> usize {
    let input: Vec<usize> = input
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let list = knot_rounds(input, list_size, 1);

    list[0] * list[1]
}

fn solve_b_for(input: &str) -> String {
    let list = knot_hash(input);

    let mut output = String::new();
    let mut list_iter = list.iter();

    for _ in 0..16 {
        let mut xor = 0;

        for _ in 0..16 {
            xor ^= list_iter.next().unwrap();
        }

        output.push_str(&format!("{xor:02x}"));
    }

    output
}

#[test]
fn a_example() {
    assert_eq!(solve_a_size(5, "3,4,1,5"), 12);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(INPUT), 23715);
}

#[test]
fn b_example() {
    assert_eq!(solve_b_for(""), "a2582a3a0e66e6e86e3812dcb672a272");
    assert_eq!(solve_b_for("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
    assert_eq!(solve_b_for("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
    assert_eq!(solve_b_for("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(INPUT), "541dc3180fd4b72881e39cf925a50253");
}

fn solve_a() -> PuzzleResult {
    solve_a_for(INPUT).into()
}

fn solve_b() -> PuzzleResult {
    PuzzleResult::Multiline(solve_b_for(INPUT))
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
