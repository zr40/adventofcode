use std::collections::BTreeMap;

#[allow(dead_code)]
const EXAMPLE: &str = include_str!("../input/7_example");
const INPUT: &str = include_str!("../input/7");

fn parse_input(input: &str) -> (Vec<String>, BTreeMap<String, usize>) {
    let mut dirs = vec!["/".to_owned()];
    let mut files: BTreeMap<String, usize> = BTreeMap::new();

    let mut cwd = vec!["/".to_string()];

    for line in input.lines() {
        if line == "$ ls" {
        } else if line == "$ cd /" {
            cwd.clear();
            cwd.push("/".to_string());
        } else if line == "$ cd .." {
            cwd.pop();
        } else if line.starts_with("$ cd ") {
            let name = line.split_at(5).1;
            cwd.push(format!("{}{name}/", cwd.last().unwrap()));
        } else {
            let (size, name) = line.split_once(' ').unwrap();
            if size == "dir" {
                let full_path = format!("{}{name}/", cwd.last().unwrap());
                dirs.push(full_path);
            } else {
                let full_path = format!("{}{name}", cwd.last().unwrap());
                files.insert(full_path, size.parse().unwrap());
            }
        }
    }

    (dirs, files)
}

fn solve_a_for(input: &str) -> usize {
    let (dirs, files) = parse_input(input);

    let mut total_size_sum = 0;
    for dir in dirs {
        let mut total_size = 0;
        for (full_path, size) in files.iter() {
            if full_path.starts_with(&dir) {
                total_size += size;
            }
        }
        if total_size <= 100000 {
            total_size_sum += total_size;
        }
    }
    total_size_sum
}

fn solve_b_for(input: &str) -> usize {
    let (dirs, files) = parse_input(input);

    let mut space_used = 0;
    for size in files.values() {
        space_used += size;
    }

    let space_unused = 70000000 - space_used;
    let extra_space_needed = 30000000 - space_unused;

    let mut smallest_deleted = space_used;
    for dir in dirs {
        let mut total_size = 0;
        for (full_path, size) in files.iter() {
            if full_path.starts_with(&dir) {
                total_size += size;
            }
        }
        if total_size >= extra_space_needed && total_size < smallest_deleted {
            smallest_deleted = total_size;
        }
    }
    smallest_deleted
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for(EXAMPLE), 95437);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(INPUT), 1743217);
}

#[test]
fn b_example() {
    assert_eq!(solve_b_for(EXAMPLE), 24933642);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(INPUT), 8319096);
}

pub fn solve_a() {
    println!("{}", solve_a_for(INPUT));
}

pub fn solve_b() {
    println!("{}", solve_b_for(INPUT));
}
