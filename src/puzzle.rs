use std::time::{Duration, Instant};

use phf::{phf_ordered_map, OrderedMap};

use crate::aoc2023;
use crate::puzzle_result::PuzzleResult;

type SeparatePuzzle = fn() -> PuzzleResult;
type PairedPuzzle = fn() -> (PuzzleResult, PuzzleResult);

pub(crate) enum Day {
    Separate {
        a: SeparatePuzzle,
        b: SeparatePuzzle,
    },
    Pair(PairedPuzzle),
}

enum DayTiming {
    Separate { a: Duration, b: Duration },
    Pair(Duration),
}

struct DayResult {
    a: PuzzleResult,
    b: PuzzleResult,
    timing: DayTiming,
}

impl Day {
    fn run(&self) -> DayResult {
        match self {
            Day::Separate { a, b } => {
                let start = Instant::now();
                let result_a = a();
                let duration_a = start.elapsed();

                let start = Instant::now();
                let result_b = b();
                let duration_b = start.elapsed();

                DayResult {
                    a: result_a,
                    b: result_b,
                    timing: DayTiming::Separate {
                        a: duration_a,
                        b: duration_b,
                    },
                }
            }
            Day::Pair(p) => {
                let start = Instant::now();
                let (result_a, result_b) = p();
                let duration = start.elapsed();

                DayResult {
                    a: result_a,
                    b: result_b,
                    timing: DayTiming::Pair(duration),
                }
            }
        }
    }
}

type Year = &'static OrderedMap<u16, Day>;

pub(crate) static YEARS: OrderedMap<u16, Year> = phf_ordered_map! {
    2023u16 => &aoc2023::PUZZLES,
};

pub(crate) fn run_all_in_year(year: u16, puzzles: Year) {
    println!("Year {year}");

    let mut total_duration = Duration::ZERO;

    println!(" Day |  Part One |  Part Two |      Duration       ");
    println!("-----+-----------+-----------+----------+----------");
    for (name, day) in puzzles.into_iter() {
        let result = day.run();

        match result.timing {
            DayTiming::Separate { a, b } => {
                println!(
                    "  {name:>2} | {:>9} | {:>9} | {:>5.2} ms | {:>5.2} ms",
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
                    "  {name:>2} | {:>9} | {:>9} | (pair) {:>9.2} ms",
                    result.a,
                    result.b,
                    total.as_micros() as f64 / 1000.0
                );
                total_duration += total;
            }
        };
    }
    println!(
        "Total run time: {:.1} ms",
        total_duration.as_micros() as f64 / 1000.0
    );
}
