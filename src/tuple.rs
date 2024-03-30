use std::ops::{Add, Div, Mul, Neg, Sub};
use crate::utils;

#[derive(Debug, Copy, Clone)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: u8,
}


impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: u8) -> Self {
        Tuple { x, y, z, w }
    }

    pub fn is_point(&self) -> bool {
        self.w == 1
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0
    }

    pub fn point(x: f64, y: f64, z: f64) -> Self {
        Tuple { x, y, z, w: 1 }
    }

    pub fn vector(x: f64, y: f64, z: f64) -> Self {
        Tuple { x, y, z, w: 0 }
    }

    pub fn origin() -> Self {
        Self::point(0.0, 0.0, 0.0)
    }

    pub fn zero_vector() -> Self {
        Self::vector(0.0, 0.0, 0.0)
    }

    pub fn abs(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z + (self.w * self.w) as f64).sqrt()
    }

    pub fn norm(&self) -> Self {
        *self / self.abs()
    }

    pub fn dot(&self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + (self.w * rhs.w) as f64
    }

    pub fn cross(&self, rhs: Self) -> Self {
        Self::vector(self.y * rhs.z - self.z * rhs.y, self.z * rhs.x - self.x * rhs.z, self.x * rhs.y - self.y * rhs.x)
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        if self.w != other.w {
            false
        } else {
            (self.x - other.x).abs() < utils::F64_ERROR_MARGIN &&
                (self.y - other.y).abs() < utils::F64_ERROR_MARGIN &&
                (self.z - other.z).abs() < utils::F64_ERROR_MARGIN
        }
    }
}

impl Add for Tuple {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::zero_vector() - self
    }
}

impl Mul<f64> for Tuple {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w,
        }
    }
}

impl Div<f64> for Tuple {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tuple::Tuple;

    #[test]
    fn test_tuple_point() {
        let a = Tuple::new(4.3, -4.2, 3.1, 1);

        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert!(a.is_point());
        assert!(!a.is_vector());
    }

    #[test]
    fn test_tuple_vector() {
        let a = Tuple::new(4.3, -4.2, 3.1, 0);

        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert!(!a.is_point());
        assert!(a.is_vector());
    }

    #[test]
    fn test_point() {
        let a = Tuple::point(4.3, -4.2, 3.1);
        assert!(a.is_point());
        assert!(!a.is_vector());
    }

    #[test]
    fn test_vector() {
        let a = Tuple::vector(4.3, -4.2, 3.1);
        assert!(!a.is_point());
        assert!(a.is_vector());
    }

    #[test]
    fn test_equality() {
        let a = Tuple::new(4.3, -4.2, 3.1, 1);
        let b = Tuple::new(4.3, -4.2, 3.1, 0);
        let c = Tuple::new(4.3, -4.20000001, 3.1, 0);

        assert_ne!(a, b);
        assert!(c != a && c != b && c == c);
    }

    #[test]
    fn test_add_tuples() {
        let a = Tuple::new(4.3, -4.2, 3.1, 1);
        let b = Tuple::new(1.0, 0.0, 1.0, 0);
        let c = Tuple::new(5.3, -4.2, 4.1, 1);
        assert_eq!(a + b, c);
    }

    #[test]
    fn test_subtract_points_gives_vector() {
        let a = Tuple::point(3.0, 2.0, 1.0);
        let b = Tuple::point(5.0, 6.0, 7.0);
        let c = Tuple::vector(-2.0, -4.0, -6.0);

        assert_eq!(a - b, c);
    }

    #[test]
    fn test_subtract_vector_from_point_gives_point() {
        let a = Tuple::point(3.0, 2.0, 1.0);
        let b = Tuple::vector(5.0, 6.0, 7.0);
        let c = Tuple::point(-2.0, -4.0, -6.0);

        assert_eq!(a - b, c);
    }

    #[test]
    fn test_subtract_vector_from_vector_gives_vector() {
        let a = Tuple::vector(3.0, 2.0, 1.0);
        let b = Tuple::vector(5.0, 6.0, 7.0);
        let c = Tuple::vector(-2.0, -4.0, -6.0);

        assert_eq!(a - b, c);
    }

    #[test]
    fn test_negation() {
        let a = Tuple::vector(3.0, 2.0, 1.0);
        let b = Tuple::vector(-3.0, -2.0, -1.0);

        assert_eq!(-a, b);
        assert!((-a).is_vector());
    }

    #[test]
    fn test_scalar_multiplication() {
        let a = Tuple::vector(3.0, 2.0, 1.0);
        let x = 2.0;
        let c = Tuple::vector(6.0, 4.0, 2.0);

        assert_eq!(a * x, c);
    }

    #[test]
    fn test_scalar_division() {
        let a = Tuple::vector(3.0, 2.0, 1.0);
        let x = 2.0;
        let c = Tuple::vector(1.5, 1.0, 0.5);

        assert_eq!(a / x, c);
    }

    #[test]
    fn test_abs() {
        assert_eq!(Tuple::vector(1.0, 0.0, 0.0).abs(), 1.0);
        assert_eq!(Tuple::vector(0.0, 1.0, 0.0).abs(), 1.0);
        assert_eq!(Tuple::vector(0.0, 0.0, 1.0).abs(), 1.0);
        assert_eq!(Tuple::vector(1.0, 2.0, 3.0).abs(), f64::sqrt(14.0));
        assert_eq!(Tuple::vector(-1.0, -2.0, -3.0).abs(), f64::sqrt(14.0));
    }

    #[test]
    fn test_normalize() {
        let a = Tuple::vector(4.0, 0.0, 0.0);
        let b = Tuple::vector(1.0, 0.0, 0.0);

        assert_eq!(a.norm(), b);

        let c = Tuple::vector(1.0, 2.0, 3.0);
        assert_eq!(c.norm(), c / f64::sqrt(14.0));
        assert_eq!(c.norm().abs(), 1.0);
    }

    #[test]
    fn test_dot_product() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);

        assert_eq!(a.dot(b), 20.0);
    }

    #[test]
    fn test_cross_product() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);
        let c = Tuple::vector(-1.0, 2.0, -1.0);
        assert_eq!(a.cross(b), c);
        assert_eq!(b.cross(a), -c);
    }
}
