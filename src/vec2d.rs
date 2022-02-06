use std::ops::{Index, IndexMut};

pub struct Vec2D<T> {
    cols: usize,
    data: Vec<T>,
}

impl<T> Vec2D<T> {
    pub fn new(cols: usize, data: Vec<T>) -> Self {
        Self { cols, data }
    }

    pub fn coordinates(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.data.len()).map(move |x| (x / self.cols, x % self.cols))
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl<T> Index<usize> for Vec2D<T> {
    type Output = [T];

    fn index(&self, row: usize) -> &[T] {
        let row_start = row * self.cols;
        &self.data[row_start..row_start + self.cols]
    }
}

impl<T> Index<(usize, usize)> for Vec2D<T> {
    type Output = T;

    fn index(&self, (row, col): (usize, usize)) -> &T {
        &self.data[row * self.cols + col]
    }
}

impl<T> IndexMut<usize> for Vec2D<T> {
    fn index_mut(&mut self, row: usize) -> &mut [T] {
        let row_start = row * self.cols;
        &mut self.data[row_start..row_start + self.cols]
    }
}

impl<T> IndexMut<(usize, usize)> for Vec2D<T> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut T {
        &mut self.data[row * self.cols + col]
    }
}
