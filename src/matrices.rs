
use std::ops::Index;

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
}
