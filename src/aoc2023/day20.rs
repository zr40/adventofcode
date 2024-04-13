use std::collections::{HashMap, VecDeque};

use crate::PuzzleResult;

#[cfg(test)]
const EXAMPLE_1: &str = include_str!("input/20_example1");
#[cfg(test)]
const EXAMPLE_2: &str = include_str!("input/20_example2");
const INPUT: &str = include_str!("input/20");

#[derive(Clone, Copy, Debug)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug)]
enum State {
    On,
    Off,
}

#[allow(variant_size_differences)]
#[derive(Debug)]
enum ModuleType {
    Broadcast,
    FlipFlop { state: State },
    Conjunction { memory: HashMap<String, Pulse> },
}

#[derive(Debug)]
struct Module<'a> {
    module_type: ModuleType,
    destinations: Vec<&'a str>,
    inputs: Vec<String>,
}

fn parse(input: &str) -> HashMap<String, Module<'_>> {
    let mut modules: HashMap<String, Module<'_>> = input
        .lines()
        .map(|line| {
            let (name, destinations) = line.split_once(" -> ").unwrap();

            let (module_type, name) = if name == "broadcaster" {
                (ModuleType::Broadcast, "broadcaster")
            } else {
                let (module_type, name) = name.split_at(1);
                (
                    match module_type {
                        "%" => ModuleType::FlipFlop { state: State::Off },
                        "&" => ModuleType::Conjunction {
                            memory: HashMap::new(),
                        },
                        module_type => {
                            panic!("unknown module type '{module_type}'");
                        }
                    },
                    name,
                )
            };

            let name = name.to_string();
            let destinations = destinations.split(", ").collect();
            (
                name,
                Module {
                    module_type,
                    destinations,
                    inputs: vec![],
                },
            )
        })
        .collect();

    let mut destinations = vec![];

    for (source, module) in &modules {
        for destination in &module.destinations {
            destinations.push((source.clone(), *destination));
        }
    }

    for (source, destination) in destinations {
        if let Some(module) = modules.get_mut(destination) {
            module.inputs.push(source.clone());

            if let Module {
                module_type: ModuleType::Conjunction { memory },
                ..
            } = module
            {
                memory.insert(source.to_string(), Pulse::Low);
            }
        }
    }

    modules
}

struct Queue<'a> {
    source: &'a str,
    destination: &'a str,
    pulse: Pulse,
}

impl Module<'_> {
    fn receive(&mut self, pulse: Pulse, source: &str) -> Option<Pulse> {
        match &mut self.module_type {
            ModuleType::Broadcast => Some(pulse),
            ModuleType::Conjunction { memory } => {
                memory.insert(source.to_string(), pulse);
                Some(
                    if memory.values().all(|v| match v {
                        Pulse::High => true,
                        Pulse::Low => false,
                    }) {
                        Pulse::Low
                    } else {
                        Pulse::High
                    },
                )
            }
            ModuleType::FlipFlop { ref mut state } => match (pulse, &state) {
                (Pulse::Low, State::Off) => {
                    *state = State::On;
                    Some(Pulse::High)
                }
                (Pulse::Low, State::On) => {
                    *state = State::Off;
                    Some(Pulse::Low)
                }
                (Pulse::High, _) => None,
            },
        }
    }
}

fn solve_a_for(input: &str) -> usize {
    let mut modules = parse(input);

    let mut low_pulses_sent = 0;
    let mut high_pulses_sent = 0;

    let mut queue = VecDeque::new();

    for _ in 0..1000 {
        queue.push_back(Queue {
            source: "",
            destination: "broadcaster",
            pulse: Pulse::Low,
        });

        while let Some(item) = queue.pop_front() {
            match item.pulse {
                Pulse::Low => low_pulses_sent += 1,
                Pulse::High => high_pulses_sent += 1,
            }

            if let Some(module) = modules.get_mut(item.destination) {
                if let Some(output) = module.receive(item.pulse, item.source) {
                    for destination in &module.destinations {
                        queue.push_back(Queue {
                            source: item.destination,
                            destination,
                            pulse: output,
                        });
                    }
                }
            }
        }
    }

    low_pulses_sent * high_pulses_sent
}

fn solve_b_for(input: &str) -> usize {
    let modules = parse(input);

    let mut output = 1;

    let qt = modules
        .values()
        .find(|module| module.destinations.contains(&"rx"))
        .unwrap();

    for bb_name in &qt.inputs {
        let bb = &modules[bb_name];
        assert_eq!(bb.inputs.len(), 1);

        let vj_name = &bb.inputs[0];
        let vj = &modules[vj_name];

        let lc_name = vj
            .inputs
            .iter()
            .find(|name| {
                let modu = &modules[*name];

                (modu.inputs.len() == 1) && (modu.destinations.len() == 1)
            })
            .unwrap();

        let mut lc = &modules[lc_name];
        let mut acc = 0;

        loop {
            acc *= 2;
            if lc.destinations.len() == 2 || (lc.destinations.len() == 1 && lc.inputs.len() == 1) {
                acc += 1;
            }

            if lc.inputs.iter().any(|i| i == "broadcaster") {
                break;
            }

            lc = &modules[if lc.inputs.len() == 1 {
                &lc.inputs[0]
            } else {
                lc.inputs.iter().find(|i| *i != vj_name).unwrap()
            }];
        }

        output *= acc;
    }

    output
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for(EXAMPLE_1), 32000000);
    assert_eq!(solve_a_for(EXAMPLE_2), 11687500);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(INPUT), 879834312);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(INPUT), 243037165713371);
}

pub fn solve_a() -> PuzzleResult {
    solve_a_for(INPUT).into()
}

pub fn solve_b() -> PuzzleResult {
    solve_b_for(INPUT).into()
}
