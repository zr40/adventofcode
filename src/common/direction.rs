#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn step(
        self,
        x: usize,
        y: usize,
        bound_x: usize,
        bound_y: usize,
    ) -> Option<(usize, usize)> {
        match self {
            Direction::North if y == 0 => None,
            Direction::East if x + 1 == bound_x => None,
            Direction::South if y + 1 == bound_y => None,
            Direction::West if x == 0 => None,
            Direction::North => Some((x, y - 1)),
            Direction::East => Some((x + 1, y)),
            Direction::South => Some((x, y + 1)),
            Direction::West => Some((x - 1, y)),
        }
    }

    pub fn left(self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    pub fn right(self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    pub fn step_unbounded(self, x: isize, y: isize) -> (isize, isize) {
        match self {
            Direction::North => (x, y - 1),
            Direction::East => (x + 1, y),
            Direction::South => (x, y + 1),
            Direction::West => (x - 1, y),
        }
    }
}
