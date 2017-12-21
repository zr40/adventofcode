use std::io::Read;
use std::fs::File;

fn solve(input: &str) -> u32 {
    let mut sum = 0;

    let (first, second) = input.split_at(input.len() / 2);

    for (a, b) in first.chars().zip(second.chars()) {
        if a == b {
            sum += a.to_digit(10).unwrap();
        }
    }

    sum * 2
}

#[test]
fn test() {
    assert_eq!(solve("1212"), 6);
    assert_eq!(solve("1221"), 0);
    assert_eq!(solve("123425"), 4);
    assert_eq!(solve("123123"), 12);
    assert_eq!(solve("12131415"), 4);
}

fn main() {
    let mut f = File::open("input/1").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    println!("{}", solve(input.trim()));
}
