use std::{fmt, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign}};

pub struct Vec3<T: Arithmetic> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Arithmetic> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn distance_squared(&self, other: &Vec3<T>) -> T {
        let a = other.x.clone() - self.x.clone();
        let b = other.y.clone() - self.y.clone();
        let c = other.z.clone() - self.z.clone();
        a * a + b * b + c * c
    }
}

impl<T: Arithmetic + fmt::Display> fmt::Debug for Vec3<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

// Define trait
#[rustfmt::skip]
pub trait Arithmetic:
    Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self>
        + AddAssign + SubAssign + MulAssign + DivAssign + Copy + Ord + Sized { }

#[rustfmt::skip]
impl<T> Arithmetic for T where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>
        + AddAssign + SubAssign + MulAssign + DivAssign + Copy + Ord { }
