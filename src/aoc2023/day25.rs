use std::collections::HashMap;
use std::collections::hash_map::Entry;

use crate::PuzzleResult;
use crate::day::Day;

#[cfg(test)]
const EXAMPLE: &str = include_str!("input/25_example");
const INPUT: &str = include_str!("input/25");

fn parse(input: &str) -> HashMap<usize, Vec<usize>> {
    let mut known_nodes: HashMap<&str, usize> = HashMap::new();
    let mut routes: HashMap<usize, Vec<usize>> = HashMap::new();

    // hardcoded from input
    known_nodes.insert("bbg", 0);
    known_nodes.insert("htb", 1);
    known_nodes.insert("dlk", 2);
    known_nodes.insert("pjj", 3);
    known_nodes.insert("htj", 4);
    known_nodes.insert("pcc", 5);

    for line in input.lines() {
        let (src, rest) = line.split_once(": ").unwrap();

        if !known_nodes.contains_key(src) {
            known_nodes.insert(src, known_nodes.len());
        }

        let src = known_nodes[src];

        for item in rest.split(' ') {
            if !known_nodes.contains_key(item) {
                known_nodes.insert(item, known_nodes.len());
            }
            let item = known_nodes[item];

            match (src, item) {
                (0, 1) | (2, 3) | (4, 5) | (1, 0) | (3, 2) | (5, 4) => continue,
                _ => {}
            }

            match routes.entry(src) {
                Entry::Occupied(mut entry) => {
                    entry.get_mut().push(item);
                }
                Entry::Vacant(entry) => {
                    entry.insert(vec![item]);
                }
            }

            match routes.entry(item) {
                Entry::Occupied(mut entry) => {
                    entry.get_mut().push(src);
                }
                Entry::Vacant(entry) => {
                    entry.insert(vec![src]);
                }
            }
        }
    }
    routes
}

fn solve_for(input: &str) -> usize {
    let mut routes = parse(input);
    let mut found = 0;
    let mut queue = vec![*routes.keys().next().unwrap()];
    while let Some(item) = queue.pop() {
        if let Some(items) = routes.remove(&item) {
            found += 1;
            queue.extend(items);
        }
    }

    found * routes.len()
}

#[test]
#[ignore = "depends on input"]
fn example() {
    assert_eq!(solve_for(EXAMPLE), 54);
}

#[test]
fn puzzle() {
    assert_eq!(solve_for(INPUT), 538560);
}

fn solve() -> PuzzleResult {
    solve_for(INPUT).into()
}

pub(super) static DAY: Day = Day::Single(solve);
