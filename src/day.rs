use std::time::{Duration, Instant};

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

pub(crate) enum DayTiming {
    Separate {
        a: Duration,
        b: Duration,
    },
    Pair(Duration),
    #[cfg(debug_assertions)]
    SkipSlow,
}

pub(crate) struct DayResult {
    pub(crate) a: PuzzleResult,
    pub(crate) b: PuzzleResult,
    pub(crate) timing: DayTiming,
}

impl Day {
    pub(crate) fn run(&self) -> DayResult {
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

                #[cfg(debug_assertions)]
                if let PuzzleResult::SkipSlow = result_a {
                    DayResult {
                        a: result_a,
                        b: result_b,
                        timing: DayTiming::SkipSlow,
                    }
                } else {
                    DayResult {
                        a: result_a,
                        b: result_b,
                        timing: DayTiming::Pair(duration),
                    }
                }

                #[cfg(not(debug_assertions))]
                DayResult {
                    a: result_a,
                    b: result_b,
                    timing: DayTiming::Pair(duration),
                }
            }
        }
    }
}
