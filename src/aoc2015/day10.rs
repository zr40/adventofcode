use crate::PuzzleResult;

const INPUT: &str = include_str!("input/10");

fn solve_for(input: &str, iterations: usize) -> usize {
    if iterations == 0 {
        input.len()
    } else {
        let mut buf = String::new();

        let mut digits = input.chars();
        let mut current_digit = digits.next().unwrap();
        let mut current_count = 1;

        for digit in digits {
            if digit == current_digit {
                current_count += 1;
            } else {
                buf.push_str(&format!("{current_count}{current_digit}"));
                current_digit = digit;
                current_count = 1;
            }
        }

        buf.push_str(&format!("{current_count}{current_digit}"));

        solve_for(&buf, iterations - 1)
    }
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_for(INPUT, 40), 329356);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_for(INPUT, 50), 4666278);
}

pub fn solve_a() -> PuzzleResult {
    solve_for(INPUT, 40).into()
}

pub fn solve_b() -> PuzzleResult {
    solve_for(INPUT, 50).into()
}
