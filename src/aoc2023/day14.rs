use std::collections::HashMap;

use crate::PuzzleResult;

#[cfg(test)]
const EXAMPLE: &str = include_str!("input/14_example");
const INPUT: &str = include_str!("input/14");

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum Space {
    Empty,
    Cube,
    Boulder,
}

fn parse(input: &str) -> Vec<Vec<Space>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    '.' => Space::Empty,
                    '#' => Space::Cube,
                    'O' => Space::Boulder,
                    ch => panic!("unknown contents '{ch}'"),
                })
                .collect()
        })
        .collect()
}

fn solve_a_for(input: &str) -> usize {
    let mut map = parse(input);

    move_north(&mut map);

    map.iter()
        .enumerate()
        .map(|(row, line)| {
            line.iter()
                .map(|space| match space {
                    Space::Boulder => map.len() - row,
                    _ => 0,
                })
                .sum::<usize>()
        })
        .sum()
}

const TARGET: usize = 1000000000;

fn solve_b_for(input: &str) -> usize {
    let mut map = parse(input);
    let mut seen = HashMap::new();

    seen.insert(map.clone(), 0);

    let mut n = 0;
    while n < TARGET {
        n += 1;
        cycle(&mut map);

        if seen.contains_key(&map) {
            let interval = n - seen.get(&map).unwrap();

            let remaining = TARGET - n;
            n += remaining - (remaining % interval);
            break;
        }

        seen.insert(map.clone(), n);
    }

    while n < TARGET {
        n += 1;
        cycle(&mut map);
    }

    map.iter()
        .enumerate()
        .map(|(row, line)| {
            line.iter()
                .map(|space| match space {
                    Space::Boulder => map.len() - row,
                    _ => 0,
                })
                .sum::<usize>()
        })
        .sum()
}

fn move_north(map: &mut [Vec<Space>]) {
    // ^
    for y in 1..map.len() {
        for x in 0..map[0].len() {
            if let Space::Boulder = map[y][x] {
                let mut target = y;
                for to_y in (0..y).rev() {
                    match map[to_y][x] {
                        Space::Empty => target = to_y,
                        _ => {
                            break;
                        }
                    }
                }

                if target != y {
                    map[y][x] = Space::Empty;
                    map[target][x] = Space::Boulder;
                }
            }
        }
    }
}

fn move_west(map: &mut [Vec<Space>]) {
    // <
    for y in 0..map[0].len() {
        for x in 1..map.len() {
            if let Space::Boulder = map[y][x] {
                let mut target = x;
                for to_x in (0..x).rev() {
                    match map[y][to_x] {
                        Space::Empty => target = to_x,
                        _ => {
                            break;
                        }
                    }
                }

                if target != x {
                    map[y][x] = Space::Empty;
                    map[y][target] = Space::Boulder;
                }
            }
        }
    }
}

fn move_south(map: &mut [Vec<Space>]) {
    // v
    for src_y in (0..map.len() - 1).rev() {
        for x in 0..map[0].len() {
            if let Space::Boulder = map[src_y][x] {
                let mut target_y = src_y;
                for (check_y, check_row) in map.iter().enumerate().skip(src_y + 1) {
                    match check_row[x] {
                        Space::Empty => target_y = check_y,
                        _ => {
                            break;
                        }
                    }
                }

                if target_y != src_y {
                    map[src_y][x] = Space::Empty;
                    map[target_y][x] = Space::Boulder;
                }
            }
        }
    }
}

fn move_east(map: &mut [Vec<Space>]) {
    // >
    for y in 0..map.len() {
        for src_x in (0..map[0].len() - 1).rev() {
            if let Space::Boulder = map[y][src_x] {
                let mut target_x = src_x;
                for check_x in src_x + 1..map[y].len() {
                    match map[y][check_x] {
                        Space::Empty => target_x = check_x,
                        _ => {
                            break;
                        }
                    }
                }

                if target_x != src_x {
                    map[y][src_x] = Space::Empty;
                    map[y][target_x] = Space::Boulder;
                }
            }
        }
    }
}

fn cycle(map: &mut [Vec<Space>]) {
    move_north(map);
    move_west(map);
    move_south(map);
    move_east(map);
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for(EXAMPLE), 136);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(INPUT), 108826);
}

#[test]
fn b_example() {
    assert_eq!(solve_b_for(EXAMPLE), 64);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(INPUT), 99291);
}

pub fn solve_a() -> PuzzleResult {
    solve_a_for(INPUT).into()
}

pub fn solve_b() -> PuzzleResult {
    solve_b_for(INPUT).into()
}
