use std::collections::BTreeMap;

use itertools::Itertools;

use crate::common::aoc::input_for;
use crate::day::Day;
use crate::puzzle_result::PuzzleResult;

#[cfg(test)]
const EXAMPLE: &str = include_str!("example/4");

struct Room {
    name: String,
    sector_id: u32,
    checksum: String,
}

impl Room {
    fn from(line: &str) -> Room {
        let (rest, _) = line.rsplit_once(']').unwrap();
        let (rest, checksum) = rest.rsplit_once('[').unwrap();
        let (name, sector_id) = rest.rsplit_once('-').unwrap();
        let sector_id = sector_id.parse().unwrap();

        Room {
            name: name.to_owned(),
            sector_id,
            checksum: checksum.to_owned(),
        }
    }

    fn checksum_valid(&self) -> bool {
        let mut chars = BTreeMap::new();

        for ch in self.name.chars() {
            if ch == '-' {
                continue;
            }

            chars.entry(ch).and_modify(|e| *e += 1).or_insert(1);
        }

        let checksum: String = chars
            .iter()
            .sorted_by_key(|(_, count)| -*count)
            .take(5)
            .map(|(ch, _)| ch)
            .collect();

        checksum == self.checksum
    }

    fn decrypted_name(&self) -> String {
        self.name
            .chars()
            .map(|ch| match ch {
                '-' => ' ',
                ch => {
                    char::from_u32((ch as u32 + self.sector_id - ('a' as u32)) % 26 + ('a' as u32))
                        .unwrap()
                }
            })
            .collect()
    }
}

fn solve_a_for(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|line| {
            let room = Room::from(line);
            room.checksum_valid().then_some(room.sector_id)
        })
        .sum()
}

fn solve_b_for(input: &str) -> u32 {
    input
        .lines()
        .find_map(|line| {
            let room = Room::from(line);
            (room.checksum_valid() && room.decrypted_name() == "northpole object storage")
                .then_some(room)
        })
        .unwrap()
        .sector_id
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for(EXAMPLE), 1514);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(&input_for(2016, 4)), 409147);
}

fn solve_a() -> PuzzleResult {
    solve_a_for(&input_for(2016, 4)).into()
}

#[test]
fn b_example() {
    assert_eq!(
        Room::from("qzmt-zixmtkozy-ivhz-343[]").decrypted_name(),
        "very encrypted name"
    );
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(&input_for(2016, 4)), 991);
}

fn solve_b() -> PuzzleResult {
    solve_b_for(&input_for(2016, 4)).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
