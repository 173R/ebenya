use std::ops::Sub;
use num::Float;

#[derive(Clone, Copy, Debug)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Float> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn normalize(self) -> Self {
        let length = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        Self { x: self.x / length, y: self.y / length, z: self.z / length }
    }

    pub fn cross(self, other: Vector3<T>) -> Vector3<T> {
        Vector3::new(
            (self.y * other.z) - (self.z * other.y),
            (self.z * other.x) - (self.x * other.z), 
            (self.x * other.y) - (self.y * other.x),
        )
    }
}

impl<T: Float> Sub<Vector3<T>> for Vector3<T> {
    type Output = Vector3<T>;
    fn sub(self, rhs: Vector3<T>) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Vector4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T> Vector4<T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }
}


// impl<T> From<Vector4<T>> for [T; 4] {
//     fn from(v: Vector4<T>) -> Self {
//         return [v.x, v.y, v.z, v.w ]
//     }
// }