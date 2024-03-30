use std::ops::Mul;
use std::vec;
use crate::tuple::Tuple;
use crate::utils;

#[derive(Debug)]
pub(crate) struct Matrix {
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
        self.values[self.flatten_index(i, j)]
    }

    pub fn set(&mut self, i: usize, j: usize, value: f64) {
        let idx = self.flatten_index(i, j);
        self.values[idx] = value;
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
            self.values[0] * self.values[3] - self.values[1] * self.values[2]
        } else {
            let mut determinant = 0.0;
            for i in 0..self.columns {
                determinant = determinant + self.get(0, i) * self.cofactor(0, i);
            }
            determinant
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

    fn minor(&self, row: usize, column: usize) -> f64 {
        self.submatrix(row, column).det()
    }

    fn cofactor(&self, row: usize, column: usize) -> f64 {
        let mut cofactor = self.minor(row, column);
        if (row + column) % 2 == 1 {
            cofactor = -1.0 * cofactor;
        }
        cofactor
    }

    fn flatten_index(&self, i: usize, j: usize) -> usize {
        i * self.columns + j
    }

    pub(crate) fn inverse(&self) -> Self {
        let determinant = self.det();
        if determinant.abs() < f64::EPSILON {
            panic!("Cannot invert non-invertible matrix!")
        }

        let mut values = vec![0.0; self.values.len()];
        for i in 0..self.rows {
            for j in 0..self.columns {
                // note the transpose here
                values[self.flatten_index(j, i)] = self.cofactor(i, j) / determinant
            }
        }

        Self::new(values, self.rows, self.columns)
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        if !self.same_dimensions(other) {
            return false;
        }

        for i in 0..self.values.len() {
            if (self.values[i] - other.values[i]).abs() > utils::F64_ERROR_MARGIN {
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

    #[test]
    fn test_3x3_minor() {
        let m = Matrix::new(vec![
            3.0, 5.0, 0.0,
            2.0, -1.0, -7.0,
            6.0, -1.0, 5.0,
        ], 3, 3);

        assert_eq!(m.submatrix(1, 0).det(), 25.0);
        assert_eq!(m.minor(1, 0), 25.0);
    }

    #[test]
    fn test_2x2_cofactor() {
        let m = Matrix::new(vec![
            3.0, 5.0, 0.0,
            2.0, -1.0, -7.0,
            6.0, -1.0, 5.0,
        ], 3, 3);

        assert_eq!(m.minor(0, 0), -12.0);
        assert_eq!(m.cofactor(0, 0), -12.0);
        assert_eq!(m.minor(1, 0), 25.0);
        assert_eq!(m.cofactor(1, 0), -25.0);
    }

    #[test]
    fn test_3x3_determinant() {
        let m = Matrix::new(vec![
            1.0, 2.0, 6.0,
            -5.0, 8.0, -4.0,
            2.0, 6.0, 4.0,
        ], 3, 3);

        assert_eq!(m.cofactor(0, 0), 56.0);
        assert_eq!(m.cofactor(0, 1), 12.0);
        assert_eq!(m.cofactor(0, 2), -46.0);
        assert_eq!(m.det(), -196.0);
    }

    #[test]
    fn test_4x4_determinant() {
        let m = Matrix::new(vec![
            -2.0, -8.0, 3.0, 5.0,
            -3.0, 1.0, 7.0, 3.0,
            1.0, 2.0, -9.0, 6.0,
            -6.0, 7.0, 7.0, -9.0,
        ], 4, 4);

        assert_eq!(m.cofactor(0, 0), 690.0);
        assert_eq!(m.cofactor(0, 1), 447.0);
        assert_eq!(m.cofactor(0, 2), 210.0);
        assert_eq!(m.cofactor(0, 3), 51.0);
        assert_eq!(m.det(), -4071.0);
    }

    #[test]
    fn test_inverse() {
        let m = Matrix::new(vec![
            -5.0, 2.0, 6.0, -8.0,
            3.0, 1.0, -5.0, 3.0,
            9.0, 0.0, 1.3, 2.3,
            1.9, 2.0, 1.1, 0.0,
        ], 4, 4);

        assert_eq!(&m * &m.inverse(), Matrix::identity(4));
    }

    #[test]
    fn test_inverse_identity() {
        let i = Matrix::identity(4);
        assert_eq!(i.inverse(), i);
    }

    #[test]
    fn test_inverse_equation() {
        // C = AB => A = CB^-1
        let m = Matrix::new(vec![
            1.0, 3.0, -1.0, 2.0,
            3.2, 3.1, -1.1, 99.0,
            12.0, 0.0, 1.0, 2.0,
            12.0, 13.0, 1.0, 2.0,
        ], 4, 4);
        let n = &m.transpose();
        let c = &m * &n;

        assert_eq!(&c * &n.inverse(), m);
    }
}