use super::coordinate::Coordinate;

pub trait Grid<T> {
    fn at(&self, coord: Coordinate) -> &T;
    fn bounds(&self) -> Coordinate;
}

impl<T> Grid<T> for Vec<Vec<T>> {
    fn at(&self, coord: Coordinate) -> &T {
        &self[coord.y][coord.x]
    }

    fn bounds(&self) -> Coordinate {
        Coordinate {
            x: self[0].len(),
            y: self.len(),
        }
    }
}
