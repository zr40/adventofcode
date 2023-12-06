use std::time::Duration;

use crate::day::{Day, DayTiming};
use crate::{aoc2015, aoc2023};

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
        year: 2023,
        days: aoc2023::DAYS,
    },
];

impl Year {
    pub(crate) fn run_all(&self) {
        println!("Year {}", self.year);

        let mut total_duration = Duration::ZERO;

        println!(" Day |  Part One |  Part Two |      Duration       ");
        println!("-----+-----------+-----------+----------+----------");
        for (index, day) in self.days.iter().enumerate() {
            let result = day.run();

            match result.timing {
                DayTiming::Separate { a, b } => {
                    println!(
                        "  {:>2} | {:>9} | {:>9} | {:>5.2} ms | {:>5.2} ms",
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
                        "  {:>2} | {:>9} | {:>9} | (pair) {:>9.2} ms",
                        index + 1,
                        result.a,
                        result.b,
                        total.as_micros() as f64 / 1000.0
                    );
                    total_duration += total;
                }
                #[cfg(debug_assertions)]
                DayTiming::SkipSlow => {
                    println!("  {:>2} | slow puzzle skipped in debug build", index + 1);
                }
            };
        }
        println!(
            "Total run time: {:.1} ms\n",
            total_duration.as_micros() as f64 / 1000.0
        );
    }
}
