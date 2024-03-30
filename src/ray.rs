use crate::tuple::Tuple;

struct Ray {
    origin: Tuple,
    direction: Tuple,
}

impl Ray {
    fn new(origin: Tuple, direction: Tuple) -> Self {
        if origin.is_vector() || direction.is_point() {
            panic!("origin must be point and direction must be vector");
        }
        Self { origin, direction }
    }
}

#[cfg(test)]
mod tests {
    use crate::ray::Ray;
    use crate::tuple::Tuple;

    #[test]
    fn create_ray() {
        let origin = Tuple::point(1.0, 2.0, 3.0);
        let direction = Tuple::vector(4.0, 5.0, 6.0);
        let r = Ray::new(
            origin,
            direction,
        );

        assert_eq!(r.origin, origin);
        assert_eq!(r.direction, direction);
    }
}