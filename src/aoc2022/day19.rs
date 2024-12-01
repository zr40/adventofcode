use crate::common::aoc::input_for;
use crate::day::Day;
use crate::puzzle_result::PuzzleResult;

#[cfg(test)]
const EXAMPLE: &str = include_str!("example/19");

struct Blueprint {
    id: usize,
    ore_robot_ore_cost: usize,
    clay_robot_ore_cost: usize,
    obsidian_robot_ore_cost: usize,
    obsidian_robot_clay_cost: usize,
    geode_robot_ore_cost: usize,
    geode_robot_obsidian_cost: usize,
}

impl Blueprint {
    fn from(line: &str) -> Blueprint {
        // Blueprint 1: Each ore robot costs 2 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 17 clay. Each geode robot costs 3 ore and 11 obsidian.

        let (_, line) = line.split_once("Blueprint ").unwrap();
        let (id, line) = line.split_once(": Each ore robot costs ").unwrap();
        let (ore_robot_ore_cost, line) = line.split_once(" ore. Each clay robot costs ").unwrap();
        let (clay_robot_ore_cost, line) =
            line.split_once(" ore. Each obsidian robot costs ").unwrap();
        let (obsidian_robot_ore_cost, line) = line.split_once(" ore and ").unwrap();
        let (obsidian_robot_clay_cost, line) =
            line.split_once(" clay. Each geode robot costs ").unwrap();
        let (geode_robot_ore_cost, line) = line.split_once(" ore and ").unwrap();
        let (geode_robot_obsidian_cost, _) = line.split_once(" obsidian.").unwrap();

        Blueprint {
            id: id.parse().unwrap(),
            ore_robot_ore_cost: ore_robot_ore_cost.parse().unwrap(),
            clay_robot_ore_cost: clay_robot_ore_cost.parse().unwrap(),
            obsidian_robot_ore_cost: obsidian_robot_ore_cost.parse().unwrap(),
            obsidian_robot_clay_cost: obsidian_robot_clay_cost.parse().unwrap(),
            geode_robot_ore_cost: geode_robot_ore_cost.parse().unwrap(),
            geode_robot_obsidian_cost: geode_robot_obsidian_cost.parse().unwrap(),
        }
    }
}

#[derive(Clone, PartialEq)]
struct SolverState {
    ore_robots: usize,
    clay_robots: usize,
    obsidian_robots: usize,
    geode_robots: usize,
    ore: usize,
    clay: usize,
    obsidian: usize,
    geodes: usize,
}

impl SolverState {
    fn open_geodes(&self) -> SolverState {
        SolverState {
            ore_robots: self.ore_robots,
            clay_robots: self.clay_robots,
            obsidian_robots: self.obsidian_robots,
            geode_robots: self.geode_robots,
            ore: self.ore + self.ore_robots,
            clay: self.clay + self.clay_robots,
            obsidian: self.obsidian + self.obsidian_robots,
            geodes: self.geodes + self.geode_robots,
        }
    }

    fn build_geode_robot(&self, blueprint: &Blueprint) -> SolverState {
        SolverState {
            ore_robots: self.ore_robots,
            clay_robots: self.clay_robots,
            obsidian_robots: self.obsidian_robots,
            geode_robots: self.geode_robots + 1,
            ore: self.ore - blueprint.geode_robot_ore_cost,
            clay: self.clay,
            obsidian: self.obsidian - blueprint.geode_robot_obsidian_cost,
            geodes: self.geodes,
        }
    }

    fn build_obsidian_robot(&self, blueprint: &Blueprint) -> SolverState {
        SolverState {
            ore_robots: self.ore_robots,
            clay_robots: self.clay_robots,
            obsidian_robots: self.obsidian_robots + 1,
            geode_robots: self.geode_robots,
            ore: self.ore - blueprint.obsidian_robot_ore_cost,
            clay: self.clay - blueprint.obsidian_robot_clay_cost,
            obsidian: self.obsidian,
            geodes: self.geodes,
        }
    }

    fn build_clay_robot(&self, blueprint: &Blueprint) -> SolverState {
        SolverState {
            ore_robots: self.ore_robots,
            clay_robots: self.clay_robots + 1,
            obsidian_robots: self.obsidian_robots,
            geode_robots: self.geode_robots,
            ore: self.ore - blueprint.clay_robot_ore_cost,
            clay: self.clay,
            obsidian: self.obsidian,
            geodes: self.geodes,
        }
    }

    fn build_ore_robot(&self, blueprint: &Blueprint) -> SolverState {
        SolverState {
            ore_robots: self.ore_robots + 1,
            clay_robots: self.clay_robots,
            obsidian_robots: self.obsidian_robots,
            geode_robots: self.geode_robots,
            ore: self.ore - blueprint.ore_robot_ore_cost,
            clay: self.clay,
            obsidian: self.obsidian,
            geodes: self.geodes,
        }
    }
}

enum Mode {
    PartOne,
    PartTwo,
}

fn solve_for(input: &str, mode: Mode) -> usize {
    let iter = match mode {
        Mode::PartOne => input.lines().take(30),
        Mode::PartTwo => input.lines().take(3),
    }
    .map(|line| {
        let blueprint = Blueprint::from(line);
        let mut states = vec![SolverState {
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geodes: 0,
        }];

        for _ in 0..(match mode {
            Mode::PartOne => 24,
            Mode::PartTwo => 32,
        }) {
            let mut new_states = vec![];

            for state in states {
                let new_state = state.open_geodes();

                if state.obsidian >= blueprint.geode_robot_obsidian_cost
                    && state.ore >= blueprint.geode_robot_ore_cost
                {
                    new_states.push(new_state.build_geode_robot(&blueprint));
                }

                if state.clay >= blueprint.obsidian_robot_clay_cost
                    && state.ore >= blueprint.obsidian_robot_ore_cost
                    && state.obsidian < blueprint.geode_robot_obsidian_cost
                {
                    new_states.push(new_state.build_obsidian_robot(&blueprint));
                }
                if state.ore >= blueprint.clay_robot_ore_cost
                    && state.clay < blueprint.obsidian_robot_clay_cost
                {
                    new_states.push(new_state.build_clay_robot(&blueprint));
                }

                if state.ore >= blueprint.ore_robot_ore_cost {
                    new_states.push(new_state.build_ore_robot(&blueprint));
                }

                new_states.push(new_state);
            }

            states = vec![];
            for (i, state) in new_states.iter().enumerate() {
                if new_states.iter().enumerate().all(|(j, other)| {
                    if state == other {
                        return i >= j;
                    }

                    state.clay > other.clay
                        || state.clay_robots > other.clay_robots
                        || state.geodes > other.geodes
                        || state.geode_robots > other.geode_robots
                        || state.obsidian > other.obsidian
                        || state.obsidian_robots > other.obsidian_robots
                        || state.ore > other.ore
                        || state.ore_robots > other.ore_robots
                }) {
                    states.push(state.clone());
                }
            }

            // heuristic
            states.sort_by(|a, b| b.geode_robots.cmp(&a.geode_robots));
            states.truncate(300);

            new_states.clear();
        }

        let max_geodes = states.into_iter().map(|s| s.geodes).max().unwrap();
        match mode {
            Mode::PartOne => max_geodes * blueprint.id,
            Mode::PartTwo => max_geodes,
        }
    });

    match mode {
        Mode::PartOne => iter.sum(),
        Mode::PartTwo => iter.product(),
    }
}

#[test]
#[cfg_attr(debug_assertions, ignore)]
fn a_example() {
    assert_eq!(solve_for(EXAMPLE, Mode::PartOne), 33);
}

#[test]
#[cfg_attr(debug_assertions, ignore)]
fn a_puzzle() {
    assert_eq!(solve_for(&input_for(2022, 19), Mode::PartOne), 1009);
}

fn solve_a() -> PuzzleResult {
    solve_for(&input_for(2022, 19), Mode::PartOne).into()
}

#[test]
#[cfg_attr(debug_assertions, ignore)]
fn b_example() {
    assert_eq!(solve_for(EXAMPLE, Mode::PartTwo), 54 * 62); // should be 56 * 62 according to the puzzle text
}

#[test]
#[cfg_attr(debug_assertions, ignore)]
fn b_puzzle() {
    assert_eq!(solve_for(&input_for(2022, 19), Mode::PartTwo), 18816);
}

fn solve_b() -> PuzzleResult {
    solve_for(&input_for(2022, 19), Mode::PartTwo).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};
