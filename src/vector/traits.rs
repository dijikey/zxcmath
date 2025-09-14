use std::ops::*;
use crate::vector::Vector;
// ADD
impl Add<Vector> for Vector {
    type Output = Vector;
    #[inline]
    fn add(self, rhs: Vector) -> Self::Output {
        Vector{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign<Vector> for Vector {
    #[inline]
    fn add_assign(&mut self, rhs: Vector) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
// ADD SCALAR
impl Add<f64> for Vector {
    type Output = Vector;

    fn add(self, rhs: f64) -> Self::Output {
        Vector{
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl AddAssign<f64> for Vector {
    fn add_assign(&mut self, rhs: f64) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
    }
}
// SUB
impl Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
        Vector{
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign<Vector> for Vector {
    fn sub_assign(&mut self, rhs: Vector) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}
// SUB SCALAR
impl Sub<f64> for Vector {
    type Output = Vector;

    fn sub(self, rhs: f64) -> Self::Output {
        Vector{
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

impl SubAssign<f64> for Vector {
    fn sub_assign(&mut self, rhs: f64) {
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
    }
}
// Mul
impl Mul<Vector> for Vector {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector{
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}
impl MulAssign<Vector> for Vector {
    fn mul_assign(&mut self, rhs: Vector) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}
// MUL SCALAR
impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector{
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl MulAssign<f64> for Vector {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}
// DIV
impl Div<Vector> for Vector {
    type Output = Vector;
    fn div(self, rhs: Vector) -> Self::Output {
        Vector{
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}
impl DivAssign<Vector> for Vector {
    fn div_assign(&mut self, rhs: Vector) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}
// DIV SCALAR
impl Div<f64> for Vector {
    type Output = Vector;
    fn div(self, rhs: f64) -> Self::Output {
        Vector{
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
impl DivAssign<f64> for Vector {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}
// OTHER
impl Neg for Vector {
    type Output = Vector;
    fn neg(self) -> Self::Output {
        Vector{
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Default for Vector {
    fn default() -> Self { Vector{ x: 0.0, y: 0.0, z: 0.0, } }
}

impl From<[f64; 3]> for Vector {
    fn from(array: [f64; 3]) -> Self {
        Vector::new(array[0], array[1], array[2])
    }
}