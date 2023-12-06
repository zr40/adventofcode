use phf::{phf_ordered_map, OrderedMap};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

pub static PUZZLES: OrderedMap<&'static str, fn() -> ()> = phf_ordered_map! {
    "1a" => day1::solve_a,
    "1b" => day1::solve_b,
    "2a" => day2::solve_a,
    "2b" => day2::solve_b,
    "3a" => day3::solve_a,
    "3b" => day3::solve_b,
    "4a" => day4::solve_a,
    "4b" => day4::solve_b,
    "5a" => day5::solve_a,
    "5b" => day5::solve_b,
    "6a" => day6::solve_a,
    "6b" => day6::solve_b,
};
