pub struct Spiral(Box<dyn Iterator<Item = (usize, usize)>>);

impl Spiral {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self(Spiral::from_attr((0, rows), (0, cols)))
    }

    fn from_attr(
        (sr, er): (usize, usize),
        (sc, ec): (usize, usize),
    ) -> Box<dyn Iterator<Item = (usize, usize)>> {
        if sr > er || sc > ec {
            return Box::new(std::iter::empty());
        }
        Box::new(
            (sc..ec)
                .map(move |c| (sr, c))
                .chain((sr + 1..er).map(move |r| (r, ec - 1)))
                .chain((sc..ec - 1).rev().map(move |c| (er - 1, c)))
                .chain((sr + 1..er - 1).rev().map(move |r| (r, sc)))
                .chain(Spiral::from_attr((sr + 1, er - 1), (sc + 1, ec - 1))),
        )
    }
}

impl Iterator for Spiral {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
