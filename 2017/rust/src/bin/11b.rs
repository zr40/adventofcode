use std::io::Read;
use std::fs::File;

fn solve(input: &str) -> i32 {
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    let mut max = 0;

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

        let xa = x.abs();
        let ya = y.abs();

        max = max.max(if xa > ya {
            xa
        } else {
            xa + (ya - xa) / 2
        });
    }

    max
}

fn main() {
    let mut f = File::open("input/11").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    println!("{}", solve(input.trim()));
}
