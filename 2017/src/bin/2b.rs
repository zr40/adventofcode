use std::io::Read;
use std::fs::File;

fn solve(input: &str) -> u32 {
    let mut sum = 0;

    for line in input.lines() {
        let cells: Vec<u32> = line.split_whitespace().map(|x| u32::from_str_radix(x, 10).unwrap()).collect();

        'outer: for first in cells.iter() {
            for second in cells.iter() {
                if first != second {
                    let largest = first.max(second);
                    let smallest = first.min(second);

                    if largest % smallest == 0 {
                        sum += largest / smallest;
                        break 'outer;
                    }
                }
            }
        }
    }

    sum
}

#[test]
fn test() {
    assert_eq!(solve("5 9 2 8\n9 4 7 3\n3 8 6 5"), 9);
}

fn main() {
    let mut f = File::open("input/2").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    println!("{}", solve(input.trim()));
}
