use crate::intersection::Intersections;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::tuple::Tuple;

pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

pub trait Intersect: std::fmt::Debug {
    fn ray_intersections(&self, ray: &Ray) -> Intersections;
    fn get_transform(&self) -> Option<Matrix>;
    fn get_material(&self) -> Option<Material>;
}

impl Ray {
    fn new(origin: Tuple, direction: Tuple) -> Self {
        if origin.is_vector() || direction.is_point() {
            panic!("origin must be point and direction must be vector");
        }
        Self { origin, direction }
    }

    pub fn position(&self, t: f64) -> Tuple {
        self.origin + self.direction * t
    }

    pub(crate) fn intersect(&self, object: &impl Intersect) -> Intersections {
        let transform = object.get_transform();

        match transform {
            None => object.ray_intersections(self),
            Some(t) => {
                let ray_transformed = self.transform(&t.inverse());
                object.ray_intersections(&ray_transformed)
            }
        }
    }

    fn transform(&self, transformation: &Matrix) -> Self {
        Self::new(
            transformation * self.origin,
            transformation * self.direction,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::ray::Ray;
    use crate::sphere::Sphere;
    use crate::transformations::{scaling, translation};
    use crate::tuple::Tuple;

    #[test]
    fn create_ray() {
        let origin = Tuple::point(1.0, 2.0, 3.0);
        let direction = Tuple::vector(4.0, 5.0, 6.0);
        let r = Ray::new(origin, direction);

        assert_eq!(r.origin, origin);
        assert_eq!(r.direction, direction);
    }

    #[test]
    fn compute_position() {
        let origin = Tuple::point(2.0, 3.0, 4.0);
        let direction = Tuple::vector(1.0, 0.0, 0.0);
        let r = Ray::new(origin, direction);

        assert_eq!(r.position(0.0), origin);
        assert_eq!(r.position(1.0), Tuple::point(3.0, 3.0, 4.0));
        assert_eq!(r.position(-1.0), Tuple::point(1.0, 3.0, 4.0));
        assert_eq!(r.position(2.5), Tuple::point(4.5, 3.0, 4.0));
    }

    #[test]
    fn test_intersect_sphere() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));

        let s = Sphere::unit();
        let xs = r.intersect(&s);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].time, 4.0);
        assert_eq!(xs[1].time, 6.0);
    }

    #[test]
    fn test_intersect_sphere_at_tangent() {
        let r = Ray::new(Tuple::point(0.0, 1.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));

        let s = Sphere::unit();
        let xs = r.intersect(&s);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].time, 5.0);
        assert_eq!(xs[1].time, 5.0);
    }

    #[test]
    fn test_does_not_intersect_sphere() {
        let r = Ray::new(Tuple::point(0.0, 1.1, -5.0), Tuple::vector(0.0, 0.0, 1.0));

        let s = Sphere::unit();
        let xs = r.intersect(&s);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn test_intersect_from_inside() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));

        let s = Sphere::unit();
        let xs = r.intersect(&s);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].time, -1.0);
        assert_eq!(xs[1].time, 1.0);
    }

    #[test]
    fn test_intersect_from_behind() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));

        let s = Sphere::unit();
        let xs = r.intersect(&s);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].time, -6.0);
        assert_eq!(xs[1].time, -4.0);
    }

    #[test]
    fn test_intersection_has_object_reference() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -0.5), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::unit();
        let xs = r.intersect(&s);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].object, s);
        assert_eq!(xs[1].object, s);
    }

    #[test]
    fn translate_ray() {
        let ray = Ray::new(Tuple::point(1.0, 2.0, 3.0), Tuple::vector(0.0, 1.0, 0.0));
        let m = translation(3.0, 4.0, 5.0);

        let r2: Ray = ray.transform(&m);

        assert_eq!(r2.origin, Tuple::point(4.0, 6.0, 8.0));
        assert_eq!(r2.direction, Tuple::vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn scale_ray() {
        let ray = Ray::new(Tuple::point(1.0, 2.0, 3.0), Tuple::vector(0.0, 1.0, 0.0));
        let m = scaling(2.0, 3.0, 4.0);

        let r2: Ray = ray.transform(&m);

        assert_eq!(r2.origin, Tuple::point(2.0, 6.0, 12.0));
        assert_eq!(r2.direction, Tuple::vector(0.0, 3.0, 0.0));
    }

    #[test]
    fn test_intersection_with_scaled_sphere() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let mut s = Sphere::unit();
        s.set_transform(&scaling(2.0, 2.0, 2.0));

        let xs = r.intersect(&s);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].object, s);
        assert_eq!(xs[1].object, s);

        assert_eq!(xs[0].time, 3.0);
        assert_eq!(xs[1].time, 7.0);
    }

    #[test]
    fn test_intersection_with_translated_sphere() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let mut s = Sphere::unit();
        s.set_transform(&translation(5.0, 0.0, 0.0));

        let xs = r.intersect(&s);

        assert_eq!(xs.len(), 0);
    }
}
