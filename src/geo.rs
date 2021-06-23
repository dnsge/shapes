use std::fmt;

#[derive(Copy, Clone)]
pub struct Point2(pub f32, pub f32);

#[derive(Copy, Clone)]
pub struct Point3(pub f32, pub f32, pub f32);

#[derive(Copy, Clone)]
pub struct Point4(pub f32, pub f32, pub f32, pub f32);

impl Point4 {
    pub fn hom_to_euc(&self) -> Point3 {
        assert_ne!(self.3, 0.0); // don't handle points at infinity

        if self.3 == 1.0 {
            Point3(
                self.0,
                self.1,
                self.2,
            )
        } else {
            Point3(
                self.0 / self.3,
                self.1 / self.3,
                self.2 / self.3,
            )
        }
    }

    pub fn add(&self, x: f32, y: f32, z: f32, w: f32) -> Point4 {
        Point4(self.0 + x, self.1 + y, self.2 + z, self.3 + w)
    }

}

impl Point3 {
    pub fn hom_to_euc(&self) -> Point2 {
        assert_ne!(self.2, 0.0); // don't handle points at infinity

        if self.2 == 1.0 {
            Point2(self.0, self.1)
        } else {
            Point2(self.0 / self.2, self.1 / self.2)
        }
    }

    pub fn euc_to_hom(&self) -> Point4 {
        Point4(self.0, self.1, self.2, 1.0)
    }

    pub fn add(&self, x: f32, y: f32, z: f32) -> Point3 {
        Point3(self.0 + x, self.1 + y, self.2 + z)
    }
}

impl fmt::Display for Point2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl fmt::Display for Point3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.0, self.1, self.2)
    }
}

impl fmt::Display for Point4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.0, self.1, self.2, self.3)
    }
}
