use crate::day::Day;
use crate::puzzle_result::PuzzleResult;

#[cfg(test)]
const EXAMPLE: &str = include_str!("input/2_example");
const INPUT: &str = include_str!("input/2");

const KEYPAD_A: &[&[char]] = &[
    &[' ', ' ', ' ', ' ', ' '],
    &[' ', '1', '2', '3', ' '],
    &[' ', '4', '5', '6', ' '],
    &[' ', '7', '8', '9', ' '],
    &[' ', ' ', ' ', ' ', ' '],
];

const KEYPAD_B: &[&[char]] = &[
    &[' ', ' ', ' ', ' ', ' ', ' ', ' '],
    &[' ', ' ', ' ', '1', ' ', ' ', ' '],
    &[' ', ' ', '2', '3', '4', ' ', ' '],
    &[' ', '5', '6', '7', '8', '9', ' '],
    &[' ', ' ', 'A', 'B', 'C', ' ', ' '],
    &[' ', ' ', ' ', 'D', ' ', ' ', ' '],
    &[' ', ' ', ' ', ' ', ' ', ' ', ' '],
];

fn solve_for(input: &str, layout: &[&[char]]) -> String {
    let mut code = String::new();

    let (mut x, mut y) = layout
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, col)| match col {
                '5' => Some((x, y)),
                _ => None,
            })
        })
        .unwrap();

    for line in input.lines() {
        for ch in line.chars() {
            let (new_x, new_y) = match ch {
                'U' => (x, y - 1),
                'D' => (x, y + 1),
                'L' => (x - 1, y),
                'R' => (x + 1, y),
                _ => panic!("unknown direction {ch}"),
            };

            if layout[new_y][new_x] != ' ' {
                x = new_x;
                y = new_y;
            }
        }
        code.push(layout[y][x]);
    }

    code
}

#[test]
fn a_example() {
    assert_eq!(solve_for(EXAMPLE, KEYPAD_A), "1985");
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_for(INPUT, KEYPAD_A), "73597");
}

#[test]
fn b_example() {
    assert_eq!(solve_for(EXAMPLE, KEYPAD_B), "5DB3");
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_for(INPUT, KEYPAD_B), "A47DA");
}

fn solve_a() -> PuzzleResult {
    solve_for(INPUT, KEYPAD_A).into()
}

fn solve_b() -> PuzzleResult {
    solve_for(INPUT, KEYPAD_B).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
