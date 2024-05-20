use std::collections::BTreeSet;

use itertools::Itertools;

use crate::day::Day;
use crate::puzzle_result::PuzzleResult;

const INPUT: &str = include_str!("input/7");

fn has_abba(sequence: &str) -> bool {
    for (a, b, c, d) in sequence.chars().tuple_windows() {
        if (a, b) == (d, c) && a != b {
            return true;
        }
    }
    false
}

fn supports_tls(input: &str) -> bool {
    let mut rest = input;
    while rest.contains('[') {
        let (_, inside) = rest.split_once('[').unwrap();
        let (inside, after) = inside.split_once(']').unwrap();

        if has_abba(inside) {
            return false;
        }
        rest = after;
    }

    has_abba(input)
}

fn solve_a_for(input: &str) -> usize {
    input.lines().filter(|line| supports_tls(line)).count()
}

fn supports_ssl(input: &str) -> bool {
    let mut hypernet: bool = false;

    let mut abas = BTreeSet::new();
    let mut babs = BTreeSet::new();

    for (a, b, c) in input.chars().tuple_windows() {
        match (a, b, c) {
            ('[', _, _) => {
                hypernet = true;
                continue;
            }
            (']', _, _) => {
                hypernet = false;
                continue;
            }
            (_, '[' | ']', _) | (_, _, '[' | ']') => continue,
            _ => {}
        }

        if a == c && a != b {
            if hypernet {
                let bab = format!("{b}{a}{b}");
                babs.insert(bab);
            } else {
                let aba = format!("{a}{b}{a}");
                abas.insert(aba);
            }
        }
    }

    !abas.is_disjoint(&babs)
}

fn solve_b_for(input: &str) -> usize {
    input.lines().filter(|line| supports_ssl(line)).count()
}

#[test]
fn a_example() {
    assert!(supports_tls("abba[mnop]qrst"));
    assert!(!supports_tls("abcd[bddb]xyyx"));
    assert!(!supports_tls("aaaa[qwer]tyui"));
    assert!(supports_tls("ioxxoj[asdfgh]zxcvbn"));
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(INPUT), 105);
}

#[test]
fn b_example() {
    assert!(supports_ssl("aba[bab]xyz"));
    assert!(!supports_ssl("xyx[xyx]xyx"));
    assert!(supports_ssl("aaa[kek]eke"));
    assert!(supports_ssl("zazbz[bzb]cdb"));
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(INPUT), 258);
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
