use crate::PuzzleResult;

#[cfg(test)]
const EXAMPLE: &str = include_str!("input/14_example");
const INPUT: &str = include_str!("input/14");

enum Mode {
    Distance,
    Points,
}

struct Reindeer {
    velocity: u32,
    fly_duration: u32,
    rest_duration: u32,
    distance: u32,
    points: u32,
}

fn solve_for(input: &str, mode: Mode, duration: u32) -> u32 {
    let mut reindeer = vec![];
    for line in input.lines() {
        let (_, rest) = line.split_once(" can fly ").unwrap();
        let (velocity, rest) = rest.split_once(" km/s for ").unwrap();
        let (fly_duration, rest) = rest
            .split_once(" seconds, but then must rest for ")
            .unwrap();
        let (rest_duration, _) = rest.split_once(" seconds.").unwrap();

        reindeer.push(Reindeer {
            velocity: velocity.parse().unwrap(),
            fly_duration: fly_duration.parse().unwrap(),
            rest_duration: rest_duration.parse().unwrap(),
            distance: 0,
            points: 0,
        });
    }

    for t in 0..duration {
        for r in &mut reindeer {
            let cycle = r.fly_duration + r.rest_duration;
            let cycle_time = t % cycle;

            if cycle_time < r.fly_duration {
                r.distance += r.velocity;
            }
        }

        match mode {
            Mode::Distance => {}
            Mode::Points => {
                let max = reindeer.iter().map(|r| r.distance).max().unwrap();
                for r in &mut reindeer {
                    if r.distance == max {
                        r.points += 1;
                    }
                }
            }
        }
    }

    match mode {
        Mode::Distance => reindeer.into_iter().map(|r| r.distance).max().unwrap(),
        Mode::Points => reindeer.into_iter().map(|r| r.points).max().unwrap(),
    }
}

#[test]
fn a_example() {
    assert_eq!(solve_for(EXAMPLE, Mode::Distance, 1000), 1120);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_for(INPUT, Mode::Distance, 2503), 2640);
}

#[test]
fn b_example() {
    assert_eq!(solve_for(EXAMPLE, Mode::Points, 1000), 689);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_for(INPUT, Mode::Points, 2503), 1102);
}

pub fn solve_a() -> PuzzleResult {
    solve_for(INPUT, Mode::Distance, 2503).into()
}

pub fn solve_b() -> PuzzleResult {
    solve_for(INPUT, Mode::Points, 2503).into()
}
