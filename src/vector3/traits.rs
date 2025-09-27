use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use crate::Vector3;

impl<T> Add<&Vector3<T>> for &Vector3<T>
where T: Clone+Copy+Add<Output=T>{
    type Output = Vector3<T>;
    #[inline]
    fn add(self, rhs: &Vector3<T>) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T> AddAssign<&Vector3<T>> for Vector3<T>
where T: Clone+Copy+AddAssign<T>{
    #[inline]
    fn add_assign(&mut self, rhs: &Vector3<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
// ADD SCALAR
impl<T> Add<T> for &Vector3<T>
where T: Clone+Copy+Add<Output=T>{
    type Output = Vector3<T>;

    fn add(self, rhs: T) -> Self::Output {
        Self::Output{
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl<T> AddAssign<T> for Vector3<T>
where T: Clone+Copy+AddAssign<T>{
    fn add_assign(&mut self, rhs: T) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
    }
}
// SUB
impl<T> Sub<&Vector3<T>> for &Vector3<T>
where T: Clone+Copy+Sub<Output=T>{
    type Output = Vector3<T>;

    fn sub(self, rhs: &Vector3<T>) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T> SubAssign<&Vector3<T>> for Vector3<T>
where T: Clone+Copy+SubAssign<T>{
    fn sub_assign(&mut self, rhs: &Vector3<T>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}
// SUB SCALAR
impl<T> Sub<T> for &Vector3<T>
where T: Clone+Copy+Sub<Output=T>{
    type Output = Vector3<T>;

    fn sub(self, rhs: T) -> Self::Output {
        Self::Output {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

impl<T> SubAssign<T> for Vector3<T>
where T: Clone+Copy+SubAssign<T>{
    fn sub_assign(&mut self, rhs: T) {
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
    }
}
// Mul
impl<T> Mul<&Vector3<T>> for &Vector3<T>
where T: Clone+Copy+Mul<Output=T>{
    type Output = Vector3<T>;

    fn mul(self, rhs: &Vector3<T>) -> Self::Output {
        Self::Output {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}
impl<T> MulAssign<&Vector3<T>> for Vector3<T>
where T: Clone+Copy+MulAssign<T>{
    fn mul_assign(&mut self, rhs: &Vector3<T>) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}
// MUL SCALAR
impl<T> Mul<T> for &Vector3<T>
where T: Clone+Copy+Mul<Output=T>{
    type Output = Vector3<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T> MulAssign<T> for Vector3<T>
where T: Clone+Copy+MulAssign<T>{
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}
// DIV
impl<T> Div<&Vector3<T>> for &Vector3<T>
where T: Clone+Copy+Div<Output=T>{
    type Output = Vector3<T>;
    fn div(self, rhs: &Vector3<T>) -> Self::Output {
        Self::Output {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}
impl<T> DivAssign<&Vector3<T>> for Vector3<T>
where T: Clone+Copy+DivAssign<T>{
    fn div_assign(&mut self, rhs: &Vector3<T>) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}
// DIV SCALAR
impl<T> Div<T> for &Vector3<T>
where T: Clone+Copy+Div<Output=T>{
    type Output = Vector3<T>;
    fn div(self, rhs: T) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
impl<T> DivAssign<T> for Vector3<T>
where T: Clone+Copy+DivAssign<T>{
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}
// OTHER
impl<T> Neg for Vector3<T>
where T: Clone+Copy+Neg<Output=T>{
    type Output = Vector3<T>;
    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T> Default for Vector3<T>
where T: Default + Copy + Clone{
    fn default() -> Self {
        Self{
            x: T::default(),
            y: T::default(),
            z: T::default(),
        }
    }
}