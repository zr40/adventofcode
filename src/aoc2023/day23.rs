use std::collections::BTreeMap;

use crate::common::coordinate::Coordinate;
use crate::common::direction::Direction;
use crate::common::grid::Grid;
use crate::PuzzleResult;

#[cfg(test)]
const EXAMPLE: &str = include_str!("input/23_example");
const INPUT: &str = include_str!("input/23");

enum Mode {
    Slopes,
    Paths,
}

#[derive(Clone, Copy)]
enum Tile {
    Path,
    Forest,
    SlopeRight,
    SlopeDown,
}

fn parse(input: &str, mode: Mode) -> Vec<Vec<Tile>> {
    let map: Vec<Vec<Tile>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| match (ch, &mode) {
                    ('.', _) => Tile::Path,
                    ('#', _) => Tile::Forest,
                    ('>', Mode::Slopes) => Tile::SlopeRight,
                    ('v', Mode::Slopes) => Tile::SlopeDown,
                    ('>' | 'v', Mode::Paths) => Tile::Path,
                    (ch, _) => panic!("unexpected tile '{ch}'"),
                })
                .collect()
        })
        .collect();

    map
}

fn determine_edges(map: Vec<Vec<Tile>>) -> BTreeMap<Coordinate, Vec<(Coordinate, usize)>> {
    let mut paths = BTreeMap::new();
    let mut queue = vec![Coordinate { x: 1, y: 0 }];

    let bounds = map.bounds();

    while let Some(coord) = queue.pop() {
        if paths.contains_key(&coord) {
            continue;
        }
        let mut options = vec![];
        for mut direction in Direction::ALL {
            let mut coord = coord;
            let mut distance = 0;

            while let Some(new_coord) = direction.step(coord, bounds) {
                // move in the previously selected direction
                match (map.at(new_coord), direction) {
                    (Tile::Forest, _) => break,
                    (Tile::Path, _) => {}
                    (Tile::SlopeRight, Direction::Left) => break,
                    (Tile::SlopeRight, Direction::Right) => {}
                    (Tile::SlopeRight, _) => panic!("invalid direction for tile"),
                    (Tile::SlopeDown, Direction::Up) => break,
                    (Tile::SlopeDown, Direction::Down) => {}
                    (Tile::SlopeDown, _) => panic!("invalid direction for tile"),
                }
                coord = new_coord;
                distance += 1;

                // check new directions
                let directions: Vec<_> = Direction::ALL
                    .iter()
                    .filter(|dir| {
                        if dir.opposite() == direction {
                            false
                        } else {
                            match dir.step(coord, bounds) {
                                Some(coord) => match (map.at(coord), **dir) {
                                    (Tile::Forest, _) => false,
                                    (Tile::Path, _) => true,
                                    (Tile::SlopeRight, Direction::Left) => false,
                                    (Tile::SlopeRight, Direction::Right) => true,
                                    (Tile::SlopeRight, _) => {
                                        panic!("invalid direction for tile")
                                    }
                                    (Tile::SlopeDown, Direction::Up) => false,
                                    (Tile::SlopeDown, Direction::Down) => true,
                                    (Tile::SlopeDown, _) => {
                                        panic!("invalid direction for tile")
                                    }
                                },
                                None => false,
                            }
                        }
                    })
                    .collect();

                if directions.len() == 1 {
                    // only one direction possible, continue
                    direction = *directions[0];
                    continue;
                }
                // dead end or junction
                break;
            }
            if distance > 0 {
                options.push((coord, distance));
                queue.push(coord);
            }
        }
        paths.insert(coord, options);
    }

    paths
}

struct State {
    position: Coordinate,
    distance: usize,
    path: Vec<Coordinate>,
}

fn solve_for(input: &str, mode: Mode) -> usize {
    let map = parse(input, mode);
    let target = Coordinate {
        x: map[0].len() - 2,
        y: map.len() - 1,
    };
    let edges_at = determine_edges(map);

    let mut queue = vec![State {
        position: Coordinate { x: 1, y: 0 },
        distance: 0,
        path: vec![],
    }];

    let mut max = 0;

    while let Some(state) = queue.pop() {
        if state.position == target {
            max = max.max(state.distance);
            continue;
        }

        let edges = edges_at.get(&state.position).unwrap();
        for (coord, dist) in edges {
            if state.path.contains(coord) {
                continue;
            }
            let mut new_path = state.path.clone();
            new_path.push(*coord);
            queue.push(State {
                position: *coord,
                distance: dist + state.distance,
                path: new_path,
            });
        }
    }
    max
}

#[test]
fn a_example() {
    assert_eq!(solve_for(EXAMPLE, Mode::Slopes), 94);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_for(INPUT, Mode::Slopes), 2310);
}

#[test]
fn b_example() {
    assert_eq!(solve_for(EXAMPLE, Mode::Paths), 154);
}

#[test]
#[cfg_attr(debug_assertions, ignore)]
fn b_puzzle() {
    assert_eq!(solve_for(INPUT, Mode::Paths), 6738);
}

pub fn solve_a() -> PuzzleResult {
    solve_for(INPUT, Mode::Slopes).into()
}

pub fn solve_b() -> PuzzleResult {
    #[cfg(debug_assertions)]
    return PuzzleResult::SkipSlow;

    #[cfg(not(debug_assertions))]
    solve_for(INPUT, Mode::Paths).into()
}

#[cfg(debug_assertions)]
#[allow(dead_code)]
fn dead_code() {
    solve_for(INPUT, Mode::Paths);
}
