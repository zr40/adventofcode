use std::io::Read;
use std::fs::File;
use std::collections::HashMap;
use std::collections::VecDeque;

enum DanceMove {
    Spin(usize),
    Exchange(usize,usize),
    Partner(usize,usize),
}

fn solve(len: usize, iterations: usize, input: &str) -> String {
    let program_names = vec!("a","b","c","d","e","f","g","h","i","j","k","l","m","n","o","p");
    let mut program_numbers = HashMap::new();
    let mut programs = VecDeque::new();

    let mut formations_seen = HashMap::new();

    for (index, name) in program_names.iter().enumerate() {
        if index == len {
            break;
        }

        program_numbers.insert(*name, index);
        programs.push_back(index);
    }

    let input: Vec<DanceMove> = input.split(",").map(|dance_move| {
        let (the_move, rest) = dance_move.split_at(1);

        match the_move {
            "s" => DanceMove::Spin(usize::from_str_radix(rest, 10).unwrap()),
            "x" => {
                let mut iter = rest.split("/");
                let a = usize::from_str_radix(iter.next().unwrap(), 10).unwrap();
                let b = usize::from_str_radix(iter.next().unwrap(), 10).unwrap();

                DanceMove::Exchange(a,b)
            },
            "p" => {
                let mut iter = rest.split("/");
                let a = iter.next().unwrap();
                let b = iter.next().unwrap();

                let a = program_numbers.get(a).unwrap();
                let b = program_numbers.get(b).unwrap();

                DanceMove::Partner(*a,*b)
            }
            x => panic!("Unknown move {}", x),
        }
    }).collect();

    for iteration in 0..iterations {
        for dance_move in input.iter() {
            match dance_move {
                &DanceMove::Spin(size) => {
                    for _ in 0..size {
                        let p = programs.pop_back().unwrap();
                        programs.push_front(p);
                    }
                }
                &DanceMove::Exchange(a,b) => {
                    programs.swap(a, b);
                }
                &DanceMove::Partner(a,b) => {
                    let a = programs.iter().position(|x| *x == a).unwrap();
                    let b = programs.iter().position(|x| *x == b).unwrap();
                    programs.swap(a, b);
                }
            }
        }

        if let Some(x) = formations_seen.get(&programs) {
            if (iterations - iteration - 1) % (iteration - x) == 0 {
                break;
            }
        }

        formations_seen.insert(programs.clone(), iteration);
    }

    let mut output = String::new();
    for program in programs {
        output.push_str(program_names[program]);
    }

    output
}

#[test]
fn test() {
    assert_eq!(solve(5, 2, "s1,x3/4,pe/b"), "ceadb");
}

fn main() {
    let mut f = File::open("input/16").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    println!("{}", solve(16, 1_000_000_000, input.trim()));
}
