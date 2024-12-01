use crate::PuzzleResult;
use crate::common::aoc::input_for;
use crate::day::Day;

#[cfg(test)]
const EXAMPLE: &str = include_str!("example/2");

fn solve_a_for(input: &str) -> usize {
    let mut sum_of_possible_games = 0;

    for line in input.lines() {
        let mut words = line.split(' ');

        let game = words.nth(1).unwrap();
        let game: usize = game[..(game.len() - 1)].parse().unwrap();

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        while let Some(amount) = words.next() {
            let amount = amount.parse().unwrap();
            let color = words.next().unwrap();
            if color.starts_with("red") {
                red = red.max(amount);
            } else if color.starts_with("green") {
                green = green.max(amount);
            } else if color.starts_with("blue") {
                blue = blue.max(amount);
            } else {
                panic!("unexpected color {color}");
            }
        }

        if red <= 12 && green <= 13 && blue <= 14 {
            sum_of_possible_games += game;
        }
    }

    sum_of_possible_games
}

fn solve_b_for(input: &str) -> u32 {
    let mut sum_of_game_power = 0;

    for line in input.lines() {
        let mut words = line.split(' ');
        words.nth(1); // skip game id

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        while let Some(amount) = words.next() {
            let amount = amount.parse().unwrap();
            let color = words.next().unwrap();
            if color.starts_with("red") {
                red = red.max(amount);
            } else if color.starts_with("green") {
                green = green.max(amount);
            } else if color.starts_with("blue") {
                blue = blue.max(amount);
            } else {
                panic!("unexpected color {color}");
            }
        }

        let game_power = red * green * blue;
        sum_of_game_power += game_power;
    }

    sum_of_game_power
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for(EXAMPLE), 8);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(&input_for(2023, 2)), 3099);
}

fn solve_a() -> PuzzleResult {
    solve_a_for(&input_for(2023, 2)).into()
}

#[test]
fn b_example() {
    assert_eq!(solve_b_for(EXAMPLE), 2286);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(&input_for(2023, 2)), 72970);
}

fn solve_b() -> PuzzleResult {
    solve_b_for(&input_for(2023, 2)).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
