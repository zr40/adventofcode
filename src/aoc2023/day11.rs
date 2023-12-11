use crate::PuzzleResult;

#[cfg(test)]
const EXAMPLE: &str = include_str!("input/11_example");
const INPUT: &str = include_str!("input/11");

const PART_A: usize = 2;
const PART_B: usize = 1000000;

fn solve_for(input: &str, expansion: usize) -> usize {
    let mut current_row_index = 0;
    let mut rows = vec![];
    let mut cols = vec![];
    let mut galaxies = vec![];

    for (row, line) in input.lines().enumerate() {
        if current_row_index == 0 {
            cols.append(&mut vec![false; line.len()]);
        }

        rows.push(current_row_index);
        if line.contains('#') {
            current_row_index += 1;
        } else {
            current_row_index += expansion;
        }

        for (col, item) in line.chars().enumerate() {
            if item == '#' {
                cols[col] = true;
                galaxies.push((row, col));
            }
        }
    }

    let mut current_col_index = 0;
    let cols: Vec<_> = cols
        .into_iter()
        .map(|col| {
            if col {
                current_col_index += 1;
            } else {
                current_col_index += expansion;
            }
            current_col_index
        })
        .collect();

    for (row, col) in &mut galaxies {
        *row = rows[*row];
        *col = cols[*col];
    }

    (0..galaxies.len() - 1)
        .map(|a| {
            (a + 1..galaxies.len())
                .map(|b| {
                    galaxies[a].0.abs_diff(galaxies[b].0) + galaxies[a].1.abs_diff(galaxies[b].1)
                })
                .sum::<usize>()
        })
        .sum()
}

#[test]
fn a_example() {
    assert_eq!(solve_for(EXAMPLE, PART_A), 374);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_for(INPUT, PART_A), 10077850);
}

#[test]
fn b_example() {
    assert_eq!(solve_for(EXAMPLE, 10), 1030);
    assert_eq!(solve_for(EXAMPLE, 100), 8410);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_for(INPUT, PART_B), 504715068438);
}

pub fn solve_a() -> PuzzleResult {
    solve_for(INPUT, PART_A).into()
}

pub fn solve_b() -> PuzzleResult {
    solve_for(INPUT, PART_B).into()
}
