use std::collections::BTreeMap;

#[cfg(test)]
const EXAMPLE: &str = include_str!("../input/5_example");
const INPUT: &str = include_str!("../input/5");

#[derive(Debug)]
struct Map {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

fn solve_a_for(input: &str) -> u64 {
    let mut lines = input.lines();
    let mut seeds = lines.next().unwrap().split(' ');
    seeds.next();
    let seeds: Vec<u64> = seeds.map(|seed| seed.parse().unwrap()).collect();

    let mut maps = vec![];
    assert_eq!(lines.next().unwrap(), "");

    while let Some(line) = lines.next() {
        let mut current_map = vec![];
        assert!(line.ends_with(" map:"));

        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            }

            let mut parts = line.split(' ').map(|n| n.parse().unwrap());

            current_map.push(Map {
                destination_range_start: parts.next().unwrap(),
                source_range_start: parts.next().unwrap(),
                range_length: parts.next().unwrap(),
            });
        }
        maps.push(current_map);
    }

    seeds
        .into_iter()
        .map(|mut seed| {
            for map in &maps {
                for item in map {
                    if (item.source_range_start..(item.source_range_start + item.range_length))
                        .contains(&seed)
                    {
                        seed = seed - item.source_range_start + item.destination_range_start;
                        break;
                    }
                }
            }
            seed
        })
        .min()
        .unwrap()
}

fn solve_b_for(input: &str) -> u64 {
    let mut lines = input.lines();
    let mut seeds = lines.next().unwrap().split(' ');
    seeds.next();
    let mut current_ranges: BTreeMap<u64, u64> = BTreeMap::new();

    while let Some(range_start) = seeds.next() {
        current_ranges.insert(
            range_start.parse().unwrap(),
            seeds.next().unwrap().parse().unwrap(),
        );
    }

    while let Some(line) = lines.next() {
        if line.is_empty() {
            continue;
        }
        assert!(line.ends_with(" map:"));

        let mut new_ranges: BTreeMap<u64, u64> = BTreeMap::new();

        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            }

            let mut parts = line.split(' ').map(|n| n.parse().unwrap());

            let map_destination_start: u64 = parts.next().unwrap();
            let map_source_start: u64 = parts.next().unwrap();
            let map_length: u64 = parts.next().unwrap();
            let map_source_end = map_source_start + map_length;

            let overlapping_ranges: Vec<(u64, u64)> = current_ranges
                .extract_if(|current_start, current_length| {
                    let current_end = *current_start + *current_length;

                    *current_start < map_source_end && map_source_start < current_end
                })
                .collect();

            for (current_start, current_length) in overlapping_ranges {
                let current_end = current_start + current_length;

                if current_start < map_source_start {
                    current_ranges.insert(current_start, map_source_start - current_start);
                }

                if current_end > map_source_end {
                    current_ranges.insert(map_source_end, current_end - map_source_end);
                }

                let overlap_start = current_start.max(map_source_start);
                let overlap_end = current_end.min(map_source_end);
                new_ranges.insert(
                    overlap_start + map_destination_start - map_source_start,
                    overlap_end - overlap_start,
                );
            }
        }
        new_ranges.append(&mut current_ranges);
        current_ranges = new_ranges;
    }

    *current_ranges.first_key_value().unwrap().0
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for(EXAMPLE), 35);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(INPUT), 457535844);
}

#[test]
fn b_example() {
    assert_eq!(solve_b_for(EXAMPLE), 46);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(INPUT), 41222968);
}

pub fn solve_a() {
    println!("{}", solve_a_for(INPUT));
}

pub fn solve_b() {
    println!("{}", solve_b_for(INPUT));
}
