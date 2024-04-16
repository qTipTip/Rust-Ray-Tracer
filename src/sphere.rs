use crate::intersection::{Intersection, Intersections};
use crate::material::Material;
use crate::matrix::Matrix;
use crate::normals::Normal;
use crate::ray;
use crate::ray::Intersect;
use crate::tuple::Tuple;

#[derive(Debug, PartialEq, Clone)]
pub struct Sphere {
    radius: f64,
    origin: Tuple,
    transform: Matrix,
    material: Material,
}

impl Sphere {
    pub(crate) fn new(radius: f64, origin: Tuple) -> Self {
        Sphere {
            radius,
            origin,
            transform: Matrix::identity(4),
            material: Material::default(),
        }
    }

    pub(crate) fn set_transform(&mut self, transform: &Matrix) {
        self.transform = transform.clone();
    }

    pub fn set_material(&mut self, material: Material) {
        self.material = material
    }

    pub fn unit() -> Self {
        Sphere::new(1.0, Tuple::origin())
    }
}

impl Normal for Sphere {
    fn normal_at(&self, point: Tuple) -> Tuple {
        let transform = self.transform.clone();
        let transform_inverse = transform.inverse();
        let object_point = transform_inverse.clone() * point;
        let object_normal = object_point - self.origin;
        let mut world_normal = transform_inverse.transpose() * object_normal;

        // When transforming the normal, we mess up the "vector"-status of the normal.
        // Resetting that here.
        world_normal.w = 0;
        world_normal.norm()
    }
}

impl ray::Intersect for Sphere {
    fn ray_intersections(&self, ray: &ray::Ray) -> Intersections {
        let sphere_to_ray = ray.origin - self.origin;

        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

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

    fn get_material(&self) -> Option<Material> {
        Option::from(self.material.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;
    use crate::transformations::{rotation_z, scaling, translation};
    use std::f64::consts::PI;

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

    #[test]
    fn test_sphere_normal_after_translate() {
        let mut s = Sphere::unit();
        s.set_transform(&translation(0.0, 1.0, 0.0));

        let n = s.normal_at(Tuple::point(0.0, 1.70711, -0.70711));
        assert_eq!(
            n,
            Tuple::vector(0.0, 0.7071067811865475, -0.7071067811865476)
        );
        assert_eq!(n, n.norm());
    }

    #[test]
    fn test_sphere_normal_after_transform() {
        let mut s = Sphere::unit();
        s.set_transform(&(scaling(1.0, 0.5, 1.0) * rotation_z(PI / 5.0)));

        let a = f64::sqrt(2.0) / 2.0;
        let n = s.normal_at(Tuple::point(0.0, a, -a));
        assert_eq!(
            n,
            Tuple::vector(0.0, 0.9701425001453319, -0.24253562503633294)
        );
        assert_eq!(n, n.norm());
    }

    #[test]
    fn sphere_has_default_material() {
        let s = Sphere::unit();
        assert_eq!(s.material, Material::default());
    }

    #[test]
    fn sphere_can_be_assigned_material() {
        let mut s = Sphere::unit();
        let m = Material::new(Color::new(1.0, 0.0, 1.0), 1.0, 2.0, 3.0, 4.0);
        s.set_material(m);
        assert_eq!(s.material, m);
    }
}
