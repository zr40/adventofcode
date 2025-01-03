use crate::common::aoc::input_for;
#[cfg(test)]
use crate::common::const_bool_array::str_to_bool_array;
use crate::common::ocr::ocr;
use crate::day::Day;
use crate::puzzle_result::PuzzleResult;

#[cfg(test)]
const EXAMPLE: &str = include_str!("example/8");
#[cfg(test)]
const EXAMPLE_EXPECTED: [bool; 21] = str_to_bool_array(include_str!("example/8_expected"));

struct Screen {
    width: usize,
    height: usize,
    pixels: Vec<bool>,
}

impl Screen {
    fn new(width: usize, height: usize) -> Screen {
        Screen {
            width,
            height,
            pixels: vec![false; width * height],
        }
    }

    fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn get(&self, x: usize, y: usize) -> bool {
        self.pixels[self.index(x, y)]
    }

    fn set(&mut self, x: usize, y: usize, val: bool) {
        let index = self.index(x, y);
        self.pixels[index] = val;
    }

    fn rect(&mut self, x: usize, y: usize) {
        for x in 0..x {
            for y in 0..y {
                self.set(x, y, true);
            }
        }
    }

    fn rotate_column(&mut self, x: usize, pixels: usize) {
        for _ in 0..pixels {
            let temp = self.get(x, self.height - 1);
            for y in (1..self.height).rev() {
                self.set(x, y, self.get(x, y - 1));
            }
            self.set(x, 0, temp);
        }
    }

    fn rotate_row(&mut self, y: usize, pixels: usize) {
        for _ in 0..pixels {
            let temp = self.get(self.width - 1, y);
            for x in (1..self.width).rev() {
                self.set(x, y, self.get(x - 1, y));
            }
            self.set(0, y, temp);
        }
    }

    fn pixels_lit(&self) -> usize {
        self.pixels.iter().filter(|x| **x).count()
    }

    fn process(&mut self, input: &str) {
        for line in input.lines() {
            let (command, rest) = line.split_once(' ').unwrap();

            match command {
                "rect" => {
                    let (x, y) = rest.split_once('x').unwrap();
                    let x: usize = x.parse().unwrap();
                    let y: usize = y.parse().unwrap();
                    self.rect(x, y);
                }
                "rotate" => {
                    let (rotate, rest) = rest.split_once(' ').unwrap();
                    let (_, rest) = rest.split_once('=').unwrap();
                    let (xy, pixels) = rest.split_once(" by ").unwrap();

                    let xy: usize = xy.parse().unwrap();
                    let pixels: usize = pixels.parse().unwrap();

                    match rotate {
                        "column" => self.rotate_column(xy, pixels),
                        "row" => self.rotate_row(xy, pixels),
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            }
        }
    }
}

fn solve_for(input: &str, width: usize, height: usize) -> (usize, String) {
    let mut screen = Screen::new(width, height);

    screen.process(input);

    (screen.pixels_lit(), ocr(&screen.pixels, screen.width))
}

#[test]
fn a_example() {
    let mut screen = Screen::new(7, 3);
    screen.process(EXAMPLE);
    assert_eq!(screen.pixels, EXAMPLE_EXPECTED);
    assert_eq!(screen.pixels_lit(), 6);
}

#[test]
fn puzzle() {
    let (pixels_lit, display) = solve_for(&input_for(2016, 8), 50, 6);
    assert_eq!(pixels_lit, 115);
    assert_eq!(display, "EFEYKFRFIJ");
}

fn solve_both() -> (PuzzleResult, PuzzleResult) {
    let (pixels_lit, display) = solve_for(&input_for(2016, 8), 50, 6);
    (pixels_lit.into(), display.into())
}

pub(super) static DAY: Day = Day::Pair(solve_both);
