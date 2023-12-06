#[cfg(test)]
const EXAMPLE: &str = include_str!("../input/6_example");
const INPUT: &str = include_str!("../input/6");

fn solve_a_for(input: &str) -> usize {
    let mut lines = input.lines();
    let times = lines
        .next()
        .unwrap()
        .split(' ')
        .filter_map(|t| t.parse::<u32>().ok());
    let distances = lines
        .next()
        .unwrap()
        .split(' ')
        .filter_map(|t| t.parse::<u32>().ok());

    times
        .zip(distances)
        .map(|(time, distance)| {
            (1..time)
                .filter(|duration| duration * (time - duration) > distance)
                .count()
        })
        .product()
}

fn solve_b_for(input: &str) -> usize {
    let mut lines = input.lines();

    let time = lines
        .next()
        .unwrap()
        .split_once(' ')
        .unwrap()
        .1
        .chars()
        .fold(0, |acc, t| match t.to_digit(10) {
            None => acc,
            Some(d) => acc * 10 + d as u64,
        });
    let distance = lines
        .next()
        .unwrap()
        .split_once(' ')
        .unwrap()
        .1
        .chars()
        .fold(0, |acc, t| match t.to_digit(10) {
            None => acc,
            Some(d) => acc * 10 + d as u64,
        });

    (1..time)
        .filter(|duration| duration * (time - duration) > distance)
        .count()
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for(EXAMPLE), 288);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(INPUT), 1413720);
}

#[test]
fn b_example() {
    assert_eq!(solve_b_for(EXAMPLE), 71503);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(INPUT), 30565288);
}

pub fn solve_a() {
    println!("{}", solve_a_for(INPUT));
}

pub fn solve_b() {
    println!("{}", solve_b_for(INPUT));
}
