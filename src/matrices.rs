
use crate::tuples::Tuple;
use std::ops::Mul;
use std::ops::Index;

#[derive(PartialEq, Debug)]
struct Matrix4([[f32;4];4]);

impl Matrix4 {

    fn new(m00: f32, m01: f32, m02: f32, m03: f32,
        m10: f32, m11: f32, m12: f32, m13: f32,
        m20: f32, m21: f32, m22: f32, m23: f32,
        m30: f32, m31: f32, m32: f32, m33: f32) -> Matrix4 {
            Matrix4 {
                0: [[m00, m01, m02, m03], [m10, m11, m12, m13], [m20, m21, m22, m23], [m30, m31, m32, m33]],
            }
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

struct Matrix2([[f32;2];2]);

impl Matrix2 {

    fn new(m00: f32, m01: f32, m10: f32, m11: f32) -> Matrix2 {
        Matrix2 {
            0: [[m00, m01], [m10, m11]],
        }
    }
}

impl Index<(usize, usize)> for Matrix2 {

    type Output = f32;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.0[index.0][index.1]
     }
}

struct Matrix3([[f32;3];3]);

impl Matrix3 {

    fn new(m00: f32, m01: f32, m02: f32,
        m10: f32, m11: f32, m12: f32,
        m20: f32, m21: f32, m22: f32) -> Matrix3 {
            Matrix3 {
                0: [[m00, m01, m02], [m10, m11, m12], [m20, m21, m22]],
            }
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
}
