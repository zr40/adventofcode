use std::io::Read;
use std::fs::File;

fn solve(input: &str) -> usize {
    let steps = usize::from_str_radix(input, 10).unwrap();

    let mut position = 0;
    let mut output = 0;

    for i in 1..50000001 {
        position = (position + steps) % i + 1;

        if position == 1 {
            output = i;
        }
    }

    output
}

fn main() {
    let mut f = File::open("input/17").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    println!("{}", solve(input.trim()));
}
