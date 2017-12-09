use std::io::Read;
use std::fs::File;
use std::collections::HashMap;

fn solve(input: &str) -> i32 {
    let mut registers = HashMap::new();

    for line in input.lines() {
        let mut tokens = line.split_whitespace();

        let register = tokens.next().unwrap();
        let instruction = tokens.next().unwrap();
        let amount = i32::from_str_radix(tokens.next().unwrap(), 10).unwrap();
        tokens.next(); // "if"
        let condition_register = tokens.next().unwrap();
        let comparison = tokens.next().unwrap();
        let comparison_value = i32::from_str_radix(tokens.next().unwrap(), 10).unwrap();

        let if_register_value = *registers.get(condition_register).unwrap_or(&0);
        if match comparison {
            "!=" => if_register_value != comparison_value,
            "<" => if_register_value < comparison_value,
            "<=" => if_register_value <= comparison_value,
            "==" => if_register_value == comparison_value,
            ">" => if_register_value > comparison_value,
            ">=" => if_register_value >= comparison_value,
            unknown => panic!("Unknown comparison {}", unknown),
        } {
            let register_value = *registers.get(register).unwrap_or(&0);
            registers.insert(register, match instruction {
                "inc" => register_value + amount,
                "dec" => register_value - amount,
                unknown => panic!("Unknown instruction {}", unknown),
            });
        }
    }

    *registers.values().max().unwrap()
}

#[test]
fn test() {
    assert_eq!(solve("b inc 5 if a > 1\na inc 1 if b < 5\nc dec -10 if a >= 1\nc inc -20 if c == 10"), 1);
}

fn main() {
    let mut f = File::open("input/8").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    println!("{}", solve(input.trim()));
}
