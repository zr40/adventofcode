use crate::Day;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

pub(crate) static DAYS: [Day; 6] = [
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
];
