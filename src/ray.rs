use crate::tuple::Tuple;

pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

pub trait Intersect {
    fn ray_intersections(&self, ray: &Ray) -> Vec<f64>;
}

impl Ray {
    fn new(origin: Tuple, direction: Tuple) -> Self {
        if origin.is_vector() || direction.is_point() {
            panic!("origin must be point and direction must be vector");
        }
        Self { origin, direction }
    }

    fn position(&self, t: f64) -> Tuple {
        self.origin + self.direction * t
    }

    fn intersect(&self, object: &impl Intersect) -> Vec<f64> {
        object.ray_intersections(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::ray::Ray;
    use crate::sphere::Sphere;
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

    #[test]
    fn compute_position() {
        let origin = Tuple::point(2.0, 3.0, 4.0);
        let direction = Tuple::vector(1.0, 0.0, 0.0);
        let r = Ray::new(
            origin,
            direction,
        );

        assert_eq!(r.position(0.0), origin);
        assert_eq!(r.position(1.0), Tuple::point(3.0, 3.0, 4.0));
        assert_eq!(r.position(-1.0), Tuple::point(1.0, 3.0, 4.0));
        assert_eq!(r.position(2.5), Tuple::point(4.5, 3.0, 4.0));
    }

    #[test]
    fn test_intersect_sphere() {
        let r = Ray::new(
            Tuple::point(0.0, 0.0, -5.0),
            Tuple::vector(0.0, 0.0, 1.0),
        );

        let s = Sphere::new(1.0, Tuple::origin());
        let xs = r.intersect(&s);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], 4.0);
        assert_eq!(xs[1], 6.0);
    }

    #[test]
    fn test_intersect_sphere_at_tangent() {
        let r = Ray::new(
            Tuple::point(0.0, 1.0, -5.0),
            Tuple::vector(0.0, 0.0, 1.0),
        );

        let s = Sphere::new(1.0, Tuple::origin());
        let xs = r.intersect(&s);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], 5.0);
        assert_eq!(xs[1], 5.0);
    }

    #[test]
    fn test_does_not_intersect_sphere() {
        let r = Ray::new(
            Tuple::point(0.0, 1.1, -5.0),
            Tuple::vector(0.0, 0.0, 1.0),
        );

        let s = Sphere::new(1.0, Tuple::origin());
        let xs = r.intersect(&s);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn test_intersect_from_inside() {
        let r = Ray::new(
            Tuple::point(0.0, 0.0, 0.0),
            Tuple::vector(0.0, 0.0, 1.0),
        );

        let s = Sphere::new(1.0, Tuple::origin());
        let xs = r.intersect(&s);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], -1.0);
        assert_eq!(xs[1], 1.0);
    }

    #[test]
    fn test_intersect_from_behind() {
        let r = Ray::new(
            Tuple::point(0.0, 0.0, 5.0),
            Tuple::vector(0.0, 0.0, 1.0),
        );

        let s = Sphere::new(1.0, Tuple::origin());
        let xs = r.intersect(&s);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], -6.0);
        assert_eq!(xs[1], -4.0);
    }
}