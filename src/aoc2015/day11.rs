use itertools::Itertools;

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
fn solve_for(input: &str) -> String {
    let mut password = input.as_bytes().to_vec();

    increment(&mut password);

    while !is_valid(&password) {
        increment(&mut password);
    }

    String::from_utf8(password).unwrap()
}

#[test]
fn example() {
    assert_eq!(solve_for("abcdefgh"), "abcdffaa");
    assert_eq!(solve_for("ghijklmn"), "ghjaabcc");
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_for(INPUT), "vzbxxyzz");
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_for(&solve_for(INPUT)), "vzcaabcc");
}

pub fn solve_a() -> PuzzleResult {
    solve_for(INPUT).into()
}

pub fn solve_b() -> PuzzleResult {
    solve_for(&solve_for(INPUT)).into()
}
