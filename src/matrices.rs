use crate::tuples::{Scalar, Tuple};
use approx::AbsDiffEq;
use std::ops::{Index, Mul};

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Matrix4([[Scalar; 4]; 4]);

impl Matrix4 {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        m00: Scalar,
        m01: Scalar,
        m02: Scalar,
        m03: Scalar,
        m10: Scalar,
        m11: Scalar,
        m12: Scalar,
        m13: Scalar,
        m20: Scalar,
        m21: Scalar,
        m22: Scalar,
        m23: Scalar,
        m30: Scalar,
        m31: Scalar,
        m32: Scalar,
        m33: Scalar,
    ) -> Matrix4 {
        Matrix4([
            [m00, m01, m02, m03],
            [m10, m11, m12, m13],
            [m20, m21, m22, m23],
            [m30, m31, m32, m33],
        ])
    }

    pub const IDENTITY: Matrix4 = Matrix4::new(
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    );

    pub fn transpose(&self) -> Matrix4 {
        let mut result = [[0.0; 4]; 4];
        for (i, row) in result.iter_mut().enumerate() {
            for (j, item) in row.iter_mut().enumerate() {
                *item = self.0[j][i];
            }
        }
        Matrix4(result)
    }

    fn sub_matrix(&self, l: usize, k: usize) -> Matrix3 {
        let mut result = [[0.0; 3]; 3];
        for (i, row) in result.iter_mut().enumerate() {
            for (j, item) in row.iter_mut().enumerate() {
                *item = self.0[if i < l { i } else { i + 1 }][if j < k { j } else { j + 1 }]
            }
        }
        Matrix3(result)
    }

    fn minor(&self, l: usize, k: usize) -> Scalar {
        self.sub_matrix(l, k).determinant()
    }

    fn cofactor(&self, l: usize, k: usize) -> Scalar {
        (if (l + k) % 2 == 0 { 1.0 } else { -1.0 }) * self.minor(l, k)
    }

    fn determinant(&self) -> Scalar {
        self.0[0][0] * self.cofactor(0, 0)
            + self.0[0][1] * self.cofactor(0, 1)
            + self.0[0][2] * self.cofactor(0, 2)
            + self.0[0][3] * self.cofactor(0, 3)
    }

    pub fn is_invertible(&self) -> bool {
        self.determinant() != 0.
    }

    pub fn inverse(&self) -> Matrix4 {
        let det = self.determinant();
        let mut result = [[0.; 4]; 4];
        for (i, row) in result.iter_mut().enumerate() {
            for (j, item) in row.iter_mut().enumerate() {
                *item = self.cofactor(j, i) / det;
            }
        }
        Matrix4(result)
    }
}

impl Index<(usize, usize)> for Matrix4 {
    type Output = Scalar;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.0[index.0][index.1]
    }
}

impl Mul<Matrix4> for Matrix4 {
    type Output = Matrix4;

    fn mul(self, other: Matrix4) -> Self::Output {
        let mut result = [[0.0; 4]; 4];
        for (i, row) in result.iter_mut().enumerate() {
            for (j, item) in row.iter_mut().enumerate() {
                *item = self.0[i][0] * other.0[0][j]
                    + self.0[i][1] * other.0[1][j]
                    + self.0[i][2] * other.0[2][j]
                    + self.0[i][3] * other.0[3][j];
            }
        }
        Matrix4(result)
    }
}

impl Mul<Tuple> for Matrix4 {
    type Output = Tuple;

    fn mul(self, other: Tuple) -> Self::Output {
        Tuple::new(
            self.0[0][0] * other.x
                + self.0[0][1] * other.y
                + self.0[0][2] * other.z
                + self.0[0][3] * other.w,
            self.0[1][0] * other.x
                + self.0[1][1] * other.y
                + self.0[1][2] * other.z
                + self.0[1][3] * other.w,
            self.0[2][0] * other.x
                + self.0[2][1] * other.y
                + self.0[2][2] * other.z
                + self.0[2][3] * other.w,
            self.0[3][0] * other.x
                + self.0[3][1] * other.y
                + self.0[3][2] * other.z
                + self.0[3][3] * other.w,
        )
    }
}

impl AbsDiffEq for Matrix4 {
    type Epsilon = Scalar;

    fn default_epsilon() -> Self::Epsilon {
        Self::Epsilon::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        for i in 0..4 {
            for j in 0..4 {
                if !self.0[i][j].abs_diff_eq(&other.0[i][j], epsilon) {
                    return false;
                }
            }
        }
        true
    }
}

#[derive(PartialEq, Debug)]
pub struct Matrix2([[Scalar; 2]; 2]);

impl Matrix2 {
    pub fn new(m00: Scalar, m01: Scalar, m10: Scalar, m11: Scalar) -> Matrix2 {
        Matrix2([[m00, m01], [m10, m11]])
    }

    pub fn determinant(&self) -> Scalar {
        self.0[0][0] * self.0[1][1] - self.0[0][1] * self.0[1][0]
    }
}

impl Index<(usize, usize)> for Matrix2 {
    type Output = Scalar;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.0[index.0][index.1]
    }
}

#[derive(PartialEq, Debug)]
pub struct Matrix3([[Scalar; 3]; 3]);

impl Matrix3 {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        m00: Scalar,
        m01: Scalar,
        m02: Scalar,
        m10: Scalar,
        m11: Scalar,
        m12: Scalar,
        m20: Scalar,
        m21: Scalar,
        m22: Scalar,
    ) -> Matrix3 {
        Matrix3([[m00, m01, m02], [m10, m11, m12], [m20, m21, m22]])
    }

    fn sub_matrix(&self, l: usize, k: usize) -> Matrix2 {
        let mut result = [[0.0; 2]; 2];
        for (i, row) in result.iter_mut().enumerate() {
            for (j, item) in row.iter_mut().enumerate() {
                *item = self.0[if i < l { i } else { i + 1 }][if j < k { j } else { j + 1 }]
            }
        }
        Matrix2(result)
    }

    fn minor(&self, l: usize, k: usize) -> Scalar {
        self.sub_matrix(l, k).determinant()
    }

    fn cofactor(&self, l: usize, k: usize) -> Scalar {
        (if (l + k) % 2 == 0 { 1.0 } else { -1.0 }) * self.minor(l, k)
    }

    pub fn determinant(&self) -> Scalar {
        self.0[0][0] * self.cofactor(0, 0)
            + self.0[0][1] * self.cofactor(0, 1)
            + self.0[0][2] * self.cofactor(0, 2)
    }
}

impl Index<(usize, usize)> for Matrix3 {
    type Output = Scalar;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.0[index.0][index.1]
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tuples::Tuple;
    use approx::assert_abs_diff_eq;

    #[test]
    fn creating_4x4_matrix() {
        let m = Matrix4::new(
            1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.5, 15.5, 16.5,
        );
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
        let a = Matrix4::new(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        );
        let b = Matrix4::new(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        );
        assert_eq!(a, b);
    }

    #[test]
    fn matrix_equality_with_different_matrices() {
        let a = Matrix4::new(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        );
        let b = Matrix4::new(
            2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0,
        );
        assert_ne!(a, b);
    }

    #[test]
    fn multiplying_two_matrices() {
        let a = Matrix4::new(
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        );
        let b = Matrix4::new(
            -2.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, -1.0, 4.0, 3.0, 6.0, 5.0, 1.0, 2.0, 7.0, 8.0,
        );
        assert_eq!(
            a * b,
            Matrix4::new(
                20.0, 22.0, 50.0, 48.0, 44.0, 54.0, 114.0, 108.0, 40.0, 58.0, 110.0, 102.0, 16.0,
                26.0, 46.0, 42.0
            )
        );
    }

    #[test]
    fn matrix_multiplied_by_tuple() {
        let a = Matrix4::new(
            1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0,
        );
        let b = Tuple::new(1.0, 2.0, 3.0, 1.0);

        assert_eq!(a * b, Tuple::new(18.0, 24.0, 33.0, 1.0));
    }

    #[test]
    fn multiplying_matrix_by_identity_matrix() {
        let a = Matrix4::new(
            0.0, 1.0, 2.0, 4.0, 1.0, 2.0, 4.0, 8.0, 2.0, 4.0, 8.0, 16.0, 4.0, 8.0, 16.0, 32.0,
        );
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
            0.0, 9.0, 3.0, 0.0, 9.0, 8.0, 0.0, 8.0, 1.0, 8.0, 5.0, 3.0, 0.0, 0.0, 5.0, 8.0,
        );
        assert_eq!(
            a.transpose(),
            Matrix4::new(
                0.0, 9.0, 1.0, 0.0, 9.0, 8.0, 8.0, 0.0, 3.0, 0.0, 5.0, 5.0, 0.0, 8.0, 3.0, 8.0
            )
        );
    }

    #[test]
    fn transposing_identity_matrix() {
        let a = Matrix4::IDENTITY.transpose();
        assert_eq!(a, Matrix4::IDENTITY);
    }

    #[test]
    fn determinant_2x2_matrix() {
        let a = Matrix2::new(1.0, 5.0, -3.0, 2.0);
        assert_eq!(a.determinant(), 17.0);
    }

    #[test]
    fn submatrix_of_3x3_matrix_is_2x2_matrix() {
        let a = Matrix3::new(1.0, 5.0, 0.0, -3.0, 2.0, 7.0, 0.0, 6.0, -3.0);
        assert_eq!(a.sub_matrix(0, 2), Matrix2::new(-3.0, 2.0, 0.0, 6.0));
    }

    #[test]
    fn submatrix_of_4x4_matrix_is_3x3_matrix() {
        let a = Matrix4::new(
            -6.0, 1.0, 1.0, 6.0, -8.0, 5.0, 8.0, 6.0, -1.0, 0.0, 8.0, 2.0, -7.0, 1.0, -1.0, 1.0,
        );
        assert_eq!(
            a.sub_matrix(2, 1),
            Matrix3::new(-6.0, 1.0, 6.0, -8.0, 8.0, 6.0, -7.0, -1.0, 1.0)
        );
    }

    #[test]
    fn minor_of_3x3_matrix() {
        let a = Matrix3::new(3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0);
        let b = a.sub_matrix(1, 0);

        assert_eq!(b.determinant(), 25.0);
        assert_eq!(a.minor(1, 0), 25.0);
    }

    #[test]
    fn cofactor_of_3x3_matrix() {
        let a = Matrix3::new(3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0);

        assert_eq!(a.minor(0, 0), -12.0);
        assert_eq!(a.cofactor(0, 0), -12.0);
        assert_eq!(a.minor(1, 0), 25.0);
        assert_eq!(a.cofactor(1, 0), -25.0);
    }

    #[test]
    fn determinant_of_3x3_matrix() {
        let a = Matrix3::new(1.0, 2.0, 6.0, -5.0, 8.0, -4.0, 2.0, 6.0, 4.0);
        assert_eq!(a.cofactor(0, 0), 56.0);
        assert_eq!(a.cofactor(0, 1), 12.0);
        assert_eq!(a.cofactor(0, 2), -46.0);
        assert_eq!(a.determinant(), -196.0);
    }

    #[test]
    fn determinant_of_4x4_matrix() {
        let a = Matrix4::new(
            -2.0, -8.0, 3.0, 5.0, -3.0, 1.0, 7.0, 3.0, 1.0, 2.0, -9.0, 6.0, -6.0, 7.0, 7.0, -9.0,
        );
        assert_eq!(a.cofactor(0, 0), 690.0);
        assert_eq!(a.cofactor(0, 1), 447.0);
        assert_eq!(a.cofactor(0, 2), 210.0);
        assert_eq!(a.cofactor(0, 3), 51.0);
        assert_eq!(a.determinant(), -4071.0);
    }

    #[test]
    fn testing_inveritble_matrix_for_invertibility() {
        let a = Matrix4::new(
            6., 4., 4., 4., 5., 5., 7., 6., 4., -9., 3., -7., 9., 1., 7., -6.,
        );
        assert_eq!(a.determinant(), -2120.);
        assert!(a.is_invertible());
    }

    #[test]
    fn testing_non_inveritble_matrix_for_invertibility() {
        let a = Matrix4::new(
            -4., 2., -3., -3., 9., 6., 2., 6., 0., -5., 1., -5., 0., 0., 0., -0.,
        );
        assert_eq!(a.determinant(), -0.);
        assert!(!a.is_invertible());
    }

    #[test]
    fn inverse_of_matrix() {
        let a = Matrix4::new(
            -5., 2., 6., -8., 1., -5., 1., 8., 7., 7., -6., -7., 1., -3., 7., 4.,
        );
        let b = a.inverse();

        assert_eq!(a.determinant(), 532.);
        assert_eq!(a.cofactor(2, 3), -160.);
        assert_eq!(b[(3, 2)], -160. / 532.);
        assert_eq!(a.cofactor(3, 2), 105.);
        assert_eq!(b[(2, 3)], 105. / 532.);
        assert_abs_diff_eq!(
            b,
            Matrix4::new(
                0.21805, 0.45113, 0.24060, -0.04511, -0.80827, -1.45677, -0.44361, 0.52068,
                -0.07895, -0.22368, -0.05263, 0.19737, -0.52256, -0.81391, -0.30075, 0.30639
            ),
            epsilon = 0.00001
        );
    }

    #[test]
    fn inverse_of_matrix_2() {
        let a = Matrix4::new(
            8., -5., 9., 2., 7., 5., 6., 1., -6., 0., 9., 6., -3., 0., -9., -4.,
        );

        assert_abs_diff_eq!(
            a.inverse(),
            Matrix4::new(
                -0.15385, -0.15385, -0.28205, -0.53846, -0.07692, 0.12308, 0.02564, 0.03077,
                0.35897, 0.35897, 0.43590, 0.92308, -0.69231, -0.69231, -0.76923, -1.92308
            ),
            epsilon = 0.00001
        );
    }

    #[test]
    fn inverse_of_matrix_3() {
        let a = Matrix4::new(
            9., 3., 0., 9., -5., -2., -6., -3., -4., 9., 6., 4., -7., 6., 6., 2.,
        );

        assert_abs_diff_eq!(
            a.inverse(),
            Matrix4::new(
                -0.04074, -0.07778, 0.14444, -0.22222, -0.07778, 0.03333, 0.36667, -0.33333,
                -0.02901, -0.14630, -0.10926, 0.12963, 0.17778, 0.06667, -0.26667, 0.33333
            ),
            epsilon = 0.00001
        );
    }

    #[test]
    fn multiplying_product_by_its_inverse() {
        let a = Matrix4::new(
            3., -9., 7., 3., 3., -8., 2., -9., -4., 4., 4., 1., -6., 5., -1., 1.,
        );
        let b = Matrix4::new(
            8., 2., 2., 2., 3., -1., 7., 0., 7., 0., 5., 4., 6., -2., 0., 5.,
        );
        let c = a * b;

        assert_abs_diff_eq!(c * b.inverse(), a, epsilon = 0.000001);
    }
}
