use crate::PuzzleResult;
use crate::common::aoc::input_for;
use crate::day::Day;

#[cfg(test)]
const EXAMPLE_A1: &str = include_str!("example/10a_1");
#[cfg(test)]
const EXAMPLE_A2: &str = include_str!("example/10a_2");
#[cfg(test)]
const EXAMPLE_A3: &str = include_str!("example/10a_3");
#[cfg(test)]
const EXAMPLE_A4: &str = include_str!("example/10a_4");
#[cfg(test)]
const EXAMPLE_B1: &str = include_str!("example/10b_1");
#[cfg(test)]
const EXAMPLE_B2: &str = include_str!("example/10b_2");
#[cfg(test)]
const EXAMPLE_B3: &str = include_str!("example/10b_3");

#[derive(Debug)]
enum Tile {
    Ground,
    PipeNorthSouth,
    PipeEastWest,
    PipeNorthEast,
    PipeNorthWest,
    PipeSouthWest,
    PipeSouthEast,
    Start,
}

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Tile {
    fn connects(&self, direction: Direction) -> bool {
        matches!(
            (direction, self),
            (
                Direction::North,
                Tile::PipeNorthEast | Tile::PipeNorthSouth | Tile::PipeNorthWest,
            ) | (
                Direction::East,
                Tile::PipeEastWest | Tile::PipeNorthEast | Tile::PipeSouthEast
            ) | (
                Direction::South,
                Tile::PipeSouthEast | Tile::PipeSouthWest | Tile::PipeNorthSouth,
            ) | (
                Direction::West,
                Tile::PipeEastWest | Tile::PipeNorthWest | Tile::PipeSouthWest
            )
        )
    }

    fn output(&self, input: Direction) -> Direction {
        match (input, self) {
            (Direction::North, Tile::PipeNorthSouth)
            | (Direction::East, Tile::PipeNorthWest)
            | (Direction::West, Tile::PipeNorthEast) => Direction::North,
            (Direction::North, Tile::PipeSouthEast)
            | (Direction::East, Tile::PipeEastWest)
            | (Direction::South, Tile::PipeNorthEast) => Direction::East,
            (Direction::East, Tile::PipeSouthWest)
            | (Direction::South, Tile::PipeNorthSouth)
            | (Direction::West, Tile::PipeSouthEast) => Direction::South,
            (Direction::North, Tile::PipeSouthWest)
            | (Direction::South, Tile::PipeNorthWest)
            | (Direction::West, Tile::PipeEastWest) => Direction::West,
            (input, tile) => panic!("no output found for {tile:?} coming in from {input:?}"),
        }
    }
}

fn parse(input: &str) -> (Vec<Vec<Tile>>, (usize, usize)) {
    let mut start_x = 0;
    let mut start_y = 0;

    let map = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, ch)| match ch {
                    '.' => Tile::Ground,
                    '|' => Tile::PipeNorthSouth,
                    '-' => Tile::PipeEastWest,
                    'L' => Tile::PipeNorthEast,
                    'J' => Tile::PipeNorthWest,
                    '7' => Tile::PipeSouthWest,
                    'F' => Tile::PipeSouthEast,
                    'S' => {
                        start_x = x;
                        start_y = y;
                        Tile::Start
                    }
                    ch => panic!("unknown tile '{ch}'"),
                })
                .collect()
        })
        .collect();

    (map, (start_x, start_y))
}

fn solve_a_for(input: &str) -> usize {
    let (map, (start_x, start_y)) = parse(input);

    let mut x = start_x;
    let mut y = start_y;
    let mut direction = if map[y - 1][x].connects(Direction::South) {
        Direction::North
    } else if map[y + 1][x].connects(Direction::North) {
        Direction::South
    } else if map[y][x + 1].connects(Direction::East) {
        Direction::West
    } else if map[y][x - 1].connects(Direction::West) {
        Direction::East
    } else {
        panic!("could not find connecting pipe")
    };
    let mut steps = 0;
    loop {
        match direction {
            Direction::North => y -= 1,
            Direction::East => x += 1,
            Direction::South => y += 1,
            Direction::West => x -= 1,
        };
        steps += 1;
        let tile = &map[y][x];
        if let Tile::Start = tile {
            break;
        }
        direction = tile.output(direction);
    }
    steps / 2
}

fn solve_b_for(input: &str) -> usize {
    let (mut map, (start_x, start_y)) = parse(input);
    let mut enclosure = vec![];
    for _ in 0..map.len() {
        enclosure.push(vec![false; map[0].len()]);
    }

    let mut x = start_x;
    let mut y = start_y;
    let mut direction = if y != 0 && map[y - 1][x].connects(Direction::South) {
        Direction::North
    } else if y != map.len() - 1 && map[y + 1][x].connects(Direction::North) {
        Direction::South
    } else if x != map[0].len() - 1 && map[y][x + 1].connects(Direction::East) {
        Direction::West
    } else if x != 0 && map[y][x - 1].connects(Direction::West) {
        Direction::East
    } else {
        panic!("could not find connecting pipe")
    };

    loop {
        match direction {
            Direction::North => y -= 1,
            Direction::East => x += 1,
            Direction::South => y += 1,
            Direction::West => x -= 1,
        };

        let tile = &map[y][x];

        enclosure[y][x] = true;

        if let Tile::Start = tile {
            break;
        }
        direction = tile.output(direction);
    }

    map[start_y][start_x] = match (
        map[start_y][start_x + 1].connects(Direction::West),
        map[start_y + 1][start_x].connects(Direction::North),
    ) {
        (true, true) => Tile::PipeSouthEast,
        (false, true) => Tile::PipeSouthWest,
        (true, false) => Tile::PipeNorthEast,
        (false, false) => Tile::PipeNorthWest,
    };

    for (y, row) in map.iter_mut().enumerate() {
        for (x, tile) in row.iter_mut().enumerate() {
            if !enclosure[y][x] {
                *tile = Tile::Ground;
            }
        }
    }

    let mut escape_map = build_escape_map(&map);
    flood_fill(&mut escape_map);

    escape_map
        .iter()
        .enumerate()
        .filter(|(index, _)| index % 2 == 1)
        .map(|(_, line)| {
            line.iter()
                .enumerate()
                .filter(|(index, contained)| index % 2 == 1 && **contained)
                .count()
        })
        .sum()
}

fn build_escape_map(map: &Vec<Vec<Tile>>) -> Vec<Vec<bool>> {
    let mut escape_map = vec![vec![true; map[0].len() * 2 + 1]];
    for row in map {
        let mut escape_line = vec![true];
        for tile in row {
            match tile {
                Tile::Ground => {
                    escape_line.push(true);
                    escape_line.push(true);
                }
                Tile::PipeEastWest | Tile::PipeNorthEast | Tile::PipeSouthEast => {
                    escape_line.push(false);
                    escape_line.push(false);
                }
                _ => {
                    escape_line.push(false);
                    escape_line.push(true);
                }
            }
        }
        escape_map.push(escape_line);
        let mut escape_line = vec![true];
        for tile in row {
            match tile {
                Tile::PipeSouthEast | Tile::PipeSouthWest | Tile::PipeNorthSouth => {
                    escape_line.push(false);
                }
                _ => escape_line.push(true),
            }
            escape_line.push(true);
        }
        escape_map.push(escape_line);
    }

    escape_map
}

#[allow(clippy::ptr_arg)]
fn flood_fill(map: &mut Vec<Vec<bool>>) {
    let mut flood = vec![(0, 0)];
    while let Some((x, y)) = flood.pop() {
        if let Some(row) = map.get(y) {
            if let Some(true) = row.get(x) {
                flood.push((x + 1, y));
                flood.push((x.max(1) - 1, y));
                flood.push((x, y + 1));
                flood.push((x, y.max(1) - 1));
                map[y][x] = false;
            }
        }
    }
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for(EXAMPLE_A1), 4);
    assert_eq!(solve_a_for(EXAMPLE_A2), 4);
    assert_eq!(solve_a_for(EXAMPLE_A3), 8);
    assert_eq!(solve_a_for(EXAMPLE_A4), 8);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(&input_for(2023, 10)), 6942);
}

fn solve_a() -> PuzzleResult {
    solve_a_for(&input_for(2023, 10)).into()
}

#[test]
fn b_example() {
    assert_eq!(solve_b_for(EXAMPLE_B1), 4);
    assert_eq!(solve_b_for(EXAMPLE_B2), 8);
    assert_eq!(solve_b_for(EXAMPLE_B3), 10);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(&input_for(2023, 10)), 297);
}

fn solve_b() -> PuzzleResult {
    solve_b_for(&input_for(2023, 10)).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
