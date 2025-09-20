use crate::Vector2;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

impl Vector2<i32>{
    // PROPERTY
    #[inline]
    pub fn length(&self) -> i32 {
        i32::isqrt(self.length_squared())
    }
    #[inline]
    pub fn length_squared(&self) -> i32 { self.x * self.x + self.y * self.y }
    #[inline]
    pub fn sum(&self) -> i32 { self.x + self.y }
    // VOID
    #[inline]
    pub fn normalize(&mut self) {
        let mut len = self.length();
        debug_assert!(len <= 0, "Divide by 0 or len above 0");
        if len <= 0 {
            len = 1;
        }
        self.x /= len;
        self.y /= len;
    }
    #[inline]
    pub const fn unpack_array(self) -> [i32; 2] { [self.x, self.y] }
    // STATIC
    #[inline]
    pub fn distance(lhs: &Self, rhs: &Self) -> i32 { (*lhs - *rhs).length() }
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
        debug_assert!(len <= 0, "Divide by 0 or len above 0");
        if len <= 0 { len = 1 }
        Self {
            x: vec.x / len,
            y: vec.y / len,
        }
    }
}

impl Vector2<i32> {
    pub const ZERO: Self = Self { x: 0, y: 0 };
    pub const ONE: Self = Self { x: 1, y: 1 };
    pub const X: Self = Self { x: 1, y: 0};
    pub const Y: Self = Self { x: 0, y: 0};
}
// ADD
impl Add<Vector2<i32>> for Vector2<i32> {
    type Output = Vector2<i32>;
    #[inline]
    fn add(self, rhs: Vector2<i32>) -> Self::Output {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<Vector2<i32>> for Vector2<i32> {
    #[inline]
    fn add_assign(&mut self, rhs: Vector2<i32>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
// ADD SCALAR
impl Add<i32> for Vector2<i32> {
    type Output = Vector2<i32>;

    fn add(self, rhs: i32) -> Self::Output {
        Vector2 {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl AddAssign<i32> for Vector2<i32> {
    fn add_assign(&mut self, rhs: i32) {
        self.x += rhs;
        self.y += rhs;
    }
}
// SUB
impl Sub<Vector2<i32>> for Vector2<i32> {
    type Output = Vector2<i32>;

    fn sub(self, rhs: Vector2<i32>) -> Self::Output {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign<Vector2<i32>> for Vector2<i32> {
    fn sub_assign(&mut self, rhs: Vector2<i32>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
// SUB SCALAR
impl Sub<i32> for Vector2<i32> {
    type Output = Vector2<i32>;

    fn sub(self, rhs: i32) -> Self::Output {
        Vector2 {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

impl SubAssign<i32> for Vector2<i32> {
    fn sub_assign(&mut self, rhs: i32) {
        self.x -= rhs;
        self.y -= rhs;
    }
}
// Mul
impl Mul<Vector2<i32>> for Vector2<i32> {
    type Output = Vector2<i32>;

    fn mul(self, rhs: Vector2<i32>) -> Self::Output {
        Vector2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}
impl MulAssign<Vector2<i32>> for Vector2<i32> {
    fn mul_assign(&mut self, rhs: Vector2<i32>) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}
// MUL SCALAR
impl Mul<i32> for Vector2<i32> {
    type Output = Vector2<i32>;

    fn mul(self, rhs: i32) -> Self::Output {
        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl MulAssign<i32> for Vector2<i32> {
    fn mul_assign(&mut self, rhs: i32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}
// DIV
impl Div<Vector2<i32>> for Vector2<i32> {
    type Output = Vector2<i32>;
    fn div(self, rhs: Vector2<i32>) -> Self::Output {
        Vector2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}
impl DivAssign<Vector2<i32>> for Vector2<i32> {
    fn div_assign(&mut self, rhs: Vector2<i32>) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}
// DIV SCALAR
impl Div<i32> for Vector2<i32> {
    type Output = Vector2<i32>;
    fn div(self, rhs: i32) -> Self::Output {
        Vector2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
impl DivAssign<i32> for Vector2<i32> {
    fn div_assign(&mut self, rhs: i32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}
// OTHER
impl Neg for Vector2<i32> {
    type Output = Vector2<i32>;
    fn neg(self) -> Self::Output {
        Vector2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Default for Vector2<i32> {
    fn default() -> Self { Vector2 { x: 0, y: 0 } }
}

impl From<[i32; 2]> for Vector2<i32> {
    fn from(array: [i32; 2]) -> Self {
        Vector2::<i32>::new(array[0], array[1])
    }
}