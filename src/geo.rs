use crate::matrix::Matrix;
use crate::render::make_rotation_matrix;
use std::{default, fmt, ops};

#[derive(Copy, Clone, PartialEq)]
pub struct Point<const D: usize> {
    coords: [f32; D],
}

impl<const D: usize> Point<D> {
    pub const fn dimension() -> usize {
        D
    }

    pub fn new(coords: [f32; D]) -> Point<D> {
        Point { coords }
    }

    pub fn coords(&self) -> [f32; D] {
        self.coords
    }

    pub fn add(&self, other: [f32; D]) -> Point<D> {
        let mut res: [f32; D] = [0.0; D];
        for i in 0..D {
            res[i] = self.coords[i] + other[i];
        }
        Point::new(res)
    }

    pub fn sub(&self, other: [f32; D]) -> Point<D> {
        let mut res: [f32; D] = [0.0; D];
        for i in 0..D {
            res[i] = self.coords[i] - other[i];
        }
        Point::new(res)
    }

    pub fn scale(&self, by: f32) -> Point<D> {
        let mut res: [f32; D] = [0.0; D];
        for i in 0..D {
            res[i] = self.coords[i] * by;
        }
        Point::new(res)
    }

    pub fn add_point(&self, other: Point<D>) -> Point<D> {
        self.add(other.coords)
    }

    pub fn sub_point(&self, other: Point<D>) -> Point<D> {
        self.sub(other.coords)
    }

    pub fn dot(&self, other: Point<D>) -> f32 {
        let mut total: f32 = 0.0;
        for i in 0..D {
            total += self[i] * other[i];
        }
        total
    }

    pub fn mid(&self, other: Point<D>) -> Point<D> {
        let mut res: [f32; D] = [0.0; D];
        for i in 0..D {
            res[i] = (self.coords[i] + other.coords[i]) / 2.0;
        }
        Point::new(res)
    }

    pub fn magnitude(&self) -> f32 {
        let mut total: f32 = 0.0;
        for i in 0..D {
            total += self[i].powi(2);
        }
        total.sqrt()
    }

    pub fn normalize(&self) -> Point<D> {
        self.scale(self.magnitude().recip())
    }

    pub fn first(&self) -> f32 {
        self.coords[0]
    }

    pub fn last(&self) -> f32 {
        self.coords[D - 1]
    }

    pub fn to_wide_matrix(&self) -> Matrix<1, D> {
        Matrix::new([self.coords])
    }

    pub fn to_tall_matrix(&self) -> Matrix<D, 1> {
        let mut res: Matrix<D, 1> = Matrix::default();
        for i in 0..D {
            res[(0, i)] = self.coords[i];
        }
        res
    }
}

impl<const D: usize> default::Default for Point<D> {
    fn default() -> Self {
        Point { coords: [0.0; D] }
    }
}

impl<const D: usize> ops::Index<usize> for Point<D> {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.coords[index]
    }
}

impl<const D: usize> ops::IndexMut<usize> for Point<D> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.coords[index]
    }
}

impl<const D: usize> fmt::Display for Point<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = write!(f, "("); // silence, compiler
        for i in 0..D {
            let _ = write!(f, "{}", self.coords[i]);
            if i != D - 1 {
                let _ = write!(f, ", ");
            }
        }
        write!(f, ")")
    }
}

pub type Point2 = Point<2>;
pub type Point3 = Point<3>;
pub type Point4 = Point<4>;

impl Point4 {
    pub fn hom_to_euc(&self) -> Point3 {
        assert_ne!(self[3], 0.0); // don't handle points at infinity

        if self[3] == 1.0 {
            Point3::new([self[0], self[1], self[2]])
        } else {
            Point3::new([self[0] / self[3], self[1] / self[3], self[2] / self[3]])
        }
    }
}

impl Point3 {
    pub fn hom_to_euc(&self) -> Point2 {
        assert_ne!(self.coords[2], 0.0); // don't handle points at infinity

        if self.coords[2] == 1.0 {
            Point2::new([self.coords[0], self.coords[1]])
        } else {
            Point2::new([
                self.coords[0] / self.coords[2],
                self.coords[1] / self.coords[2],
            ])
        }
    }

    pub fn euc_to_hom(&self) -> Point4 {
        Point4::new([self.coords[0], self.coords[1], self.coords[2], 1.0])
    }

    pub fn cross(&self, other: Point3) -> Point3 {
        Point3::new([
            self.coords[1] * other.coords[2] - self.coords[2] * other.coords[1],
            -(self.coords[0] * other.coords[2] - self.coords[2] * other.coords[0]),
            self.coords[0] * other.coords[1] - self.coords[1] * other.coords[0],
        ])
    }

    pub fn to_tuple(&self) -> (f32, f32, f32) {
        (self.coords[0], self.coords[1], self.coords[2])
    }
}

pub fn rotate_point(p: Point3, center: Point3, rot: (f32, f32, f32)) -> Point3 {
    rotate_point_with_matrix(p, center, &make_rotation_matrix(rot.0, rot.1, rot.2))
}

pub fn rotate_point_with_matrix(p: Point3, center: Point3, rot_matrix: &Matrix<3, 3>) -> Point3 {
    // 1. Translate p so that center is now at origin
    let mut n = p.sub_point(center);
    // 2. Rotate n about origin by rot
    n = *rot_matrix * n;
    // 3. Translate p back towards center
    n.add_point(center)
}
