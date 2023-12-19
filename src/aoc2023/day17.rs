use std::collections::btree_map::Entry;
use std::collections::{BTreeMap, BinaryHeap};

use crate::common::direction::Direction;
use crate::PuzzleResult;

#[cfg(test)]
const EXAMPLE: &str = include_str!("input/17_example");
#[cfg(test)]
const EXAMPLE_B: &str = include_str!("input/17b_example");
#[cfg_attr(debug_assertions, allow(dead_code))]
const INPUT: &str = include_str!("input/17");

#[cfg_attr(debug_assertions, allow(dead_code))]
fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap()).collect())
        .collect()
}

#[cfg_attr(debug_assertions, allow(dead_code))]
enum Mode {
    Crucible,
    UltraCrucible,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
struct State {
    x: usize,
    y: usize,
    direction: Direction,
    straight_length: usize,
}

#[derive(PartialEq, Eq)]
struct QueueItem {
    state: State,
    distance: u32,
}

impl PartialOrd for QueueItem {
    #[allow(clippy::non_canonical_partial_ord_impl)]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.distance.partial_cmp(&self.distance)
    }
}

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.distance.cmp(&self.distance)
    }
}

#[cfg_attr(debug_assertions, allow(dead_code))]
fn solve_for(input: &str, mode: Mode) -> u32 {
    let map = parse(input);
    let mut visited: BTreeMap<State, u32> = BTreeMap::new();
    let mut queue = BinaryHeap::new();

    queue.push(QueueItem {
        state: State {
            x: 0,
            y: 0,
            direction: Direction::East,
            straight_length: 0,
        },
        distance: 0,
    });

    while let Some(QueueItem {
        state: visit,
        distance,
    }) = queue.pop()
    {
        let entry = visited.entry(visit);
        match entry {
            Entry::Occupied(mut entry) => {
                let previous_distance = entry.get_mut();
                if *previous_distance <= distance {
                    continue;
                }
                *previous_distance = distance;
            }
            Entry::Vacant(e) => {
                e.insert(distance);
            }
        }

        if visit.x == map[0].len() - 1 && visit.y == map.len() - 1 {
            continue;
        }

        if visit.straight_length
            < match mode {
                Mode::Crucible => 3,
                Mode::UltraCrucible => 10,
            }
        {
            if let Some((new_x, new_y)) =
                visit
                    .direction
                    .step(visit.x, visit.y, map[0].len(), map.len())
            {
                queue.push(QueueItem {
                    state: State {
                        x: new_x,
                        y: new_y,
                        direction: visit.direction,
                        straight_length: visit.straight_length + 1,
                    },
                    distance: distance + map[new_y][new_x],
                });
            }
        }

        if match mode {
            Mode::Crucible => true,
            Mode::UltraCrucible => visit.straight_length >= 4 && visit.straight_length <= 10,
        } {
            let left = visit.direction.left();
            if let Some((new_x, new_y)) = left.step(visit.x, visit.y, map[0].len(), map.len()) {
                queue.push(QueueItem {
                    state: State {
                        x: new_x,
                        y: new_y,
                        direction: left,
                        straight_length: 1,
                    },
                    distance: distance + map[new_y][new_x],
                });
            }
            let right = visit.direction.right();
            if let Some((new_x, new_y)) = right.step(visit.x, visit.y, map[0].len(), map.len()) {
                queue.push(QueueItem {
                    state: State {
                        x: new_x,
                        y: new_y,
                        direction: right,
                        straight_length: 1,
                    },
                    distance: distance + map[new_y][new_x],
                });
            }
        }
    }

    visited
        .into_iter()
        .filter(|(v, _)| {
            v.x == map[0].len() - 1
                && v.y == map.len() - 1
                && match mode {
                    Mode::Crucible => true,
                    Mode::UltraCrucible => v.straight_length >= 4,
                }
        })
        .map(|(_, d)| d)
        .min()
        .unwrap()
}

#[test]
fn a_example() {
    assert_eq!(solve_for(EXAMPLE, Mode::Crucible), 102);
}

#[test]
#[cfg_attr(debug_assertions, ignore)]
fn a_puzzle() {
    assert_eq!(solve_for(INPUT, Mode::Crucible), 635);
}

#[test]
fn b_example() {
    assert_eq!(solve_for(EXAMPLE, Mode::UltraCrucible), 94);
    assert_eq!(solve_for(EXAMPLE_B, Mode::UltraCrucible), 71);
}

#[test]
#[cfg_attr(debug_assertions, ignore)]
fn b_puzzle() {
    assert_eq!(solve_for(INPUT, Mode::UltraCrucible), 734);
}

pub fn solve_a() -> PuzzleResult {
    #[cfg(debug_assertions)]
    return PuzzleResult::SkipSlow;

    #[cfg(not(debug_assertions))]
    solve_for(INPUT, Mode::Crucible).into()
}

pub fn solve_b() -> PuzzleResult {
    #[cfg(debug_assertions)]
    return PuzzleResult::SkipSlow;

    #[cfg(not(debug_assertions))]
    solve_for(INPUT, Mode::UltraCrucible).into()
}
