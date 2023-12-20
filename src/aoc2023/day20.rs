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
        if let Some(Module {
            module_type: ModuleType::Conjunction { memory },
            ..
        }) = modules.get_mut(destination)
        {
            memory.insert(source, Pulse::Low);
        }
    }

    modules
}

struct Queue<'a> {
    source: &'a str,
    destination: &'a str,
    pulse: Pulse,
}

enum Mode {
    PartA,
    PartB,
}

fn solve_for(input: &str, mode: Mode) -> usize {
    let mut modules = parse(input);

    let mut low_pulses_sent = 0;
    let mut high_pulses_sent = 0;

    let mut queue = VecDeque::new();

    // FIXME part 2, this will not work on other inputs
    let mut bb = 0;
    let mut kk = 0;
    let mut mr = 0;
    let mut gl = 0;

    for i in 1.. {
        match &mode {
            Mode::PartA => {
                if i == 1001 {
                    break;
                }
            }
            Mode::PartB => {}
        }

        queue.push_back(Queue {
            source: "",
            destination: "broadcaster",
            pulse: Pulse::Low,
        });

        while let Some(item) = queue.pop_front() {
            match (&mode, item.pulse, item.destination) {
                (Mode::PartA, Pulse::Low, _) => low_pulses_sent += 1,
                (Mode::PartA, Pulse::High, _) => high_pulses_sent += 1,
                (Mode::PartB, Pulse::Low, "bb") => {
                    if bb == 0 {
                        bb = i;
                    }
                }
                (Mode::PartB, Pulse::Low, "kk") => {
                    if kk == 0 {
                        kk = i;
                    }
                }
                (Mode::PartB, Pulse::Low, "mr") => {
                    if mr == 0 {
                        mr = i;
                    }
                }
                (Mode::PartB, Pulse::Low, "gl") => {
                    if gl == 0 {
                        gl = i;
                    }
                }
                (Mode::PartB, _, _) => {}
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

        if let Mode::PartB = &mode {
            if bb != 0 && kk != 0 && mr != 0 && gl != 0 {
                return bb * kk * mr * gl;
            }
        }
    }

    low_pulses_sent * high_pulses_sent
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

#[test]
fn a_example() {
    assert_eq!(solve_for(EXAMPLE_1, Mode::PartA), 32000000);
    assert_eq!(solve_for(EXAMPLE_2, Mode::PartA), 11687500);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_for(INPUT, Mode::PartA), 879834312);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_for(INPUT, Mode::PartB), 243037165713371);
}

pub fn solve_a() -> PuzzleResult {
    solve_for(INPUT, Mode::PartA).into()
}

pub fn solve_b() -> PuzzleResult {
    solve_for(INPUT, Mode::PartB).into()
}
