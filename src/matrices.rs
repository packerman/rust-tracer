
use crate::tuples::Tuple;
use std::ops::Mul;
use std::ops::Index;

#[derive(PartialEq, Debug, Copy, Clone)]
struct Matrix4([[f32;4];4]);

impl Matrix4 {

    const fn new(m00: f32, m01: f32, m02: f32, m03: f32,
        m10: f32, m11: f32, m12: f32, m13: f32,
        m20: f32, m21: f32, m22: f32, m23: f32,
        m30: f32, m31: f32, m32: f32, m33: f32) -> Matrix4 {
            Matrix4 {
                0: [[m00, m01, m02, m03], [m10, m11, m12, m13], [m20, m21, m22, m23], [m30, m31, m32, m33]],
            }
        }

    const IDENTITY: Matrix4 = Matrix4::new(1.0, 0.0, 0.0, 0.0,
                                            0.0, 1.0, 0.0, 0.0,
                                            0.0, 0.0, 1.0, 0.0,
                                            0.0, 0.0, 0.0, 1.0);

    fn transpose(&self) -> Matrix4 {
        let mut result = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                result[i][j] = self.0[j][i];
            }
        }
        Matrix4 { 0 : result }
    }

    fn sub_matrix(&self, l: usize, k: usize) -> Matrix3 {
        let mut result = [[0.0; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                result[i][j] = self.0[if i < l { i } else { i + 1 }][if j < k { j } else { j + 1 }]
            }
        }
        Matrix3 { 0 : result }
    }

    fn minor(&self, l: usize, k: usize) -> f32 {
        self.sub_matrix(l, k).determinant()
    }

    fn cofactor(&self, l: usize, k: usize) -> f32 {
        (if l + k % 2 == 0 { 1.0 } else { -1.0 })*self.minor(l, k)
    }

    fn determinant(&self) -> f32 {
        self.0[0][0] * self.cofactor(0, 0) +
        self.0[0][1] * self.cofactor(0, 1) +
        self.0[0][2] * self.cofactor(0, 2) +
        self.0[0][3] * self.cofactor(0, 3)
    }
}

impl Index<(usize, usize)> for Matrix4 {

    type Output = f32;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.0[index.0][index.1]
     }
}

impl Mul<Matrix4> for Matrix4 {

    type Output = Matrix4;

    fn mul(self, other: Matrix4) -> Self::Output {
        let mut result = [[0.0;4];4];
        for i in 0..4 {
            for j in 0..4 {
                result[i][j] = self.0[i][0] * other.0[0][j] +
                                self.0[i][1] * other.0[1][j] +
                                self.0[i][2] * other.0[2][j] +
                                self.0[i][3] * other.0[3][j];
            }
        }
        Matrix4 { 0: result }
     }
}

impl Mul<Tuple> for Matrix4 {

    type Output = Tuple;

    fn mul(self, other: Tuple) -> Self::Output {
        Tuple::new(
            self.0[0][0] * other.x() + self.0[0][1] * other.y() + self.0[0][2] * other.z() + self.0[0][3] * other.w(),
            self.0[1][0] * other.x() + self.0[1][1] * other.y() + self.0[1][2] * other.z() + self.0[1][3] * other.w(),
            self.0[2][0] * other.x() + self.0[2][1] * other.y() + self.0[2][2] * other.z() + self.0[2][3] * other.w(),
            self.0[3][0] * other.x() + self.0[3][1] * other.y() + self.0[3][2] * other.z() + self.0[3][3] * other.w()
        )
     }
}

#[derive(PartialEq, Debug)]
struct Matrix2([[f32;2];2]);

impl Matrix2 {

    fn new(m00: f32, m01: f32, m10: f32, m11: f32) -> Matrix2 {
        Matrix2 {
            0: [[m00, m01], [m10, m11]],
        }
    }

    fn determinant(&self) -> f32 {
        self.0[0][0] * self.0[1][1] - self.0[0][1] * self.0[1][0]
    }
}

impl Index<(usize, usize)> for Matrix2 {

    type Output = f32;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.0[index.0][index.1]
     }
}

#[derive(PartialEq, Debug)]
struct Matrix3([[f32;3];3]);

impl Matrix3 {

    fn new(m00: f32, m01: f32, m02: f32,
        m10: f32, m11: f32, m12: f32,
        m20: f32, m21: f32, m22: f32) -> Matrix3 {
            Matrix3 {
                0: [[m00, m01, m02], [m10, m11, m12], [m20, m21, m22]],
            }
        }

    fn sub_matrix(&self, l: usize, k: usize) -> Matrix2 {
        let mut result = [[0.0; 2]; 2];
        for i in 0..2 {
            for j in 0..2 {
                result[i][j] = self.0[if i < l { i } else { i + 1 }][if j < k { j } else { j + 1 }]
            }
        }
        Matrix2 { 0 : result }
    }

    fn minor(&self, l: usize, k: usize) -> f32 {
        self.sub_matrix(l, k).determinant()
    }

    fn cofactor(&self, l: usize, k: usize) -> f32 {
        (if l + k % 2 == 0 { 1.0 } else { -1.0 })*self.minor(l, k)
    }

    fn determinant(&self) -> f32 {
        self.0[0][0] * self.cofactor(0, 0) + self.0[0][1] * self.cofactor(0, 1) + self.0[0][2] * self.cofactor(0, 2)
    }
}

impl Index<(usize, usize)> for Matrix3 {

    type Output = f32;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.0[index.0][index.1]
     }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tuples::Tuple;

    #[test]
    fn creating_4x4_matrix() {
        let m = Matrix4::new(1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.5, 15.5, 16.5);
        assert_eq!(m[(0, 0)], 1.0);
        assert_eq!(m[(0, 3)], 4.0);
        assert_eq!(m[(1, 0)], 5.5);
        assert_eq!(m[(1, 2)], 7.5);
        assert_eq!(m[(2, 2)], 11.0);
        assert_eq!(m[(3, 0)], 13.5);
        assert_eq!(m[(3, 2)], 15.5);
    }

    #[test]
    fn creating_2x2_matrix() {
        let m = Matrix2::new(-3.0, 5.0, 1.0, -2.0);
        assert_eq!(m[(0, 0)], -3.0);
        assert_eq!(m[(0, 1)], 5.0);
        assert_eq!(m[(1, 0)], 1.0);
        assert_eq!(m[(1, 1)], -2.0);
    }

    #[test]
    fn creating_3x3_matrix() {
        let m = Matrix3::new(-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.0);
        assert_eq!(m[(0, 0)], -3.0);
        assert_eq!(m[(1, 1)], -2.0);
        assert_eq!(m[(2, 2)], 1.0);
    }

    #[test]
    fn matrix_equality_with_identical_matrices() {
        let a = Matrix4::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0);
        let b = Matrix4::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0);
        assert_eq!(a, b);
    }

    #[test]
    fn matrix_equality_with_different_matrices() {
        let a = Matrix4::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0);
        let b = Matrix4::new(2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0);
        assert_ne!(a, b);
    }

    #[test]
    fn multiplying_two_matrices() {
        let a = Matrix4::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0);
        let b = Matrix4::new(-2.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, -1.0, 4.0, 3.0, 6.0, 5.0, 1.0, 2.0, 7.0, 8.0);
        assert_eq!(a * b, Matrix4::new(20.0, 22.0, 50.0, 48.0,
                                    44.0, 54.0, 114.0, 108.0,
                                    40.0, 58.0, 110.0, 102.0,
                                    16.0, 26.0, 46.0, 42.0));
    }

    #[test]
    fn matrix_multiplied_by_tuple() {
        let a = Matrix4::new(1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0);
        let b = Tuple::new(1.0, 2.0, 3.0, 1.0);

        assert_eq!(a * b, Tuple::new(18.0, 24.0, 33.0, 1.0));
    }

    #[test]
    fn multiplying_matrix_by_identity_matrix() {
        let a = Matrix4::new(0.0, 1.0, 2.0, 4.0, 1.0, 2.0, 4.0, 8.0, 2.0, 4.0, 8.0, 16.0, 4.0, 8.0, 16.0, 32.0);
        assert_eq!(a * Matrix4::IDENTITY, a);
    }

    #[test]
    fn multiplying_identity_matrix_by_tuple() {
        let a = Tuple::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(Matrix4::IDENTITY * a, a);
    }

    #[test]
    fn transposing_matrix() {
        let a = Matrix4::new(
            0.0, 9.0, 3.0, 0.0,
            9.0, 8.0, 0.0, 8.0,
            1.0, 8.0, 5.0, 3.0,
            0.0, 0.0, 5.0, 8.0
        );
        assert_eq!(a.transpose(), Matrix4::new(
                                    0.0, 9.0, 1.0, 0.0,
                                    9.0, 8.0, 8.0, 0.0,
                                    3.0, 0.0, 5.0, 5.0,
                                    0.0, 8.0, 3.0, 8.0));
    }

    #[test]
    fn transposing_identity_matrix() {
        let a = Matrix4::IDENTITY.transpose();
        assert_eq!(a, Matrix4::IDENTITY);
    }

    #[test]
    fn determinant_2x2_matrix() {
        let a = Matrix2::new(1.0, 5.0,
                                -3.0, 2.0);
        assert_eq!(a.determinant(), 17.0);
    }

    #[test]
    fn submatrix_of_3x3_matrix_is_2x2_matrix() {
        let a = Matrix3::new(
            1.0, 5.0, 0.0,
            -3.0, 2.0, 7.0,
            0.0, 6.0, -3.0
        );
        assert_eq!(a.sub_matrix(0, 2), Matrix2::new(
                                        -3.0, 2.0,
                                        0.0, 6.0));
    }

    #[test]
    fn submatrix_of_4x4_matrix_is_3x3_matrix() {
        let a = Matrix4::new(
            -6.0, 1.0, 1.0, 6.0,
            -8.0, 5.0, 8.0, 6.0,
            -1.0, 0.0, 8.0, 2.0,
            -7.0, 1.0, -1.0, 1.0
        );
        assert_eq!(a.sub_matrix(2, 1), Matrix3::new(
                                        -6.0, 1.0, 6.0,
                                        -8.0, 8.0, 6.0,
                                        -7.0, -1.0, 1.0));
    }

    #[test]
    fn minor_of_3x3_matrix() {
        let a = Matrix3::new(
            3.0, 5.0, 0.0,
            2.0, -1.0, -7.0,
            6.0, -1.0, 5.0,
        );
        let b = a.sub_matrix(1, 0);

        assert_eq!(b.determinant(), 25.0);
        assert_eq!(a.minor(1, 0), 25.0);
    }

    #[test]
    fn cofactor_of_3x3_matrix() {
        let a = Matrix3::new(
            3.0, 5.0, 0.0,
            2.0, -1.0, -7.0,
            6.0, -1.0, 5.0,
        );

        assert_eq!(a.minor(0, 0), -12.0);
        assert_eq!(a.cofactor(0, 0), -12.0);
        assert_eq!(a.minor(1, 0), 25.0);
        assert_eq!(a.cofactor(1, 0), -25.0);
    }

    #[test]
    fn determinant_of_3x3_matrix() {
        let a = Matrix3::new(
            1.0, 2.0, 6.0,
            -5.0, 8.0, -4.0,
            2.0, 6.0, 4.0
        );
        assert_eq!(a.cofactor(0, 0), 56.0);
        assert_eq!(a.cofactor(0, 1), 12.0);
        assert_eq!(a.cofactor(0, 2), -46.0);
        assert_eq!(a.determinant(), -196.0);
    }


    #[test]
    fn determinant_of_4x4_matrix() {
        let a = Matrix4::new(
            -2.0, -8.0, 3.0, 5.0,
            -3.0, 1.0, 7.0, 3.0,
            1.0, 2.0, -9.0, 6.0,
            -6.0, 7.0, 7.0, -9.0
        );
        assert_eq!(a.cofactor(0, 0), 690.0);
        assert_eq!(a.cofactor(0, 1), 447.0);
        assert_eq!(a.cofactor(0, 2), 210.0);
        assert_eq!(a.cofactor(0, 3), 51.0);
        assert_eq!(a.determinant(), -4071.0);
    }
}
