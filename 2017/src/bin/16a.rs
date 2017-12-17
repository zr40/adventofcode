use std::io::Read;
use std::fs::File;

fn solve(len: usize, input: &str) -> String {
    let mut programs = vec!("a","b","c","d","e","f","g","h","i","j","k","l","m","n","o","p");
    programs.truncate(len);

    for dance_move in input.split(",") {
        let (the_move, rest) = dance_move.split_at(1);
        match the_move {
            "s" => {
                for _ in 0..(usize::from_str_radix(rest, 10).unwrap()) {
                    let p = programs.pop().unwrap();
                    programs.insert(0, p);
                }
            },
            "x" => {
                let mut iter = rest.split("/");
                let a = usize::from_str_radix(iter.next().unwrap(), 10).unwrap();
                let b = usize::from_str_radix(iter.next().unwrap(), 10).unwrap();

                programs.swap(a, b);
            },
            "p" => {
                let mut iter = rest.split("/");
                let a = iter.next().unwrap();
                let b = iter.next().unwrap();

                let a = programs.iter().position(|x| *x == a).unwrap();
                let b = programs.iter().position(|x| *x == b).unwrap();
                programs.swap(a, b);
            }
            x => panic!("Unknown move {}", x),
        }
    }

    let mut output = String::new();
    for program in programs {
        output.push_str(program);
    }

    output
}

#[test]
fn test() {
    assert_eq!(solve(5, "s1"), "eabcd");
    assert_eq!(solve(5, "s1,x3/4"), "eabdc");
    assert_eq!(solve(5, "s1,x3/4,pe/b"), "baedc");
}

fn main() {
    let mut f = File::open("input/16").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    println!("{}", solve(16, input.trim()));
}
