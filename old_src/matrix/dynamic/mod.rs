use crate::Matrix;

mod traits;

impl Matrix {
    #[inline]
    pub fn new(values: Vec<f32>, rows: usize, cols: usize) -> Self {
        if values.len() != rows * cols {
            panic!("Rows and Cols are not the same length vector");
        }

        Self { data: values, rows, cols }
    }
    #[inline]
    pub fn default(rows: usize, cols: usize) -> Self {
        Self::new(vec![0.0; rows * cols], rows, cols)
    }
    #[inline]
    pub fn identity(size: usize) -> Self {
        let mut matrix = Self::default(size, size);
        for i in 1..=size {
            matrix.set(i, i, 1.0);
        }
        matrix
    }
    #[inline]
    pub fn is_equals(lhs: &Matrix, rhs: &Matrix) -> bool { lhs.size() == rhs.size() }
    #[inline]
    pub fn is_square(matrix: &Matrix) -> bool { matrix.rows == matrix.cols }
    #[inline]
    pub fn get(&self, i: usize, j: usize) -> f32 { self.data[i * self.cols + j] }
    #[inline]
    pub fn set(&mut self, i: usize, j: usize, value: f32) { self.data.insert(i * self.cols + j, value); }
    // PROPERTY
    pub const fn rows(&self) -> usize { self.rows }
    pub const fn cols(&self) -> usize { self.cols }
    pub const fn size(&self) -> usize { self.cols * self.rows }
}
