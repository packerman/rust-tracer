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

#[cfg(test)]
mod tests {

    use super::*;

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
}
