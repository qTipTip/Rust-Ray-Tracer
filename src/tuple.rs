struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: u8,
}

impl Tuple {
    fn is_point(&self) -> bool {
        self.w == 1
    }

    fn is_vector(&self) -> bool {
        self.w == 0
    }
}


#[cfg(test)]
mod tests {
    use crate::tuple::Tuple;

    #[test]
    fn test_tuple_point() {
        let a = Tuple { x: 4.3, y: -4.2, z: 3.1, w: 1 };

        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert!(a.is_point());
        assert!(!a.is_vector());
    }

    #[test]
    fn test_tuple_vector() {
        let a = Tuple { x: 4.3, y: -4.2, z: 3.1, w: 0 };

        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert!(!a.is_point());
        assert!(a.is_vector());
    }
}