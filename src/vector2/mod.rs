use crate::Vector2;

mod traits;

impl Vector2 {
    // CONSTRUCTOR
    #[inline]
    pub const fn new(x: f64, y: f64) -> Self { Self { x, y } }
    #[inline]
    pub const fn from_i32(x: i32, y: i32) -> Self { Self::new(x as f64, y as f64) }
    // PROPERTY
    #[inline]
    pub fn magnitude(&self) -> f64 { self.magnitude_squared().sqrt() }
    #[inline]
    pub const fn magnitude_squared(&self) -> f64 { self.x * self.x + self.y * self.y }
    #[inline]
    pub const fn sum(&self) -> f64 { self.x + self.y }
    // VOID
    #[inline]
    pub fn normalize(&mut self) {
        let mut len = self.magnitude();
        debug_assert!(len <= 0.0, "Divide by 0 or len above 0");
        if len <= 0.0 { len = f64::EPSILON }
        self.x /= len;
        self.y /= len;
    }
    #[inline]
    pub const fn unpack(self) -> (f64, f64) { (self.x, self.y) }
    #[inline]
    pub const fn unpack_array(self) -> [f64; 2] { [self.x, self.y] }
    // STATIC
    #[inline]
    pub fn lerp(vec: &Self, vec2: &Self, t: f64) -> Self {
        *vec * (1.0 - t) + *vec2 * t
    }
    #[inline]
    pub fn distance(lhs: &Self, rhs: &Self) -> f64 { (*lhs - *rhs).magnitude() }
    #[inline]
    pub fn min(lhs: &Self, rhs: &Self) -> Self {
        Self {
            x: lhs.x.min(rhs.x),
            y: lhs.y.min(rhs.y),
        }
    }
    #[inline]
    pub fn max(lhs: &Self, rhs: &Self) -> Self {
        Self {
            x: lhs.x.max(rhs.x),
            y: lhs.y.max(rhs.y),
        }
    }
    #[inline]
    pub fn abs(vec: &Self) -> Self { Self::new(vec.x.abs(), vec.y.abs()) }
    #[inline]
    /// Is DOT function
    pub fn dot(lhs: &Self, rhs: &Self) -> f64 { lhs.x * rhs.x + lhs.y * rhs.y }
    #[inline]
    pub fn normalized(vec: &Self) -> Self {
        let mut len = vec.magnitude();
        debug_assert!(len <= 0.0, "Divide by 0 or len above 0");
        if len < f64::EPSILON {
            /* CODE */
        }
        if len <= 0.0 { len = f64::EPSILON }
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

impl Vector2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
    pub const ONE: Self = Self { x: 1.0, y: 1.0 };
    pub const X: Self = Self { x: 1.0, y: 0.0};
    pub const Y: Self = Self { x: 0.0, y: 1.0};
    pub const Z: Self = Self { x: 0.0, y: 0.0};
}
