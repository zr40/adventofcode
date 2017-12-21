use std::io::Read;
use std::fs::File;

fn solve(input: &str) -> u32 {
    let mut iter = input.split("\n");
    let mut prev_a = u64::from_str_radix(iter.next().unwrap().split(" ").last().unwrap(), 10).unwrap();
    let mut prev_b = u64::from_str_radix(iter.next().unwrap().split(" ").last().unwrap(), 10).unwrap();
    let mut count = 0;

    for _ in 0..40_000_000 {
        let a = (prev_a * 16807) % 2147483647;
        let b = (prev_b * 48271) % 2147483647;

        if a & 0b1111_1111_1111_1111 == b & 0b1111_1111_1111_1111 {
            count += 1;
        }

        prev_a = a;
        prev_b = b;
    }

    count
}

#[test]
fn test() {
    assert_eq!(solve("65\n8921"), 588);
}

fn main() {
    let mut f = File::open("input/15").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    println!("{}", solve(input.trim()));
}
