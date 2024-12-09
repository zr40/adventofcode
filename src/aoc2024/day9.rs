use itertools::Itertools;

use crate::PuzzleResult;
use crate::common::aoc::input_for;
use crate::day::Day;

#[cfg(test)]
const EXAMPLE: &str = include_str!("example/9");

fn solve_a_for(input: &str) -> usize {
    let disk_map = input.chars().map(|ch| ch as u16 - '0' as u16).collect_vec();

    let mut blocks = vec![];
    let mut file_id = 0;
    let mut disk_map_iter = disk_map.into_iter();
    loop {
        if let Some(len) = disk_map_iter.next() {
            (0..len).for_each(|_| blocks.push(Some(file_id)));
        } else {
            break;
        }
        file_id += 1;
        if let Some(len) = disk_map_iter.next() {
            (0..len).for_each(|_| blocks.push(None));
        } else {
            break;
        }
    }

    let mut back = blocks.len();
    for front in 0..blocks.len() {
        if front > back {
            continue;
        }
        if blocks[front].is_some() {
            continue;
        }
        loop {
            if back == 0 {
                break;
            }
            back -= 1;
            if blocks[back].is_some() {
                blocks.swap(front, back);
                break;
            }
        }
    }

    blocks
        .into_iter()
        .flatten()
        .enumerate()
        .map(|(idx, file_id)| idx * file_id)
        .sum()
}

fn solve_b_for(input: &str) -> usize {
    let disk_map = input.chars().map(|ch| ch as u16 - '0' as u16).collect_vec();

    let mut blocks = vec![];
    let mut file_id = 0;
    let mut disk_map_iter = disk_map.into_iter();

    loop {
        if let Some(len) = disk_map_iter.next() {
            blocks.push((Some(file_id), len));
        } else {
            break;
        }
        file_id += 1;
        if let Some(len) = disk_map_iter.next() {
            blocks.push((None, len));
        } else {
            break;
        }
    }

    let mut back = blocks.len() - 1;
    while back > 0 {
        let (file_id, file_len) = blocks[back];
        if file_id.is_none() {
            back -= 1;
            continue;
        }

        let target = blocks
            .iter()
            .take(back)
            .position(|(file_id, len)| file_id.is_none() && *len >= file_len);

        if let Some(target) = target {
            let target_len = blocks[target].1;

            blocks.swap(back, target);
            if target_len == file_len {
                back -= 1;
            } else {
                blocks[back].1 = file_len;
                blocks.insert(target + 1, (None, target_len - file_len));
            }
        } else {
            back -= 1;
        }
    }

    blocks
        .into_iter()
        .flat_map(|(file_id, len)| (0..len).map(move |_| file_id))
        .enumerate()
        .map(
            |(idx, file_id)| {
                if let Some(id) = file_id { idx * id } else { 0 }
            },
        )
        .sum()
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for(EXAMPLE), 1928);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(&input_for(2024, 9)), 6291146824486);
}

fn solve_a() -> PuzzleResult {
    solve_a_for(&input_for(2024, 9)).into()
}

#[test]
fn b_example() {
    assert_eq!(solve_b_for(EXAMPLE), 2858);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(&input_for(2024, 9)), 6307279963620);
}

fn solve_b() -> PuzzleResult {
    solve_b_for(&input_for(2024, 9)).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
