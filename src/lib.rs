extern crate ndarray;

pub trait GOLPattern {
    fn place(&self, read: &mut ndarray::Array2<bool>, i: usize, j: usize);
    fn get_dims(&self) -> (usize, usize);
}

#[derive(Debug)]
pub struct Blinker {}

impl GOLPattern for Blinker {
    fn place(&self, read: &mut ndarray::Array2<bool>, i: usize, j: usize) {
        read[[i, j + 1]] = true;
        read[[i + 1, j + 1]] = true;
        read[[i + 2, j + 1]] = true;
    }

    fn get_dims(&self) -> (usize, usize) {
        return (3, 3)
    }
}

#[derive(Debug)]
pub struct Glider {}

impl GOLPattern for Glider {
    fn place(&self, read: &mut ndarray::Array2<bool>, i: usize, j: usize) {
        read[[i, j + 1]] = true;
        read[[i + 1, j + 2]] = true;
        read[[i + 2, j]] = true;
        read[[i + 2, j + 1]] = true;
        read[[i + 2, j + 2]] = true;
    }

    fn get_dims(&self) -> (usize, usize) {
        return (3, 3)
    }
}
