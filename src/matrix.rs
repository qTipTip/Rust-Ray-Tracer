use std::ops::Mul;
use std::vec;
use crate::tuple::Tuple;

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

    pub fn identity(dim: usize) -> Self {
        let mut values = vec![0.0; dim * dim];
        for i in 0..dim {
            values[i + i * dim] = 1.0;
        }

        Self {
            values,
            rows: dim,
            columns: dim,
        }
    }

    pub fn transpose(&self) -> Self {
        let mut values = vec![0.0; self.values.len()];
        for i in 0..self.rows {
            for j in 0..self.columns {
                values[j * self.rows + i] = self.values[i * self.columns + j]
            }
        }

        Matrix::new(values, self.columns, self.rows)
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

    fn det(&self) -> f64 {
        if (self.rows, self.columns) == (2, 2) {
            return self.values[0] * self.values[3] - self.values[1] * self.values[2];
        } else {
            return 0.0;
        }
    }

    fn remove_row(&self, row: usize) -> Self {
        let mut values = vec![0.0; (self.rows - 1) * self.columns];
        let mut running_idx = 0;
        for i in 0..self.rows {
            if i == row {
                continue;
            }
            for j in 0..self.columns {
                let idx = i * self.columns + j;
                values[running_idx] = self.values[idx];
                running_idx += 1;
            }
        }
        Self {
            values,
            rows: self.rows - 1,
            columns: self.columns,
        }
    }

    fn remove_column(&self, column: usize) -> Self {
        self.transpose().remove_row(column).transpose()
    }

    fn submatrix(&self, row: usize, column: usize) -> Self {
        self.remove_row(row).remove_column(column)
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
        let mut result_values = vec![0.0; self.rows * rhs.columns];
        for i in 0..self.rows {
            for j in 0..rhs.columns {
                let mut sum = 0.0;
                for k in 0..self.columns {
                    sum += self.values[i * self.columns + k] * rhs.values[j + k * rhs.columns]
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

impl Mul<&Matrix> for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: &Matrix) -> Self::Output {
        let mut result_values = vec![0.0; self.rows * rhs.columns];
        for i in 0..self.rows {
            for j in 0..rhs.columns {
                let mut sum = 0.0;
                for k in 0..self.columns {
                    sum += self.values[i * self.columns + k] * rhs.values[j + k * rhs.columns]
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

impl Mul<Tuple> for Matrix {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        let rhs_matrix = Matrix::new(vec![rhs.x, rhs.y, rhs.z, rhs.w as f64], 4, 1);
        let result_matrix = self * rhs_matrix;

        Tuple::new(result_matrix.values[0], result_matrix.values[1], result_matrix.values[2], result_matrix.values[3] as u8)
    }
}


#[cfg(test)]
mod tests {
    use crate::matrix::Matrix;
    use crate::tuple::Tuple;

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

    #[test]
    fn matrix_tuple_multiplication() {
        let m = Matrix::new(vec![
            1.0, 2.0, 3.0, 4.0,
            2.0, 4.0, 4.0, 2.0,
            8.0, 6.0, 4.0, 1.0,
            0.0, 0.0, 0.0, 1.0,
        ], 4, 4);

        let b = Tuple::new(1.0, 2.0, 3.0, 1);

        assert_eq!(m * b, Tuple::new(18.0, 24.0, 33.0, 1));
    }

    #[test]
    fn identity_matrix_matrix_multiplication() {
        let m = Matrix::new(vec![
            1.0, 2.0, 3.0, 4.0,
            2.0, 4.0, 4.0, 2.0,
            8.0, 6.0, 4.0, 1.0,
            0.0, 0.0, 0.0, 1.0,
        ], 4, 4);
        let i = Matrix::identity(4);

        assert_eq!(&m * &i, m);
    }

    #[test]
    fn identity_matrix_tuple_multiplication() {
        let b = Tuple::new(1.0, 2.0, 3.0, 1);
        let i = Matrix::identity(4);

        assert_eq!(i * b, b);
    }

    #[test]
    fn test_transposition() {
        let m = Matrix::new(vec![
            1.0, 2.0,
            3.0, 4.0,
            5.0, 6.0,
        ], 3, 2);
        let n = Matrix::new(vec![
            1.0, 3.0, 5.0,
            2.0, 4.0, 6.0,
        ], 2, 3);

        assert_eq!(m.transpose(), n);
    }

    #[test]
    fn test_identity_transposition() {
        let i = Matrix::identity(10);

        assert_eq!(i.transpose(), i);
    }

    #[test]
    fn test_2x2_determinant() {
        let m = Matrix::new(vec![1.0, 2.0, 3.0, 4.0], 2, 2);
        assert_eq!(m.det(), -2.0);
    }

    #[test]
    fn test_remove_row() {
        let m = Matrix::new(vec![
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0,
        ], 2, 3);

        let n = Matrix::new(vec![
            4.0, 5.0, 6.0,
        ], 1, 3);

        assert_eq!(m.remove_row(0), n);
    }

    #[test]
    fn test_remove_column() {
        let m = Matrix::new(vec![
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0,
        ], 2, 3);

        let n = Matrix::new(vec![
            2.0, 3.0,
            5.0, 6.0,
        ], 2, 2);

        assert_eq!(m.remove_column(0), n);
    }

    #[test]
    fn test_2x2_submatrix() {
        let m = Matrix::new(vec![
            1.0, 5.0, 0.0,
            -3.0, 2.0, 7.0,
            0.0, 6.0, -3.0,
        ], 3, 3);
        let s = Matrix::new(vec![
            -3.0, 2.0,
            0.0, 6.0,
        ], 2, 2);

        assert_eq!(m.submatrix(0, 2), s);
    }

    #[test]
    fn test_3x3_submatrix() {
        let m = Matrix::new(vec![
            -6.0, 1.0, 1.0, 6.0,
            -8.0, 5.0, 8.0, 6.0,
            -1.0, 0.0, 8.0, 2.0,
            -7.0, 1.0, -1.0, 1.0,
        ], 4, 4);
        let s = Matrix::new(vec![
            5.0, 8.0, 6.0,
            0.0, 8.0, 2.0,
            1.0, -1.0, 1.0,
        ], 3, 3);

        assert_eq!(m.submatrix(0, 0), s);
    }
}