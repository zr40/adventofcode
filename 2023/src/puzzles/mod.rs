use phf::{phf_ordered_map, OrderedMap};

mod day1;
mod day2;

pub static PUZZLES: OrderedMap<&'static str, fn() -> ()> = phf_ordered_map! {
    "1a" => day1::solve_a,
    "1b" => day1::solve_b,
    "2a" => day2::solve_a,
    "2b" => day2::solve_b,
};
