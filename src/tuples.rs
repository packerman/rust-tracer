use approx::AbsDiffEq;
use std::ops;

#[derive(PartialEq, Debug)]
struct Tuple {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

type Point = Tuple;

impl Tuple {

    fn new(x: f32, y: f32, z: f32, w: f32) -> Tuple {
        Tuple {
            x, y, z, w,
        }
    }

    fn is_point(&self) -> bool {
        self.w == 1.0
    }

    fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    fn magnitude(&self) -> f32 {
        (self.x*self.x + self.y*self.y + self.z*self.z + self.w*self.w).sqrt()
    }

    fn normalize(&self) -> Tuple {
        let m = self.magnitude();
        Tuple::new(self.x / m, self.y / m, self.z / m, self.w / m)
    }
}

fn point(x: f32, y: f32, z: f32) -> Point {
    Tuple::new(x, y, z, 1.0)
}

fn vector(x: f32, y: f32, z: f32) -> Point {
    Tuple::new(x, y, z, 0.0)
}

impl ops::Add for Tuple {

    type Output = Self;

    fn add(self, _rhs: Self) -> Self {
        Tuple::new(self.x + _rhs.x, self.y + _rhs.y, self.z + _rhs.z, self.w + _rhs.w)
     }
}

impl ops::Sub for Tuple {

    type Output = Self;

    fn sub(self, _rhs: Self) -> Self {
        Tuple::new(self.x - _rhs.x, self.y - _rhs.y, self.z - _rhs.z, self.w - _rhs.w)
     }
}

impl ops::Neg for Tuple {

    type Output = Self;

    fn neg(self) -> Self {
        Tuple::new(-self.x, -self.y, -self.z, -self.w)
     }
}

impl ops::Mul<f32> for Tuple {

    type Output = Self;

    fn mul(self, factor: f32) -> Self {
        Tuple::new(self.x * factor, self.y * factor, self.z * factor, self.w * factor)
     }
}

impl ops::Div<f32> for Tuple {

    type Output = Self;

    fn div(self, factor: f32) -> Self {
        Tuple::new(self.x / factor, self.y / factor, self.z / factor, self.w / factor)
     }
}

impl AbsDiffEq for Tuple {

    type Epsilon = f32;

    fn default_epsilon() -> Self::Epsilon {
        f32::default_epsilon()
     }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.x.abs_diff_eq(&other.x, epsilon) &&
        self.y.abs_diff_eq(&other.y, epsilon) &&
        self.z.abs_diff_eq(&other.z, epsilon) &&
        self.w.abs_diff_eq(&other.w, epsilon)
     }
}

#[cfg(test)]
mod tests {

    use super::*;
    use approx::abs_diff_eq;

    #[test]
    fn tuple_with_w_equals_1_is_point() {
        let a = Tuple::new(4.3, -4.2, 3.1, 1.0);

        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 1.0);

        assert!(a.is_point());
        assert!(!a.is_vector())
    }

    #[test]
    fn tuple_with_w_equals_0_is_vector() {
        let a = Tuple::new(4.3, -4.2, 3.1, 0.0);

        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 0.0);

        assert!(!a.is_point());
        assert!(a.is_vector())
    }

    #[test]
    fn create_point_tuple() {
        let p = point(4.0, -4.0, 3.0);
        assert_eq!(p, Tuple::new(4.0, -4.0, 3.0, 1.0))
    }

    #[test]
    fn create_vector_tuple() {
        let p = vector(4.0, -4.0, 3.0);
        assert_eq!(p, Tuple::new(4.0, -4.0, 3.0, 0.0))
    }

    #[test]
    fn adding_tuples() {
        let a1 = Tuple::new(3.0, -2.0, 5.0, 1.0);
        let a2 = Tuple::new(-2.0, 3.0, 1.0, 0.0);

        assert_eq!(a1 + a2, Tuple::new(1.0, 1.0, 6.0, 1.0))
    }

    #[test]
    fn subtracting_two_points() {
        let p1 = point(3.0, 2.0, 1.0);
        let p2 = point(5.0, 6.0, 7.0);

        assert_eq!(p1 - p2, vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_vector_from_point() {
        let p = point(3.0, 2.0, 1.0);
        let v = vector(5.0, 6.0, 7.0);

        assert_eq!(p - v, point(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_two_vectors() {
        let v1 = vector(3.0, 2.0, 1.0);
        let v2 = vector(5.0, 6.0, 7.0);

        assert_eq!(v1 - v2, vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_vector_from_zero_vector() {
        let zero = vector(0.0, 0.0, 0.0);
        let v = vector(1.0, -2.0, 3.0);
        assert_eq!(zero - v, vector(-1.0, 2.0, -3.0));
    }

    #[test]
    fn negating_a_tuple() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(-a, Tuple::new(-1.0, 2.0, -3.0, 4.0))
    }

    #[test]
    fn multiplying_tuple_by_scalar() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a * 3.5, Tuple::new(3.5, -7.0, 10.5, -14.0));
    }

    #[test]
    fn multiplying_tuple_by_fraction() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a * 0.5, Tuple::new(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn dividing_tuple_by_scalar() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a / 2.0, Tuple::new(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn computing_magnitude_of_vector_1() {
        let v = vector(1.0, 0.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn computing_magnitude_of_vector_2() {
        let v = vector(0.0, 1.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn computing_magnitude_of_vector_3() {
        let v = vector(0.0, 0.0, 1.0);
        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn computing_magnitude_of_vector_4() {
        let v = vector(1.0, 2.0, 3.0);
        assert_eq!(v.magnitude(), 14.0_f32.sqrt());
    }

    #[test]
    fn computing_magnitude_of_vector_5() {
        let v = vector(-1.0, -2.0, -3.0);
        assert_eq!(v.magnitude(), 14.0_f32.sqrt());
    }

    #[test]
    fn normalizing_vector_1() {
        let v = vector(4.0, 0.0, 0.0);
        assert_eq!(v.normalize(), vector(1.0, 0.0, 0.0));
    }

    #[test]
    fn normalizing_vector_2() {
        let v = vector(1.0, 2.0, 3.0);
        abs_diff_eq!(v.normalize(), vector(0.26726, 0.53452, 0.80178));
    }

    #[test]
    fn magnitude_of_normalized_vector() {
        let v = vector(1.0, 2.0, 3.0);
        let norm = v.normalize();
        abs_diff_eq!(norm.magnitude(), 1.0);
    }
}
