use crate::PuzzleResult;

const INPUT: &str = include_str!("input/10");

struct Sequence {
    current_digit: u8,
    current_count: u8,
    output_length: usize,
}

struct State(pub Vec<Sequence>);

impl State {
    fn new(iterations: usize) -> State {
        let mut sequences = vec![];

        for _ in 0..iterations {
            sequences.push(Sequence {
                current_digit: b' ',
                current_count: 0,
                output_length: 0,
            });
        }

        State(sequences)
    }

    fn input(&mut self, digit: u8, index: usize) {
        let sequence = &mut self.0[index];

        if digit == sequence.current_digit {
            sequence.current_count += 1;
        } else if sequence.current_count == 0 {
            sequence.current_count = 1;
            sequence.current_digit = digit;
        } else {
            debug_assert!(sequence.current_count <= 10);

            let old_count = sequence.current_count;
            let old_digit = sequence.current_digit;

            sequence.current_count = 1;
            sequence.current_digit = digit;
            sequence.output_length += 2;

            if index + 1 < self.0.len() {
                self.input(b'0' + old_count, index + 1);
                self.input(old_digit, index + 1);
            }
        }
    }

    fn len(mut self) -> usize {
        for i in 0..self.0.len() {
            self.input(b' ', i);
        }

        self.0.last().unwrap().output_length
    }
}

fn solve_for(input: &str, iterations: usize) -> usize {
    let mut state = State::new(iterations);

    for digit in input.bytes() {
        state.input(digit, 0);
    }

    state.len()
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_for(INPUT, 40), 329356);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_for(INPUT, 50), 4666278);
}

pub fn solve_a() -> PuzzleResult {
    solve_for(INPUT, 40).into()
}

pub fn solve_b() -> PuzzleResult {
    solve_for(INPUT, 50).into()
}
