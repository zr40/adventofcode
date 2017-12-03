use std::io::Read;
use std::fs::File;

fn solve(input: &str) -> u32 {
    let mut sum = 0;

    for line in input.lines() {
        let cells: Vec<u32> = line.split_whitespace().map(|x| u32::from_str_radix(x, 10).unwrap()).collect();
        let min = cells.iter().min().unwrap();
        let max = cells.iter().max().unwrap();

        sum += max - min;
    }

    sum
}

#[test]
fn test() {
    assert_eq!(solve("5\t1\t9\t5\n7\t5\t3\n2\t4\t6\t8"), 18);
}

fn main() {
    let mut f = File::open("input/2").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    println!("{}", solve(input.trim()));
}
