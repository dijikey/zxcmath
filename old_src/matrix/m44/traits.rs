use crate::{Matrix4x4, Quaternion};
use std::ops::*;

impl Default for Matrix4x4 {
    fn default() -> Self {
        Self::new([[0.0; 4]; 4])
    }
}
/**************MUL**************/
impl Mul<Matrix4x4> for Matrix4x4 {
    type Output = Matrix4x4;
    fn mul(self, rhs: Matrix4x4) -> Self::Output {
        let mut result = Self::identity();
        for i in 0..4 {
            for j in 0..4 {
                result.data[i][j] = 0.0;
                for k in 0..4 {
                    result.data[i][j] += self.data[i][k] * rhs.data[k][j];
                }
            }
        }
        result
    }
}

impl MulAssign<Matrix4x4> for Matrix4x4 {
    fn mul_assign(&mut self, rhs: Matrix4x4) {
        for i in 0..4 {
            for j in 0..4 {
                self.data[i][j] = 0.0;
                for k in 0..4 {
                    self.data[i][j] += self.data[i][k] * rhs.data[k][j];
                }
            }
        }
    }
}
impl Mul<f32> for Matrix4x4 {
    type Output = Matrix4x4;
    fn mul(self, rhs: f32) -> Self::Output {
        let mut matrix = Self::default();
        for i in 0..4 {
            for j in 0..4 {
                matrix.data[i][j] = self.data[i][j] * rhs;
            }
        }
        matrix
    }
}
impl MulAssign<f32> for Matrix4x4 {
    fn mul_assign(&mut self, rhs: f32){
        for i in 0..4 {
            for j in 0..4 {
                self.data[i][j] *= rhs;
            }
        }
    }
}

/**************ADD**************/
impl Add<Matrix4x4> for Matrix4x4 {
    type Output = Matrix4x4;
    fn add(self, rhs: Matrix4x4) -> Self::Output {
        let mut matrix = Self::default();
        for i in 0..4 {
            for j in 0..4 {
                matrix.data[i][j] = self.data[i][j] + rhs.data[i][j];
            }
        }
        matrix
    }
}

impl AddAssign<Matrix4x4> for Matrix4x4 {
    fn add_assign(&mut self, rhs: Matrix4x4){
        for i in 0..4 {
            for j in 0..4 {
                self.data[i][j] += rhs.data[i][j];
            }
        }
    }
}
/**************SUB**************/
impl Sub<Matrix4x4> for Matrix4x4 {
    type Output = Matrix4x4;
    fn sub(self, rhs: Matrix4x4) -> Self::Output {
        let mut matrix = Self::default();
        for i in 0..4 {
            for j in 0..4 {
                matrix.data[i][j] = self.data[i][j] - rhs.data[i][j];
            }
        }
        matrix
    }
}
impl SubAssign<Matrix4x4> for Matrix4x4 {
    fn sub_assign(&mut self, rhs: Matrix4x4){
        for i in 0..4 {
            for j in 0..4 {
                self.data[i][j] -= rhs.data[i][j];
            }
        }
    }
}
/**************DIV**************/
impl Div<f32> for Matrix4x4 {
    type Output = Self;

    fn div(self, scalar: f32) -> Self {
        if scalar == 0.0 { panic!("Cannot divide by zero!"); }
        let mut result = Matrix4x4::identity();
        for i in 0..4 {
            for j in 0..4 {
                result.data[i][j] = self.data[i][j] / scalar;
            }
        }
        result
    }
}

impl From<Quaternion> for Matrix4x4 {
    fn from(value: Quaternion) -> Self {
        let xx = value.x*value.x;
        let xy = value.x*value.y;
        let xz = value.x*value.z;
        let xw = value.x*value.w;
        
        let yy = value.y*value.y;
        let yz = value.y*value.z;
        let yw = value.y*value.w;
        
        let zz = value.z*value.z;
        let zw = value.z*value.w;
        
        Matrix4x4::new([
            [1.0-2.0*(yy+zz), 2.0*(xy+zw), 2.0*(xz+yw), 0.0],
            [2.0*(xy-zw), 1.0-2.0*(xx+zz), 2.0*(yz+xw), 0.0],
            [2.0*(xz+yw), 2.0*(yz-xw), 1.0-2.0*(yy+xx), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
}