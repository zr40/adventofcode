use crate::day::Day;
use crate::puzzle_result::PuzzleResult;

const INPUT: &str = include_str!("input/9");

fn solve_for(input: &str) -> (i32, i32) {
    let mut score = 0;
    let mut nesting = 0;
    let mut in_garbage = false;
    let mut next_canceled = false;
    let mut garbage_chars = 0;

    for c in input.bytes() {
        if in_garbage {
            if next_canceled {
                next_canceled = false;
            } else {
                match c {
                    b'!' => next_canceled = true,
                    b'>' => in_garbage = false,
                    _ => garbage_chars += 1,
                }
            }
        } else {
            match c {
                b'{' => {
                    nesting += 1;
                    score += nesting;
                }
                b'}' => nesting -= 1,
                b'<' => in_garbage = true,
                b',' => {}
                _ => panic!("unexpected character"),
            }
        }
    }

    (score, garbage_chars)
}

#[test]
fn a_example() {
    assert_eq!(solve_for("{}").0, 1);
    assert_eq!(solve_for("{{{}}}").0, 6);
    assert_eq!(solve_for("{{},{}}").0, 5);
    assert_eq!(solve_for("{{{},{},{{}}}}").0, 16);
    assert_eq!(solve_for("{<a>,<a>,<a>,<a>}").0, 1);
    assert_eq!(solve_for("{{<ab>},{<ab>},{<ab>},{<ab>}}").0, 9);
    assert_eq!(solve_for("{{<!!>},{<!!>},{<!!>},{<!!>}}").0, 9);
    assert_eq!(solve_for("{{<a!>},{<a!>},{<a!>},{<ab>}}").0, 3);
}

#[test]
fn b_example() {
    assert_eq!(solve_for("<>").1, 0);
    assert_eq!(solve_for("<random characters>").1, 17);
    assert_eq!(solve_for("<<<<>").1, 3);
    assert_eq!(solve_for("<{!>}>").1, 2);
    assert_eq!(solve_for("<!!>").1, 0);
    assert_eq!(solve_for("<!!!>>").1, 0);
    assert_eq!(solve_for("<{o\"i!a,<{i<a>").1, 10);
}

#[test]
fn puzzle() {
    assert_eq!(solve_for(INPUT), (10820, 5547));
}

fn solve_both() -> (PuzzleResult, PuzzleResult) {
    let (a, b) = solve_for(INPUT);
    (a.into(), b.into())
}

pub(super) static DAY: Day = Day::Pair(solve_both);
