use crate::day::Day;
use crate::puzzle_result::PuzzleResult;

#[cfg(test)]
const EXAMPLE: &str = include_str!("input/10_example");
const INPUT: &str = include_str!("input/10");

struct Cpu {
    x: i32,
    cycle: i32,
    signal_strength: i32,
    display: String,
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
            display: String::with_capacity(41 * 6),
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
        if (self.cycle - 1) % 40 + 1 == 40 {
            self.display.push('\n');
        }

        self.cycle += 1;

        if self.cycle % 40 == 20 {
            self.signal_strength += self.x * self.cycle;
        }

        let pixel = (self.cycle - 1) % 40;

        if self.x >= pixel - 1 && self.x <= pixel + 1 {
            self.display.push('#');
        } else {
            self.display.push('.');
        }
    }
}

fn solve_for(input: &str) -> (i32, String) {
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

#[test]
fn example() {
    let (signal_strength, display) = solve_for(EXAMPLE);
    assert_eq!(signal_strength, 13140);
    assert_eq!(
        display,
        "\
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
    );
}

#[test]
fn puzzle() {
    let (signal_strength, display) = solve_for(INPUT);
    assert_eq!(signal_strength, 14060);
    assert_eq!(
        display,
        "\
###...##..###..#..#.####.#..#.####...##.
#..#.#..#.#..#.#.#..#....#.#..#.......#.
#..#.#..#.#..#.##...###..##...###.....#.
###..####.###..#.#..#....#.#..#.......#.
#....#..#.#....#.#..#....#.#..#....#..#.
#....#..#.#....#..#.#....#..#.####..##.."
    );
}

fn solve_both() -> (PuzzleResult, PuzzleResult) {
    let (signal_strength, display) = solve_for(INPUT);
    (signal_strength.into(), PuzzleResult::Multiline(display))
}

pub(super) static DAY: Day = Day::Pair(solve_both);
