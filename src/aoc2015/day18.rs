use crate::PuzzleResult;

#[cfg(test)]
const EXAMPLE: &str = include_str!("input/18_example");
const INPUT: &str = include_str!("input/18");

enum Mode {
    Normal,
    StuckCorners,
}

fn solve_for(input: &str, mode: Mode, steps: usize) -> usize {
    let mut grid: Vec<Vec<bool>> = input
        .lines()
        .map(|l| l.bytes().map(|b| b == b'#').collect())
        .collect();

    if let Mode::StuckCorners = mode {
        stuck_corners(&mut grid);
    }

    for _ in 0..steps {
        grid = step(grid);

        if let Mode::StuckCorners = mode {
            stuck_corners(&mut grid);
        }
    }

    grid.into_iter()
        .map(|l| l.into_iter().filter(|c| *c).count())
        .sum()
}

fn step(grid: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    grid.iter()
        .enumerate()
        .map(|(cur_y, line)| {
            line.iter()
                .enumerate()
                .map(|(cur_x, current_on)| {
                    let mut neighbors_on = 0;
                    for y in 0..=2 {
                        if y == 0 && cur_y == 0 {
                            continue;
                        }
                        if y == 2 && cur_y == grid.len() - 1 {
                            continue;
                        }

                        let neighbor_y = cur_y + y - 1;

                        for x in 0..=2 {
                            if x == 0 && cur_x == 0 {
                                continue;
                            }
                            if x == 2 && cur_x == line.len() - 1 {
                                continue;
                            }
                            if x == 1 && y == 1 {
                                continue;
                            }

                            let neighbor_x = cur_x + x - 1;

                            if grid[neighbor_y][neighbor_x] {
                                neighbors_on += 1;
                            }
                        }
                    }

                    neighbors_on == 3 || (*current_on && neighbors_on == 2)
                })
                .collect()
        })
        .collect()
}

#[allow(clippy::ptr_arg)]
fn stuck_corners(grid: &mut Vec<Vec<bool>>) {
    *grid.first_mut().unwrap().first_mut().unwrap() = true;
    *grid.first_mut().unwrap().last_mut().unwrap() = true;
    *grid.last_mut().unwrap().first_mut().unwrap() = true;
    *grid.last_mut().unwrap().last_mut().unwrap() = true;
}

#[test]
fn a_example() {
    assert_eq!(solve_for(EXAMPLE, Mode::Normal, 4), 4);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_for(INPUT, Mode::Normal, 100), 1061);
}

#[test]
fn b_example() {
    assert_eq!(solve_for(EXAMPLE, Mode::StuckCorners, 5), 17);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_for(INPUT, Mode::StuckCorners, 100), 1006);
}

pub fn solve_a() -> PuzzleResult {
    solve_for(INPUT, Mode::Normal, 100).into()
}

pub fn solve_b() -> PuzzleResult {
    solve_for(INPUT, Mode::StuckCorners, 100).into()
}
