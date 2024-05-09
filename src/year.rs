use std::time::Duration;

use crate::day::{Day, DayTiming};
use crate::puzzle_result::PuzzleResult;
use crate::{aoc2015, aoc2016, aoc2022, aoc2023};

pub(crate) struct Year {
    pub(crate) year: u16,
    days: &'static [Day],
}

pub(crate) static YEARS: &[Year] = &[
    Year {
        year: 2015,
        days: aoc2015::DAYS,
    },
    Year {
        year: 2016,
        days: aoc2016::DAYS,
    },
    Year {
        year: 2022,
        days: aoc2022::DAYS,
    },
    Year {
        year: 2023,
        days: aoc2023::DAYS,
    },
];

impl Year {
    pub(crate) fn run_all(&self) {
        println!("Year {}", self.year);

        let mut total_duration = Duration::ZERO;

        println!(" Day |     Part One    |     Part Two    |         Duration          ");
        println!("-----+-----------------+-----------------+-------------+-------------");
        for (index, day) in self.days.iter().enumerate() {
            let result = day.run();

            match result.timing {
                DayTiming::Separate { a, b } => {
                    println!(
                        "  {:>2} | {:>15} | {:>15} | {:>8.2} ms | {:>8.2} ms",
                        index + 1,
                        result.a,
                        result.b,
                        a.as_micros() as f64 / 1000.0,
                        b.as_micros() as f64 / 1000.0,
                    );
                    total_duration += a;
                    total_duration += b;
                }
                DayTiming::Pair(total) => {
                    println!(
                        "  {:>2} | {:>15} | {:>15} |      (pair) | {:>8.2} ms",
                        index + 1,
                        result.a,
                        result.b,
                        total.as_micros() as f64 / 1000.0
                    );
                    total_duration += total;
                }
                DayTiming::Single(a) => {
                    println!(
                        "  {:>2} | {:>15} | {:>15} | {:>8.2} ms |",
                        index + 1,
                        result.a,
                        result.b,
                        a.as_micros() as f64 / 1000.0
                    );
                    total_duration += a;
                }
                #[cfg(debug_assertions)]
                DayTiming::SkipSlow => {
                    println!("  {:>2} | slow puzzle skipped in debug build", index + 1);
                }
            };
            if let PuzzleResult::Multiline(v) = result.a {
                println!("{v}");
            }
            if let PuzzleResult::Multiline(v) = result.b {
                println!("{v}");
            }
        }
        println!(
            "Total run time: {:.1} ms\n",
            total_duration.as_micros() as f64 / 1000.0
        );
    }
}
