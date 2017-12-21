use std::io::Read;
use std::fs::File;

fn solve(input: &str) -> usize {
    let steps = usize::from_str_radix(input, 10).unwrap();

    let mut buffer = vec!(0);
    let mut position = 0;

    for i in 1..2018 {
        position = (position + steps) % i + 1;

        buffer.insert(position, i);
    }

    buffer[(position + 1) % 2018]
}

#[test]
fn test() {
    assert_eq!(solve("3"), 638);
}

fn main() {
    let mut f = File::open("input/17").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    println!("{}", solve(input.trim()));
}
