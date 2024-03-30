use crate::matrix::Matrix;

fn translation(x: f64, y: f64, z: f64) -> Matrix {
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


}