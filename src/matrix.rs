use std::{default, fmt, ops};

use crate::world::Point;

#[derive(Copy, Clone)]
pub struct Matrix<const A: usize, const B: usize> {
    data: [[f32; B]; A],
}

impl<const A: usize, const B: usize> Matrix<A, B> {
    pub fn new(data: [[f32; B]; A]) -> Matrix<A, B> {
        Matrix { data }
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

    pub fn transpose(&self) -> Matrix<B, A> {
        let mut res: Matrix<B, A> = Matrix::default();
        for y in 0..A {
            let row = self.data[y];
            for x in 0..B {
                res[(y, x)] = row[x];
            }
        }
        res
    }

    pub fn scalar(&self, by: f32) -> Matrix<A, B> {
        let mut res: Matrix<A, B> = Matrix::default();
        for y in 0..A {
            for x in 0..B {
                res[(x, y)] = self.data[y][x] * by;
            }
        }
        res
    }
}

impl<const A: usize, const B: usize> default::Default for Matrix<A, B> {
    fn default() -> Self {
        Matrix {
            data: [[0.0; B]; A],
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

impl<const A: usize, const B: usize> ops::Mul<f32> for Matrix<A, B> {
    type Output = Matrix<A, B>;

    fn mul(self, rhs: f32) -> Self::Output {
        self.scalar(rhs)
    }
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
        let as_matrix: Matrix<B, 1> = rhs.to_matrix();
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

impl Matrix<2, 2> {
    pub fn determinant(&self) -> f32 {
        self.data[0][0] * self.data[1][1] - self.data[0][1] * self.data[1][0]
    }
}

impl Matrix<3, 3> {
    pub fn determinant(&self) -> f32 {
        let a = (self.data[1][1] * self.data[2][2] - self.data[1][2] * self.data[2][1]);
        let b = (self.data[1][0] * self.data[2][2] - self.data[1][2] * self.data[2][0]);
        let c = (self.data[1][0] * self.data[2][1] - self.data[1][1] * self.data[2][0]);
        (self.data[0][0] * a) - (self.data[0][1] * b) + (self.data[0][2] * c)
    }

    pub fn matrix_of_minors(&self) -> Matrix<3, 3> {
        let mut res: Matrix<3, 3> = Matrix::default();
        for yi in 0..3 {
            for xi in 0..3 {
                let minor = self.minor_matrix::<2>(xi, yi);
                res[(xi, yi)] = minor.determinant();
            }
        }
        res
    }

    pub fn inverse(&self) -> Matrix<3, 3> {
        let minors: Matrix<3, 3> = self.matrix_of_minors();
        let adjugate: Matrix<3, 3> = minors.apply_alternating_signs().transpose();
        let det = self.determinant_from_minors(minors);
        adjugate.scalar(det.recip())
    }
}

impl Matrix<4, 4> {
    pub fn determinant(&self) -> f32 {
        let a = self.minor_matrix::<3>(0, 0).determinant();
        let b = self.minor_matrix::<3>(1, 0).determinant();
        let c = self.minor_matrix::<3>(2, 0).determinant();
        let d = self.minor_matrix::<3>(3, 0).determinant();
        (self.data[0][0] * a) - (self.data[0][1] * b) + (self.data[0][2] * c)
            - (self.data[0][3] * d)
    }

    fn matrix_of_minors(&self) -> Matrix<4, 4> {
        let mut res: Matrix<4, 4> = Matrix::default();
        for yi in 0..4 {
            for xi in 0..4 {
                res[(xi, yi)] = self.minor_matrix::<3>(xi, yi).determinant();
            }
        }
        res
    }

    pub fn inverse(&self) -> Matrix<4, 4> {
        let minors: Matrix<4, 4> = self.matrix_of_minors();
        let adjugate: Matrix<4, 4> = minors.apply_alternating_signs().transpose();
        let det = self.determinant_from_minors(minors);
        adjugate.scalar(det.recip())
    }
}

// square matrix implementations
impl<const A: usize> Matrix<A, A> {
    // Until rust has constant expressions, we have to use a placeholder B and enforce B = A - 1
    pub fn minor_matrix<const B: usize>(&self, remove_x: usize, remove_y: usize) -> Matrix<B, B> {
        debug_assert!(B < A, "minor matrix result must be smaller");
        debug_assert_eq!(A - B, 1, "minor matrix result must be one size smaller");

        let mut res: Matrix<B, B> = Matrix::default();
        let mut res_x: usize = 0;
        let mut res_y: usize = 0;

        for yi in 0..A {
            if yi == remove_y {
                continue;
            }

            for xi in 0..A {
                if xi == remove_x {
                    continue;
                }
                res[(res_x, res_y)] = self.data[yi][xi];
                res_x += 1;
            }
            res_y += 1;
            res_x = 0;
        }

        res
    }

    pub fn determinant_from_minors(&self, minors: Matrix<A, A>) -> f32 {
        let mut sum: f32 = 0.0;
        for x in 0..A {
            if x % 2 == 0 {
                sum += self.data[0][x] * minors.data[0][x]
            } else {
                sum -= self.data[0][x] * minors.data[0][x]
            }
        }
        sum
    }

    fn apply_alternating_signs(&self) -> Matrix<A, A> {
        let mut res: Matrix<A, A> = Matrix::default();
        for yi in 0..A {
            for xi in 0..A {
                let multiplier = if ((xi % 2) + (yi % 2)) % 2 == 0 {
                    1.0
                } else {
                    -1.0
                };

                res[(xi, yi)] = self[(xi, yi)] * multiplier;
            }
        }
        res
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
