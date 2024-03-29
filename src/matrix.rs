use std::ops::Mul;
use std::vec;

#[derive(Debug)]
struct Matrix {
    values: Vec<f64>,
    rows: usize,
    columns: usize,
}

impl Matrix {
    pub fn new(values: Vec<f64>, rows: usize, columns: usize) -> Self {
        Self {
            values,
            rows,
            columns,
        }
    }

    pub fn get(&self, i: usize, j: usize) -> f64 {
        self.values[i * self.columns + j]
    }

    fn same_dimensions(&self, other: &Self) -> bool {
        if self.values.len() != other.values.len() {
            return false;
        }

        if self.rows != other.rows || self.columns != other.columns {
            return false;
        }
        return true;
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        if !self.same_dimensions(other) {
            return false;
        }

        for i in 0..self.values.len() {
            if (self.values[i] - other.values[i]).abs() > f64::EPSILON {
                return false;
            }
        }

        return true;
    }
}

impl Mul for Matrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result_values = vec![0.0; self.values.len()];
        for i in 0..self.rows {
            for j in 0..rhs.columns {
                let mut sum = 0.0;
                for k in 0..self.columns {
                    sum += self.values[i * self.columns + k] * rhs.values[j + k * self.rows]
                }
                result_values[i * rhs.columns + j] = sum;
            }
        }

        Matrix {
            values: result_values,
            rows: self.rows,
            columns: rhs.columns,
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::matrix::Matrix;

    #[test]
    fn construct_matrix() {
        let m = Matrix::new(vec![-3.0, 5.0, 1.0, -2.0], 2, 2);

        assert_eq!(m.get(0, 0), -3.0);
        assert_eq!(m.get(0, 1), 5.0);
        assert_eq!(m.get(1, 0), 1.0);
        assert_eq!(m.get(1, 1), -2.0);
    }

    #[test]
    fn matrix_comparison() {
        let m1 = Matrix::new(vec![-3.0, 5.0, 1.0, -2.0], 2, 2);
        let m2 = Matrix::new(vec![-3.0000001, 5.0, 1.0, -2.0], 2, 2);
        let m3 = Matrix::new(vec![-3.0, 5.0, 1.0, -2.0, 1.0], 3, 2);
        let m4 = Matrix::new(vec![-3.0, 5.0, 1.0, -2.0, 1.0], 2, 3);

        assert_eq!(m1, m1);
        assert_ne!(m1, m3);
        assert_ne!(m1, m2);
        assert_ne!(m3, m4);
    }

    #[test]
    fn matrix_multiplication() {
        let m1 = Matrix::new(vec![
            1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 8.0, 7.0, 6.0,
            5.0, 4.0, 3.0, 2.0,
        ], 4, 4,
        );
        let m2 = Matrix::new(vec![
            -2.0, 1.0, 2.0, 3.0,
            3.0, 2.0, 1.0, -1.0,
            4.0, 3.0, 6.0, 5.0,
            1.0, 2.0, 7.0, 8.0,
        ], 4, 4,
        );
        let m3 = Matrix::new(vec![
            20.0, 22.0, 50.0, 48.0,
            44.0, 54.0, 114.0, 108.0,
            40.0, 58.0, 110.0, 102.0,
            16.0, 26.0, 46.0, 42.0,
        ], 4, 4,
        );

        assert_eq!(m1 * m2, m3)
    }
}