use std::io::Read;
use std::fs::File;
use std::collections::HashMap;

fn solve(input: &str) -> i32 {
    let mut banks: Vec<i32> = input.split_whitespace().map(|x| i32::from_str_radix(x, 10).unwrap()).collect();
    let mut cycles = 0;

    let mut configurations_seen: HashMap<Vec<i32>, i32> = HashMap::new();

    while !configurations_seen.contains_key(&banks) {
        configurations_seen.insert(banks.clone(), cycles);
        cycles += 1;

        let mut blocks = *banks.iter().max().unwrap();
        let redistribution_position = banks.iter().position(|x| *x == blocks).unwrap();

        banks[redistribution_position] = 0;

        let mut remaining = banks.len() as i32;
        let mut index = redistribution_position;
        while remaining != 0 {
            let redistributed = blocks / remaining;
            blocks -= redistributed;
            banks[index] += redistributed;

            if index == 0 {
                index = banks.len() - 1;
            } else {
                index -= 1;
            }
            remaining -= 1;
        }
    }

    cycles - configurations_seen.get(&banks).unwrap()
}

#[test]
fn test() {
    assert_eq!(solve("0 2 7 0"), 4);
}

fn main() {
    let mut f = File::open("input/6").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    println!("{}", solve(input.trim()));
}
