use num::Float;
pub struct Matrix<T: Float, const R: usize, const C: usize>(pub [[T; C]; R]);


impl<T: Float + Default, const R: usize, const C: usize> Matrix<T, R, C> {
    pub fn new() -> Self {
        Matrix ([[Default::default(); C]; R]) 
    }
}

pub type Matrix4x4<T> = Matrix<T, 4, 4>;

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