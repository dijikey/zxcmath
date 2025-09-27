use std::ops::{Add, Mul, Sub};
use crate::Vector3;

mod float;
mod traits;

// impl<T: Copy + Clone> Vector3<T>{
// 
// }


impl<T: Copy + Clone> Vector3<T>{
    #[inline]
    pub const fn new(x: T, y: T, z: T) -> Self { Self { x, y, z } }

    #[inline]
    pub const fn as_ptr(&self) -> *const T { [self.x, self.y, self.z].as_ptr() }
}

impl<T: Copy + Clone + Add<Output=T>> Vector3<T>{
    #[inline]
    pub fn sum(&self) -> T { self.x + self.y + self.z }
}

impl<T: Copy + Clone + Mul<Output=T> + Add<Output=T>> Vector3<T>{
    #[inline]
    pub fn dot_product(lhs: &Self, rhs: &Self) -> T { lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z }
    #[inline]
    pub fn length_squared(&self) -> T { self.x * self.x + self.y * self.y + self.z * self.z }
}

impl<T: Copy + Clone + Mul<Output=T> + Sub<Output=T>> Vector3<T>{
    #[inline]
    pub fn cross_product(lhs: &Self, rhs: &Self) -> Self {
        Self {
            x: lhs.y * rhs.z - lhs.z * rhs.y,
            y: lhs.z * rhs.x - lhs.x * rhs.z,
            z: lhs.x * rhs.y - lhs.y * rhs.x,
        }
    }

}