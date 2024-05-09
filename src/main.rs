#![feature(btree_extract_if)]
#![feature(float_next_up_down)]
#![feature(iter_map_windows)]

use std::env::args;

use day::Day;
use puzzle_result::PuzzleResult;
use year::YEARS;

mod aoc2015;
mod aoc2016;
mod aoc2022;
mod aoc2023;
mod common;
mod day;
mod puzzle_result;
mod year;

fn main() {
    match args().nth(1).as_deref() {
        None => YEARS.last().unwrap().run_all(),
        Some("all") => {
            for year in YEARS {
                year.run_all();
            }
        }
        Some(year) => {
            let year: u16 = year.parse().unwrap();

            match YEARS.iter().find(|y| y.year == year) {
                Some(year) => year.run_all(),
                None => println!("unknown year {year}"),
            }
        }
    }
}
