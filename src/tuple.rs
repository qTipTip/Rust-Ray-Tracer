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
}