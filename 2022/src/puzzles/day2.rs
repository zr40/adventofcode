#[allow(dead_code)]
const EXAMPLE: &str = include_str!("../input/2_example");
const INPUT: &str = include_str!("../input/2");

enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn from(input: &str) -> Choice {
        match input {
            "A" | "X" => Choice::Rock,
            "B" | "Y" => Choice::Paper,
            "C" | "Z" => Choice::Scissors,
            unexpected => panic!("unexpected {unexpected}"),
        }
    }

    fn points(self) -> u32 {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }
}

enum Outcome {
    Loss,
    Draw,
    Win,
}

impl Outcome {
    fn from(input: &str) -> Outcome {
        match input {
            "X" => Outcome::Loss,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            unexpected => panic!("unexpected {unexpected}"),
        }
    }

    fn points(self) -> u32 {
        match self {
            Outcome::Loss => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

fn solve_a_for(input: &str) -> u32 {
    let mut score = 0;

    for line in input.lines() {
        let theirs = Choice::from(&line[0..1]);
        let ours = Choice::from(&line[2..3]);

        let outcome = match (&ours, theirs) {
            (Choice::Rock, Choice::Paper)
            | (Choice::Paper, Choice::Scissors)
            | (Choice::Scissors, Choice::Rock) => Outcome::Loss,
            (Choice::Rock, Choice::Rock)
            | (Choice::Paper, Choice::Paper)
            | (Choice::Scissors, Choice::Scissors) => Outcome::Draw,
            (Choice::Rock, Choice::Scissors)
            | (Choice::Paper, Choice::Rock)
            | (Choice::Scissors, Choice::Paper) => Outcome::Win,
        };

        score += ours.points() + outcome.points();
    }

    score
}

fn solve_b_for(input: &str) -> u32 {
    let mut score = 0;

    for line in input.lines() {
        let theirs = Choice::from(&line[0..1]);
        let outcome = Outcome::from(&line[2..3]);

        let ours = match (theirs, &outcome) {
            (Choice::Rock, Outcome::Loss) => Choice::Scissors,
            (Choice::Rock, Outcome::Draw) => Choice::Rock,
            (Choice::Rock, Outcome::Win) => Choice::Paper,
            (Choice::Paper, Outcome::Loss) => Choice::Rock,
            (Choice::Paper, Outcome::Draw) => Choice::Paper,
            (Choice::Paper, Outcome::Win) => Choice::Scissors,
            (Choice::Scissors, Outcome::Loss) => Choice::Paper,
            (Choice::Scissors, Outcome::Draw) => Choice::Scissors,
            (Choice::Scissors, Outcome::Win) => Choice::Rock,
        };

        score += ours.points() + outcome.points();
    }

    score
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for(EXAMPLE), 15);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(INPUT), 12855);
}

#[test]
fn b_example() {
    assert_eq!(solve_b_for(EXAMPLE), 12);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(INPUT), 13726);
}

pub fn solve_a() {
    println!("{}", solve_a_for(INPUT));
}

pub fn solve_b() {
    println!("{}", solve_b_for(INPUT));
}
