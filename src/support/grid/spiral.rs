use super::{Coord, Grid};
pub struct SpiralCoordIterator(Box<dyn Iterator<Item = Coord>>);

impl SpiralCoordIterator {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self(SpiralCoordIterator::from_attr((0, rows), (0, cols)))
    }

    fn from_attr((sr, er): Coord, (sc, ec): Coord) -> Box<dyn Iterator<Item = Coord>> {
        if sr > er || sc > ec {
            return Box::new(std::iter::empty());
        }
        Box::new(
            (sc..ec)
                .map(move |c| (sr, c))
                .chain((sr + 1..er).map(move |r| (r, ec - 1)))
                .chain((sc..ec - 1).rev().map(move |c| (er - 1, c)))
                .chain((sr + 1..er - 1).rev().map(move |r| (r, sc)))
                .chain(SpiralCoordIterator::from_attr(
                    (sr + 1, er - 1),
                    (sc + 1, ec - 1),
                )),
        )
    }
}

impl Iterator for SpiralCoordIterator {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

pub struct Spiral<'a, T> {
    grid: &'a Grid<T>,
    spiralcoorditerator: SpiralCoordIterator,
}

impl<'a, T> Spiral<'a, T> {
    pub fn new(grid: &'a Grid<T>) -> Self {
        Self {
            grid,
            spiralcoorditerator: SpiralCoordIterator::new(grid.nrows(), grid.ncols()),
        }
    }
}

impl<'a, T> Iterator for Spiral<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        Some(&self.grid[self.spiralcoorditerator.next().unwrap()])
    }
}
