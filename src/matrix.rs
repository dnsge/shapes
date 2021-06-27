use std::{default, fmt, ops};

use crate::geo::{Point};

#[derive(Copy, Clone)]
pub struct Matrix<const A: usize, const B: usize> {
    data: [[f32; B]; A],
}

impl<const A: usize, const B: usize> Matrix<A, B> {
    pub fn new(data: [[f32; B]; A]) -> Matrix<A, B> {
        Matrix {
            data
        }
    }

    pub fn row(&self, col: usize) -> Option<[f32; B]> {
        if col >= A {
            None
        } else {
            Some(self.data[col])
        }
    }

    pub fn col(&self, row: usize) -> Option<[f32; A]> {
        if row >= B {
            return None;
        }

        let mut res: [f32; A] = [0.0; A];
        for i in 0..A {
            res[i] = self.data[i][row];
        }

        return Some(res);
    }
}

impl<const A: usize, const B: usize> default::Default for Matrix<A, B> {
    fn default() -> Self {
        Matrix {
            data: [[0.0; B]; A]
        }
    }
}

impl<const A: usize> Matrix<A, 1> {
    pub fn tall_to_point(&self) -> Point<A> {
        Point::new(self.col(0).unwrap())
    }
}

impl<const B: usize> Matrix<1, B> {
    pub fn wide_to_point(&self) -> Point<B> {
        Point::new(self.row(0).unwrap())
    }
}

fn vector_dot<const N: usize>(lh: [f32; N], rh: [f32; N]) -> f32 {
    let mut sum: f32 = 0.0;
    for i in 0..N {
        sum += lh[i] * rh[i];
    }
    sum
}

impl<const A: usize, const B: usize, const C: usize> ops::Mul<Matrix<B, C>> for Matrix<A, B> {
    type Output = Matrix<A, C>;

    fn mul(self, rhs: Matrix<B, C>) -> Self::Output {
        let mut res: Matrix<A, C> = Matrix::default();

        // matrix product
        // row of left multiplied by column of right
        for y in 0..A {
            for x in 0..C {
                let curr_row: [f32; B] = self.row(y).unwrap();
                let curr_col: [f32; B] = rhs.col(x).unwrap();
                res[(x, y)] = vector_dot(curr_row, curr_col);
            }
        }

        res
    }
}

impl<const A: usize, const B: usize> ops::Mul<Point<B>> for Matrix<A, B> {
    type Output = Point<A>;

    /* e.g.
     *
     * [2 0 0]   [2]     [4]
     * [0 2 0] * [3] --> [6]
     * [0 0 2]   [4]     [8]
     * [0 0 0]           [0]
     *  4 x 3 * 3 x 1 = 4 x 1
     *  A X B * B x 1 = A x 1
     */
    fn mul(self, rhs: Point<B>) -> Self::Output {
        let as_matrix: Matrix<B, 1> = rhs.to_tall_matrix();
        (self * as_matrix).tall_to_point()
    }
}

impl<const A: usize, const B: usize> ops::Index<(usize, usize)> for Matrix<A, B> {
    type Output = f32;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.1][index.0]
    }
}

impl<const A: usize, const B: usize> ops::IndexMut<(usize, usize)> for Matrix<A, B> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.1][index.0]
    }
}

impl<const A: usize, const B: usize> fmt::Display for Matrix<A, B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = write!(f, "[\n");
        for row in 0..A {
            let _ = write!(f, "[");
            for col in 0..B {
                let _ = write!(f, "{}", self[(col, row)]);
                if col != B - 1 {
                    let _ = write!(f, ", ");
                }
            }
            let _ = write!(f, "]\n");
        }
        write!(f, "]")
    }
}
