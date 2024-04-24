use std::collections::HashSet;

use crate::day::Day;
use crate::puzzle_result::PuzzleResult;

#[cfg(test)]
const EXAMPLE: &str = include_str!("input/3_example");
const INPUT: &str = include_str!("input/3");

fn solve_a_for(input: &str) -> u32 {
    let mut sum = 0;

    for line in input.lines() {
        let len = line.len();
        let middle = len / 2;
        let (left, right) = line.split_at(middle);

        let mut item_types_seen = HashSet::new();
        for item_type in left.chars() {
            item_types_seen.insert(item_type);
        }

        let duplicate_item_type = right
            .chars()
            .find(|item_type| item_types_seen.contains(item_type))
            .expect("no duplicate item type found");
        sum += item_type_to_priority(duplicate_item_type);
    }

    sum
}

fn solve_b_for(input: &str) -> u32 {
    let mut sum = 0u32;

    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        let mut common_item_types = item_types(line);

        let elf_item_types = item_types(lines.next().unwrap());
        let item_types_to_check = common_item_types.clone();
        for item_type in item_types_to_check {
            if !elf_item_types.contains(&item_type) {
                common_item_types.remove(&item_type);
            }
        }
        let elf_item_types = item_types(lines.next().unwrap());
        let item_types_to_check = common_item_types.clone();
        for item_type in item_types_to_check {
            if !elf_item_types.contains(&item_type) {
                common_item_types.remove(&item_type);
            }
        }

        sum += item_type_to_priority(common_item_types.into_iter().next().unwrap());
    }
    sum
}

fn item_type_to_priority(item_type: char) -> u32 {
    item_type as u32
        - if item_type.is_uppercase() {
            'A' as u32 - 27
        } else {
            'a' as u32 - 1
        }
}

fn item_types(line: &str) -> HashSet<char> {
    let mut item_types = HashSet::new();
    for item_type in line.chars() {
        item_types.insert(item_type);
    }

    item_types
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for(EXAMPLE), 157);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(INPUT), 7446);
}

#[test]
fn b_example() {
    assert_eq!(solve_b_for(EXAMPLE), 70);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(INPUT), 2646);
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
