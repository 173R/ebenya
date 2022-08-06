use std::{fmt::Debug, ops::Mul};

use num::Float;

#[derive(Copy, Clone)]
pub struct SquareMatrix<T: Float, const S: usize> {
    pub data: [[T; S]; S],
}

impl<T: Float + Default + Debug, const S: usize> SquareMatrix<T, S> {
    pub fn new() -> Self {
        Self {
            data: [[Default::default(); S]; S] 
        }
    }

    pub fn new_from_array(array: [[T; S]; S]) -> Self {
        Self {
            data: array 
        }
    }

    pub fn new_translation(translation: &[T; S]) -> Self {
        let mut matrix = Self::new_indentity();
        for row in 0..S {
            matrix.data[row][S - 1] = translation[row];
        }

        matrix
    }

    pub fn new_indentity() -> Self {
        Self::new_scale(&[T::one(); S])
    } 

    pub fn new_scale(scale: &[T; S]) -> Self {
        let mut scale_matrix = Self::new();
        for row in 0..scale_matrix.data.len() {
            for col in 0..scale_matrix.data[row].len() {
                if row == col {
                    scale_matrix.data[row][col] = scale[row];
                }
            }
        }
        scale_matrix
    }

    // pub fn transpose(&mut self) {
    //     let mut matrix = *self;
    //     for (index, row) in matrix.0.iter().enumerate() {
    //        for value in row {
    //         self.0[index][] = value;
    //        } 
    //     }
    // }

}


impl<T: Float> From<Matrix4x4<T>> for [[T; 4]; 4] {
    fn from(matrix: Matrix4x4<T>) -> Self {
        matrix.data
    }
}

impl<T, const S: usize> Mul<SquareMatrix<T, S>> for SquareMatrix<T, S>
where
    T: Float + Default + Debug + std::ops::AddAssign,
{
    type Output = SquareMatrix<T, S>;
    fn mul(self, rhs: SquareMatrix<T, S>) -> SquareMatrix<T, S> {
        let mut result = Self::new();
        for i in 0..S {
            for j in 0..S {
                for k in 0..S  {
                    result.data[j][i] += self.data[i][k] * rhs.data[k][i];   
                }
            }
        }

        result
    }
}

impl<T: Float + Debug, const S: usize> Debug for SquareMatrix<T, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.data[..].fmt(f)
    }
}

pub type Matrix4x4<T> = SquareMatrix<T, 4>;