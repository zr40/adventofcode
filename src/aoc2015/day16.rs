use crate::day::Day;
use crate::PuzzleResult;

const INPUT: &str = include_str!("input/16");

enum Mode {
    Exact,
    Ranges,
}

fn solve_for(input: &str, mode: Mode) -> i32 {
    'outer: for line in input.lines() {
        let (_, rest) = line.split_once("Sue ").unwrap();
        let (sue, rest) = rest.split_once(": ").unwrap();

        for attribute in rest.split(", ") {
            let (attribute, value) = attribute.split_once(": ").unwrap();

            let value = value.parse().unwrap();

            match (attribute, value, &mode) {
                ("children", 3, _) => continue,
                ("children", _, _) => continue 'outer,
                ("cats", 7, Mode::Exact) => continue,
                ("cats", 8.., Mode::Ranges) => continue,
                ("cats", _, _) => continue 'outer,
                ("samoyeds", 2, _) => continue,
                ("samoyeds", _, _) => continue 'outer,
                ("pomeranians", 3, Mode::Exact) => continue,
                ("pomeranians", ..=2, Mode::Ranges) => continue,
                ("pomeranians", _, _) => continue 'outer,
                ("akitas", 0, _) => continue,
                ("akitas", _, _) => continue 'outer,
                ("vizslas", 0, _) => continue,
                ("vizslas", _, _) => continue 'outer,
                ("goldfish", 5, Mode::Exact) => continue,
                ("goldfish", ..=4, Mode::Ranges) => continue,
                ("goldfish", _, _) => continue 'outer,
                ("trees", 3, Mode::Exact) => continue,
                ("trees", 4.., Mode::Ranges) => continue,
                ("trees", _, _) => continue 'outer,
                ("cars", 2, _) => continue,
                ("cars", _, _) => continue 'outer,
                ("perfumes", 1, _) => continue,
                ("perfumes", _, _) => continue 'outer,
                (att, _, _) => panic!("unknown attribute '{att}'"),
            }
        }

        return sue.parse().unwrap();
    }

    panic!("no solution found");
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_for(INPUT, Mode::Exact), 373);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_for(INPUT, Mode::Ranges), 260);
}

fn solve_a() -> PuzzleResult {
    solve_for(INPUT, Mode::Exact).into()
}

fn solve_b() -> PuzzleResult {
    solve_for(INPUT, Mode::Ranges).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
