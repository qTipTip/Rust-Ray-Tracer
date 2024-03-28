use std::ops::{Add, Sub};

#[derive(PartialEq, Debug)]
struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: u8,
}

impl Tuple {
    fn new(x: f64, y: f64, z: f64, w: u8) -> Tuple {
        Tuple { x, y, z, w }
    }

    fn is_point(&self) -> bool {
        self.w == 1
    }

    fn is_vector(&self) -> bool {
        self.w == 0
    }

    fn point(x: f64, y: f64, z: f64) -> Tuple {
        Tuple { x, y, z, w: 1 }
    }

    fn vector(x: f64, y: f64, z: f64) -> Tuple {
        Tuple { x, y, z, w: 0 }
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
        Self{
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
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
        let c = Tuple::new(4.3, -4.20000000000001, 3.1, 0);

        assert!(a != b);
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
}