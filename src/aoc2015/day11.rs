use itertools::Itertools;

use crate::day::Day;
use crate::PuzzleResult;

const INPUT: &str = "vzbxkghb";

fn increment(password: &mut [u8]) {
    for i in 0..password.len() {
        if password[i] == b'i' || password[i] == b'o' || password[i] == b'l' {
            password[i] += 1;

            for ch in password.iter_mut().skip(i + 1) {
                *ch = b'a';
            }
            return;
        }
    }

    for ch in password.iter_mut().rev() {
        if *ch == b'z' {
            *ch = b'a';
            continue;
        } else if *ch == b'i' - 1 || *ch == b'o' - 1 || *ch == b'l' - 1 {
            *ch += 2;
        } else {
            *ch += 1;
        }
        return;
    }
}

fn contains_straight(password: &[u8]) -> bool {
    password
        .windows(3)
        .any(|w| w[0] == w[1] - 1 && w[0] == w[2] - 2)
}

fn contains_no_forbidden_letters(password: &[u8]) -> bool {
    password
        .iter()
        .all(|p| *p != b'i' && *p != b'o' && *p != b'l')
}

fn contains_two_pairs(password: &[u8]) -> bool {
    password
        .windows(2)
        .filter_map(|pair| {
            if pair[0] == pair[1] {
                Some(pair[0])
            } else {
                None
            }
        })
        .unique()
        .count()
        >= 2
}

fn is_valid(password: &[u8]) -> bool {
    contains_straight(password)
        && contains_no_forbidden_letters(password)
        && contains_two_pairs(password)
}
fn solve_for(input: &str) -> (String, String) {
    let mut password = input.as_bytes().to_vec();

    increment(&mut password);

    while !is_valid(&password) {
        increment(&mut password);
    }

    let a = String::from_utf8(password.clone()).unwrap();

    increment(&mut password);

    while !is_valid(&password) {
        increment(&mut password);
    }

    let b = String::from_utf8(password).unwrap();

    (a, b)
}

#[test]
fn example() {
    assert_eq!(solve_for("abcdefgh").0, "abcdffaa");
    assert_eq!(solve_for("ghijklmn").0, "ghjaabcc");
}

#[test]
fn puzzle() {
    let (a, b) = solve_for(INPUT);
    assert_eq!(a, "vzbxxyzz");
    assert_eq!(b, "vzcaabcc");
}

fn solve_both() -> (PuzzleResult, PuzzleResult) {
    let (a, b) = solve_for(INPUT);

    (a.into(), b.into())
}

pub(super) static DAY: Day = Day::Pair(solve_both);
