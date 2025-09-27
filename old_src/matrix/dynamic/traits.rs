use std::fmt::{Display, Formatter};
use std::ops::*;
use crate::matrix::dynamic::Matrix;
/**************ADD**************/
impl Add<Matrix> for Matrix {
    type Output = Matrix;
    fn add(self, rhs: Matrix) -> Self::Output {
        if Matrix::is_equals(&self, &rhs) { panic!("Matrix dimensions do not match") }
        let mut matrix = Matrix::default(self.rows, self.cols);

        for i in 0..self.data.len() {
            matrix.data[i] = rhs.data[i] + self.data[i];
        }

        matrix
    }
}
impl AddAssign<Matrix> for Matrix {
    fn add_assign(&mut self, rhs: Matrix) {
        if Matrix::is_equals(&self, &rhs) { panic!("Matrix dimensions do not match") }
        for i in 0..self.data.len() {
            self.data[i] += rhs.data[i];
        }
    }
}
/**************MUL**************/
impl Mul<Matrix> for Matrix {
    type Output = Self;
    fn mul(self, rhs: Matrix) -> Self::Output {
        //if self.rows != other.cols {  }
        let mut values: Vec<f32> = Vec::new();
        for i in 0..self.rows() {
            for j in 0..rhs.cols() {
                let mut element: f32 = 0.0;
                for k in 0..self.cols() { element += self.get(i, k) * rhs.get(k, j); }
                values.insert(j + i * rhs.cols, element);
            }
        }

        Self::new(values, self.rows, rhs.cols)
    }
}

impl MulAssign<Matrix> for Matrix {
    fn mul_assign(&mut self, rhs: Matrix) {
        for i in 0..self.rows() {
            for j in 0..rhs.cols() {
                let mut element: f32 = 0.0;
                for k in 0..self.cols() { element += self.get(i, k) * rhs.get(k, j); }
                self.data.insert(j + i * rhs.cols, element);
            }
        }
    }
}

impl Mul<f32> for Matrix {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        let mut matrix = Self::from(self);
        for i in 0..matrix.data.len() {
            matrix.data[i] *= rhs;
        }

        matrix
    }
}

impl MulAssign<f32> for Matrix {
    fn mul_assign(&mut self, rhs: f32) {
        for i in 0..self.data.len() {
            self.data[i] *= rhs;
        }
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = String::from(format!("Matrix{}x{}[\n", self.rows, self.cols));

        for col in 1..=self.cols {
            for row in 1..=self.rows {
                s.push_str(format!("{},", self.get(row, col)).as_str());
            }
            if col == self.cols {
                s.push_str(" ]");
            } else {
                s.push_str("\n");
            }
        }
        write!(f, "{}", s)
    }
}
