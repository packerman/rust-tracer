use crate::shapes::ShapeProperties;
use crate::tuples::Tuple;
use crate::tuples::Vector;
use crate::tuples::Point;
use crate::rays::Ray;
use crate::intersections::Intersection;
use crate::shapes::Shape;
use crate::tuples::Scalar;

#[derive(Debug)]
pub struct Plane(ShapeProperties);

impl Plane {

    pub fn new() -> Plane {
        Plane {
            0: ShapeProperties::new(),
        }
    }
}

impl Shape for Plane {

    fn properties(&self) -> &ShapeProperties {
        &self.0
    }

    fn properties_mut(&mut self) -> &mut ShapeProperties {
        &mut self.0
    }

    fn local_intersect(&self, ray: &Ray) -> Vec<Intersection> {
        if ray.direction.y.abs() < Scalar::EPSILON {
            vec![]
        } else {
            let t = - ray.origin.y / ray.direction.y;
            vec![Intersection::new(t, self)]
        }
    }

    fn local_normal_at(&self, _point: &Point) -> Vector {
        Tuple::vector(0., 1., 0.)
    }
}

#[cfg(test)]
mod tests {

    use crate::tuples::Tuple;
    use super::*;

    #[test]
    fn the_normal_of_a_plane_is_constant_everywhere() {
        let p = Plane::new();

        let n1 = p.local_normal_at(&Tuple::point(0., 0., 0.));
        let n2 = p.local_normal_at(&Tuple::point(10., 0., -10.));
        let n3 = p.local_normal_at(&Tuple::point(-5., 0., 150.));

        assert_eq!(n1, Tuple::vector(0., 1., 0.));
        assert_eq!(n2, Tuple::vector(0., 1., 0.));
        assert_eq!(n3, Tuple::vector(0., 1., 0.));
    }

    #[test]
    fn intersect_with_a_ray_parallel_to_the_plane() {
        let p = Plane::new();
        let r = Ray::new(Tuple::point(0., 10., 0.), Tuple::vector(0., 0., 1.));

        let xs = p.local_intersect(&r);

        assert!(xs.is_empty())
    }

    #[test]
    fn intersect_with_a_coplanar_ray() {
        let p = Plane::new();
        let r = Ray::new(Tuple::point(0., 0., 0.), Tuple::vector(0., 0., 1.));

        let xs = p.local_intersect(&r);

        assert!(xs.is_empty())
    }

    #[test]
    fn a_ray_intersect_a_plane_from_above() {
        let p = Plane::new();
        let r = Ray::new(Tuple::point(0., 1., 0.), Tuple::vector(0., -1., 0.));

        let xs = p.local_intersect(&r);

        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.);
        assert_eq!(xs[0].object as *const dyn Shape as *const u8, &p as *const dyn Shape as *const u8);
    }

    #[test]
    fn a_ray_intersect_a_plane_from_below() {
        let p = Plane::new();
        let r = Ray::new(Tuple::point(0., -1., 0.), Tuple::vector(0., 1., 0.));

        let xs = p.local_intersect(&r);

        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.);
        assert_eq!(xs[0].object as *const dyn Shape as *const u8, &p as *const dyn Shape as *const u8);
    }
}
