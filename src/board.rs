extern crate ndarray;

use std::cmp;
use std::fmt;

use self::ndarray::Dimension;

use patterns;

pub struct Board {
    a: ndarray::Array2<bool>,
    b: ndarray::Array2<bool>,
    generation: i32
}

impl Board {
    pub fn new(dims: (usize, usize)) -> Board {
        Board {
            a: ndarray::Array2::<bool>::default(dims),
            b: ndarray::Array2::<bool>::default(dims),
            generation: 0
        }
    }

    pub fn next_generation(&mut self) {
        let (read, write) = if self.generation % 2 == 0 {
            (&self.a, &mut self.b)
        } else {
            (&self.b, &mut self.a)
        };

        let (rows, columns) = get_dims(&read);
        for i in 0..rows {
            for j in 0..columns {
                let neighbors = count_neighbors(&read, i, j);

                let prev_state = read[[i, j]];
                let new_state = next_state(prev_state, neighbors);

                write[[i, j]] = new_state;
            }
        }

        self.generation += 1;
    }

    pub fn place_pattern<T: patterns::GOLPattern + fmt::Debug>(&mut self, pattern: T, i: usize, j: usize) {
        let read = if self.generation % 2 == 0 {
            &mut self.a
        } else {
            &mut self.b
        };

        if !check_fits(&read, &pattern, i, j) {
            panic!("{:?} does not fit at ({}, {})", pattern, i, j);
        }


        pattern.place(read, i, j);
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let read = if self.generation % 2 == 0 {
            &self.a
        } else {
            &self.b
        };

        let mut contents = String::new();

        for row in read.genrows() {
            for &cell in row {
                if cell == true {
                    contents.push_str("x");
                } else {
                    contents.push_str("o");
                }
            }
            contents.push_str("\n");
        }

        write!(f, "{}", contents)
    }
}

fn get_dims<T>(read: &ndarray::Array2<T>) -> (usize, usize) {
    match read.raw_dim().into_pattern() {
        (r, c) => (r, c)
    }
}

fn count_neighbors(read: &ndarray::Array2<bool>, i: usize, j: usize) -> i32 {
    let (rows, columns) = get_dims(&read);

    let start_row = if i == 0 { 0 } else { i - 1 };
    let end_row = cmp::min(rows, i + 2);

    let start_column = if j == 0 { 0 } else { j - 1 };
    let end_column = cmp::min(columns, j + 2);

    let mut count = 0;
    for i2 in start_row..end_row {
        for j2 in start_column..end_column {
            if (i2 != i || j2 != j) && read[[i2, j2]] {
                count += 1;
            }
        }
    }

    count
}

fn next_state(prev_state: bool, neighbors: i32) -> bool {
    if prev_state && neighbors < 2 {
        false
    } else if prev_state && neighbors < 4 {
        true
    } else if prev_state {
        false
    } else if neighbors == 3 {
        true
    } else {
        false
    }
}

fn check_fits<T: patterns::GOLPattern>(read: &ndarray::Array2<bool>, pattern: &T, i: usize, j: usize) -> bool {
    let (rows, columns) = get_dims(&read);
    let (height, width) = pattern.get_dims();

    let end_row = i + height - 1;
    let end_column = j + width - 1;

    end_row < rows && end_column < columns
}