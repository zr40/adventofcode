use crate::PuzzleResult;

const INPUT: &str = include_str!("input/2");

fn solve_a_for(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut dims: Vec<u32> = line.split('x').map(|n| n.parse().unwrap()).collect();
            dims.sort();

            (dims[0] * dims[1] + dims[1] * dims[2] + dims[2] * dims[0]) * 2 + dims[0] * dims[1]
        })
        .sum()
}

fn solve_b_for(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut dims: Vec<u32> = line.split('x').map(|n| n.parse().unwrap()).collect();
            dims.sort();

            (dims[0] + dims[1]) * 2 + dims.iter().product::<u32>()
        })
        .sum()
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for("2x3x4"), 58);
    assert_eq!(solve_a_for("1x1x10"), 43);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(INPUT), 1606483);
}

#[test]
fn b_example() {
    assert_eq!(solve_b_for("2x3x4"), 34);
    assert_eq!(solve_b_for("1x1x10"), 14);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(INPUT), 3842356);
}

pub fn solve_a() -> PuzzleResult {
    solve_a_for(INPUT).into()
}

pub fn solve_b() -> PuzzleResult {
    solve_b_for(INPUT).into()
}
