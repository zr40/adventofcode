use crate::common::aoc::input_for;
#[cfg(test)]
use crate::common::const_bool_array::str_to_bool_array;
use crate::common::ocr::ocr;
use crate::day::Day;
use crate::puzzle_result::PuzzleResult;

#[cfg(test)]
const EXAMPLE: &str = include_str!("example/10");
#[cfg(test)]
const EXAMPLE_B_EXPECTED: [bool; 240] = str_to_bool_array(include_str!("example/10b_expected"));

struct Cpu {
    x: i32,
    cycle: i32,
    signal_strength: i32,
    display: [bool; 40 * 6],
}

enum Instruction {
    AddX(i32),
    Noop,
}

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            x: 1,
            cycle: 0,
            signal_strength: 0,
            display: [false; 40 * 6],
        }
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::AddX(x) => {
                self.tick();
                self.tick();
                self.x += x;
            }
            Instruction::Noop => {
                self.tick();
            }
        }
    }

    fn tick(&mut self) {
        self.cycle += 1;

        if self.cycle % 40 == 20 {
            self.signal_strength += self.x * self.cycle;
        }

        let pixel = (self.cycle - 1) % 40;

        if self.x >= pixel - 1 && self.x <= pixel + 1 {
            self.display[(self.cycle - 1) as usize] = true;
        } else {
            // self.display.push('.');
        }
    }
}

fn solve_without_ocr(input: &str) -> (i32, [bool; 240]) {
    let mut cpu = Cpu::new();
    for line in input.lines() {
        let mut line = line.split(' ');
        cpu.execute(match line.next().unwrap() {
            "addx" => Instruction::AddX(line.next().unwrap().parse().unwrap()),
            "noop" => Instruction::Noop,
            unknown => panic!("unknown instruction {unknown}"),
        });
    }
    (cpu.signal_strength, cpu.display)
}

fn solve_for(input: &str) -> (i32, String) {
    let (signal_strength, display) = solve_without_ocr(input);

    (signal_strength, ocr(&display, 40))
}

#[test]
fn example() {
    let (signal_strength, display) = solve_without_ocr(EXAMPLE);
    assert_eq!(signal_strength, 13140);
    assert_eq!(display, EXAMPLE_B_EXPECTED);
}

#[test]
fn puzzle() {
    let (signal_strength, display) = solve_for(&input_for(2022, 10));
    assert_eq!(signal_strength, 14060);
    assert_eq!(display, "PAPKFKEJ");
}

fn solve_both() -> (PuzzleResult, PuzzleResult) {
    let (signal_strength, display) = solve_for(&input_for(2022, 10));
    (signal_strength.into(), display.into())
}

pub(super) static DAY: Day = Day::Pair(solve_both);
