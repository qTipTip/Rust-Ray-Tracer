use crate::matrix::Matrix;

pub fn translation(x: f64, y: f64, z: f64) -> Matrix {
    let mut t = Matrix::identity(4);
    t.set(0, 3, x);
    t.set(1, 3, y);
    t.set(2, 3, z);

    t
}

pub fn scaling(x: f64, y: f64, z: f64) -> Matrix {
    let mut t = Matrix::identity(4);
    t.set(0, 0, x);
    t.set(1, 1, y);
    t.set(2, 2, z);

    t
}

pub fn rotation_x(r: f64) -> Matrix {
    let mut t = Matrix::identity(4);
    t.set(1, 1, f64::cos(r));
    t.set(1, 2, -f64::sin(r));
    t.set(2, 1, f64::sin(r));
    t.set(2, 2, f64::cos(r));

    t
}

pub fn rotation_y(r: f64) -> Matrix {
    let mut t = Matrix::identity(4);
    t.set(0, 0, f64::cos(r));
    t.set(0, 2, f64::sin(r));
    t.set(2, 0, -f64::sin(r));
    t.set(2, 2, f64::cos(r));

    t
}

pub fn rotation_z(r: f64) -> Matrix {
    let mut t = Matrix::identity(4);
    t.set(0, 0, f64::cos(r));
    t.set(0, 1, -f64::sin(r));
    t.set(1, 0, f64::sin(r));
    t.set(1, 1, f64::cos(r));

    t
}

pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix {
    let mut t = Matrix::identity(4);
    t.set(0, 1, xy);
    t.set(0, 2, xz);
    t.set(1, 0, yx);
    t.set(1, 2, yz);
    t.set(2, 0, zx);
    t.set(2, 1, zy);

    t
}

#[cfg(test)]
mod tests {
    use crate::transformations::{rotation_x, scaling, translation};
    use crate::tuple::Tuple;
    use std::f64::consts::PI;

    mod translation {
        use crate::transformations::translation;
        use crate::tuple::Tuple;

        #[test]
        fn multiply_by_translation_matrix() {
            let p = Tuple::point(-3.0, 4.0, 5.0);
            let t = translation(5.0, -3.0, 2.0);

            assert_eq!(t * p, Tuple::point(2.0, 1.0, 7.0));
        }

        #[test]
        fn multiply_by_inverse_translation() {
            let t = translation(5.0, -3.0, 2.0);
            let p = Tuple::point(-3.0, 4.0, 5.0);

            assert_eq!(t.inverse() * p, Tuple::point(-8.0, 7.0, 3.0));
        }

        #[test]
        fn translate_vector_does_not_change_it() {
            let t = translation(5.0, -3.0, 2.0);
            let v = Tuple::vector(-3.0, 4.0, 5.0);

            assert_eq!(t * v, v);
        }
    }

    mod scaling {
        use crate::transformations::scaling;
        use crate::tuple::Tuple;

        #[test]
        fn scaling_a_point() {
            let t = scaling(2.0, 3.0, 4.0);
            let p = Tuple::point(-4.0, 6.0, 8.0);

            assert_eq!(t * p, Tuple::point(-8.0, 18.0, 32.0));
        }

        #[test]
        fn scaling_a_vector() {
            let t = scaling(2.0, 3.0, 4.0);
            let v = Tuple::vector(-4.0, 6.0, 8.0);

            assert_eq!(t * v, Tuple::vector(-8.0, 18.0, 32.0));
        }

        #[test]
        fn inverse_scaling() {
            let t = scaling(2.0, 3.0, 4.0);
            let v = Tuple::vector(-4.0, 6.0, 8.0);

            assert_eq!(t.inverse() * v, Tuple::vector(-2.0, 2.0, 2.0));
        }

        #[test]
        fn reflection_is_negative_scaling() {
            let t = scaling(-1.0, 1.0, 1.0);
            let p = Tuple::point(2.0, 3.0, 4.0);

            assert_eq!(t * p, Tuple::point(-2.0, 3.0, 4.0));
        }
    }

    mod rotations {
        use crate::transformations::{rotation_x, rotation_y, rotation_z};
        use crate::tuple::Tuple;
        use std::f64::consts::PI;

        #[test]
        fn rotate_around_x_axis() {
            let p = Tuple::point(0.0, 1.0, 0.0);
            let r1 = rotation_x(PI / 4.0);
            let r2 = rotation_x(PI / 2.0);

            assert_eq!(
                r1 * p,
                Tuple::point(0.0, f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0,)
            );
            assert_eq!(r2 * p, Tuple::point(0.0, 0.0, 1.0));
        }

        #[test]
        fn inverse_rotate_around_x_axis() {
            let p = Tuple::point(0.0, 1.0, 0.0);
            let r1 = rotation_x(PI / 4.0);

            assert_eq!(
                r1.inverse() * p,
                Tuple::point(0.0, f64::sqrt(2.0) / 2.0, -f64::sqrt(2.0) / 2.0,)
            );
        }

        #[test]
        fn rotate_around_y_axis() {
            let p = Tuple::point(0.0, 0.0, 1.0);
            let r1 = rotation_y(PI / 4.0);
            let r2 = rotation_y(PI / 2.0);

            assert_eq!(
                r1 * p,
                Tuple::point(f64::sqrt(2.0) / 2.0, 0.0, f64::sqrt(2.0) / 2.0,)
            );
            assert_eq!(r2 * p, Tuple::point(1.0, 0.0, 0.0));
        }

        #[test]
        fn rotate_around_z_axis() {
            let p = Tuple::point(0.0, 1.0, 0.0);
            let r1 = rotation_z(PI / 4.0);
            let r2 = rotation_z(PI / 2.0);

            assert_eq!(
                r1 * p,
                Tuple::point(-f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0, 0.0,)
            );
            assert_eq!(r2 * p, Tuple::point(-1.0, 0.0, 0.0));
        }
    }

    mod shearing {
        use crate::transformations::shearing;
        use crate::tuple::Tuple;

        #[test]
        fn shear_x_rel_y() {
            let t = shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
            let p = Tuple::point(2.0, 3.0, 4.0);

            assert_eq!(t * p, Tuple::point(5.0, 3.0, 4.0));
        }

        #[test]
        fn shear_x_rel_z() {
            let t = shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
            let p = Tuple::point(2.0, 3.0, 4.0);

            assert_eq!(t * p, Tuple::point(6.0, 3.0, 4.0));
        }

        #[test]
        fn shear_y_rel_x() {
            let t = shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
            let p = Tuple::point(2.0, 3.0, 4.0);

            assert_eq!(t * p, Tuple::point(2.0, 5.0, 4.0));
        }

        #[test]
        fn shear_y_rel_z() {
            let t = shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
            let p = Tuple::point(2.0, 3.0, 4.0);

            assert_eq!(t * p, Tuple::point(2.0, 7.0, 4.0));
        }

        #[test]
        fn shear_z_rel_x() {
            let t = shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
            let p = Tuple::point(2.0, 3.0, 4.0);

            assert_eq!(t * p, Tuple::point(2.0, 3.0, 6.0));
        }

        #[test]
        fn shear_z_rel_y() {
            let t = shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
            let p = Tuple::point(2.0, 3.0, 4.0);

            assert_eq!(t * p, Tuple::point(2.0, 3.0, 7.0));
        }
    }

    #[test]
    fn apply_transformations_in_sequence() {
        let p = Tuple::point(1.0, 0.0, 1.0);
        let a = rotation_x(PI / 2.0);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);

        let p2 = a * p;
        assert_eq!(p2, Tuple::point(1.0, -1.0, 0.0));

        let p3 = b * p2;
        assert_eq!(p3, Tuple::point(5.0, -5.0, 0.0));

        let p4 = c * p3;
        assert_eq!(p4, Tuple::point(15.0, 0.0, 7.0));
    }

    fn apply_transformations_chained() {
        let p = Tuple::point(1.0, 0.0, 1.0);
        let a = rotation_x(PI / 2.0);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);

        assert_eq!(c * b * a * p, Tuple::point(15.0, 0.0, 7.0));
    }
}
