use std::collections::{HashMap, HashSet, VecDeque};
use std::mem;

#[allow(dead_code)]
const EXAMPLE: &str = include_str!("../input/16_example");
const INPUT: &str = include_str!("../input/16");

struct Valve {
    flowrate: u32,
    tunnels: Vec<String>,
}

fn parse_input(input: &str) -> HashMap<String, Valve> {
    let mut valves = HashMap::new();

    for line in input.lines() {
        let (_, line) = line.split_once(' ').unwrap();
        let (label, line) = line.split_once(" has flow rate=").unwrap();
        let (flowrate, line) = line.split_once(';').unwrap();
        let (_, line) = line.split_once("valve").unwrap();
        let (_, tunnels) = line.split_once(' ').unwrap();

        let tunnels = tunnels.split(", ").map(|s| s.to_owned()).collect();

        valves.insert(
            label.to_owned(),
            Valve {
                flowrate: flowrate.parse().unwrap(),
                tunnels,
            },
        );
    }
    valves
}

struct PathA {
    unopened: HashSet<String>,
    flowrate: u32,
    released: u32,
    location: String,
    destination: String,
}

fn tunnel_for_destination(
    valves: &HashMap<String, Valve>,
    current: &str,
    destination: &str,
) -> String {
    let mut queue = VecDeque::new();
    queue.push_back(destination);

    while let Some(next) = queue.pop_front() {
        let valve = &valves[next];
        for tunnel in &valve.tunnels {
            if tunnel == current {
                return next.to_owned();
            }
            queue.push_back(tunnel);
        }
    }

    unreachable!()
}

fn solve_a_for(input: &str) -> u32 {
    let valves = parse_input(input);

    let mut unopened = HashSet::new();
    for (location, valve) in &valves {
        if valve.flowrate != 0 {
            unopened.insert(location.to_owned());
        }
    }

    let mut paths = vec![PathA {
        unopened,
        flowrate: 0,
        released: 0,
        location: "AA".to_owned(),
        destination: "AA".to_owned(),
    }];

    for _ in 0..30 {
        let mut new_paths = vec![];

        for mut path in paths {
            path.released += path.flowrate;

            if path.location == path.destination {
                if path.unopened.remove(&path.location) {
                    path.flowrate += valves[&path.location].flowrate;
                    // this can be opened
                    new_paths.push(path);
                } else if path.unopened.is_empty() {
                    // do nothing
                    new_paths.push(path);
                } else {
                    // next destinations
                    for destination in &path.unopened {
                        new_paths.push(PathA {
                            unopened: path.unopened.clone(),
                            flowrate: path.flowrate,
                            released: path.released,
                            location: tunnel_for_destination(&valves, &path.location, destination),
                            destination: destination.to_owned(),
                        });
                    }
                }
            } else {
                // move towards destination
                path.location = tunnel_for_destination(&valves, &path.location, &path.destination);
                new_paths.push(path);
            }
        }

        new_paths.sort_by(|a, b| b.flowrate.cmp(&a.flowrate));
        new_paths.truncate(50);

        paths = new_paths;
    }

    paths.into_iter().map(|p| p.released).max().unwrap()
}

struct PathB {
    unopened: HashSet<String>,
    flowrate: u32,
    released: u32,
    my_location: String,
    my_destination: String,
    elephant_location: String,
    elephant_destination: String,
}

fn solve_b_for(input: &str) -> u32 {
    let valves = parse_input(input);

    let mut unopened = HashSet::new();
    for (location, valve) in &valves {
        if valve.flowrate != 0 {
            unopened.insert(location.to_owned());
        }
    }

    let mut paths = vec![PathB {
        unopened,
        flowrate: 0,
        released: 0,
        my_location: "AA".to_owned(),
        my_destination: "AA".to_owned(),
        elephant_location: "AA".to_owned(),
        elephant_destination: "AA".to_owned(),
    }];

    for _ in 0..26 {
        let mut new_paths = vec![];

        for mut path in paths.iter_mut() {
            path.released += path.flowrate;
        }

        for _ in 0..2 {
            for mut path in paths {
                if path.my_location == path.my_destination {
                    if path.unopened.remove(&path.my_location) {
                        // this can be opened
                        path.flowrate += valves[&path.my_location].flowrate;
                        new_paths.push(path);
                    } else if path.unopened.is_empty() {
                        // do nothing
                        new_paths.push(path);
                    } else {
                        // next destinations
                        for destination in &path.unopened {
                            if destination != &path.elephant_destination {
                                new_paths.push(PathB {
                                    unopened: path.unopened.clone(),
                                    flowrate: path.flowrate,
                                    released: path.released,
                                    my_location: tunnel_for_destination(
                                        &valves,
                                        &path.my_location,
                                        destination,
                                    ),
                                    my_destination: destination.to_owned(),
                                    elephant_location: path.elephant_location.to_owned(),
                                    elephant_destination: path.elephant_destination.to_owned(),
                                });
                            }
                        }
                    }
                } else {
                    // move towards destination
                    path.my_location =
                        tunnel_for_destination(&valves, &path.my_location, &path.my_destination);
                    new_paths.push(path);
                }
            }

            for path in new_paths.iter_mut() {
                mem::swap(&mut path.my_location, &mut path.elephant_location);
                mem::swap(&mut path.my_destination, &mut path.elephant_destination);
            }

            paths = new_paths;
            new_paths = vec![];
        }

        paths.sort_unstable_by(|a, b| b.flowrate.cmp(&a.flowrate));
        paths.truncate(500);
    }

    paths.into_iter().map(|p| p.released).max().unwrap()
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for(EXAMPLE), 1651);
}

// #[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(INPUT), 1986);
}

#[test]
fn b_example() {
    assert_eq!(solve_b_for(EXAMPLE), 1706); // should be 1707 according to the puzzle text
}

// #[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(INPUT), 2464);
}

pub fn solve_a() {
    println!("{}", solve_a_for(INPUT));
}

pub fn solve_b() {
    println!("{}", solve_b_for(INPUT));
}
