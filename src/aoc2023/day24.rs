use rational::Rational;

use crate::PuzzleResult;

#[cfg(test)]
const EXAMPLE: &str = include_str!("input/24_example");
const INPUT: &str = include_str!("input/24");

struct Hailstone {
    px: i64,
    py: i64,
    pz: i64,
    vx: i64,
    vy: i64,
    vz: i64,
}

fn parse(input: &str) -> Vec<Hailstone> {
    input
        .lines()
        .map(|line| {
            let (px, line) = line.split_once(", ").unwrap();
            let (py, line) = line.split_once(", ").unwrap();
            let (pz, line) = line.split_once(" @ ").unwrap();
            let (vx, line) = line.split_once(", ").unwrap();
            let (vy, vz) = line.split_once(", ").unwrap();
            Hailstone {
                px: px.parse().unwrap(),
                py: py.parse().unwrap(),
                pz: pz.parse().unwrap(),
                vx: vx.parse().unwrap(),
                vy: vy.parse().unwrap(),
                vz: vz.parse().unwrap(),
            }
        })
        .collect()
}

fn solve_a_for(input: &str, min: i64, max: i64) -> usize {
    let hailstones = parse(input);
    let mut crossed_paths = 0;
    for i in 0..hailstones.len() - 1 {
        let l1 = &hailstones[i];
        for j in i + 1..hailstones.len() {
            let l2 = &hailstones[j];

            // https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection#Given_two_points_on_each_line

            let x1 = Rational::from(l1.px);
            let y1 = Rational::from(l1.py);
            let x2 = Rational::from(l1.px + l1.vx);
            let y2 = Rational::from(l1.py + l1.vy);
            let x3 = Rational::from(l2.px);
            let y3 = Rational::from(l2.py);
            let x4 = Rational::from(l2.px + l2.vx);
            let y4 = Rational::from(l2.py + l2.vy);

            let denominator = ((x1 - x2) * (y3 - y4)) - ((y1 - y2) * (x3 - x4));
            if denominator == 0 {
                // parallel
                continue;
            }

            let px = ((((x1 * y2) - (y1 * x2)) * (x3 - x4))
                - ((x1 - x2) * ((x3 * y4) - (y3 * x4))))
                / denominator;
            let py = ((((x1 * y2) - (y1 * x2)) * (y3 - y4))
                - ((y1 - y2) * ((x3 * y4) - (y3 * x4))))
                / denominator;

            let iax = (px - l1.px) / l1.vx;
            let iay = (py - l1.py) / l1.vy;
            let ibx = (px - l2.px) / l2.vx;
            let iby = (py - l2.py) / l2.vy;

            // println!(
            //     "\nA: {} {} @ {} {}\nB: {} {} @ {} {}\np: {px} {py}\nA intersection time x: {iax}\nA intersection time y: {iay}\nB intersection time x: {ibx}\nB intersection time y: {iby}",
            //     l1.px, l1.py, l1.vx, l1.vy, l2.px, l2.py, l2.vx, l2.vy,
            // );

            // sanity check
            assert_eq!(iax, iay);
            assert_eq!(ibx, iby);

            if px < min || px > max || py < min || py > max {
                // outside test area
                continue;
            }

            let a_in_past =
                (px < l1.px && l1.vx.is_positive()) || (px > l1.px && l1.vx.is_negative());
            let b_in_past =
                (px < l2.px && l2.vx.is_positive()) || (px > l2.px && l2.vx.is_negative());

            if a_in_past || b_in_past {
                continue;
            }

            crossed_paths += 1;
        }
    }
    crossed_paths
}

fn solve_b_for(input: &str) -> usize {
    todo!()
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for(EXAMPLE, 7, 27), 2);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(INPUT, 200000000000000, 400000000000000), 13910);
}

#[test]
fn b_example() {
    assert_eq!(solve_b_for(EXAMPLE), 47);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(INPUT), 0);
}

pub fn solve_a() -> PuzzleResult {
    solve_a_for(INPUT, 200000000000000, 400000000000000).into()
}

pub fn solve_b() -> PuzzleResult {
    solve_b_for(INPUT).into()
}
