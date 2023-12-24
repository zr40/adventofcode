use crate::Day;

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
mod day22;
mod day23;
mod day24;
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
    Day::Separate {
        a: day16::solve_a,
        b: day16::solve_b,
    },
    Day::Separate {
        a: day17::solve_a,
        b: day17::solve_b,
    },
    Day::Separate {
        a: day18::solve_a,
        b: day18::solve_b,
    },
    Day::Separate {
        a: day19::solve_a,
        b: day19::solve_b,
    },
    Day::Separate {
        a: day20::solve_a,
        b: day20::solve_b,
    },
    Day::Separate {
        a: day21::solve_a,
        b: day21::solve_b,
    },
    Day::Separate {
        a: day22::solve_a,
        b: day22::solve_b,
    },
    Day::Separate {
        a: day23::solve_a,
        b: day23::solve_b,
    },
    Day::Separate {
        a: day24::solve_a,
        b: day24::solve_b,
    },
];
