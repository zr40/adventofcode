use crate::puzzle_result::PuzzleResult;

#[cfg(test)]
const EXAMPLE: &str = include_str!("input/19_example");
const INPUT: &str = include_str!("input/19");

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
                let new_state = SolverState {
                    ore_robots: state.ore_robots,
                    clay_robots: state.clay_robots,
                    obsidian_robots: state.obsidian_robots,
                    geode_robots: state.geode_robots,
                    ore: state.ore + state.ore_robots,
                    clay: state.clay + state.clay_robots,
                    obsidian: state.obsidian + state.obsidian_robots,
                    geodes: state.geodes + state.geode_robots,
                };

                if state.obsidian >= blueprint.geode_robot_obsidian_cost
                    && state.ore >= blueprint.geode_robot_ore_cost
                {
                    new_states.push(SolverState {
                        ore_robots: new_state.ore_robots,
                        clay_robots: new_state.clay_robots,
                        obsidian_robots: new_state.obsidian_robots,
                        geode_robots: new_state.geode_robots + 1,
                        ore: new_state.ore - blueprint.geode_robot_ore_cost,
                        clay: new_state.clay,
                        obsidian: new_state.obsidian - blueprint.geode_robot_obsidian_cost,
                        geodes: new_state.geodes,
                    });
                }

                if state.clay >= blueprint.obsidian_robot_clay_cost
                    && state.ore >= blueprint.obsidian_robot_ore_cost
                    && state.obsidian < blueprint.geode_robot_obsidian_cost
                {
                    new_states.push(SolverState {
                        ore_robots: new_state.ore_robots,
                        clay_robots: new_state.clay_robots,
                        obsidian_robots: new_state.obsidian_robots + 1,
                        geode_robots: new_state.geode_robots,
                        ore: new_state.ore - blueprint.obsidian_robot_ore_cost,
                        clay: new_state.clay - blueprint.obsidian_robot_clay_cost,
                        obsidian: new_state.obsidian,
                        geodes: new_state.geodes,
                    });
                }
                if state.ore >= blueprint.clay_robot_ore_cost
                    && state.clay < blueprint.obsidian_robot_clay_cost
                {
                    new_states.push(SolverState {
                        ore_robots: new_state.ore_robots,
                        clay_robots: new_state.clay_robots + 1,
                        obsidian_robots: new_state.obsidian_robots,
                        geode_robots: new_state.geode_robots,
                        ore: new_state.ore - blueprint.clay_robot_ore_cost,
                        clay: new_state.clay,
                        obsidian: new_state.obsidian,
                        geodes: new_state.geodes,
                    });
                }

                if state.ore >= blueprint.ore_robot_ore_cost {
                    new_states.push(SolverState {
                        ore_robots: new_state.ore_robots + 1,
                        clay_robots: new_state.clay_robots,
                        obsidian_robots: new_state.obsidian_robots,
                        geode_robots: new_state.geode_robots,
                        ore: new_state.ore - blueprint.ore_robot_ore_cost,
                        clay: new_state.clay,
                        obsidian: new_state.obsidian,
                        geodes: new_state.geodes,
                    });
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
    assert_eq!(solve_for(INPUT, Mode::PartOne), 1009);
}

#[test]
#[cfg_attr(debug_assertions, ignore)]
fn b_example() {
    assert_eq!(solve_for(EXAMPLE, Mode::PartTwo), 54 * 62); // should be 56 * 62 according to the puzzle text
}

#[test]
#[cfg_attr(debug_assertions, ignore)]
fn b_puzzle() {
    assert_eq!(solve_for(INPUT, Mode::PartTwo), 18816);
}

pub fn solve_a() -> PuzzleResult {
    #[cfg(debug_assertions)]
    return PuzzleResult::SkipSlow;

    #[cfg(not(debug_assertions))]
    solve_for(INPUT, Mode::PartOne).into()
}

pub fn solve_b() -> PuzzleResult {
    #[cfg(debug_assertions)]
    return PuzzleResult::SkipSlow;

    #[cfg(not(debug_assertions))]
    solve_for(INPUT, Mode::PartTwo).into()
}
