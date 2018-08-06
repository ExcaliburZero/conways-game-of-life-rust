extern crate ndarray;
extern crate rand;

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
    /// to. (rows, columns)
    fn get_dims(&self) -> (usize, usize);
}

/// A Blinker pattern.
///
/// A line of three alive cells that blink by oscilating between a vertical
/// line and a horizontal line.
///
/// A region of 3x3 cells is resverved for placing the pattern, however it
/// currently only places a blinker in the vertical orientation in the middle
/// of the reserved area.
///
/// [Blinker page on LifeWiki](http://www.conwaylife.com/w/index.php?title=Blinker)
///
/// ```
/// let mut board = gol::Board::new(5, 5);
///
/// board.place_pattern(gol::Blinker {}, 0, 0);
///
/// let expected = String::from(
/// "oxooo
/// oxooo
/// oxooo
/// ooooo
/// ooooo
/// "
/// );
///
/// assert_eq!(format!("{}", board), expected);
/// ```
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

/// A Glider pattern.
///
/// An arrangement of cells in a 3x3 region that slowly moves across the board.
///
/// When placed on the board, only the classic phase and orientation is used.
///
/// [Glider page on LifeWiki](http://www.conwaylife.com/wiki/Glider)
///
/// ```
/// let mut board = gol::Board::new(5, 5);
///
/// board.place_pattern(gol::Glider {}, 0, 0);
///
/// let expected = String::from(
/// "oxooo
/// ooxoo
/// xxxoo
/// ooooo
/// ooooo
/// "
/// );
///
/// assert_eq!(format!("{}", board), expected);
/// ```
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

/// A random pattern.
///
/// A field of cells that are filled with a random pattern.
///
/// ```
/// let mut board = gol::Board::new(5, 5);
///
/// board.place_pattern(gol::RandomField {
///     rows: 5, columns: 5
/// }, 0, 0);
/// ```
#[derive(Debug)]
pub struct RandomField {
    pub rows: usize,
    pub columns: usize
}

impl GOLPattern for RandomField {
    fn place(&self, read: &mut ndarray::Array2<bool>, i: usize, j: usize) {
        for i2 in 0..self.rows {
            for j2 in 0..self.columns {
                let r = i + i2;
                let c = j + j2;

                read[[r, c]] = rand::random::<bool>();
            }
        }
    }

    fn get_dims(&self) -> (usize, usize) {
        return (self.rows, self.columns)
    }
}
