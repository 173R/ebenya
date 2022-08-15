/*
    Col-major square matrix
*/

use std::{fmt::Debug, ops::Mul};

use num::{Float, cast};

use crate::vmath::Vector3;

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

    // pub fn new_translation(translation: &[T; S]) -> Self {
    //     let mut translation_matrix = Self::new_indent();
    //     for row in 0..S {
    //         translation_matrix.data[S - 1][row] = translation[row];
    //     }

    //     translation_matrix
    // }

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

// Matrix 4 x 4
pub type Matrix4x4<T> = SquareMatrix<T, 4>;

impl<T: Float + Default + Debug> Matrix4x4<T> {
    pub fn new_perspective(width: T, height: T, near: T, far: T, fovy: T) -> Self {
        let aspect = width / height;
        let two: T = cast(2).unwrap();
        //На основе fovy вычисляем фокусное расстояние
        let focal_lenght: T = T::one() / (fovy / two).to_radians().tan();
        //let fov_x = T::from_f32(2.0).unwrap() * (aspect / focal_lenght).atan().to_degrees();

        [
            [focal_lenght / aspect, T::zero(), T::zero(), T::zero()], 
            [T::zero(), focal_lenght, T::zero(), T::zero()], 
            [T::zero(), T::zero(), (far)/(far - near), T::one()], 
            [T::zero(), T::zero(), -(near * far)/(far-near), T::zero()], 
        ].into()
    }

    pub fn new_translation(translation: Vector3<T>) -> Self {
        let mut translation_matrix = Self::new_indent();

        translation_matrix.data[3][0] = translation.x;
        translation_matrix.data[3][1] = translation.y;
        translation_matrix.data[3][2] = translation.z;
        translation_matrix.data[3][3] = T::one();
    
        translation_matrix
    }
}


    

// impl<T> Mul<Vector4<T>> for Matrix4x4<T>
// where
//     T: Float + Default + Debug + std::ops::AddAssign,
// {
//     type Output = SquareMatrix<T, S>;
//     fn mul(self, rhs: SquareMatrix<T, S>) -> SquareMatrix<T, S> {
//         let mut result = Self::new();
//         for i in 0..S {
//             for j in 0..S {
//                 for k in 0..S  {
//                     result.data[j][i] += self.data[k][i] * rhs.data[j][k];   
//                 }
//             }
//         }

//         result
//     }
// }


impl<T: Float> From<Matrix4x4<T>> for [[T; 4]; 4] {
    fn from(matrix: Matrix4x4<T>) -> Self {
        matrix.data
    }
}

impl<T: Float> From<[[T; 4]; 4]> for Matrix4x4<T> {
    fn from(array: [[T; 4]; 4]) -> Self {
        Self { data: array }
    }
}