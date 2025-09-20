use std::ops::{Add, AddAssign, Mul};

pub mod prelude;
mod vector;
mod vector2;
mod matrix;
mod quaternion;
/*
*   __     __  _____    ____   _____    ___    ____  
*   \ \   / / | ____|  / ___| |_   _|  / _ \  |  _ \ 
*    \ \ / /  |  _|   | |       | |   | | | | | |_) |
*     \ V /   | |___  | |___    | |   | |_| | |  _ < 
*      \_/    |_____|  \____|   |_|    \___/  |_| \_\
*/
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector3{
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector2<T>
where T: Clone + Copy{
    pub x: T,
    pub y: T,
}
/*
*    __  __     _   _____  ____   ___ __  __  
*   |  \/  |   / \ |_   _||  _ \ |_ _|\ \/ / 
*   | |\/| |  / _ \  | |  | |_) | | |  \  /  
*   | |  | | / ___ \ | |  |  _ <  | |  /  \  
*   |_|  |_|/_/   \_\|_|  |_| \_\|___|/_/\_\ 
*   p.s Column-Major
*/
#[derive(Debug, Clone, PartialEq)]
// #[cfg(feature = "matrix4x4")]
/// USED BY DEFAULT COLUMN-MAJOR
pub struct Matrix4x4 {
    pub data: [[f32; 4]; 4],
}
#[derive(Debug, Clone, PartialEq)]
// #[cfg(feature = "matrix3x3")]
/// USED BY DEFAULT COLUMN-MAJOR
pub struct Matrix3x3 {
    pub data: [[f32; 3]; 3],
}