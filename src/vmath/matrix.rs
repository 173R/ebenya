/*
    Col-major square matrix
*/

use std::{fmt::Debug, ops::Mul};

use num::{Float};

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

    pub fn new_indent() -> Self {
        Self::new_scale(&[T::one(); S])
    } 

    pub fn new_scale(scale: &[T; S]) -> Self {
        let mut scale_matrix = Self::new();
        for row_i in 0..S {
            for col_i in 0..S {
                if row_i == col_i {
                    scale_matrix.data[row_i][col_i] = scale[row_i];
                }
            }
        }

        scale_matrix
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
                    result.data[j][i] += self.data[k][i] * rhs.data[j][k];   
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
