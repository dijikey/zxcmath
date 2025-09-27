use std::ops::{Add, AddAssign, Mul, MulAssign};
use crate::Quaternion;
/***************Mul****************/
impl Mul<Quaternion> for Quaternion{
    type Output = Quaternion;
    fn mul(self, rhs: Quaternion) -> Quaternion{
        Quaternion::new(
            self.w * rhs.x + self.x * rhs.w + self.y * rhs.z - self.z * rhs.y,
            self.w * rhs.y - self.x * rhs.z + self.y * rhs.w + self.z * rhs.x,
            self.w * rhs.z + self.x * rhs.y - self.y * rhs.x + self.z * rhs.w,
            self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,
        )
    }
}

impl MulAssign<Quaternion> for Quaternion{
    fn mul_assign(&mut self, rhs: Quaternion){
        self.x = self.w * rhs.x + self.x * rhs.w + self.y * rhs.z - self.z * rhs.y;
        self.y = self.w * rhs.y - self.x * rhs.z + self.y * rhs.w + self.z * rhs.x;
        self.z = self.w * rhs.z + self.x * rhs.y - self.y * rhs.x + self.z * rhs.w;
        self.w = self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z;
    }
}
/***************Add****************/
impl Add<Quaternion> for Quaternion{
    type Output = Quaternion;
    fn add(self, rhs: Quaternion) -> Quaternion{
        Quaternion::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w
        )
    }
}
impl AddAssign<Quaternion> for Quaternion{
    fn add_assign(&mut self, rhs: Quaternion) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
}