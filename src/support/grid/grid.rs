use super::spiral::Spiral;
use std::ops::{Index, IndexMut};

pub struct Grid<T>(Vec<Vec<T>>);
pub type Coord = (usize, usize);

impl<T> Grid<T> {
    pub fn new(value: Vec<Vec<T>>) -> Self {
        Self(value)
    }
    pub fn nrows(&self) -> usize {
        self.0.len()
    }
    pub fn ncols(&self) -> usize {
        self.0[0].len()
    }
    pub fn into_inner(self) -> Vec<Vec<T>> {
        self.0
    }
    pub fn get(&self, coord: Coord) -> Option<&T> {
        self.0.get(coord.0).unwrap().get(coord.1)
    }
    pub fn get_mut(&mut self, coord: Coord) -> Option<&mut T> {
        self.0.get_mut(coord.0).unwrap().get_mut(coord.1)
    }
    pub fn adj4(&self, coord: Coord) -> impl Iterator<Item = Coord> + '_ {
        self.diamond(1, coord)
    }
    #[rustfmt::skip]
    pub fn adj8(&self, coord: Coord) -> Vec<Coord> {
        match coord.0 {
            0 => match coord.1 {
                0 => vec![(0, 1), (1, 0), (1, 1)],
                y if y == self.ncols() - 1 => vec![(0, y - 1), (1, y), (1, y - 1)],
                y => vec![(0, y - 1), (0, y + 1), (1, y - 1), (1, y), (1, y + 1)],
            },
            x if x == self.nrows() - 1 => match coord.1 {
                0 => vec![(x, 1), (x - 1, 0), (x - 1, 1)],
                y if y == self.ncols() - 1 => vec![(x, y - 1), (x - 1, y), (x - 1, y - 1)],
                y => vec![ (x, y - 1), (x, y + 1), (x - 1, y - 1), (x - 1, y), (x - 1, y + 1), ],
            },
            x => match coord.1 {
                0 => vec![(x - 1, 0), (x - 1, 1), (x, 1), (x + 1, 0), (x + 1, 1)],
                y if y == self.ncols() - 1 => vec![ (x - 1, y - 1), (x - 1, y), (x, y - 1), (x + 1, y - 1), (x + 1, y), ],
                y => vec![ (x - 1, y - 1), (x - 1, y), (x - 1, y + 1), (x, y - 1), (x, y + 1), (x + 1, y - 1), (x + 1, y), (x + 1, y + 1), ],
            },
        }
    }
    pub fn spiral(&self) -> Spiral<'_, T> {
        Spiral::new(self)
    }

    pub fn diamond(&self, diameter: usize, coord: Coord) -> impl Iterator<Item = Coord> + '_ {
        let rows = coord.0.saturating_sub(diameter)..(coord.0 + diameter + 1).min(self.nrows());
        let cols = (0..=diameter)
            .chain((0..diameter).rev())
            .map(move |d| {
                if d == 0 {
                    vec![coord.1]
                } else {
                    let mut cells = vec![];
                    if coord.1 >= d {
                        cells.push(coord.1 - d);
                    }
                    if coord.1 + d < self.ncols() {
                        cells.push(coord.1 + d);
                    }
                    cells
                }
            })
            .skip(if coord.0 >= diameter {
                0
            } else {
                diameter - coord.0
            });
        rows.zip(cols)
            .flat_map(|(r, cs)| cs.into_iter().map(move |c| (r, c)))
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
