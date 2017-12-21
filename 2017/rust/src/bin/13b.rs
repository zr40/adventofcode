use std::io::Read;
use std::fs::File;

fn solve(input: &str) -> u32 {
    let mut delay = 0;

    let input: Vec<(u32, u32)> = input.split("\n").map(|x| {
        let mut iter = x.split(": ");
        let position = u32::from_str_radix(iter.next().unwrap(), 10).unwrap();
        let range = u32::from_str_radix(iter.next().unwrap(), 10).unwrap();

        (position, range)
    }).collect();

    loop {
        if input.iter().all(|&(position, range)| {
            let x = (position + delay) % (range * 2 - 2);

            x != 0
        }) {
            break;
        }

        delay += 1;
    }

    delay
}

#[test]
fn test() {
    assert_eq!(solve("0: 3\n1: 2\n4: 4\n6: 4"), 10);
}

fn main() {
    let mut f = File::open("input/13").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    println!("{}", solve(input.trim()));
}
