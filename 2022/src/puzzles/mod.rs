use phf::{phf_ordered_map, OrderedMap};

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

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
    "7a" => day7::solve_a,
    "7b" => day7::solve_b,
    "8a" => day8::solve_a,
    "8b" => day8::solve_b,
    "9a" => day9::solve_a,
    "9b" => day9::solve_b,
    "10a" => day10::solve_both,
    "11a" => day11::solve_a,
    "11b" => day11::solve_b,
    "12a" => day12::solve_a,
    "12b" => day12::solve_b,
    "13a" => day13::solve_a,
    "13b" => day13::solve_b,
    "14a" => day14::solve_a,
    "14b" => day14::solve_b,
    "15a" => day15::solve_a,
    "15b" => day15::solve_b,
    "16a" => day16::solve_a,
    "16b" => day16::solve_b,
    "17a" => day17::solve_a,
    "17b" => day17::solve_b,
    "18a" => day18::solve_a,
    "18b" => day18::solve_b,
    "19a" => day19::solve_a,
    "19b" => day19::solve_b,
    "20a" => day20::solve_a,
    "20b" => day20::solve_b,
    "21a" => day21::solve_a,
    "21b" => day21::solve_b,
};
