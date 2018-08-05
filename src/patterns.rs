extern crate ndarray;

/// A pattern for Conway's Game of Life that can be written to a game board.
///
/// ```
/// extern crate gol;
/// extern crate ndarray;
///
/// // Pattern that draws a 2x2 block
/// struct Block {}
///
/// impl gol::GOLPattern for Block {
///     fn place(&self, read: &mut ndarray::Array2<bool>, i: usize, j: usize) {
///         read[[i, j]] = true;
///         read[[i + 1, j]] = true;
///         read[[i, j + 1]] = true;
///         read[[i + 1, j + 1]] = true;
///     }
///
///     fn get_dims(&self) -> (usize, usize) {
///         (2, 2)
///     }
/// }
/// ```
pub trait GOLPattern {
    /// Places the pattern onto the given game board starting at the given
    /// position.
    ///
    /// This function must not change any values outside the region of the
    /// given starting position plus the pattern dimensions returned by
    /// `get_dims`.
    fn place(&self, read: &mut ndarray::Array2<bool>, i: usize, j: usize);

    /// Returns the dimensions of the region that the pattern will be written
    /// to.
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
