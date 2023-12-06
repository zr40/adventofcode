use std::fmt::Display;

pub(crate) enum PuzzleResult {
    I32(i32),
    U32(u32),
    U64(u64),
    Usize(usize),
    #[allow(dead_code)]
    Todo,
    #[cfg(debug_assertions)]
    SkipSlow,
}

impl From<i32> for PuzzleResult {
    fn from(value: i32) -> Self {
        Self::I32(value)
    }
}

impl From<u32> for PuzzleResult {
    fn from(value: u32) -> Self {
        Self::U32(value)
    }
}

impl From<u64> for PuzzleResult {
    fn from(value: u64) -> Self {
        Self::U64(value)
    }
}

impl From<usize> for PuzzleResult {
    fn from(value: usize) -> Self {
        Self::Usize(value)
    }
}

impl Display for PuzzleResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::I32(v) => v.fmt(f),
            Self::U32(v) => v.fmt(f),
            Self::U64(v) => v.fmt(f),
            Self::Usize(v) => v.fmt(f),
            Self::Todo => "TODO".fmt(f),
            #[cfg(debug_assertions)]
            Self::SkipSlow => "Skipped".fmt(f),
        }
    }
}
