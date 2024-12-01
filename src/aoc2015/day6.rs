use crate::PuzzleResult;
use crate::common::aoc::input_for;
use crate::day::Day;

enum Operation {
    Toggle,
    TurnOn,
    TurnOff,
}

struct Line {
    operation: Operation,
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize,
}

fn parse_line(line: &str) -> Line {
    let mut tokens = line.split(' ');
    let operation = match tokens.next().unwrap() {
        "toggle" => Operation::Toggle,
        "turn" => match tokens.next().unwrap() {
            "on" => Operation::TurnOn,
            "off" => Operation::TurnOff,
            token => panic!("unexpected operation 'turn {token}'"),
        },
        token => panic!("unexpected instruction '{token}'"),
    };

    let (start_x, start_y) = tokens.next().unwrap().split_once(',').unwrap();
    tokens.next();
    let (end_x, end_y) = tokens.next().unwrap().split_once(',').unwrap();

    Line {
        operation,
        start_x: start_x.parse().unwrap(),
        start_y: start_y.parse().unwrap(),
        end_x: end_x.parse().unwrap(),
        end_y: end_y.parse().unwrap(),
    }
}

fn solve_a_for(input: &str) -> usize {
    let mut grid: Box<[bool]> = vec![false; 1000000].into_boxed_slice();

    for line in input.lines() {
        let line = parse_line(line);

        for y in line.start_y..=line.end_y {
            let y = y * 1000;
            for x in line.start_x..=line.end_x {
                let coord = x + y;

                grid[coord] = match line.operation {
                    Operation::Toggle => !grid[coord],
                    Operation::TurnOn => true,
                    Operation::TurnOff => false,
                };
            }
        }
    }

    grid.iter().filter(|c| **c).count()
}

fn solve_b_for(input: &str) -> u32 {
    let mut grid: Box<[u32]> = vec![0; 1000000].into_boxed_slice();

    for line in input.lines() {
        let line = parse_line(line);

        for y in line.start_y..=line.end_y {
            let y = y * 1000;
            for x in line.start_x..=line.end_x {
                let coord = x + y;

                grid[coord] = match (&line.operation, grid[coord]) {
                    (Operation::Toggle, brightness) => brightness + 2,
                    (Operation::TurnOn, brightness) => brightness + 1,
                    (Operation::TurnOff, 0) => 0,
                    (Operation::TurnOff, brightness) => brightness - 1,
                };
            }
        }
    }

    grid.iter().sum()
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(&input_for(2015, 6)), 400410);
}

fn solve_a() -> PuzzleResult {
    solve_a_for(&input_for(2015, 6)).into()
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(&input_for(2015, 6)), 15343601);
}

fn solve_b() -> PuzzleResult {
    solve_b_for(&input_for(2015, 6)).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
