use crate::puzzle_result::PuzzleResult;

#[cfg(test)]
const EXAMPLE: &str = include_str!("input/14_example");
const INPUT: &str = include_str!("input/14");

type Coords = (i32, i32);

enum Mode {
    Abyss,
    Floor,
}

fn parse_coords(input: &str) -> Coords {
    let (x, y) = input.split_once(',').unwrap();
    (x.parse().unwrap(), y.parse().unwrap())
}

const HEIGHT: i32 = 165;
const WIDTH: i32 = 500;

fn solve_for(input: &str, mode: Mode) -> usize {
    let mut cave: Vec<bool> = Vec::with_capacity((WIDTH * HEIGHT) as usize);
    cave.resize((WIDTH * HEIGHT) as usize, false);
    let mut deepest = 0;

    for line in input.lines() {
        let mut coords = line.split(" -> ");

        let mut start = parse_coords(coords.next().unwrap());
        cave[(start.1 * WIDTH + start.0) as usize] = true;
        deepest = deepest.max(start.1);

        for end in coords {
            let end = parse_coords(end);
            deepest = deepest.max(end.1);

            while start != end {
                start = (
                    start.0 + (end.0 - start.0).signum(),
                    start.1 + (end.1 - start.1).signum(),
                );
                cave[(start.1 * WIDTH + start.0) as usize] = true;
            }
        }
    }

    let mut units = 0;

    loop {
        let mut sand = (500, 0);
        loop {
            if let Mode::Abyss = mode {
                if sand.1 == deepest {
                    return units;
                }
            }

            if let Mode::Floor = mode {
                if sand.1 == deepest + 1 {
                    cave[((sand.1) * WIDTH + sand.0) as usize] = true;
                    units += 1;
                    break;
                }
            }

            if !cave[((sand.1 + 1) * WIDTH + sand.0) as usize] {
                sand = (sand.0, sand.1 + 1);
            } else if !cave[((sand.1 + 1) * WIDTH + sand.0 - 1) as usize] {
                sand = (sand.0 - 1, sand.1 + 1);
            } else if !cave[((sand.1 + 1) * WIDTH + sand.0 + 1) as usize] {
                sand = (sand.0 + 1, sand.1 + 1);
            } else {
                if let Mode::Floor = mode {
                    if sand == (500, 0) {
                        return units + 1;
                    }
                }
                cave[((sand.1) * WIDTH + sand.0) as usize] = true;
                units += 1;
                break;
            }
        }
    }
}

#[test]
fn a_example() {
    assert_eq!(solve_for(EXAMPLE, Mode::Abyss), 24);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_for(INPUT, Mode::Abyss), 901);
}

#[test]
fn b_example() {
    assert_eq!(solve_for(EXAMPLE, Mode::Floor), 93);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_for(INPUT, Mode::Floor), 24589);
}

pub fn solve_a() -> PuzzleResult {
    solve_for(INPUT, Mode::Abyss).into()
}

pub fn solve_b() -> PuzzleResult {
    solve_for(INPUT, Mode::Floor).into()
}
