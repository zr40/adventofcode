use crate::PuzzleResult;

#[cfg(test)]
const EXAMPLE: &str = include_str!("input/8_example");
const INPUT: &str = include_str!("input/8");

fn solve_a_for(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut waste = 2;
            let mut iter = line.chars();
            while let Some(ch) = iter.next() {
                if ch == '\\' {
                    match iter.next() {
                        Some('x') => {
                            waste += 3;
                            iter.next();
                            iter.next();
                        }
                        Some('\\' | '"') => waste += 1,
                        other => panic!("unknown {other:?}"),
                    }
                }
            }
            waste
        })
        .sum()
}

fn solve_b_for(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.chars().filter(|ch| *ch == '"' || *ch == '\\').count() + 2)
        .sum()
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for(EXAMPLE), 12);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(INPUT), 1350);
}

#[test]
fn b_example() {
    assert_eq!(solve_b_for(EXAMPLE), 19);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(INPUT), 2085);
}

pub fn solve_a() -> PuzzleResult {
    solve_a_for(INPUT).into()
}

pub fn solve_b() -> PuzzleResult {
    solve_b_for(INPUT).into()
}
