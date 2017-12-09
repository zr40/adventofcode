use std::io::Read;
use std::fs::File;

fn solve(input: &str) -> i32 {
    let mut score = 0;
    let mut nesting = 0;
    let mut in_garbage = false;
    let mut next_canceled = false;

    for c in input.chars() {
        if in_garbage {
            if next_canceled {
                next_canceled = false;
            } else {
                match c {
                    '!' => next_canceled = true,
                    '>' => in_garbage = false,
                    _ => {}
                }
            }
        } else {
            match c {
                '{' => {
                    nesting += 1;
                    score += nesting;
                },
                '}' => nesting -= 1,
                '<' => in_garbage = true,
                ',' => {},
                x => panic!("Unknown character {}", x),
            }
        }
    }
    score
}

#[test]
fn test() {
    assert_eq!(solve("{}"), 1);
    assert_eq!(solve("{{{}}}"), 6);
    assert_eq!(solve("{{},{}}"), 5);
    assert_eq!(solve("{{{},{},{{}}}}"), 16);
    assert_eq!(solve("{<a>,<a>,<a>,<a>}"), 1);
    assert_eq!(solve("{{<ab>},{<ab>},{<ab>},{<ab>}}"), 9);
    assert_eq!(solve("{{<!!>},{<!!>},{<!!>},{<!!>}}"), 9);
    assert_eq!(solve("{{<a!>},{<a!>},{<a!>},{<ab>}}"), 3);
}

fn main() {
    let mut f = File::open("input/9").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    println!("{}", solve(input.trim()));
}
