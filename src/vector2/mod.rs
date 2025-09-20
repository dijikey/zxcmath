use std::ops::{Add, Mul};
use crate::Vector2;

mod f32;
mod i32;

impl<T> Vector2<T>
where T: Clone+Copy{
    #[inline]
    pub const fn new(x: T, y: T) -> Self { Self { x, y } }
}

impl<T> Vector2<T>
where T: Clone + Copy + Mul<Output = T> + Add<Output = T>{
    pub fn dot(lhs: &Self, rhs: &Self) -> T { lhs.x * rhs.x + lhs.y * rhs.y }
}