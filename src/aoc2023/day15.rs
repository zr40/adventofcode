use crate::day::Day;
use crate::PuzzleResult;

#[cfg(test)]
const EXAMPLE: &str = include_str!("input/15_example");
const INPUT: &str = include_str!("input/15");

fn hash(step: &str) -> usize {
    step.chars()
        .fold(0, |acc, ch| ((acc + ch as usize) * 17) % 256)
}

fn solve_a_for(input: &str) -> usize {
    input.split(',').map(hash).sum()
}

enum Operation {
    Minus,
    Equals,
}

struct Lens {
    label: String,
    value: usize,
}

fn solve_b_for(input: &str) -> usize {
    let mut hashmap: Vec<Vec<Lens>> = vec![];
    for _ in 0..256 {
        hashmap.push(vec![]);
    }

    for step in input.split(',') {
        let (operation, label, value) = if step.contains('-') {
            (Operation::Minus, &step[..step.len() - 1], "")
        } else {
            let (label, value) = step.split_once('=').unwrap();
            (Operation::Equals, label, value)
        };

        let hash = hash(label);
        let slot = &mut hashmap[hash];

        let existing_lens = slot.iter().position(|lens| lens.label == label);

        match (operation, existing_lens) {
            (Operation::Minus, Some(index)) => {
                slot.remove(index);
            }
            (Operation::Minus, None) => {}
            (Operation::Equals, Some(index)) => {
                let value = value.parse().unwrap();
                slot[index].value = value;
            }
            (Operation::Equals, None) => {
                let value = value.parse().unwrap();
                slot.push(Lens {
                    label: label.to_owned(),
                    value,
                });
            }
        }
    }

    hashmap
        .into_iter()
        .enumerate()
        .map(|(index, slot)| {
            slot.into_iter()
                .enumerate()
                .map(|(index2, lens)| (index2 + 1) * lens.value)
                .sum::<usize>()
                * (index + 1)
        })
        .sum()
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for(EXAMPLE), 1320);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(INPUT), 516070);
}

#[test]
fn b_example() {
    assert_eq!(solve_b_for(EXAMPLE), 145);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(INPUT), 244981);
}

fn solve_a() -> PuzzleResult {
    solve_a_for(INPUT).into()
}

fn solve_b() -> PuzzleResult {
    solve_b_for(INPUT).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
