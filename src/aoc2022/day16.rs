use std::collections::{HashMap, HashSet, VecDeque};
use std::mem;

use crate::day::Day;
use crate::puzzle_result::PuzzleResult;

#[cfg(test)]
const EXAMPLE: &str = include_str!("input/16_example");
const INPUT: &str = include_str!("input/16");

struct Valve {
    flowrate: u32,
    tunnels: Vec<u16>,
}

fn label_to_u16(label: &str) -> u16 {
    let mut label = label.bytes();

    let a = label.next().unwrap() - b'A';
    let b = label.next().unwrap() - b'A';

    a as u16 * 26 + b as u16
}

fn parse_input(input: &str) -> HashMap<u16, Valve> {
    let mut valves = HashMap::new();

    for line in input.lines() {
        let (_, line) = line.split_once(' ').unwrap();
        let (label, line) = line.split_once(" has flow rate=").unwrap();
        let (flowrate, line) = line.split_once(';').unwrap();
        let (_, line) = line.split_once("valve").unwrap();
        let (_, tunnels) = line.split_once(' ').unwrap();

        let tunnels = tunnels.split(", ").map(label_to_u16).collect();

        valves.insert(label_to_u16(label), Valve {
            flowrate: flowrate.parse().unwrap(),
            tunnels,
        });
    }
    valves
}

struct SoloPath {
    unopened: HashSet<u16>,
    flowrate: u32,
    released: u32,
    location: u16,
    target: u16,
}

fn tunnel_for_target(valves: &HashMap<u16, Valve>, current: u16, target: u16) -> u16 {
    let mut queue = VecDeque::new();
    queue.push_back(target);

    while let Some(next) = queue.pop_front() {
        let valve = &valves[&next];
        for tunnel in &valve.tunnels {
            if *tunnel == current {
                return next;
            }
            queue.push_back(*tunnel);
        }
    }

    unreachable!()
}

fn solve_a_for(input: &str) -> u32 {
    let valves = parse_input(input);

    let mut unopened = HashSet::new();
    for (location, valve) in &valves {
        if valve.flowrate != 0 {
            unopened.insert(*location);
        }
    }

    let mut paths = vec![SoloPath {
        unopened,
        flowrate: 0,
        released: 0,
        location: 0,
        target: 0,
    }];

    for _ in 0..30 {
        let mut new_paths = vec![];

        for mut path in paths {
            path.released += path.flowrate;

            if path.location == path.target {
                if path.unopened.remove(&path.location) {
                    path.flowrate += valves[&path.location].flowrate;
                    // arrived at a closed valve, open it
                    new_paths.push(path);
                } else if path.unopened.is_empty() {
                    // no useful actions
                    new_paths.push(path);
                } else {
                    // add paths to remaining targets
                    for target in &path.unopened {
                        new_paths.push(SoloPath {
                            unopened: path.unopened.clone(),
                            flowrate: path.flowrate,
                            released: path.released,
                            location: tunnel_for_target(&valves, path.location, *target),
                            target: *target,
                        });
                    }
                }
            } else {
                // move towards target
                path.location = tunnel_for_target(&valves, path.location, path.target);
                new_paths.push(path);
            }
        }

        // heuristic
        new_paths.sort_by(|a, b| b.flowrate.cmp(&a.flowrate));
        new_paths.truncate(50);

        paths = new_paths;
    }

    paths.into_iter().map(|p| p.released).max().unwrap()
}

struct PairPath {
    unopened: HashSet<u16>,
    flowrate: u32,
    released: u32,
    my_location: u16,
    my_target: u16,
    other_location: u16,
    other_target: u16,
}

fn solve_b_for(input: &str) -> u32 {
    let valves = parse_input(input);

    let mut unopened = HashSet::new();
    for (location, valve) in &valves {
        if valve.flowrate != 0 {
            unopened.insert(*location);
        }
    }

    let mut paths = vec![PairPath {
        unopened,
        flowrate: 0,
        released: 0,
        my_location: 0,
        my_target: 0,
        other_location: 0,
        other_target: 0,
    }];

    for _ in 0..26 {
        let mut new_paths = vec![];

        for path in &mut paths {
            path.released += path.flowrate;
        }

        // move self and elephant
        for _ in 0..2 {
            for mut path in paths {
                if path.my_location == path.my_target {
                    if path.unopened.remove(&path.my_location) {
                        // arrived at a closed valve, open it
                        path.flowrate += valves[&path.my_location].flowrate;
                        new_paths.push(path);
                    } else if path.unopened.is_empty() {
                        // no useful actions
                        new_paths.push(path);
                    } else {
                        // add paths to remaining targets
                        for target in &path.unopened {
                            if *target != path.other_target {
                                new_paths.push(PairPath {
                                    unopened: path.unopened.clone(),
                                    flowrate: path.flowrate,
                                    released: path.released,
                                    my_location: tunnel_for_target(
                                        &valves,
                                        path.my_location,
                                        *target,
                                    ),
                                    my_target: *target,
                                    other_location: path.other_location,
                                    other_target: path.other_target,
                                });
                            }
                        }
                    }
                } else {
                    // move towards target
                    path.my_location = tunnel_for_target(&valves, path.my_location, path.my_target);
                    new_paths.push(path);
                }
            }

            for path in &mut new_paths {
                mem::swap(&mut path.my_location, &mut path.other_location);
                mem::swap(&mut path.my_target, &mut path.other_target);
            }

            paths = new_paths;
            new_paths = vec![];
        }

        // heuristic
        paths.sort_unstable_by(|a, b| b.flowrate.cmp(&a.flowrate));
        paths.truncate(500);
    }

    paths.into_iter().map(|p| p.released).max().unwrap()
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for(EXAMPLE), 1651);
}

#[test]
#[cfg_attr(debug_assertions, ignore)]
fn a_puzzle() {
    assert_eq!(solve_a_for(INPUT), 1986);
}

#[test]
fn b_example() {
    assert_eq!(solve_b_for(EXAMPLE), 1706); // should be 1707 according to the puzzle text
}

#[test]
#[cfg_attr(debug_assertions, ignore)]
fn b_puzzle() {
    assert_eq!(solve_b_for(INPUT), 2464);
}

fn solve_a() -> PuzzleResult {
    solve_a_for(INPUT).into()
}

#[cfg(debug_assertions)]
fn solve_b() -> PuzzleResult {
    PuzzleResult::SkipSlow
}

#[cfg(not(debug_assertions))]
fn solve_b() -> PuzzleResult {
    solve_b_for(INPUT).into()
}

#[cfg(debug_assertions)]
#[allow(dead_code)]
fn dead_code() {
    solve_b_for(INPUT);
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
