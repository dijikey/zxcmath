use crate::Vector;

mod traits;

impl Vector {
    // CONSTRUCTOR
    #[inline]
    pub const fn new(x: f64, y: f64, z: f64) -> Self { Self { x, y, z } }
    // #[inline]
    // pub const fn from_array(arr: [f32; 3]) -> Self { Self::new(arr[0], arr[1], arr[2]) }
    #[inline]
    pub const fn from_i32(x: i32, y: i32, z: i32) -> Self { Self::new(x as f64, y as f64, z as f64) }
    // PROPERTY
    #[inline]
    pub fn magnitude(&self) -> f64 { self.magnitude_squared().sqrt() }
    #[inline]
    pub const fn magnitude_squared(&self) -> f64 { self.x * self.x + self.y * self.y + self.z * self.z }
    #[inline]
    pub const fn sum(&self) -> f64 { self.x + self.y + self.z }
    // VOID
    #[inline]
    pub fn normalize(&mut self) {
        let mut len = self.magnitude();
        debug_assert!(len <= 0.0, "Divide by 0 or len above 0");
        if len <= 0.0 { len = f64::EPSILON }
        self.x /= len;
        self.y /= len;
        self.z /= len;
    }
    #[inline]
    pub const fn unpack(self) -> (f64, f64, f64) { (self.x, self.y, self.z) }
    #[inline]
    pub const fn unpack_array(self) -> [f64; 3] { [self.x, self.y, self.z] }
    // STATIC
    #[inline]
    pub fn projection(lhs: &Vector, rhs: &Vector) -> Vector {
        let normalize = Vector::normalized(&lhs);
        let scl = Vector::dot(&lhs, &rhs);
        normalize * scl
    }
    #[inline]
    pub fn lerp(vec: &Vector, vec2: &Vector, t: f64) -> Vector {
        *vec * (1.0 - t) + *vec2 * t
    }
    #[inline]
    pub fn distance(lhs: &Vector, rhs: &Vector) -> f64 { (*lhs - *rhs).magnitude() }
    #[inline]
    pub const fn cross_product(lhs: &Vector, rhs: &Vector) -> Vector {
        Vector {
            x: lhs.y * rhs.z - lhs.z * rhs.y,
            y: lhs.z * rhs.x - lhs.x * rhs.z,
            z: lhs.x * rhs.y - lhs.y * rhs.x,
        }
    }
    #[inline]
    pub fn min(lhs: &Self, rhs: &Self) -> Self {
        Self {
            x: lhs.x.min(rhs.x),
            y: lhs.y.min(rhs.y),
            z: lhs.z.min(rhs.z),
        }
    }
    #[inline]
    pub fn max(lhs: &Self, rhs: &Self) -> Self {
        Self {
            x: lhs.x.max(rhs.x),
            y: lhs.y.max(rhs.y),
            z: lhs.z.max(rhs.z),
        }
    }
    #[inline]
    pub fn abs(vec: &Vector) -> Self { Self::new(vec.x.abs(), vec.y.abs(), vec.z.abs()) }
    #[inline]
    /// Is DOT function
    pub fn dot(lhs: &Vector, rhs: &Vector) -> f64 { lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z }
    #[inline]
    pub fn normalized(vec: &Vector) -> Vector {
        let mut len = vec.magnitude();
        debug_assert!(len <= 0.0, "Divide by 0 or len above 0");
        if len < f64::EPSILON { 
            /* CODE */ 
        }
        if len <= 0.0 { len = f64::EPSILON }
        Vector {
            x: vec.x / len,
            y: vec.y / len,
            z: vec.z / len,
        }
    }
    // Boolean check
    #[inline]
    pub fn is_nan(&self) -> bool { self.x.is_nan() || self.y.is_nan() || self.z.is_nan() }
    #[inline]
    pub fn is_infinite(&self) -> bool { self.x.is_infinite() || self.y.is_infinite() || self.z.is_infinite() }
}

impl Vector {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0, z: 0.0 };
    pub const ONE: Self = Self { x: 1.0, y: 1.0, z: 1.0 };
    pub const X: Self = Self { x: 1.0, y: 0.0, z: 0.0 };
    pub const Y: Self = Self { x: 0.0, y: 1.0, z: 0.0 };
    pub const Z: Self = Self { x: 0.0, y: 0.0, z: 1.0 };
}
