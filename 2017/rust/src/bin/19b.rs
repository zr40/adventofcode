use std::io::Read;
use std::fs::File;
use std::collections::HashMap;

enum Direction {
    Up,
    Left,
    Right,
    Down,
}

fn solve(input: &str) -> usize {
    let mut field = HashMap::new();
    let mut start_col = None;

    for (row, line) in input.split("\n").enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c != ' ' {
                field.insert((col, row), c);

                if start_col == None {
                    start_col = Some(col);
                }
            }
        }
    }

    let mut x = start_col.unwrap();
    let mut y = 0;
    let mut direction = Direction::Down;
    let mut steps = 0;

    while let Some(c) = field.get(&(x,y)) {
        steps += 1;
        if *c == '+' {
            direction = match direction {
                Direction::Down | Direction::Up => match field.get(&(x-1,y)) {
                    Some(_) => Direction::Left,
                    None => Direction::Right,
                },
                Direction::Left | Direction::Right => match field.get(&(x,y-1)){
                    Some(_) => Direction::Up,
                    None => Direction::Down,
                },
            }
        }

        match direction {
            Direction::Down => y += 1,
            Direction::Up => y -= 1,
            Direction::Left => x -= 1,
            Direction::Right => x += 1,
        }
    }

    steps
}

#[test]
fn test() {
    assert_eq!(solve("     |          \n     |  +--+    \n     A  |  C    \n F---|----E|--+ \n     |  |  |  D \n     +B-+  +--+ \n"), 38);
}

fn main() {
    let mut f = File::open("input/19").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    println!("{}", solve(input.trim_right()));
}
