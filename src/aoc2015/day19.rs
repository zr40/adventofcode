use std::collections::HashSet;

use crate::day::Day;
use crate::PuzzleResult;

#[cfg(test)]
const EXAMPLE_A1: &str = include_str!("input/19a_example1");
#[cfg(test)]
const EXAMPLE_A2: &str = include_str!("input/19a_example2");
#[cfg(test)]
const EXAMPLE_B1: &str = include_str!("input/19b_example1");
#[cfg(test)]
const EXAMPLE_B2: &str = include_str!("input/19b_example2");
const INPUT: &str = include_str!("input/19");

fn solve_a_for(input: &str) -> usize {
    let mut replacements = vec![];
    let mut new_molecules = HashSet::new();

    let mut lines = input.lines();
    for line in &mut lines {
        if line.is_empty() {
            break;
        }

        let (find, replace) = line.split_once(" => ").unwrap();

        replacements.push((find.as_bytes(), replace.as_bytes()));
    }

    let molecule = lines.next().unwrap().as_bytes();

    for i in 0..molecule.len() {
        for (find, replace) in &replacements {
            if molecule[i..].starts_with(find) {
                let mut new_molecule: Vec<u8> = vec![];
                new_molecule.extend_from_slice(&molecule[..i]);
                new_molecule.extend_from_slice(replace);
                new_molecule.extend_from_slice(&molecule[i + find.len()..]);
                new_molecules.insert(new_molecule);
            }
        }
    }

    new_molecules.len()
}

fn solve_b_for(input: &str) -> usize {
    let mut replacements = vec![];

    let mut lines = input.lines();
    for line in &mut lines {
        if line.is_empty() {
            break;
        }

        let (find, replace) = line.split_once(" => ").unwrap();

        replacements.push((find.as_bytes(), replace.as_bytes()));
    }

    let mut queue: Vec<(Vec<u8>, usize)> = Vec::new();

    queue.push((lines.next().unwrap().as_bytes().to_vec(), 0));

    while let Some((molecule, steps)) = queue.pop() {
        for i in 0..molecule.len() {
            for (replace, find) in &replacements {
                if molecule[i..].starts_with(find) {
                    let mut new_molecule: Vec<u8> = vec![];
                    new_molecule.extend_from_slice(&molecule[..i]);
                    new_molecule.extend_from_slice(replace);
                    new_molecule.extend_from_slice(&molecule[i + find.len()..]);

                    if new_molecule.len() == 1 && new_molecule[0] == b'e' {
                        // assume there is only a single solution
                        return steps + 1;
                    }

                    queue.push((new_molecule, steps + 1));
                }
            }
        }
    }

    panic!("no solution found");
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for(EXAMPLE_A1), 4);
    assert_eq!(solve_a_for(EXAMPLE_A2), 7);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(INPUT), 535);
}

#[test]
fn b_example() {
    assert_eq!(solve_b_for(EXAMPLE_B1), 3);
    assert_eq!(solve_b_for(EXAMPLE_B2), 6);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(INPUT), 212);
}

fn solve_a() -> PuzzleResult {
    solve_a_for(INPUT).into()
}

fn solve_b() -> PuzzleResult {
    solve_b_for(INPUT).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
