#![feature(btree_extract_if)]
#![feature(float_next_up_down)]

use std::env::args;

use puzzle::{run_all_in_year, Day, YEARS};
use puzzle_result::PuzzleResult;

mod aoc2023;
mod puzzle;
mod puzzle_result;

enum Mode {
    AllYears,
    SpecificYear(u16),
}

fn main() {
    let mut mode = Mode::SpecificYear(*YEARS.into_iter().last().unwrap().0);

    for arg in args().skip(1) {
        if arg == "all" {
            mode = Mode::AllYears;
        } else {
            let year = arg.parse().unwrap();
            mode = Mode::SpecificYear(year);
        }
    }

    match mode {
        Mode::AllYears => {
            for (year, puzzles) in YEARS.into_iter() {
                run_all_in_year(*year, puzzles);
            }
        }
        Mode::SpecificYear(year) => {
            let puzzles = YEARS.get(&year).expect("unknown year");
            run_all_in_year(year, puzzles);
        }
    }
}
