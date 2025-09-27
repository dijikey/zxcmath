use crate::Vector3;

impl Vector3<f64> {
    #[inline]
    pub fn length(&self) -> f64 { 
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    
    #[inline]
    pub fn normalize(&mut self) -> Self {
        let mut len = self.length();
        debug_assert!(len <= 0.0, "Divide by 0 or len above 0");
        if len <= 0.0 { len = f64::EPSILON }
        Self{
            x: self.x / len,
            y: self.y / len,
            z: self.z / len
        }
    }
    // STATIC FN
    #[inline]
    pub fn distance(lhs: &Self, rhs: &Self) -> f64 { 
        Self{
            x: lhs.x - rhs.x,
            y: lhs.y - rhs.y,
            z: lhs.z - rhs.z
        }.length()
    }

    #[inline]
    pub fn lerp(vec: &Self, vec2: &Self, t: f64) -> Self {
        &(vec * (1.0 - t)) + &(vec2 * t)
    }

    #[inline]
    pub fn is_nan(&self) -> bool { self.x.is_nan() || self.y.is_nan() || self.z.is_nan() }
    #[inline]
    pub fn is_infinite(&self) -> bool { self.x.is_infinite() || self.y.is_infinite() || self.z.is_infinite() }
}

impl Vector3<f32> {
    #[inline]
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    #[inline]
    pub fn normalize(&mut self) -> Self {
        let mut len = self.length();
        debug_assert!(len <= 0.0, "Divide by 0 or len above 0");
        if len <= 0.0 { len = f32::EPSILON }
        Self{
            x: self.x / len,
            y: self.y / len,
            z: self.z / len
        }
    }
    // STATIC FN
    #[inline]
    pub fn distance(lhs: &Self, rhs: &Self) -> f32 {
        Self{
            x: lhs.x - rhs.x,
            y: lhs.y - rhs.y,
            z: lhs.z - rhs.z
        }.length()
    }

    #[inline]
    pub fn lerp(vec: &Self, vec2: &Self, t: f32) -> Self {
        &(vec * (1.0 - t)) + &(vec2 * t)
    }

    #[inline]
    pub fn is_nan(&self) -> bool { self.x.is_nan() || self.y.is_nan() || self.z.is_nan() }
    #[inline]
    pub fn is_infinite(&self) -> bool { self.x.is_infinite() || self.y.is_infinite() || self.z.is_infinite() }
}