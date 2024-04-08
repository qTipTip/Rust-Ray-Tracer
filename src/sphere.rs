use crate::intersection::{Intersection, Intersections};
use crate::matrix::Matrix;
use crate::normals::Normal;
use crate::ray;
use crate::tuple::Tuple;

#[derive(Debug, PartialEq, Clone)]
pub struct Sphere {
    radius: f64,
    origin: Tuple,
    transform: Matrix,
}

impl Sphere {
    pub(crate) fn new(radius: f64, origin: Tuple) -> Self {
        Sphere { radius, origin, transform: Matrix::identity(4) }
    }

    pub(crate) fn set_transform(&mut self, transform: &Matrix) {
        self.transform = transform.clone();
    }

    pub fn unit() -> Self {
        Sphere::new(
            1.0,
            Tuple::origin(),
        )
    }
}

impl Normal for Sphere {
    fn normal_at(&self, point: Tuple) -> Tuple {
        (point - self.origin).norm()
    }
}

impl ray::Intersect for Sphere {
    fn ray_intersections(&self, ray: &ray::Ray) -> Intersections {
        let sphere_to_ray = ray.origin - self.origin;

        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            Intersections { objects: vec![] }
        } else {
            let disc_sqrt = discriminant.sqrt();
            let denom = 2.0 * a;
            let r1 = (-b - disc_sqrt) / (denom);
            let r2 = (-b + disc_sqrt) / (denom);

            Intersections {
                objects: vec![
                    Intersection {
                        time: r1,
                        object: self.clone(),
                    },
                    Intersection {
                        time: r2,
                        object: self.clone(),
                    },
                ],
            }
        }
    }

    fn get_transform(&self) -> Option<Matrix> {
        Option::from(self.transform.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::transformations::scaling;
    use super::*;

    #[test]
    fn test_sphere_default_transform() {
        let s = Sphere::unit();
        assert_eq!(s.transform, Matrix::identity(4));
    }

    #[test]
    fn test_sphere_set_transform() {
        let mut s = Sphere::unit();
        let t = scaling(1.0, 2.0, 0.0);

        s.set_transform(&t);

        assert_eq!(s.transform, t);
    }

    #[test]
    fn test_sphere_normal_on_x() {
        let s = Sphere::unit();
        let n = s.normal_at(Tuple::point(1.0, 0.0, 0.0));
        assert_eq!(n, Tuple::vector(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_sphere_normal_on_y() {
        let s = Sphere::unit();
        let n = s.normal_at(Tuple::point(0.0, 1.0, 0.0));
        assert_eq!(n, Tuple::vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn test_sphere_normal_on_z() {
        let s = Sphere::unit();
        let n = s.normal_at(Tuple::point(0.0, 0.0, 1.0));
        assert_eq!(n, Tuple::vector(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_sphere_normal_on_nonaxial_point() {
        let x = 3.0f64.sqrt() / 3.0;
        let s = Sphere::unit();
        let n = s.normal_at(Tuple::point(x, x, x));
        assert_eq!(n, Tuple::vector(x, x, x));
        assert_eq!(n, n.norm());
    }
}
