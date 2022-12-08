#[allow(dead_code)]
const EXAMPLE: &str = include_str!("../input/8_example");
const INPUT: &str = include_str!("../input/8");

fn parse_input(input: &str) -> Vec<Vec<i8>> {
    let mut field = vec![];

    for line in input.lines() {
        field.push(
            line.chars()
                .map(|x| (x as u32 - '0' as u32) as i8)
                .collect(),
        );
    }

    field
}

fn solve_a_for(input: &str) -> usize {
    let field = parse_input(input);
    let mut visible_trees: Vec<Vec<bool>> = vec![];
    for _ in 0..field.len() {
        visible_trees.push(vec![false; field.len()]);
    }

    for i in 0..field.len() {
        let mut highest_seen = -1;
        for j in 0..field.len() {
            if field[i][j] > highest_seen {
                highest_seen = field[i][j];
                visible_trees[i][j] = true;
            }
        }
    }

    for i in 0..field.len() {
        let mut highest_seen = -1;
        for j in 0..field.len() {
            if field[j][i] > highest_seen {
                highest_seen = field[j][i];
                visible_trees[j][i] = true;
            }
        }
    }

    for i in 0..field.len() {
        let mut highest_seen = -1;
        for j in 0..field.len() {
            let j = field.len() - j - 1;
            if field[i][j] > highest_seen {
                highest_seen = field[i][j];
                visible_trees[i][j] = true;
            }
        }
    }

    for i in 0..field.len() {
        let mut highest_seen = -1;
        for j in 0..field.len() {
            let j = field.len() - j - 1;
            if field[j][i] > highest_seen {
                highest_seen = field[j][i];
                visible_trees[j][i] = true;
            }
        }
    }

    visible_trees
        .iter()
        .map(|row| row.iter().filter(|tree| **tree).count())
        .sum()
}

fn solve_b_for(input: &str) -> usize {
    let field = parse_input(input);

    let mut best_scenic_score = 0;

    for i in 0..field.len() {
        for j in 0..field.len() {
            let mut scenic_score = 1;

            // negative i
            let mut visible_trees = 0;
            let mut candidate_i = i;
            while candidate_i > 0 {
                candidate_i -= 1;
                visible_trees += 1;

                if field[candidate_i][j] >= field[i][j] {
                    break;
                }
            }
            scenic_score *= visible_trees;

            // positive i
            let mut visible_trees = 0;
            let mut candidate_i = i;
            while candidate_i < field.len() - 1 {
                candidate_i += 1;
                visible_trees += 1;

                if field[candidate_i][j] >= field[i][j] {
                    break;
                }
            }
            scenic_score *= visible_trees;

            // negative j
            let mut visible_trees = 0;
            let mut candidate_j = j;
            while candidate_j > 0 {
                candidate_j -= 1;
                visible_trees += 1;

                if field[i][candidate_j] >= field[i][j] {
                    break;
                }
            }
            scenic_score *= visible_trees;

            // positive j
            let mut visible_trees = 0;
            let mut candidate_j = j;
            while candidate_j < field.len() - 1 {
                candidate_j += 1;
                visible_trees += 1;

                if field[i][candidate_j] >= field[i][j] {
                    break;
                }
            }
            scenic_score *= visible_trees;

            if best_scenic_score < scenic_score {
                best_scenic_score = scenic_score;
            }
        }
    }

    best_scenic_score
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for(EXAMPLE), 21);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(INPUT), 1809);
}

#[test]
fn b_example() {
    assert_eq!(solve_b_for(EXAMPLE), 8);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(INPUT), 479400);
}

pub fn solve_a() {
    println!("{}", solve_a_for(INPUT));
}

pub fn solve_b() {
    println!("{}", solve_b_for(INPUT));
}
