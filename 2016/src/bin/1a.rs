use std::io::Read;
use std::fs::File;

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

    for instruction in input.split(", ") {
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

        match orientation {
            Orientation::North => y += distance,
            Orientation::East => x += distance,
            Orientation::South => y -= distance,
            Orientation::West => x -= distance,
        }
    }

    x.abs() + y.abs()
}

#[test]
fn test() {
    assert_eq!(solve("R2, L3"), 5);
    assert_eq!(solve("R2, R2, R2"), 2);
    assert_eq!(solve("R5, L5, R5, R3"), 12);
}

fn main() {
    let mut f = File::open("input/1").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    println!("{}", solve(input.trim()));
}
