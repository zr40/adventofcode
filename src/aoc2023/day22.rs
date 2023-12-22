use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::PuzzleResult;

#[cfg(test)]
const EXAMPLE: &str = include_str!("input/22_example");
#[cfg_attr(debug_assertions, allow(dead_code))]
const INPUT: &str = include_str!("input/22");

#[derive(Debug, PartialEq, Clone)]
struct Brick {
    x_low: u32,
    y_low: u32,
    z_low: u32,
    x_high: u32,
    y_high: u32,
    z_high: u32,
}

fn parse(input: &str) -> Vec<Brick> {
    input
        .lines()
        .map(|line| {
            let (x1, line) = line.split_once(',').unwrap();
            let (y1, line) = line.split_once(',').unwrap();
            let (z1, line) = line.split_once('~').unwrap();
            let (x2, line) = line.split_once(',').unwrap();
            let (y2, z2) = line.split_once(',').unwrap();
            Brick {
                x_low: x1.parse().unwrap(),
                y_low: y1.parse().unwrap(),
                z_low: z1.parse().unwrap(),
                x_high: x2.parse().unwrap(),
                y_high: y2.parse().unwrap(),
                z_high: z2.parse().unwrap(),
            }
        })
        .collect()
}

#[cfg_attr(debug_assertions, allow(dead_code))]
fn solve_a_for(input: &str) -> usize {
    let mut bricks = parse(input);

    bricks.sort_by_key(|brick| brick.z_low);

    for i in 0..bricks.len() {
        let brick = &bricks[i];
        let support = bricks
            .iter()
            .filter_map(|other| {
                if brick != other
                    && brick.x_low <= other.x_high
                    && other.x_low <= brick.x_high
                    && brick.y_low <= other.y_high
                    && other.y_low <= brick.y_high
                    && brick.z_low >= other.z_high
                {
                    Some(other.z_high + 1)
                } else {
                    None
                }
            })
            .max()
            .unwrap_or(1);
        let brick = &mut bricks[i];
        let movement = brick.z_low - support;
        brick.z_low -= movement;
        brick.z_high -= movement;
    }

    bricks
        .par_iter()
        .filter(|removed| {
            let result = bricks.iter().any(|brick| {
                if brick == *removed {
                    false
                } else {
                    let new_z = bricks
                        .iter()
                        .filter_map(|other| {
                            if other == brick || other == *removed {
                                None
                            } else if brick != other
                                && brick.x_low <= other.x_high
                                && other.x_low <= brick.x_high
                                && brick.y_low <= other.y_high
                                && other.y_low <= brick.y_high
                                && brick.z_low >= other.z_high
                            {
                                Some(other.z_high + 1)
                            } else {
                                None
                            }
                        })
                        .max()
                        .unwrap_or(1);

                    new_z != brick.z_low
                }
            });
            !result
        })
        .count()
}

#[cfg_attr(debug_assertions, allow(dead_code))]
fn solve_b_for(input: &str) -> usize {
    let mut bricks = parse(input);

    bricks.sort_by_key(|brick| brick.z_low);

    for i in 0..bricks.len() {
        let brick = &bricks[i];
        let support = bricks
            .iter()
            .filter_map(|other| {
                if brick != other
                    && brick.x_low <= other.x_high
                    && other.x_low <= brick.x_high
                    && brick.y_low <= other.y_high
                    && other.y_low <= brick.y_high
                    && brick.z_low >= other.z_high
                {
                    Some(other.z_high + 1)
                } else {
                    None
                }
            })
            .max()
            .unwrap_or(1);
        let brick = &mut bricks[i];
        let movement = brick.z_low - support;
        brick.z_low -= movement;
        brick.z_high -= movement;
    }

    bricks
        .par_iter()
        .map(|removed| {
            let mut workspace = bricks.clone();
            let mut moved = 0;
            for i in 0..workspace.len() {
                let brick = &workspace[i];
                if brick == removed {
                    continue;
                }
                let support = workspace
                    .iter()
                    .filter_map(|other| {
                        if removed != other
                            && brick != other
                            && brick.x_low <= other.x_high
                            && other.x_low <= brick.x_high
                            && brick.y_low <= other.y_high
                            && other.y_low <= brick.y_high
                            && brick.z_low >= other.z_high
                        {
                            Some(other.z_high + 1)
                        } else {
                            None
                        }
                    })
                    .max()
                    .unwrap_or(1);
                let brick = &mut workspace[i];
                let movement = brick.z_low - support;
                if movement > 0 {
                    brick.z_low -= movement;
                    brick.z_high -= movement;
                    moved += 1;
                }
            }
            moved
        })
        .sum()
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for(EXAMPLE), 5);
}

#[test]
#[cfg_attr(debug_assertions, ignore)]
fn a_puzzle() {
    assert_eq!(solve_a_for(INPUT), 393);
}

#[test]
fn b_example() {
    assert_eq!(solve_b_for(EXAMPLE), 7);
}

#[test]
#[cfg_attr(debug_assertions, ignore)]
fn b_puzzle() {
    assert_eq!(solve_b_for(INPUT), 58440);
}

pub fn solve_a() -> PuzzleResult {
    #[cfg(debug_assertions)]
    return PuzzleResult::SkipSlow;

    #[cfg(not(debug_assertions))]
    solve_a_for(INPUT).into()
}

pub fn solve_b() -> PuzzleResult {
    #[cfg(debug_assertions)]
    return PuzzleResult::SkipSlow;

    #[cfg(not(debug_assertions))]
    solve_b_for(INPUT).into()
}
