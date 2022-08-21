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

    pub fn new_rotate(axis: Vector3<T>, angle: T) -> Self {
        let one = T::one();
        let zero = T::zero();
        let cos = angle.cos();
        let sin = angle.sin();
        let axis = axis.normalize();

        match axis {
            vec if vec.x == T::one() => {
                [
                    [one, zero, zero, zero], 
                    [zero, cos, sin, zero], 
                    [zero, -sin, cos, zero], 
                    [zero, zero, zero, one]
                ].into()
            },
            vec if vec.y == T::one() => {
                [
                    [cos, zero, -sin, zero], 
                    [zero, one, zero, zero], 
                    [sin, zero, cos, zero], 
                    [zero, zero, zero, one]
                ].into()
            },
            vec if vec.z == T::one() => {
                [
                    [cos, sin, zero, zero],
                    [-sin, cos, zero, zero], 
                    [zero, zero, one, zero], 
                    [zero, zero, zero, one]
                ].into()
            }
            _ => panic!("{}", "Wrong axis")

        }
    }
}

impl<T: Float> Mul<Vector3<T>> for Matrix4x4<T> {
    type Output = Vector3<T>;
    fn mul(self, rhs: Vector3<T>) -> Self::Output {
        Vector3::new(
            self.data[0][0] * rhs.x + self.data[1][0] * rhs.y + self.data[2][0] * rhs.z + self.data[3][0],
            self.data[0][1] * rhs.x + self.data[1][1] * rhs.y + self.data[2][1] * rhs.z + self.data[3][1],
            self.data[0][2] * rhs.x + self.data[1][2] * rhs.y + self.data[2][2] * rhs.z + self.data[3][2],
        )
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