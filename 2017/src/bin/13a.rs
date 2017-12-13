use std::io::Read;
use std::fs::File;

fn solve(input: &str) -> u32 {
    input.split("\n").map(|x| {
        let mut iter = x.split(": ");
        let position = u32::from_str_radix(iter.next().unwrap(), 10).unwrap();
        let range = u32::from_str_radix(iter.next().unwrap(), 10).unwrap();

        let x = position % (range * 2 - 2);

        if x == 0 {
            position * range
        } else {
            0
        }
    }).sum()
}

#[test]
fn test() {
    assert_eq!(solve("0: 3\n1: 2\n4: 4\n6: 4"), 24);
}

fn main() {
    let mut f = File::open("input/13").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    println!("{}", solve(input.trim()));
}
