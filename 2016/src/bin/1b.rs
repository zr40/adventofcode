use std::io::Read;
use std::fs::File;
use std::collections::HashSet;

enum Orientation {
    North,
    East,
    South,
    West,
}

fn solve(input: &str) -> i32 {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut orientation = Orientation::North;
    let mut visited = HashSet::new();

    visited.insert((0, 0));

    'outer: for instruction in input.split(", ") {
        let (direction, distance) = instruction.split_at(1);
        let distance = i32::from_str_radix(distance, 10).unwrap();

        orientation = match direction {
            "L" => match orientation {
                Orientation::North => Orientation::West,
                Orientation::East => Orientation::North,
                Orientation::South => Orientation::East,
                Orientation::West => Orientation::South,
            }
            "R" => match orientation {
                Orientation::North => Orientation::East,
                Orientation::East => Orientation::South,
                Orientation::South => Orientation::West,
                Orientation::West => Orientation::North,
            }
            x => panic!("Unknown direction {}", x),
        };

        for _ in 0..distance {
            match orientation {
                Orientation::North => y += 1,
                Orientation::East => x += 1,
                Orientation::South => y -= 1,
                Orientation::West => x -= 1,
            }

            if visited.contains(&(x, y)) {
                break 'outer;
            } else {
                visited.insert((x, y));
            }
        }
    }

    x.abs() + y.abs()
}

#[test]
fn test() {
    assert_eq!(solve("R8, R4, R4, R8"), 4);
}

fn main() {
    let mut f = File::open("input/1").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    println!("{}", solve(input.trim()));
}
