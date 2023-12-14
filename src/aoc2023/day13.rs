use crate::PuzzleResult;

#[cfg(test)]
const EXAMPLE: &str = include_str!("input/13_example");
const INPUT: &str = include_str!("input/13");

struct Pattern {
    rows: Vec<Vec<bool>>,
    cols: Vec<Vec<bool>>,
}

fn parse(input: &str) -> Vec<Pattern> {
    let mut patterns = vec![];

    patterns.push(Pattern {
        rows: vec![],
        cols: vec![],
    });
    let mut pattern = patterns.last_mut().unwrap();

    for line in input.lines() {
        if line.is_empty() {
            patterns.push(Pattern {
                rows: vec![],
                cols: vec![],
            });
            pattern = patterns.last_mut().unwrap();
        } else {
            pattern.rows.push(vec![]);
        }
        for (index, ch) in line.chars().enumerate() {
            let item = match ch {
                '.' => false,
                '#' => true,
                ch => panic!("unknown pattern '{ch}'"),
            };
            pattern.rows.last_mut().unwrap().push(item);
            if pattern.cols.len() == index {
                pattern.cols.push(vec![item]);
            } else {
                pattern.cols[index].push(item);
            }
        }
    }

    patterns
}

fn find_value(pattern: &Pattern, ignore_value: usize) -> Option<usize> {
    'outer: for candidate_mirror in 0..pattern.rows.len() - 1 {
        for offset in 0.. {
            if offset > candidate_mirror || candidate_mirror + offset + 1 >= pattern.rows.len() {
                break;
            }

            let left = candidate_mirror - offset;
            let right = candidate_mirror + offset + 1;

            if pattern.rows[left] != pattern.rows[right] {
                continue 'outer;
            }
        }
        let value = (candidate_mirror + 1) * 100;
        if value != ignore_value {
            return Some(value);
        }
    }
    'outer: for candidate_mirror in 0..pattern.cols.len() - 1 {
        for offset in 0.. {
            if offset > candidate_mirror || candidate_mirror + offset + 1 >= pattern.cols.len() {
                break;
            }

            let left = candidate_mirror - offset;
            let right = candidate_mirror + offset + 1;

            if pattern.cols[left] != pattern.cols[right] {
                continue 'outer;
            }
        }
        let value = candidate_mirror + 1;
        if value != ignore_value {
            return Some(value);
        }
    }
    None
}

fn solve_a_for(input: &str) -> usize {
    let patterns = parse(input);

    patterns
        .into_iter()
        .map(|p| find_value(&p, 0).expect("no mirror found"))
        .sum()
}

fn solve_b_for(input: &str) -> usize {
    let patterns = parse(input);

    patterns
        .into_iter()
        .enumerate()
        .map(|(i, mut p)| {
            let ignored_value = find_value(&p, 0).expect("no mirror found");

            for row in 0..p.rows.len() {
                for col in 0..p.cols.len() {
                    p.rows[row][col] = !p.rows[row][col];
                    p.cols[col][row] = !p.cols[col][row];

                    if let Some(value) = find_value(&p, ignored_value) {
                        return value;
                    }

                    p.rows[row][col] = !p.rows[row][col];
                    p.cols[col][row] = !p.cols[col][row];
                }
            }

            panic!("no smudge found in pattern {i}");
        })
        .sum()
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for(EXAMPLE), 405);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(INPUT), 40006);
}

#[test]
fn b_example() {
    assert_eq!(solve_b_for(EXAMPLE), 400);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(INPUT), 28627);
}

pub fn solve_a() -> PuzzleResult {
    solve_a_for(INPUT).into()
}

pub fn solve_b() -> PuzzleResult {
    solve_b_for(INPUT).into()
}
