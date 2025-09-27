mod traits;

use std::f32::consts::PI;
use crate::Matrix4x4;
/*
 __  __     _   _____  ____   ___ __  __  _  _         _  _   
|  \/  |   / \ |_   _||  _ \ |_ _|\ \/ / | || | __  __| || |  
| |\/| |  / _ \  | |  | |_) | | |  \  /  | || |_\ \/ /| || |_ 
| |  | | / ___ \ | |  |  _ <  | |  /  \  |__   _|>  < |__   _|
|_|  |_|/_/   \_\|_|  |_| \_\|___|/_/\_\    |_| /_/\_\   |_|  

 */
impl Matrix4x4 {
    // CONSTRUCTORS
    /// If u need fill matrix use [[ {value} ;4]; 4]
    #[inline]
    pub const fn new(data: [[f32; 4]; 4]) -> Self {
        Matrix4x4 { data }
    }
    #[inline]
    pub const fn identity() -> Self {
        Self::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0]]
        )
    }
    #[inline]
    pub fn translate(x: f32, y: f32, z: f32) -> Self {
        debug_assert!(x.is_nan() || x.is_infinite(), "X[{x}] is not correct");
        debug_assert!(y.is_nan() || y.is_infinite(), "Y[{y}] is not correct");
        debug_assert!(z.is_nan() || z.is_infinite(), "Z[{z}] is not correct");
        Self::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [ x,   z,   y, 1.0],
        ])
    }
    #[inline]
    pub fn scale(x: f32, y: f32, z: f32) -> Self {
        debug_assert!(x.is_nan() || x.is_infinite(), "X[{x}] is not correct");
        debug_assert!(y.is_nan() || y.is_infinite(), "Y[{y}] is not correct");
        debug_assert!(z.is_nan() || z.is_infinite(), "Z[{z}] is not correct");
        Self::new([
            [x, 0.0, 0.0, 0.0],
            [0.0, y, 0.0, 0.0],
            [0.0, 0.0, z, 0.0],
            [0.0, 0.0, 0.0, 1.0]]
        )
    }
    // ROTATE
    #[inline]
    pub fn rotation_y(radian: f32) -> Self {
        let (s, c) = radian.sin_cos();
        Self::new([
            [c,   0.0,  s,  0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-s,   0.0, c,   0.0],
            [0.0, 0.0, 0.0, 1.0]]
        )
    }
    #[inline]
    pub fn rotation_x(radian: f32) -> Self {
        let (s, c) = radian.sin_cos();
        Self::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0,  c,   s,  0.0],
            [0.0, -s,   c,  0.0],
            [0.0, 0.0, 0.0, 1.0]]
        )
    }
    #[inline]
    pub fn rotation_z(radian: f32) -> Self {
        let (s, c) = radian.sin_cos();
        Self::new([
            [ c,   s,  0.0, 0.0],
            [-s,   c,  0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0]]
        )
    }
    #[inline]
    pub fn transpose(matrix: Matrix4x4) -> Self {
        let mut result = Self::default();
        for i in 0..4 {
            for j in 0..4 {
                result.data[i][j] = matrix.data[j][i];
            }
        }
        result
    }
    #[inline]
    pub fn lerp(a: &Self, b: &Self, t: f32) -> Self {
        let mut result = Self::default();
        for i in 0..4 {
            for j in 0..4 {
                result.data[i][j] = a.data[i][j] + t * (b.data[i][j] - a.data[i][j]);
            }
        }
        result
    }
    // Other
    #[inline]
    pub const fn trace(&self) -> f32 {
        self.data[0][0] + self.data[1][1] + self.data[2][2] + self.data[3][3]
    }
    // OTHER
    #[inline]
    pub fn perspective(field_of_view: f32, aspect_ratio: f32, near_plane_distance: f32, far_plane_distance: f32) -> Self {
        if field_of_view <= 0.0 || field_of_view >= PI {
            panic!("Field of View must be between 0 and PI");
        }

        if aspect_ratio <= 0.0 {
            panic!("Aspect Ratio cannot be greater than 0");
        }

        if far_plane_distance <= 0.0 {
            panic!("FarPlaneDistance cannot be greater than 0");
        }

        if near_plane_distance >= far_plane_distance {
            panic!("FarPlaneDistance cannot be greater than nearPlaneDistance");
        }

        let height = 1.0 / (field_of_view * 0.5).tan();
        let width = height / aspect_ratio;
        let range = if far_plane_distance.is_infinite(){ -1.0 }
        else { far_plane_distance / (near_plane_distance - far_plane_distance) };

        Matrix4x4::new([
            [width, 0.0,   0.0,    0.0],
            [0.0,   height, 0.0,    0.0],
            [0.0,   0.0,    range, -1.0],
            [0.0,   0.0,    range * near_plane_distance, 0.0]
        ])
    }
    // UNPACK
    pub const fn unpack(self) -> [[f32; 4]; 4] {
        self.data
    }
}