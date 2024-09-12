use std::ops::{Index, IndexMut};

pub struct Grid<T>(Vec<Vec<T>>);
type Coord = (usize, usize);

impl<T> Grid<T> {
    pub fn new(value: Vec<Vec<T>>) -> Self {
        Self(value)
    }
    pub fn rows(&self) -> usize {
        self.0.len()
    }
    pub fn cols(&self) -> usize {
        self.0[0].len()
    }
    #[rustfmt::skip]
    pub fn adj4(&self, coord: Coord) -> Vec<Coord> {
        let (x, y) = coord;
        let mut ans = vec![];
        if x > 0 { ans.push((x - 1, y)) };
        if x < self.rows() - 1 { ans.push((x + 1, y)) };
        if y > 0 { ans.push((x, y-1)) };
        if y < self.cols() - 1 { ans.push((x, y+1)) };
        ans
    }
    #[rustfmt::skip]
    pub fn adj8(&self, coord: Coord) -> Vec<Coord> {
        match coord.0 {
            0 => match coord.1 {
                0 => vec![(0, 1), (1, 0), (1, 1)],
                y if y == self.cols() - 1 => vec![(0, y - 1), (1, y), (1, y - 1)],
                y => vec![(0, y - 1), (0, y + 1), (1, y - 1), (1, y), (1, y + 1)],
            },
            x if x == self.rows() - 1 => match coord.1 {
                0 => vec![(x, 1), (x - 1, 0), (x - 1, 1)],
                y if y == self.cols() - 1 => vec![(x, y - 1), (x - 1, y), (x - 1, y - 1)],
                y => vec![ (x, y - 1), (x, y + 1), (x - 1, y - 1), (x - 1, y), (x - 1, y + 1), ],
            },
            x => match coord.1 {
                0 => vec![(x - 1, 0), (x - 1, 1), (x, 1), (x + 1, 0), (x + 1, 1)],
                y if y == self.cols() - 1 => vec![ (x - 1, y - 1), (x - 1, y), (x, y - 1), (x + 1, y - 1), (x + 1, y), ],
                y => vec![ (x - 1, y - 1), (x - 1, y), (x - 1, y + 1), (x, y - 1), (x, y + 1), (x + 1, y - 1), (x + 1, y), (x + 1, y + 1), ],
            },
        }
    }
}

impl<T> Index<Coord> for Grid<T> {
    type Output = T;
    #[rustfmt::skip]
    fn index(&self, index: Coord) -> &Self::Output { &self.0[index.0][index.1] }
}

impl<T> IndexMut<Coord> for Grid<T> {
    #[rustfmt::skip]
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output { &mut self.0[index.0][index.1] }
}
