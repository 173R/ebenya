use std::{fmt::Debug, ops::Mul};

use num::Float;

#[derive(Copy, Clone)]
pub struct Matrix<T: Float, const R: usize, const C: usize>(pub [[T; C]; R]);

impl<T: Float + Default + Debug, const R: usize, const C: usize> Matrix<T, R, C> {
    pub fn new() -> Self {
        Self ([[Default::default(); C]; R])
    }

    pub fn new_indentity(diagonal: &[T; R]) -> Self {
        let mut matrix = Self::new();
        for row in 0..matrix.0.len() {
            for col in 0..matrix.0[row].len() {
                if row == col {
                    matrix.0[row][col] = diagonal[row];
                }
            }
        }
        matrix
    }

    pub fn new_translation(diagonal: &[T; R]) -> Self {
        let mut matrix = Self::new();
        for row in 0..R {
            matrix.0[row][C - 1] = diagonal[row];
        }

        matrix
    }

    //pub fn mul(matrix: Matrix<T, R, C>) -> Matrix<T, C, R> {
        
    //}
}

//Проблема с кубическими и не кубическими матрицами

impl<T, const R: usize, const C: usize> Mul<Matrix<T, R, C>> for Matrix<T, R, C>
where
    T: Float + Default + Debug + std::ops::AddAssign,
{
    type Output = Matrix<T, R, C>;
    fn mul(self, rhs: Matrix<T, R, C>) -> Matrix<T, R, C> {
        let mut result = Self::new();
        for row in 0..R {
            for col in 0..C {
                result.0[row][col] += self.0[row][col] * rhs.0[col][row];
            }
        }

        result
    }
}

impl<T: Float + Default + Debug, const R: usize, const C: usize> Debug for Matrix<T, R, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0[..].fmt(f)
    }
}

pub type Matrix4x4 = Matrix<f64, 4, 4>;

/*impl<T: Float + Default, const R: usize, const C: usize> Default for Matrix<T, R, C> {
    fn default() -> Self {
        Matrix {
            data: [[Default::default(); C]; R],
        }
    }
}*/

 
/*impl<T: Float + Default, const R: usize, const C: usize> Matrix<T, R, C> {
    fn new() -> Self {
        Matrix {
            data: [[10.0 as T, C]; R]
        }
    }
}
*/

//type Matrix4x4 = Matrix<f64, 4, 4>;
//type Matrix3x3 = Matrix<f64, 3, 3>;

/* 
impl<T: Float> Matrix4x4<T> {
    fn new() -> Self {
        Self { data: [[Default::default(); 4]; 4] }
    }
}*/