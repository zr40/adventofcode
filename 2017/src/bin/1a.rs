#![feature(io)]

use std::io::Read;
use std::fs::File;

fn solve(input: &str) -> u32 {
    let mut sum = 0;
    let mut first: Option<u32> = None;
    let mut previous: Option<u32> = None;

    for c in input.chars() {
        let digit = c.to_digit(10);

        match digit {
            Some(d) => {
                if first == None {
                    first = Some(d);
                }

                if Some(d) == previous {
                    sum += d;
                }

                previous = Some(d);
            },
            None => {},
        }
    }

    match first {
        None => {},
        Some(d) => {
            if first == previous {
                sum += d;
            }
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

    println!("{}", solve(input.as_str()));
}
