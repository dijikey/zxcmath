use crate::Matrix3x3;
use std::ops::*;

impl Default for Matrix3x3 {
    fn default() -> Self {
        Self::new([[0.0; 3]; 3])
    }
}
/**************MUL**************/
impl Mul<Matrix3x3> for Matrix3x3 {
    type Output = Matrix3x3;
    fn mul(self, rhs: Matrix3x3) -> Self::Output {
        let mut result = Self::identity();
        for i in 0..3 {
            for j in 0..3 {
                result.data[i][j] = 0.0;
                for k in 0..3 {
                    result.data[i][j] += self.data[i][k] * rhs.data[k][j];
                }
            }
        }
        result
    }
}

impl MulAssign<Matrix3x3> for Matrix3x3 {
    fn mul_assign(&mut self, rhs: Matrix3x3) {
        for i in 0..3 {
            for j in 0..3 {
                self.data[i][j] = 0.0;
                for k in 0..3 {
                    self.data[i][j] += self.data[i][k] * rhs.data[k][j];
                }
            }
        }
    }
}
impl Mul<f32> for Matrix3x3 {
    type Output = Matrix3x3;
    fn mul(self, rhs: f32) -> Self::Output {
        let mut matrix = Self::default();
        for i in 0..3 {
            for j in 0..3 {
                matrix.data[i][j] = self.data[i][j] * rhs;
            }
        }
        matrix
    }
}
impl MulAssign<f32> for Matrix3x3 {
    fn mul_assign(&mut self, rhs: f32){
        for i in 0..3 {
            for j in 0..3 {
                self.data[i][j] *= rhs;
            }
        }
    }
}

/**************ADD**************/
impl Add<Matrix3x3> for Matrix3x3 {
    type Output = Matrix3x3;
    fn add(self, rhs: Matrix3x3) -> Self::Output {
        let mut matrix = Self::default();
        for i in 0..3 {
            for j in 0..3 {
                matrix.data[i][j] = self.data[i][j] + rhs.data[i][j];
            }
        }
        matrix
    }
}

impl AddAssign<Matrix3x3> for Matrix3x3 {
    fn add_assign(&mut self, rhs: Matrix3x3){
        for i in 0..3 {
            for j in 0..3 {
                self.data[i][j] += rhs.data[i][j];
            }
        }
    }
}
/**************SUB**************/
impl Sub<Matrix3x3> for Matrix3x3 {
    type Output = Matrix3x3;
    fn sub(self, rhs: Matrix3x3) -> Self::Output {
        let mut matrix = Self::default();
        for i in 0..3 {
            for j in 0..3 {
                matrix.data[i][j] = self.data[i][j] - rhs.data[i][j];
            }
        }
        matrix
    }
}
impl SubAssign<Matrix3x3> for Matrix3x3 {
    fn sub_assign(&mut self, rhs: Matrix3x3){
        for i in 0..3 {
            for j in 0..3 {
                self.data[i][j] -= rhs.data[i][j];
            }
        }
    }
}
/**************DIV**************/
impl Div<f32> for Matrix3x3 {
    type Output = Self;

    fn div(self, scalar: f32) -> Self {
        if scalar == 0.0 { panic!("Cannot divide by zero!"); }
        let mut result = Matrix3x3::identity();
        for i in 0..3 {
            for j in 0..3 {
                result.data[i][j] = self.data[i][j] / scalar;
            }
        }
        result
    }
}