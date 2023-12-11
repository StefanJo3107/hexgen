use std::ops;

pub struct Matrix(pub [[f32; 4]; 4]);

impl ops::Add<Matrix> for Matrix {
    type Output = Matrix;

    fn add(self, rhs: Matrix) -> Self::Output {
        let mut matrix = self.0;
        for i in 0..4 {
            for j in 0..4 {
                matrix[i][j] += rhs.0[i][j];
            }
        }
        Matrix(matrix)
    }
}

impl ops::AddAssign<Matrix> for Matrix {
    fn add_assign(&mut self, rhs: Matrix) {
        for i in 0..4 {
            for j in 0..4 {
                self.0[i][j] += rhs.0[i][j];
            }
        }
    }
}

impl ops::Sub<Matrix> for Matrix {
    type Output = Matrix;

    fn sub(self, rhs: Matrix) -> Self::Output {
        let mut matrix = self.0;
        for i in 0..4 {
            for j in 0..4 {
                matrix[i][j] -= rhs.0[i][j];
            }
        }
        Matrix(matrix)
    }
}

impl ops::SubAssign<Matrix> for Matrix {
    fn sub_assign(&mut self, rhs: Matrix) {
        for i in 0..4 {
            for j in 0..4 {
                self.0[i][j] -= rhs.0[i][j];
            }
        }
    }
}

impl ops::Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Self::Output {
        let mut matrix: [[f32; 4]; 4] = Default::default();
        for i in 0..4 {
            for j in 0..4 {
                let mut value = 0f32;
                for k in 0..4 {
                    value += self.0[i][k] * rhs.0[k][j];
                }
                matrix[i][j] = value;
            }
        }
        Matrix(matrix)
    }
}

impl ops::MulAssign<Matrix> for Matrix {
    fn mul_assign(&mut self, rhs: Matrix) {
        for i in 0..4 {
            for j in 0..4 {
                let mut value = 0f32;
                for k in 0..4 {
                    value += self.0[i][k] * rhs.0[k][j];
                }
                self.0[i][j] = value;
            }
        }
    }
}