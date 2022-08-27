use std::{fmt::Debug, ops::Mul};
use num::{Float, cast};

use crate::vmath::{SquareMatrix, Vector3};


pub type Matrix4x4<T> = SquareMatrix<T, 4>;

impl<T: Float + Default + Debug + std::ops::AddAssign> Matrix4x4<T> {
    pub fn new_perspective(width: T, height: T, near: T, far: T, fovy: T) -> Self {
        let aspect = width / height;
        let two: T = cast(2).unwrap();
        //На основе fovy вычисляем фокусное расстояние
        let focal_lenght: T = T::one() / (fovy / two).to_radians().tan();
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

    pub fn new_look_at(position: Vector3<T>, target: Vector3<T>) -> Self {
        //let dir = (target - position).normalize();
        let dir = target.normalize();
        let right = Vector3::unit_y().cross(dir).normalize();
        let up = dir.cross(right); 

        Matrix4x4::from([
            [right.x, up.x, dir.x, T::zero()], 
            [right.y, up.y, dir.y, T::zero()],
            [right.z, up.z, dir.z, T::zero()],
            [-right.dot(position), -up.dot(position), -dir.dot(position), T::one()],
        ])
    }

    /* pub fn new_rotate(axis: Vector3<T>, angle: T) -> Self {
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
    } */
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