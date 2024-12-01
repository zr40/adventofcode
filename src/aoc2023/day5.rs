use std::collections::BTreeMap;

use crate::PuzzleResult;
use crate::common::aoc::input_for;
use crate::day::Day;

#[cfg(test)]
const EXAMPLE: &str = include_str!("example/5");

enum Mode {
    PartA,
    PartB,
}

fn solve_for(input: &str, mode: Mode) -> u64 {
    let mut lines = input.lines();
    let mut seeds = lines.next().unwrap().split(' ');
    seeds.next();
    let mut current_ranges: BTreeMap<u64, u64> = BTreeMap::new();

    while let Some(range_start) = seeds.next() {
        current_ranges.insert(range_start.parse().unwrap(), match mode {
            Mode::PartA => 1,
            Mode::PartB => seeds.next().unwrap().parse().unwrap(),
        });
    }

    while let Some(line) = lines.next() {
        if line.is_empty() {
            continue;
        }
        assert!(line.ends_with(" map:"));

        let mut new_ranges: BTreeMap<u64, u64> = BTreeMap::new();

        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            }

            let mut parts = line.split(' ').map(|n| n.parse().unwrap());

            let map_destination_start: u64 = parts.next().unwrap();
            let map_source_start: u64 = parts.next().unwrap();
            let map_length: u64 = parts.next().unwrap();
            let map_source_end = map_source_start + map_length;

            let overlapping_ranges: Vec<(u64, u64)> = current_ranges
                .extract_if(|current_start, current_length| {
                    let current_end = *current_start + *current_length;

                    *current_start < map_source_end && map_source_start < current_end
                })
                .collect();

            for (current_start, current_length) in overlapping_ranges {
                let current_end = current_start + current_length;

                if current_start < map_source_start {
                    current_ranges.insert(current_start, map_source_start - current_start);
                }

                if current_end > map_source_end {
                    current_ranges.insert(map_source_end, current_end - map_source_end);
                }

                let overlap_start = current_start.max(map_source_start);
                let overlap_end = current_end.min(map_source_end);
                new_ranges.insert(
                    overlap_start + map_destination_start - map_source_start,
                    overlap_end - overlap_start,
                );
            }
        }
        new_ranges.append(&mut current_ranges);
        current_ranges = new_ranges;
    }

    *current_ranges.first_key_value().unwrap().0
}

#[test]
fn a_example() {
    assert_eq!(solve_for(EXAMPLE, Mode::PartA), 35);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_for(&input_for(2023, 5), Mode::PartA), 457535844);
}

fn solve_a() -> PuzzleResult {
    solve_for(&input_for(2023, 5), Mode::PartA).into()
}

#[test]
fn b_example() {
    assert_eq!(solve_for(EXAMPLE, Mode::PartB), 46);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_for(&input_for(2023, 5), Mode::PartB), 41222968);
}

fn solve_b() -> PuzzleResult {
    solve_for(&input_for(2023, 5), Mode::PartB).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
