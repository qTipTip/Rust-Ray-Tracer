#[derive(Debug)]
struct Matrix {
    values: Vec<f64>,
    rows: usize,
    columns: usize,
}

impl Matrix {
    fn new(values: Vec<f64>, rows: usize, columns: usize) -> Self {
        Self {
            values,
            rows,
            columns,
        }
    }

    fn get(&self, i: usize, j: usize) -> f64 {
        self.values[i * self.columns + j]
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        if self.values.len() != other.values.len() {
            return false;
        }

        if self.rows != other.rows || self.columns != other.columns {
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
}