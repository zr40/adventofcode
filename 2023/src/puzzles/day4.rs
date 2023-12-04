use std::collections::HashSet;

#[allow(dead_code)]
const EXAMPLE_A: &str = include_str!("../input/4a_example");
#[allow(dead_code)]
const EXAMPLE_B: &str = include_str!("../input/4b_example");
const INPUT: &str = include_str!("../input/4");

fn solve_a_for(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut tokens = line
                .split_once(": ")
                .unwrap()
                .1
                .split(' ')
                .filter(|t| !t.is_empty());
            let winning_numbers: HashSet<u32> = tokens
                .by_ref()
                .take_while(|t| *t != "|")
                .map(|t| t.parse().unwrap())
                .collect();
            let my_numbers: HashSet<u32> = tokens.map(|t| t.parse().unwrap()).collect();

            let matching_numbers = winning_numbers.intersection(&my_numbers);

            match matching_numbers.count() {
                0 => 0,
                count => 2u32.pow((count - 1) as u32),
            }
        })
        .sum()
}

fn solve_b_for(input: &str) -> u32 {
    let mut card_copies: Vec<_> = input.lines().map(|_| 1).collect();

    for (card, line) in input.lines().enumerate() {
        let mut tokens = line
            .split_once(": ")
            .unwrap()
            .1
            .split(' ')
            .filter(|t| !t.is_empty());
        let winning_numbers: HashSet<u32> = tokens
            .by_ref()
            .take_while(|t| *t != "|")
            .map(|t| t.parse().unwrap())
            .collect();
        let my_numbers: HashSet<u32> = tokens.map(|t| t.parse().unwrap()).collect();

        let matching_numbers = winning_numbers.intersection(&my_numbers);

        for card_to_copy in 1..=matching_numbers.count() {
            card_copies[card + card_to_copy] += card_copies[card];
        }
    }

    card_copies.into_iter().sum()
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for(EXAMPLE_A), 13);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(INPUT), 23941);
}

#[test]
fn b_example() {
    assert_eq!(solve_b_for(EXAMPLE_B), 30);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(INPUT), 5571760);
}

pub fn solve_a() {
    println!("{}", solve_a_for(INPUT));
}

pub fn solve_b() {
    println!("{}", solve_b_for(INPUT));
}
