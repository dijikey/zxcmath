mod traits;

use crate::Matrix3x3;

impl Matrix3x3{
    // CONSTRUCTORS
    #[inline]
    pub const fn new(data: [[f32; 3]; 3]) -> Self {
        Self{data}
    }

    #[inline]
    pub const fn identity() -> Self {
        Self::new([
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0]]
        )
    }
    #[inline]
    pub const fn translate(x: f32, y: f32) -> Self {
        Self::new([
            [1.0, 0.0, x],
            [0.0, 1.0, y],
            [0.0, 0.0, 1.0],
        ])
    }
    #[inline]
    pub const fn scale(x: f32, y: f32) -> Self {
        Self::new([
            [x, 0.0, 0.0],
            [0.0, y, 0.0],
            [0.0, 0.0, 1.0]]
        )
    }
    #[inline]
    pub fn rotate(angle: f32) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self {
            data: [
                [cos, -sin, 0.0],
                [sin,  cos, 0.0],
                [0.0,  0.0, 1.0],
            ],
        }
    }
}