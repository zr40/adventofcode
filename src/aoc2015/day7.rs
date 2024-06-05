use std::collections::HashMap;

use crate::day::Day;
use crate::PuzzleResult;

#[cfg(test)]
const EXAMPLE: &str = include_str!("input/7_example");
const INPUT: &str = include_str!("input/7");

#[derive(Clone, Debug)]
enum Gate {
    Const(u16),
    Alias(String),
    And(String, String),
    Or(String, String),
    Not(String),
    Lshift(String, u16),
    Rshift(String, u16),
}

struct Circuit {
    wires: HashMap<String, Gate>,
    known_signals: HashMap<String, u16>,
}

fn parse(input: &str) -> Circuit {
    let mut wires: HashMap<String, Gate> = HashMap::new();

    for line in input.lines() {
        let mut tokens: Vec<_> = line.split(' ').collect();

        let output = tokens.pop().unwrap().to_owned();
        tokens.pop();

        match tokens.len() {
            1 => match tokens[0].parse() {
                Ok(c) => {
                    wires.insert(output, Gate::Const(c));
                }
                Err(_) => {
                    wires.insert(output, Gate::Alias(tokens[0].to_owned()));
                }
            },
            2 => {
                assert!(tokens[0] == "NOT");
                wires.insert(output, Gate::Not(tokens[1].to_owned()));
            }
            3 => {
                match tokens[1] {
                    "AND" => wires.insert(
                        output,
                        Gate::And(tokens[0].to_owned(), tokens[2].to_owned()),
                    ),
                    "OR" => {
                        wires.insert(output, Gate::Or(tokens[0].to_owned(), tokens[2].to_owned()))
                    }
                    "LSHIFT" => wires.insert(
                        output,
                        Gate::Lshift(tokens[0].to_owned(), tokens[2].parse().unwrap()),
                    ),
                    "RSHIFT" => wires.insert(
                        output,
                        Gate::Rshift(tokens[0].to_owned(), tokens[2].parse().unwrap()),
                    ),
                    _ => panic!("unknown instruction: {line}"),
                };
                if let Ok(n) = tokens[0].parse() {
                    wires.insert(tokens[0].to_owned(), Gate::Const(n));
                };
            }
            _ => panic!("unknown instruction: {line}"),
        };
    }

    Circuit {
        wires,
        known_signals: HashMap::new(),
    }
}

impl Circuit {
    fn evaluate(&mut self, wire: &str) -> u16 {
        if !self.known_signals.contains_key(wire) {
            let gate = self.wires[wire].clone();
            let signal: u16 = match gate {
                Gate::Const(n) => n,
                Gate::Alias(w) => self.evaluate(&w),
                Gate::And(left, right) => self.evaluate(&left) & self.evaluate(&right),
                Gate::Or(left, right) => self.evaluate(&left) | self.evaluate(&right),
                Gate::Lshift(w, n) => self.evaluate(&w) << n,
                Gate::Rshift(w, n) => self.evaluate(&w) >> n,
                Gate::Not(w) => !self.evaluate(&w),
            };
            self.known_signals.insert(wire.to_owned(), signal);
        }

        self.known_signals[wire]
    }

    fn clear_and_override(&mut self) {
        let a = self.evaluate("a");
        self.known_signals.clear();
        self.known_signals.insert("b".to_owned(), a);
    }
}

fn solve_for(input: &str) -> (u16, u16) {
    let mut circuit = parse(input);

    let one = circuit.evaluate("a");
    circuit.clear_and_override();
    let two = circuit.evaluate("a");

    (one, two)
}

#[test]
fn a_example() {
    let mut circuit = parse(EXAMPLE);

    assert_eq!(circuit.evaluate("d"), 72);
    assert_eq!(circuit.evaluate("e"), 507);
    assert_eq!(circuit.evaluate("f"), 492);
    assert_eq!(circuit.evaluate("g"), 114);
    assert_eq!(circuit.evaluate("h"), 65412);
    assert_eq!(circuit.evaluate("i"), 65079);
    assert_eq!(circuit.evaluate("x"), 123);
    assert_eq!(circuit.evaluate("y"), 456);
}

#[test]
fn puzzle() {
    let mut circuit = parse(INPUT);

    assert_eq!(circuit.evaluate("a"), 16076);
    circuit.clear_and_override();
    assert_eq!(circuit.evaluate("a"), 2797);
}

fn solve_both() -> (PuzzleResult, PuzzleResult) {
    let (a, b) = solve_for(INPUT);
    (a.into(), b.into())
}

pub(super) static DAY: Day = Day::Pair(solve_both);
