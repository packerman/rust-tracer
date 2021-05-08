use crate::rays::Ray;
use crate::tuples::Vector;
use crate::tuples::Point;
use std::cmp::Ordering;
use std::ptr;
use crate::spheres::Sphere;

#[derive(Debug, Copy, Clone)]
pub struct Intersection<'a> {
    pub t: f32,
    pub object: &'a Sphere,
}

impl PartialEq for Intersection<'_> {

    fn eq(&self, other: &Intersection<'_>) -> bool {
        self.t == other.t && ptr::eq(self.object, other.object)
    }
}

impl Intersection<'_> {

    pub fn new(t: f32, object: &Sphere) -> Intersection {
        Intersection { t, object }
    }
}

impl PartialOrd for Intersection<'_> {

    fn partial_cmp(&self, other: &Intersection) -> Option<Ordering> {
        self.t.partial_cmp(&other.t)
     }
}

pub fn intersections<'a>(mut instersections: Vec<Intersection>) -> Vec<Intersection> {
    instersections.sort_by(|a, b| a.partial_cmp(b).unwrap());
    instersections
}

pub fn hit<'a>(intersections: &'a [Intersection<'a>]) -> Option<&'a Intersection<'a>> {
    intersections.iter().find(|i| i.t > 0.)
}

pub struct Computations<'a> {
    t: f32,
    pub object: &'a Sphere,
    pub point: Point,
    pub eyev: Vector,
    pub normalv: Vector,
    inside: bool,
}

impl Computations<'_> {

    pub fn prepare<'a>(intersection: &Intersection<'a>, ray: &Ray) -> Computations<'a> {
        let point = ray.position(intersection.t);
        let mut normalv = intersection.object.normal_at(&point);
        let eyev = - ray.direction;
        let inside: bool;
        if normalv.dot(&eyev) < 0. {
            inside = true;
            normalv = - normalv;
        } else {
            inside = false;
        }
        Computations {
            t: intersection.t,
            object: intersection.object,
            point,
            eyev,
            normalv,
            inside,
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::rays::Ray;
    use crate::tuples::Tuple;
    use super::*;
    use std::ptr;

    #[test]
    fn creating_intersection() {
        let s = Sphere::new();
        let i = Intersection::new(3.5, &s);

        assert_eq!(i.t, 3.5);
        assert!(ptr::eq(i.object, &s));
    }

    #[test]
    fn aggregating_intersections() {
        let s = Sphere::new();
        let i1 = Intersection::new(1., &s);
        let i2 = Intersection::new(2., &s);

        let xs = intersections(vec![i1, i2]);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 1.);
        assert_eq!(xs[1].t, 2.);
    }

    #[test]
    fn the_hit_when_all_intersections_have_positive_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(1., &s);
        let i2 = Intersection::new(2., &s);
        let xs = intersections(vec![i2, i1]);

        let i = hit(&xs).unwrap();

        assert_eq!(i, &i1);
    }

    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-1., &s);
        let i2 = Intersection::new(1., &s);
        let xs = intersections(vec![i2, i1]);

        let i = hit(&xs).unwrap();

        assert_eq!(i, &i2);
    }

    #[test]
    fn the_hit_when_all_intersections_have_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-2., &s);
        let i2 = Intersection::new(-1., &s);
        let xs = intersections(vec![i2, i1]);

        let i = hit(&xs);

        assert_eq!(i, None);
    }

    #[test]
    fn the_hit_is_always_the_lowest_nonnegative_intersection() {
        let s = Sphere::new();
        let i1 = Intersection::new(5., &s);
        let i2 = Intersection::new(7., &s);
        let i3 = Intersection::new(-3., &s);
        let i4 = Intersection::new(2., &s);
        let xs = intersections(vec![i1, i2, i3, i4]);

        let i = hit(&xs).unwrap();

        assert_eq!(i, &i4);
    }

    #[test]
    fn precomputing_the_state_of_an_intersection() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let shape = Sphere::new();
        let i = Intersection::new(4., &shape);

        let comps = Computations::prepare(&i, &r);

        assert_eq!(comps.t, i.t);
        assert_eq!(comps.object, i.object);
        assert_eq!(comps.point, Tuple::point(0., 0., -1.));
        assert_eq!(comps.eyev, Tuple::vector(0., 0., -1.));
        assert_eq!(comps.normalv, Tuple::vector(0., 0., -1.));
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_outside() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let shape = Sphere::new();
        let i = Intersection::new(4., &shape);

        let comps = Computations::prepare(&i, &r);

        assert!(!comps.inside);
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_inside() {
        let r = Ray::new(Tuple::point(0., 0., 0.), Tuple::vector(0., 0., 1.));
        let shape = Sphere::new();
        let i = Intersection::new(1., &shape);

        let comps = Computations::prepare(&i, &r);

        assert_eq!(comps.point, Tuple::point(0., 0., 1.));
        assert_eq!(comps.eyev, Tuple::vector(0., 0., -1.));
        assert!(comps.inside);
        assert_eq!(comps.normalv, Tuple::vector(0., 0., -1.));
    }
}
