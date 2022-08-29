use std::ops::{Sub, Add, Mul};
use num::Float;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Float> Vector3<T> {
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn normalize(self) -> Self {
        let length = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        Self { x: self.x / length, y: self.y / length, z: self.z / length }
    }

    pub fn cross(self, other: Vector3<T>) -> Self {
        Self::new(
            (self.y * other.z) - (self.z * other.y),
            (self.z * other.x) - (self.x * other.z), 
            (self.x * other.y) - (self.y * other.x),
        )
    }

    pub fn dot(self, other: Vector3<T>) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn unit_y() -> Self {
        Self::new(T::zero(), T::one(), T::zero())
    }

    pub fn unit_x() -> Self {
        Self::new(T::one(), T::zero(), T::zero())
    }

    pub fn unit_z() -> Self {
        Self::new(T::zero(), T::zero(), T::one())
    }
}

impl<T: Float> Sub<Vector3<T>> for Vector3<T> {
    type Output = Vector3<T>;
    fn sub(self, rhs: Vector3<T>) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T: Float> Add<Vector3<T>> for Vector3<T> {
    type Output = Vector3<T>;
    fn add(self, rhs: Vector3<T>) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T: Float> Mul<T> for Vector3<T> {
    type Output = Vector3<T>;
    fn mul(self, rhs: T) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
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