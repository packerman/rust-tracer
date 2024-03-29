use approx::AbsDiffEq;
use std::{iter::Sum, ops, ops::AddAssign};

pub type Scalar = f64;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Tuple {
    pub x: Scalar,
    pub y: Scalar,
    pub z: Scalar,
    pub w: Scalar,
}

pub type Point = Tuple;
pub type Vector = Tuple;
pub type Color = Tuple;

impl Tuple {
    pub const fn new(x: Scalar, y: Scalar, z: Scalar, w: Scalar) -> Tuple {
        Tuple { x, y, z, w }
    }

    pub const fn point(x: Scalar, y: Scalar, z: Scalar) -> Point {
        Tuple::new(x, y, z, 1.0)
    }

    pub const fn vector(x: Scalar, y: Scalar, z: Scalar) -> Vector {
        Tuple::new(x, y, z, 0.0)
    }

    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    pub fn magnitude(&self) -> Scalar {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    pub fn normalize(&self) -> Tuple {
        let m = self.magnitude();
        Tuple::new(self.x / m, self.y / m, self.z / m, self.w / m)
    }

    pub fn dot(&self, other: &Tuple) -> Scalar {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn cross(&self, other: &Tuple) -> Vector {
        Self::vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn reflect(&self, normal: &Tuple) -> Tuple {
        *self - *normal * 2. * self.dot(normal)
    }

    pub const fn color(x: Scalar, y: Scalar, z: Scalar) -> Color {
        Tuple::new(x, y, z, 0.0)
    }

    pub fn red(&self) -> Scalar {
        self.x
    }

    pub fn green(&self) -> Scalar {
        self.y
    }

    pub fn blue(&self) -> Scalar {
        self.z
    }

    pub const BLACK: Color = Self::color(0., 0., 0.);
}

impl ops::Add<Tuple> for Tuple {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Tuple::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
            self.w + other.w,
        )
    }
}

impl ops::Sub for Tuple {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Tuple::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
            self.w - other.w,
        )
    }
}

impl ops::Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self {
        Tuple::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl ops::Mul<Scalar> for Tuple {
    type Output = Self;

    fn mul(self, factor: Scalar) -> Self {
        Tuple::new(
            self.x * factor,
            self.y * factor,
            self.z * factor,
            self.w * factor,
        )
    }
}

impl ops::Mul<Tuple> for Tuple {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Tuple::new(
            self.x * other.x,
            self.y * other.y,
            self.z * other.z,
            self.w * other.w,
        )
    }
}

impl ops::Div<Scalar> for Tuple {
    type Output = Self;

    fn div(self, factor: Scalar) -> Self {
        Tuple::new(
            self.x / factor,
            self.y / factor,
            self.z / factor,
            self.w / factor,
        )
    }
}

impl AbsDiffEq for Tuple {
    type Epsilon = Scalar;

    fn default_epsilon() -> Self::Epsilon {
        Self::Epsilon::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.x.abs_diff_eq(&other.x, epsilon)
            && self.y.abs_diff_eq(&other.y, epsilon)
            && self.z.abs_diff_eq(&other.z, epsilon)
            && self.w.abs_diff_eq(&other.w, epsilon)
    }
}

impl AddAssign for Tuple {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        };
    }
}

impl Sum for Tuple {
    fn sum<I: Iterator<Item = Tuple>>(iter: I) -> Self {
        let mut result = Tuple::new(0., 0., 0., 0.);
        for tuple in iter {
            result += tuple;
        }
        result
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use approx::assert_abs_diff_eq;
    use std::f64::consts::*;

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
        let p = Tuple::point(4.0, -4.0, 3.0);
        assert_eq!(p, Tuple::new(4.0, -4.0, 3.0, 1.0))
    }

    #[test]
    fn create_vector_tuple() {
        let p = Tuple::vector(4.0, -4.0, 3.0);
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
        let p1 = Tuple::point(3.0, 2.0, 1.0);
        let p2 = Tuple::point(5.0, 6.0, 7.0);

        assert_eq!(p1 - p2, Tuple::vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_vector_from_point() {
        let p = Tuple::point(3.0, 2.0, 1.0);
        let v = Tuple::vector(5.0, 6.0, 7.0);

        assert_eq!(p - v, Tuple::point(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_two_vectors() {
        let v1 = Tuple::vector(3.0, 2.0, 1.0);
        let v2 = Tuple::vector(5.0, 6.0, 7.0);

        assert_eq!(v1 - v2, Tuple::vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_vector_from_zero_vector() {
        let zero = Tuple::vector(0.0, 0.0, 0.0);
        let v = Tuple::vector(1.0, -2.0, 3.0);
        assert_eq!(zero - v, Tuple::vector(-1.0, 2.0, -3.0));
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
        let v = Tuple::vector(1.0, 0.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn computing_magnitude_of_vector_2() {
        let v = Tuple::vector(0.0, 1.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn computing_magnitude_of_vector_3() {
        let v = Tuple::vector(0.0, 0.0, 1.0);
        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn computing_magnitude_of_vector_4() {
        let v = Tuple::vector(1.0, 2.0, 3.0);
        assert_eq!(v.magnitude(), 14_f64.sqrt());
    }

    #[test]
    fn computing_magnitude_of_vector_5() {
        let v = Tuple::vector(-1.0, -2.0, -3.0);
        assert_eq!(v.magnitude(), 14_f64.sqrt());
    }

    #[test]
    fn normalizing_vector_1() {
        let v = Tuple::vector(4.0, 0.0, 0.0);
        assert_eq!(v.normalize(), Tuple::vector(1.0, 0.0, 0.0));
    }

    #[test]
    fn normalizing_vector_2() {
        let v = Tuple::vector(1.0, 2.0, 3.0);
        assert_abs_diff_eq!(
            v.normalize(),
            Tuple::vector(0.26726, 0.53452, 0.80178),
            epsilon = 0.00001
        );
    }

    #[test]
    fn magnitude_of_normalized_vector() {
        let v = Tuple::vector(1.0, 2.0, 3.0);
        let norm = v.normalize();
        assert_abs_diff_eq!(norm.magnitude(), 1.0);
    }

    #[test]
    fn dot_product_of_two_tuples() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);
        assert_eq!(a.dot(&b), 20.0);
    }

    #[test]
    fn cross_product_of_two_vectors() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);
        assert_eq!(a.cross(&b), Tuple::vector(-1.0, 2.0, -1.0));
        assert_eq!(b.cross(&a), Tuple::vector(1.0, -2.0, 1.0));
    }

    #[test]
    fn create_color() {
        let c = Tuple::color(-0.5, 0.4, 1.7);
        assert_eq!(c.red(), -0.5);
        assert_eq!(c.green(), 0.4);
        assert_eq!(c.blue(), 1.7);
    }

    #[test]
    fn adding_colors() {
        let c1 = Tuple::color(0.9, 0.6, 0.75);
        let c2 = Tuple::color(0.7, 0.1, 0.25);
        assert_abs_diff_eq!(c1 + c2, Tuple::color(1.6, 0.7, 1.0));
    }

    #[test]
    fn subtracting_colors() {
        let c1 = Tuple::color(0.9, 0.6, 0.75);
        let c2 = Tuple::color(0.7, 0.1, 0.25);
        assert_abs_diff_eq!(c1 - c2, Tuple::color(0.2, 0.5, 0.5));
    }

    #[test]
    fn multiplying_color_by_scalar() {
        let c = Tuple::color(0.2, 0.3, 0.4);
        assert_eq!(c * 2.0, Tuple::color(0.4, 0.6, 0.8));
    }

    #[test]
    fn multiplying_colors() {
        let c1 = Tuple::color(1.0, 0.2, 0.4);
        let c2 = Tuple::color(0.9, 1.0, 0.1);
        assert_abs_diff_eq!(c1 * c2, Tuple::color(0.9, 0.2, 0.04));
    }

    #[test]
    fn reflecting_a_vector_approaching_at_45_deg() {
        let v = Tuple::vector(1., -1., 0.);
        let n = Tuple::vector(0., 1., 0.);

        let r = v.reflect(&n);

        assert_eq!(r, Tuple::vector(1., 1., 0.));
    }

    #[test]
    fn reflecting_a_vector_of_a_slanted_surface() {
        let v = Tuple::vector(0., -1., 0.);
        let n = Tuple::vector(SQRT_2 / 2., SQRT_2 / 2., 0.);

        let r = v.reflect(&n);

        assert_abs_diff_eq!(r, Tuple::vector(1., 0., 0.));
    }
}
