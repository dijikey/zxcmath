use crate::*;

mod traits;

impl Quaternion{
    pub const RAD_TO_DEG: f32 = 360.0 / (std::f32::consts::PI * 2.0);
    #[inline]
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Quaternion { Quaternion{x, y, z, w} }
    #[inline]
    pub const fn identity() -> Quaternion { Quaternion::new(0.0, 0.0, 0.0, 1.0) }
    // #[inline]
    // pub fn euler(vec: Vector) -> Quaternion {
    //     let vec = vec * Self::RAD_TO_DEG;
    //     Quaternion::new(vec.x, vec.y, vec.z, 0.0)
    // }
    // Void fn
    pub fn normalize(&mut self){
        let mag = Self::magnitude(self);
        self.x /= mag;
        self.y /= mag;
        self.z /= mag;
        self.w /= mag;
    }
    // Static fn
    #[inline]
    pub const fn dot(lhs: &Quaternion, rhs: &Quaternion) -> f32 { lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z + lhs.w * rhs.w }
    #[inline]
    pub fn angle(lhs: &Quaternion, rhs: &Quaternion) -> f32{
        let dot = f32::min(Self::dot(lhs, rhs).abs(), 1.0);
        match Self::is_equal_dot(dot) {
            true => { 0.0 }
            false => { dot.acos() * 2.0 * Quaternion::RAD_TO_DEG }
        }
    }
    #[inline]
    pub fn magnitude(q: &Quaternion) -> f32 { (q.w.powi(2) + q.x.powi(2) + q.y.powi(2) + q.z.powi(2)).sqrt() }
    #[inline]
    pub fn conjugate(q: &Quaternion) -> Quaternion{ Quaternion::new(-q.x, -q.y, -q.z, q.w) }
    #[inline]
    pub fn normalized(q: &Quaternion) -> Quaternion {
        let mag = Self::magnitude(q);
        Self::new(q.x / mag, q.y / mag, q.z / mag, q.w / mag)
    }
    pub fn lerp(lhs: &Quaternion, rhs: &Quaternion, t: f32) -> Quaternion {
        Self::normalized(&Self::new(
            lhs.x + (rhs.x - lhs.x) * t,
            lhs.y + (rhs.y - lhs.y) * t,
            lhs.z + (rhs.z - lhs.z) * t,
            lhs.w + (rhs.w - lhs.w) * t,
        ))
    }
    #[inline]
    pub const fn is_equal_dot(dot: f32) -> bool{
        dot > 1.0 - f32::EPSILON
    }
    pub const fn unpack(self) -> [f32; 4] { [self.x, self.y, self.z, self.w] }
}