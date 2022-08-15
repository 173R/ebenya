use std::fmt::Debug;

use num::Float;

use crate::vmath::{Matrix4x4, Vector3};

pub fn lookAt<T>(position: Vector3<T>, target: Vector3<T>) -> Matrix4x4<T>
where
    T: Float + Default + std::ops::AddAssign + Debug
{
    //let dir = (target - position).normalize();
    let dir = target.normalize();
    let right 
        = Vector3::new(T::zero(), T::one(), T::zero()).cross(dir).normalize();
    let up = dir.cross(right); 

    Matrix4x4::from([
        [right.x, up.x, dir.x, T::zero()], 
        [right.y, up.y, dir.y, T::zero()],
        [right.z, up.z, dir.z, T::zero()],
        [T::zero(), T::zero(), T::zero(), T::one()],
    ]) * Matrix4x4::new_translation(
        Vector3::new(-position.x, -position.y, -position.z)
    )
}