use std::io::Read;
use std::fs::File;

fn solve(input: &str) -> i32 {
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    for direction in input.split(",") {
        match direction {
            "n" => y += 2,
            "ne" => {
                x += 1;
                y += 1;
            }
            "se" => {
                x += 1;
                y -= 1;
            }
            "s" => y -= 2,
            "sw" => {
                x -= 1;
                y -= 1;
            }
            "nw" => {
                x -= 1;
                y += 1;
            }
            other => panic!("Unknown direction {}", other),
        }
    }

    x = x.abs();
    y = y.abs();

    if x > y {
        x
    } else {
        x + (y - x) / 2
    }
}

#[test]
fn test() {
    assert_eq!(solve("ne,ne,ne"), 3);
    assert_eq!(solve("ne,ne,sw,sw"), 0);
    assert_eq!(solve("ne,ne,s,s"), 2);
    assert_eq!(solve("se,sw,se,sw,sw"), 3);

    assert_eq!(solve("ne,se"), 2);
}

fn main() {
    let mut f = File::open("input/11").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    println!("{}", solve(input.trim()));
}
