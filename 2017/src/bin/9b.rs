use std::io::Read;
use std::fs::File;

fn solve(input: &str) -> i32 {
    let mut in_garbage = false;
    let mut next_canceled = false;
    let mut garbage_chars = 0;

    for c in input.chars() {
        if in_garbage {
            if next_canceled {
                next_canceled = false;
            } else {
                match c {
                    '!' => next_canceled = true,
                    '>' => in_garbage = false,
                    _ => garbage_chars += 1,
                }
            }
        } else {
            match c {
                '{' => {},
                '}' => {},
                '<' => in_garbage = true,
                ',' => {},
                x => panic!("Unknown character {}", x),
            }
        }
    }
    garbage_chars
}

#[test]
fn test() {
    assert_eq!(solve("<>"), 0);
    assert_eq!(solve("<random characters>"), 17);
    assert_eq!(solve("<<<<>"), 3);
    assert_eq!(solve("<{!>}>"), 2);
    assert_eq!(solve("<!!>"), 0);
    assert_eq!(solve("<!!!>>"), 0);
    assert_eq!(solve("<{o\"i!a,<{i<a>"), 10);
}

fn main() {
    let mut f = File::open("input/9").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    println!("{}", solve(input.trim()));
}
