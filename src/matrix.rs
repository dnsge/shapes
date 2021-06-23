use std::fmt;

use crate::geo::{Point3, Point4};

#[derive(Copy, Clone)]
pub struct Matrix34(pub [[f32; 4]; 3]);

#[derive(Copy, Clone)]
pub struct Matrix33(pub [[f32; 3]; 3]);

impl Matrix34 {
    pub fn dot_point4(&self, p: Point4) -> Point3 {
        Point3::new([
            self.0[0][0] * p[0] + self.0[0][1] * p[1] + self.0[0][2] * p[2] + self.0[0][3] * p[3],
            self.0[1][0] * p[0] + self.0[1][1] * p[1] + self.0[1][2] * p[2] + self.0[1][3] * p[3],
            self.0[2][0] * p[0] + self.0[2][1] * p[1] + self.0[2][2] * p[2] + self.0[2][3] * p[3],
        ])
    }
}

impl Matrix33 {
    pub fn dot_point3(&self, p: Point3) -> Point3 {
        Point3::new([
            self.0[0][0] * p[0] + self.0[0][1] * p[1] + self.0[0][2] * p[2],
            self.0[1][0] * p[0] + self.0[1][1] * p[1] + self.0[1][2] * p[2],
            self.0[2][0] * p[0] + self.0[2][1] * p[1] + self.0[2][2] * p[2],
        ])
    }

    // row of self multiplied by col of m
    pub fn dot_matrix34(&self, m: Matrix34) -> Matrix34 {
        Matrix34([
            [
                self.0[0][0] * m.0[0][0] + self.0[0][1] * m.0[1][0] + self.0[0][2] * m.0[2][0],
                self.0[0][0] * m.0[0][1] + self.0[0][1] * m.0[1][1] + self.0[0][2] * m.0[2][1],
                self.0[0][0] * m.0[0][2] + self.0[0][1] * m.0[1][2] + self.0[0][2] * m.0[2][2],
                self.0[0][0] * m.0[0][3] + self.0[0][1] * m.0[1][3] + self.0[0][2] * m.0[2][3],
            ], [
                self.0[1][0] * m.0[0][0] + self.0[1][1] * m.0[1][0] + self.0[1][2] * m.0[2][0],
                self.0[1][0] * m.0[0][1] + self.0[1][1] * m.0[1][1] + self.0[1][2] * m.0[2][1],
                self.0[1][0] * m.0[0][2] + self.0[1][1] * m.0[1][2] + self.0[1][2] * m.0[2][2],
                self.0[1][0] * m.0[0][3] + self.0[1][1] * m.0[1][3] + self.0[1][2] * m.0[2][3],
            ], [
                self.0[2][0] * m.0[0][0] + self.0[2][1] * m.0[1][0] + self.0[2][2] * m.0[2][0],
                self.0[2][0] * m.0[0][1] + self.0[2][1] * m.0[1][1] + self.0[2][2] * m.0[2][1],
                self.0[2][0] * m.0[0][2] + self.0[2][1] * m.0[1][2] + self.0[2][2] * m.0[2][2],
                self.0[2][0] * m.0[0][3] + self.0[2][1] * m.0[1][3] + self.0[2][2] * m.0[2][3],
            ],
        ])
    }
}

fn format_3arr(arr: [f32; 3]) -> String {
    format!("[{}, {}, {}]", arr[0], arr[1], arr[2])
}

fn format_4arr(arr: [f32; 4]) -> String {
    format!("[{}, {}, {}, {}]", arr[0], arr[1], arr[2], arr[3])
}

impl fmt::Display for Matrix34 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[\n\t{}\n\t{}\n\t{}\n]",
               format_4arr(self.0[0]), format_4arr(self.0[1]), format_4arr(self.0[2]))
    }
}

impl fmt::Display for Matrix33 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[\n\t{}\n\t{}\n\t{}\n]",
               format_3arr(self.0[0]), format_3arr(self.0[1]), format_3arr(self.0[2]))
    }
}
