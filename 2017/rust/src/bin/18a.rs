use std::io::Read;
use std::fs::File;
use std::collections::HashMap;

fn solve(input: &str) -> i64 {
    let mut registers = HashMap::new();

    let instructions: Vec<Vec<&str>> = input.split("\n").map(|x| x.split(" ").collect()).collect();

    let mut pos = 0;
    let mut snd = 0;

    loop {
        let instruction = &instructions[pos];
        let left = i64::from_str_radix(instruction[1], 10).unwrap_or(*registers.get(instruction[1]).unwrap_or(&0));
        let right = if instruction.len() == 2 {
            0
        } else {
            i64::from_str_radix(instruction[2], 10).unwrap_or(*registers.get(instruction[2]).unwrap_or(&0))
        };

        match instruction[0] {
            "set" => {
                registers.insert(instruction[1], right);
            }
            "add" => {
                registers.insert(instruction[1], left + right);
            }
            "mul" => {
                registers.insert(instruction[1], left * right);
            }
            "mod" => {
                registers.insert(instruction[1], left % right);
            }
            "snd" => {
                snd = *registers.get(instruction[1]).unwrap_or(&0);
            }
            "rcv" => {
                if *registers.get(instruction[1]).unwrap_or(&0) > 0 {
                    return snd;
                }
            }
            "jgz" => {
                if left > 0 {
                    pos = (pos as isize + right as isize - 1) as usize;
                }
            }
            x => panic!("Unknown instruction {}", x),
        }

        pos += 1;
    }
}

#[test]
fn test() {
    assert_eq!(solve("set a 1\nadd a 2\nmul a a\nmod a 5\nsnd a\nset a 0\nrcv a\njgz a -1\nset a 1\njgz a -2"), 4);
}

fn main() {
    let mut f = File::open("input/18").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    println!("{}", solve(input.trim()));
}
