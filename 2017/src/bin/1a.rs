#![feature(io)]

use std::io::Read;
use std::fs::File;

fn solve(input: &str) -> u32 {
    let mut sum = 0;

    let mut iterator = input.chars().peekable();

    while let Some(c) = iterator.next() {
        let next = match iterator.peek() {
            None => input.chars().next().unwrap(),
            Some(x) => *x,
        };

        if c == next {
            sum += c.to_digit(10).unwrap();
        }
    }

    sum
}

#[test]
fn test() {
    assert_eq!(solve("1122"), 3);
    assert_eq!(solve("1111"), 4);
    assert_eq!(solve("1234"), 0);
    assert_eq!(solve("91212129"), 9);
}

fn main() {
    let mut f = File::open("input/1").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    println!("{}", solve(input.trim()));
}
