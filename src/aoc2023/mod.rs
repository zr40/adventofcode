use crate::Day;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

pub(crate) static DAYS: &[Day] = &[
    Day::Separate {
        a: day1::solve_a,
        b: day1::solve_b,
    },
    Day::Separate {
        a: day2::solve_a,
        b: day2::solve_b,
    },
    Day::Separate {
        a: day3::solve_a,
        b: day3::solve_b,
    },
    Day::Separate {
        a: day4::solve_a,
        b: day4::solve_b,
    },
    Day::Separate {
        a: day5::solve_a,
        b: day5::solve_b,
    },
    Day::Separate {
        a: day6::solve_a,
        b: day6::solve_b,
    },
    Day::Separate {
        a: day7::solve_a,
        b: day7::solve_b,
    },
    Day::Separate {
        a: day8::solve_a,
        b: day8::solve_b,
    },
    Day::Pair(day9::solve),
    Day::Separate {
        a: day10::solve_a,
        b: day10::solve_b,
    },
    Day::Separate {
        a: day11::solve_a,
        b: day11::solve_b,
    },
    Day::Separate {
        a: day12::solve_a,
        b: day12::solve_b,
    },
    Day::Separate {
        a: day13::solve_a,
        b: day13::solve_b,
    },
    Day::Separate {
        a: day14::solve_a,
        b: day14::solve_b,
    },
    Day::Separate {
        a: day15::solve_a,
        b: day15::solve_b,
    },
];
