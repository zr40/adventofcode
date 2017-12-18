use std::io::Read;
use std::fs::File;
use std::collections::HashMap;
use std::collections::VecDeque;

fn solve(input: &str) -> u32 {
    let instructions: Vec<Vec<&str>> = input.split("\n").map(|x| x.split(" ").collect()).collect();

    let mut registers_0 = HashMap::new();
    let mut registers_1 = HashMap::new();

    registers_0.insert("p", 0);
    registers_1.insert("p", 1);

    let mut queue_0 = VecDeque::new();
    let mut queue_1 = VecDeque::new();

    let mut pos_0 = 0;
    let mut pos_1 = 0;

    let mut current_execution_0 = true;

    let mut blocked_0 = false;
    let mut blocked_1 = false;

    let mut send_count = 0;

    loop {
        let current_registers;
        let current_queue;
        let other_queue;
        let current_pos;
        let current_blocked;
        let other_blocked;

        if current_execution_0 {
            current_registers = &mut registers_0;
            current_queue = &mut queue_0;
            other_queue = &mut queue_1;
            current_pos = &mut pos_0;
            current_blocked = &mut blocked_0;
            other_blocked = &mut blocked_1;
        } else {
            current_registers = &mut registers_1;
            current_queue = &mut queue_1;
            other_queue = &mut queue_0;
            current_pos = &mut pos_1;
            current_blocked = &mut blocked_1;
            other_blocked = &mut blocked_0;
        }

        let instruction = &instructions[*current_pos];

        match instruction[0] {
            "set" => {
                let right = i64::from_str_radix(instruction[2], 10).unwrap_or(*current_registers.get(instruction[2]).unwrap_or(&0));
                current_registers.insert(instruction[1], right);
            }
            "add" => {
                let left = *current_registers.get(instruction[1]).unwrap_or(&0);
                let right = i64::from_str_radix(instruction[2], 10).unwrap_or(*current_registers.get(instruction[2]).unwrap_or(&0));
                current_registers.insert(instruction[1], left + right);
            }
            "mul" => {
                let left = *current_registers.get(instruction[1]).unwrap_or(&0);
                let right = i64::from_str_radix(instruction[2], 10).unwrap_or(*current_registers.get(instruction[2]).unwrap_or(&0));
                current_registers.insert(instruction[1], left * right);
            }
            "mod" => {
                let left = *current_registers.get(instruction[1]).unwrap_or(&0);
                let right = i64::from_str_radix(instruction[2], 10).unwrap_or(*current_registers.get(instruction[2]).unwrap_or(&0));
                current_registers.insert(instruction[1], left % right);
            }
            "snd" => {
                let left = *current_registers.get(instruction[1]).unwrap_or(&0);
                other_queue.push_back(left);
                *other_blocked = false;
                if !current_execution_0 {
                    send_count += 1;
                }
            }
            "rcv" => {
                match current_queue.pop_front() {
                    Some(x) => {
                        current_registers.insert(instruction[1], x);
                    }
                    None => {
                        if *other_blocked {
                            break;
                        } else {
                            *current_blocked = true;
                            current_execution_0 = !current_execution_0;
                            continue;
                        }
                    }
                };
            }
            "jgz" => {
                let left = i64::from_str_radix(instruction[1], 10).unwrap_or(*current_registers.get(instruction[1]).unwrap_or(&0));
                let right = i64::from_str_radix(instruction[2], 10).unwrap_or(*current_registers.get(instruction[2]).unwrap_or(&0));
                if left > 0 {
                    *current_pos = (*current_pos as isize + right as isize - 1) as usize;
                }

            }
            x => panic!("Unknown instruction {}", x),
        }

        *current_pos += 1;
    }

    send_count
}

#[test]
fn test() {
    assert_eq!(solve("snd 1\nsnd 2\nsnd p\nrcv a\nrcv b\nrcv c\nrcv d"), 3);
}

fn main() {
    let mut f = File::open("input/18").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    println!("{}", solve(input.trim()));
}
