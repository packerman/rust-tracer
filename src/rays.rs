use crate::tuples::Vector;
use crate::tuples::Point;

struct Ray {
    origin: Point,
    direction: Vector,
}

impl Ray {

    pub fn new(origin: Point, direction: Vector) -> Ray {
        Ray {
            origin,
            direction,
        }
    }

    pub fn position(&self, t: f32) -> Point {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tuples::Tuple;

#[test]
fn creating_ray() {
    let origin = Tuple::point(1., 2., 3.);
    let direction = Tuple::vector(4., 5., 6.);

    let r = Ray::new(origin, direction);

    assert_eq!(r.origin, origin);
    assert_eq!(r.direction, direction);
}

#[test]
fn computing_a_point_from_distance() {
    let r = Ray::new(Tuple::point(2., 3., 4.), Tuple::vector(1., 0., 0.));

    assert_eq!(r.position(0.), Tuple::point(2., 3., 4.));
    assert_eq!(r.position(1.), Tuple::point(3., 3., 4.));
    assert_eq!(r.position(-1.), Tuple::point(1., 3., 4.));
    assert_eq!(r.position(2.5), Tuple::point(4.5, 3., 4.));
}


}
