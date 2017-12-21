use std::io::Read;
use std::fs::File;

fn solve(input: &str) -> i32 {
    let mut pos = 0;
    let mut jumps = 0;

    let mut jump_offsets: Vec<i32> = input.split_whitespace().map(|x| i32::from_str_radix(x, 10).unwrap()).collect();

    while pos >= 0 && pos < jump_offsets.len() as i32 {
        let offset = jump_offsets[pos as usize];
        if offset >= 3 {
            jump_offsets[pos as usize] -= 1;
        } else {
            jump_offsets[pos as usize] += 1;
        }
        pos += offset;
        jumps += 1;
    }

    jumps
}

#[test]
fn test() {
    assert_eq!(solve("0 3 0 1 -3"), 10);
}

fn main() {
    let mut f = File::open("input/5").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    println!("{}", solve(input.trim()));
}
