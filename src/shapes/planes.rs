use crate::{
    rays::Ray,
    tuples::{Point, Scalar, Tuple, Vector},
};

use super::ShapeType;

#[derive(Debug)]
pub struct Plane;

impl ShapeType for Plane {
    fn local_intersect(&self, ray: &Ray) -> Vec<Scalar> {
        if ray.direction.y.abs() < f64::EPSILON {
            vec![]
        } else {
            vec![-ray.origin.y / ray.direction.y]
        }
    }

    fn local_normal_at(&self, _point: &Point) -> Vector {
        Tuple::vector(0., 1., 0.)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_normal_of_a_plane_is_constant_everywhere() {
        let p = Plane;

        let n1 = p.local_normal_at(&Tuple::point(0., 0., 0.));
        let n2 = p.local_normal_at(&Tuple::point(10., 0., -10.));
        let n3 = p.local_normal_at(&Tuple::point(-5., 0., 150.));

        assert_eq!(n1, Tuple::vector(0., 1., 0.));
        assert_eq!(n2, Tuple::vector(0., 1., 0.));
        assert_eq!(n3, Tuple::vector(0., 1., 0.));
    }

    #[test]
    fn intersect_with_a_ray_parallel_to_the_plane() {
        let p = Plane;
        let r = Ray::new(Tuple::point(0., 10., 0.), Tuple::vector(0., 0., 1.));

        let xs = p.local_intersect(&r);

        assert!(xs.is_empty());
    }

    #[test]
    fn intersect_with_a_coplanar_ray() {
        let p = Plane;
        let r = Ray::new(Tuple::point(0., 0., 0.), Tuple::vector(0., 0., 1.));

        let xs = p.local_intersect(&r);

        assert!(xs.is_empty());
    }

    #[test]
    fn a_ray_intersecting_with_a_plane_from_above() {
        let p = Plane;
        let r = Ray::new(Tuple::point(0., 1., 0.), Tuple::vector(0., -1., 0.));

        let xs = p.local_intersect(&r);

        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0], 1.);
    }

    #[test]
    fn a_ray_intersecting_with_a_plane_from_below() {
        let p = Plane;
        let r = Ray::new(Tuple::point(0., -1., 0.), Tuple::vector(0., 1., 0.));

        let xs = p.local_intersect(&r);

        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0], 1.);
    }
}
