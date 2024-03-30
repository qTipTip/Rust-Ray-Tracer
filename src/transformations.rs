use crate::matrix::Matrix;

fn translation(x: f64, y: f64, z: f64) -> Matrix {
    let mut t = Matrix::identity(4);
    t.set(0, 3, x);
    t.set(1, 3, y);
    t.set(2, 3, z);

    t
}

#[cfg(test)]
mod tests {
    use crate::tuple::Tuple;
    use crate::transformations::translation;

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