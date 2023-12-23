use super::coordinate::Coordinate;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[allow(non_upper_case_globals)]
impl Direction {
    pub const Up: Direction = Direction::North;
    pub const Right: Direction = Direction::East;
    pub const Down: Direction = Direction::South;
    pub const Left: Direction = Direction::West;

    pub fn step(self, coord: Coordinate, bounds: Coordinate) -> Option<Coordinate> {
        match self {
            Direction::North if coord.y == 0 => None,
            Direction::East if coord.x + 1 == bounds.x => None,
            Direction::South if coord.y + 1 == bounds.y => None,
            Direction::West if coord.x == 0 => None,
            Direction::North => Some(Coordinate {
                x: coord.x,
                y: coord.y - 1,
            }),
            Direction::East => Some(Coordinate {
                x: coord.x + 1,
                y: coord.y,
            }),
            Direction::South => Some(Coordinate {
                x: coord.x,
                y: coord.y + 1,
            }),
            Direction::West => Some(Coordinate {
                x: coord.x - 1,
                y: coord.y,
            }),
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

    pub fn move_for(self, x: isize, y: isize, distance: isize) -> (isize, isize) {
        match self {
            Direction::North => (x, y - distance),
            Direction::East => (x + distance, y),
            Direction::South => (x, y + distance),
            Direction::West => (x - distance, y),
        }
    }

    pub fn opposite(self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }

    pub const ALL: [Direction; 4] = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];
}
