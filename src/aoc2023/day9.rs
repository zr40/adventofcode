use crate::PuzzleResult;

#[cfg(test)]
const EXAMPLE: &str = include_str!("input/9_example");
const INPUT: &str = include_str!("input/9");

fn solve_for(input: &str) -> (i32, i32) {
    input
        .lines()
        .map(|line| {
            let mut histories: Vec<Vec<i32>> =
                vec![line.split(' ').map(|num| num.parse().unwrap()).collect()];

            while !histories.last().unwrap().iter().all(|num| *num == 0) {
                histories.push(
                    histories
                        .last()
                        .unwrap()
                        .iter()
                        .map_windows(|[a, b]| *b - *a)
                        .collect(),
                );
            }

            histories
                .into_iter()
                .rev()
                .fold((0, 0), |(acc_a, acc_b), history| {
                    (acc_a + history.last().unwrap(), history[0] - acc_b)
                })
        })
        .fold((0, 0), |(acc_a, acc_b), (a, b)| (acc_a + a, acc_b + b))
}

#[test]
fn example() {
    assert_eq!(solve_for(EXAMPLE), (114, 2));
}

#[test]
fn puzzle() {
    assert_eq!(solve_for(INPUT), (1647269739, 864));
}

pub fn solve() -> (PuzzleResult, PuzzleResult) {
    let (a, b) = solve_for(INPUT);
    (a.into(), b.into())
}
