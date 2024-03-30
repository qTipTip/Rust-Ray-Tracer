use crate::ray;
use crate::ray::Ray;
use crate::tuple::Tuple;

pub struct Sphere {
    radius: f64,
    origin: Tuple,
}

impl Sphere {
    pub(crate) fn new(radius: f64, origin: Tuple) -> Self {
        Sphere { radius, origin }
    }
}

impl ray::Intersect for Sphere {
    fn ray_intersections(&self, ray: &Ray) -> Vec<f64> {
        let sphere_to_ray = ray.origin - self.origin;

        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            Vec::new()
        } else {
            let disc_sqrt = discriminant.sqrt();
            let denom = 2.0 * a;
            let r1 = (-b - disc_sqrt) / (denom);
            let r2 = (-b + disc_sqrt) / (denom);

            vec![r1, r2]
        }
    }
}