use std::collections::HashMap;

use crate::day::Day;
use crate::puzzle_result::PuzzleResult;

#[cfg(test)]
const EXAMPLE_A: &str = include_str!("input/20a_example");
#[cfg(test)]
const EXAMPLE_B: &str = include_str!("input/20b_example");
const INPUT: &str = include_str!("input/20");

struct Particle {
    px: i32,
    py: i32,
    pz: i32,
    vx: i32,
    vy: i32,
    vz: i32,
    ax: i32,
    ay: i32,
    az: i32,
}

fn parse(input: &str) -> Vec<Particle> {
    input
        .split('\n')
        .map(|x| {
            let x = x
                .replace("p=<", "")
                .replace(">, v=<", ",")
                .replace(">, a=<", ",")
                .replace([' ', '>'], "");
            let mut iter = x.split(',').map(|x| x.parse().unwrap());

            Particle {
                px: iter.next().unwrap(),
                py: iter.next().unwrap(),
                pz: iter.next().unwrap(),
                vx: iter.next().unwrap(),
                vy: iter.next().unwrap(),
                vz: iter.next().unwrap(),
                ax: iter.next().unwrap(),
                ay: iter.next().unwrap(),
                az: iter.next().unwrap(),
            }
        })
        .collect()
}

fn solve_a_for(input: &str) -> usize {
    let particles = parse(input);

    // This is incorrect for particles with equal |a| that have any component of
    // v change sign, and similarly for particles with equal |a| and |v| that
    // have any component of p change sign.
    //
    // Thanks to https://github.com/xocolatl for pointing this out.
    particles
        .iter()
        .enumerate()
        .min_by_key(|(_, x)| {
            (
                x.ax.abs() + x.ay.abs() + x.az.abs(),
                x.vx.abs() + x.vy.abs() + x.vz.abs(),
                x.px.abs() + x.py.abs() + x.pz.abs(),
            )
        })
        .unwrap()
        .0
}

fn solve_b_for(input: &str) -> usize {
    let mut particles: Vec<Particle> = parse(input);

    // 150 iterations upper bound is a guess that is valid for my (and probably
    // everyone's) puzzle input.
    for _ in 0..150 {
        let mut positions_seen = HashMap::new();

        for particle in &particles {
            let pos = (particle.px, particle.py, particle.pz);

            let prev_count = *positions_seen.get(&pos).unwrap_or(&0);
            positions_seen.insert(pos, prev_count + 1);
        }

        particles.retain(|x| positions_seen.get(&(x.px, x.py, x.pz)) == Some(&1));

        for particle in &mut particles {
            particle.vx += particle.ax;
            particle.vy += particle.ay;
            particle.vz += particle.az;

            particle.px += particle.vx;
            particle.py += particle.vy;
            particle.pz += particle.vz;
        }
    }

    particles.len()
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for(EXAMPLE_A), 0);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(INPUT), 376);
}

#[test]
fn b_example() {
    assert_eq!(solve_b_for(EXAMPLE_B), 1);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(INPUT), 574);
}

fn solve_a() -> PuzzleResult {
    solve_a_for(INPUT).into()
}

fn solve_b() -> PuzzleResult {
    solve_b_for(INPUT).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
