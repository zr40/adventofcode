use rational::Rational;

use crate::PuzzleResult;

#[cfg(test)]
const EXAMPLE: &str = include_str!("input/24_example");
const INPUT: &str = include_str!("input/24");

#[derive(Clone, Copy)]
struct Coord {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Clone, Copy)]
struct Hailstone {
    position: Coord,
    velocity: Coord,
}

struct HailstoneProjection {
    position: (i64, i64),
    velocity: (i64, i64),
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
                position: Coord {
                    x: px.parse().unwrap(),
                    y: py.parse().unwrap(),
                    z: pz.parse().unwrap(),
                },
                velocity: Coord {
                    x: vx.parse().unwrap(),
                    y: vy.parse().unwrap(),
                    z: vz.parse().unwrap(),
                },
            }
        })
        .collect()
}

#[allow(clippy::similar_names)]
fn solve_a_for(input: &str, min: i64, max: i64) -> usize {
    let hailstones = parse(input);
    let mut crossed_paths = 0;
    for i in 0..hailstones.len() - 1 {
        let l1 = &hailstones[i];
        for l2 in hailstones.iter().skip(i + 1) {
            // https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection#Given_two_points_on_each_line

            let x1 = Rational::from(l1.position.x);
            let y1 = Rational::from(l1.position.y);
            let x2 = Rational::from(l1.position.x + l1.velocity.x);
            let y2 = Rational::from(l1.position.y + l1.velocity.y);
            let x3 = Rational::from(l2.position.x);
            let y3 = Rational::from(l2.position.y);
            let x4 = Rational::from(l2.position.x + l2.velocity.x);
            let y4 = Rational::from(l2.position.y + l2.velocity.y);

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

            let iax = (px - l1.position.x) / l1.velocity.x;
            let iay = (py - l1.position.y) / l1.velocity.y;
            let ibx = (px - l2.position.x) / l2.velocity.x;
            let iby = (py - l2.position.y) / l2.velocity.y;

            // sanity check
            assert_eq!(iax, iay);
            assert_eq!(ibx, iby);

            if px < min || px > max || py < min || py > max {
                // outside test area
                continue;
            }

            let a_in_past = (px < l1.position.x && l1.velocity.x.is_positive())
                || (px > l1.position.x && l1.velocity.x.is_negative());
            let b_in_past = (px < l2.position.x && l2.velocity.x.is_positive())
                || (px > l2.position.x && l2.velocity.x.is_negative());

            if a_in_past || b_in_past {
                continue;
            }

            crossed_paths += 1;
        }
    }
    crossed_paths
}

#[allow(clippy::similar_names)]
fn solve_b_for(input: &str) -> i128 {
    // adapted from https://aoc.csokavar.hu/?day=24

    let mut hailstones = parse(input);

    // solution can be found with just the first 4 hailstones
    hailstones.truncate(4);

    let hailstones_xy: Vec<_> = hailstones
        .iter()
        .map(|h| HailstoneProjection {
            position: (h.position.x, h.position.y),
            velocity: (h.velocity.x, h.velocity.y),
        })
        .collect();
    let (stone_x, stone_y, v1) = solve_2d(&hailstones_xy).expect("no solution found");

    let hailstones_xz: Vec<_> = hailstones
        .iter()
        .map(|h| HailstoneProjection {
            position: (h.position.x, h.position.z),
            velocity: (h.velocity.x, h.velocity.z),
        })
        .collect();
    let (stone_x2, stone_z) = solve_1d(&hailstones_xz, v1).expect("no solution found");

    // sanity check
    assert_eq!(stone_x, stone_x2);

    let sum = stone_x + stone_y + stone_z;
    assert_eq!(sum.denominator(), 1);
    sum.numerator()
}

fn solve_2d(particles: &[HailstoneProjection]) -> Option<(Rational, Rational, i64)> {
    for v1 in -300..=300 {
        if let Some((s0, s1)) = solve_1d(particles, v1) {
            return Some((s0, s1, v1));
        }
    }
    None
}

fn solve_1d(particles: &[HailstoneProjection], v1: i64) -> Option<(Rational, Rational)> {
    for v2 in -300..=300 {
        let vel = (v1, v2);

        if let Some(stone) = intersection_2d(
            apply_reference_frame(&particles[0], vel),
            apply_reference_frame(&particles[1], vel),
        ) {
            if particles.iter().all(|p| {
                let p = apply_reference_frame(p, vel);
                ((stone.0 - p.position.0) * p.velocity.1 - (stone.1 - p.position.1) * p.velocity.0)
                    == 0
            }) {
                return Some(stone);
            }
        }
    }
    None
}

fn apply_reference_frame(p: &HailstoneProjection, v: (i64, i64)) -> HailstoneProjection {
    HailstoneProjection {
        position: p.position,
        velocity: (p.velocity.0 + v.0, p.velocity.1 + v.1),
    }
}

fn intersection_2d(
    p1: HailstoneProjection,
    p2: HailstoneProjection,
) -> Option<(Rational, Rational)> {
    let determinant = p1.velocity.0 * p2.velocity.1 - p1.velocity.1 * p2.velocity.0;

    if determinant == 0 {
        None
    } else {
        let b0 = Rational::from(p1.velocity.0 * p1.position.1 - p1.velocity.1 * p1.position.0);
        let b1 = Rational::from(p2.velocity.0 * p2.position.1 - p2.velocity.1 * p2.position.0);

        Some((
            (p2.velocity.0 * b0 - p1.velocity.0 * b1) / determinant,
            (p2.velocity.1 * b0 - p1.velocity.1 * b1) / determinant,
        ))
    }
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
    assert_eq!(solve_b_for(INPUT), 618534564836937);
}

pub fn solve_a() -> PuzzleResult {
    solve_a_for(INPUT, 200000000000000, 400000000000000).into()
}

pub fn solve_b() -> PuzzleResult {
    solve_b_for(INPUT).into()
}
