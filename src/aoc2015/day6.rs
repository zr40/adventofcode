use crate::PuzzleResult;

const INPUT: &str = include_str!("input/6");

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
    let mut grid: Vec<bool> = vec![false; 1000000];

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

    grid.into_iter().filter(|c| *c).count()
}

fn solve_b_for(input: &str) -> u32 {
    let mut grid: Vec<u32> = vec![0; 1000000];

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

    grid.into_iter().sum()
}

#[test]
#[cfg_attr(debug_assertions, ignore)]
fn a_puzzle() {
    assert_eq!(solve_a_for(INPUT), 400410);
}

#[test]
#[cfg_attr(debug_assertions, ignore)]
fn b_puzzle() {
    assert_eq!(solve_b_for(INPUT), 15343601);
}

#[cfg(debug_assertions)]
pub fn solve_a() -> PuzzleResult {
    PuzzleResult::SkipSlow
}

#[cfg(debug_assertions)]
pub fn solve_b() -> PuzzleResult {
    PuzzleResult::SkipSlow
}

#[cfg(not(debug_assertions))]
pub fn solve_a() -> PuzzleResult {
    solve_a_for(INPUT).into()
}

#[cfg(not(debug_assertions))]
pub fn solve_b() -> PuzzleResult {
    solve_b_for(INPUT).into()
}
