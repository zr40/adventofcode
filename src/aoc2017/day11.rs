use crate::day::Day;
use crate::puzzle_result::PuzzleResult;

const INPUT: &str = include_str!("input/11");

fn step(direction: &str, x: i32, y: i32) -> (i32, i32) {
    match direction {
        "n" => (x, y + 2),
        "ne" => (x + 1, y + 1),
        "se" => (x + 1, y - 1),
        "s" => (x, y - 2),
        "sw" => (x - 1, y - 1),
        "nw" => (x - 1, y + 1),
        other => panic!("Unknown direction {other}"),
    }
}

fn solve_a_for(input: &str) -> i32 {
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    for direction in input.split(',') {
        (x, y) = step(direction, x, y);
    }

    x = x.abs();
    y = y.abs();

    if x > y {
        x
    } else {
        x + (y - x) / 2
    }
}

fn solve_b_for(input: &str) -> i32 {
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    input
        .split(',')
        .map(|direction| {
            (x, y) = step(direction, x, y);

            let xa = x.abs();
            let ya = y.abs();

            if xa > ya {
                xa
            } else {
                xa + (ya - xa) / 2
            }
        })
        .max()
        .unwrap()
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for("ne,ne,ne"), 3);
    assert_eq!(solve_a_for("ne,ne,sw,sw"), 0);
    assert_eq!(solve_a_for("ne,ne,s,s"), 2);
    assert_eq!(solve_a_for("se,sw,se,sw,sw"), 3);
}

#[test]
fn test() {
    assert_eq!(solve_a_for("ne,se"), 2);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(INPUT), 761);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(INPUT), 1542);
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
