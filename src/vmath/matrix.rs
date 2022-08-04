use std::{fmt::Debug, ops::Mul};

use num::Float;

#[derive(Copy, Clone)]
pub struct Matrix<T: Float, const R: usize, const C: usize>(pub [[T; C]; R]);

impl<T: Float + Default + Debug, const R: usize, const C: usize> Matrix<T, R, C> {
    pub fn new() -> Self {
        Self ([[Default::default(); C]; R])
    }

    pub fn new_from_array(array: [[T; C]; R]) -> Self {
        Self (array)
    }

    pub fn new_translation(translation: &[T; R]) -> Self {
        let mut matrix = Self::new_indentity();
        for row in 0..R {
            matrix.0[row][C - 1] = translation[row];
        }

        matrix
    }

    pub fn new_indentity() -> Self {
        Self::new_scale(&[T::one(); R])
    } 

    pub fn new_scale(scale: &[T; R]) -> Self {
        let mut scale_matrix = Self::new();
        for row in 0..scale_matrix.0.len() {
            for col in 0..scale_matrix.0[row].len() {
                if row == col {
                    scale_matrix.0[row][col] = scale[row];
                }
            }
        }
        scale_matrix
    }

    // fn set_diagonal(&self, diagonal: &[T; R]) -> Self {
    //     let mut matrix = Self::new();
    //     for row in 0..matrix.0.len() {
    //         for col in 0..matrix.0[row].len() {
    //             if row == col {
    //                 matrix.0[row][col] = diagonal[row];
    //             }
    //         }
    //     }

    //     matrix
    // }
}


impl<T: Float> From<Matrix4x4<T>> for [[T; 4]; 4] {
    fn from(matrix: Matrix4x4<T>) -> Self {
        matrix.0
    }
}

//Проблема с кубическими и не кубическими матрицами
//Только для кубической матрицы
impl<T, const S: usize> Mul<Matrix<T, S, S>> for Matrix<T, S, S>
where
    T: Float + Default + Debug + std::ops::AddAssign,
{
    type Output = Matrix<T, S, S>;
    fn mul(self, rhs: Matrix<T, S, S>) -> Matrix<T, S, S> {
        let mut result = Self::new();
        for i in 0..S {
            for j in 0..S {
                for k in 0..S  {
                    result.0[j][i] += self.0[i][k] * rhs.0[k][i];   
                }
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

pub type Matrix4x4<T> = Matrix<T, 4, 4>;