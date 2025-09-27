use crate::Vector2;
use std::f64;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use crate::vector2::f32;

impl Vector2<f32>{
    // PROPERTY
    #[inline]
    pub fn length(&self) -> f32 {
        f32::sqrt(self.length_squared())
    }
    #[inline]
    pub fn length_squared(&self) -> f32 { self.x * self.x + self.y * self.y }
    #[inline]
    pub fn sum(&self) -> f32 { self.x + self.y }
    // VOID
    #[inline]
    pub fn normalize(&mut self) {
        let mut len = self.length();
        debug_assert!(len < f32::EPSILON, "Divide by 0 or len above 0");
        if len < f32::EPSILON {
            len = 1.0; 
        }
        self.x /= len;
        self.y /= len;
    }
    // STATIC
    #[inline]
    pub fn lerp(vec: &Self, vec2: &Self, t: f32) -> Self {
        *vec * (1.0 - t) + *vec2 * t
    }
    #[inline]
    pub fn distance(lhs: &Self, rhs: &Self) -> f32 { (*lhs - *rhs).length() }
    // #[inline]
    // pub fn min(lhs: &Self, rhs: &Self) -> Self {
    //     Self {
    //         x: lhs.x.min(rhs.x),
    //         y: lhs.y.min(rhs.y),
    //     }
    // }
    // #[inline]
    // pub fn max(lhs: &Self, rhs: &Self) -> Self {
    //     Self {
    //         x: lhs.x.max(rhs.x),
    //         y: lhs.y.max(rhs.y),
    //     }
    // }
    #[inline]
    pub fn abs(vec: &Self) -> Self { Self::new(vec.x.abs(), vec.y.abs()) }
    #[inline]
    pub fn normalized(vec: &Self) -> Self {
        let mut len = vec.length();
        debug_assert!(len < f32::EPSILON, "Divide by 0 or len above 0");
        if len < f32::EPSILON { len = 1.0 }
        Self {
            x: vec.x / len,
            y: vec.y / len,
        }
    }
    // Boolean check
    #[inline]
    pub fn is_nan(&self) -> bool { self.x.is_nan() || self.y.is_nan() }
    #[inline]
    pub fn is_infinite(&self) -> bool { self.x.is_infinite() || self.y.is_infinite() }
}

impl Vector2<f32> {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
    pub const ONE: Self = Self { x: 1.0, y: 1.0 };
    pub const X: Self = Self { x: 1.0, y: 0.0};
    pub const Y: Self = Self { x: 0.0, y: 1.0};
}
// ADD
impl Add<Vector2<f32>> for Vector2<f32> {
    type Output = Vector2<f32>;
    #[inline]
    fn add(self, rhs: Vector2<f32>) -> Self::Output {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<Vector2<f32>> for Vector2<f32> {
    #[inline]
    fn add_assign(&mut self, rhs: Vector2<f32>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
// ADD SCALAR
impl Add<f32> for Vector2<f32> {
    type Output = Vector2<f32>;

    fn add(self, rhs: f32) -> Self::Output {
        Vector2 {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl AddAssign<f32> for Vector2<f32> {
    fn add_assign(&mut self, rhs: f32) {
        self.x += rhs;
        self.y += rhs;
    }
}
// SUB
impl Sub<Vector2<f32>> for Vector2<f32> {
    type Output = Vector2<f32>;

    fn sub(self, rhs: Vector2<f32>) -> Self::Output {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign<Vector2<f32>> for Vector2<f32> {
    fn sub_assign(&mut self, rhs: Vector2<f32>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
// SUB SCALAR
impl Sub<f32> for Vector2<f32> {
    type Output = Vector2<f32>;

    fn sub(self, rhs: f32) -> Self::Output {
        Vector2 {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

impl SubAssign<f32> for Vector2<f32> {
    fn sub_assign(&mut self, rhs: f32) {
        self.x -= rhs;
        self.y -= rhs;
    }
}
// Mul
impl Mul<Vector2<f32>> for Vector2<f32> {
    type Output = Vector2<f32>;

    fn mul(self, rhs: Vector2<f32>) -> Self::Output {
        Vector2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}
impl MulAssign<Vector2<f32>> for Vector2<f32> {
    fn mul_assign(&mut self, rhs: Vector2<f32>) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}
// MUL SCALAR
impl Mul<f32> for Vector2<f32> {
    type Output = Vector2<f32>;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl MulAssign<f32> for Vector2<f32> {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}
// DIV
impl Div<Vector2<f32>> for Vector2<f32> {
    type Output = Vector2<f32>;
    fn div(self, rhs: Vector2<f32>) -> Self::Output {
        Vector2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}
impl DivAssign<Vector2<f32>> for Vector2<f32> {
    fn div_assign(&mut self, rhs: Vector2<f32>) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}
// DIV SCALAR
impl Div<f32> for Vector2<f32> {
    type Output = Vector2<f32>;
    fn div(self, rhs: f32) -> Self::Output {
        Vector2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
impl DivAssign<f32> for Vector2<f32> {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}
// OTHER
impl Neg for Vector2<f32> {
    type Output = Vector2<f32>;
    fn neg(self) -> Self::Output {
        Vector2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Default for Vector2<f32> {
    fn default() -> Self { Vector2 { x: 0.0, y: 0.0 } }
}

impl From<[f32; 2]> for Vector2<f32> {
    fn from(array: [f32; 2]) -> Self {
        Vector2::new(array[0], array[1])
    }
}