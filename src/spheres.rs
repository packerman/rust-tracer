use crate::intersections::Intersection;
use crate::tuples::Tuple;
use crate::rays::Ray;

#[derive(Debug)]
pub struct Sphere {}

impl Sphere {

    pub fn new() -> Sphere {
        Sphere { }
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let sphere_to_ray = ray.origin() - Tuple::point(0., 0., 0.);

        let a = ray.direction().dot(&ray.direction());
        let b = 2. * ray.direction().dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.;

        let discriminant = b * b - 4. * a * c;

        if discriminant < 0. {
            vec![]
        } else {
            let t1 = (- b - discriminant.sqrt()) / (2. * a);
            let t2 = (- b + discriminant.sqrt()) / (2. * a);

            vec![Intersection::new(t1, &self),
                Intersection::new(t2, &self)]
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::tuples::Tuple;
    use super::*;
    use std::ptr;

    #[test]
    fn ray_intersects_sphere_at_two_point() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let s = Sphere::new();
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t(), 4.);
        assert_eq!(xs[1].t(), 6.);
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let r = Ray::new(Tuple::point(0., 1., -5.), Tuple::vector(0., 0., 1.));
        let s = Sphere::new();
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t(), 5.);
        assert_eq!(xs[1].t(), 5.);
    }

    #[test]
    fn ray_misses_a_sphere() {
        let r = Ray::new(Tuple::point(0., 2., -5.), Tuple::vector(0., 0., 1.));
        let s = Sphere::new();
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let r = Ray::new(Tuple::point(0., 0., 0.), Tuple::vector(0., 0., 1.));
        let s = Sphere::new();
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t(), -1.);
        assert_eq!(xs[1].t(), 1.);
    }

    #[test]
    fn sphere_is_behind_a_ray() {
        let r = Ray::new(Tuple::point(0., 0., 5.), Tuple::vector(0., 0., 1.));
        let s = Sphere::new();
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t(), -6.);
        assert_eq!(xs[1].t(), -4.);
    }

    #[test]
    fn intersect_sets_the_object_on_the_intersection() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let s = Sphere::new();

        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        assert!(ptr::eq(xs[0].object(), &s));
        assert!(ptr::eq(xs[1].object(), &s));
    }
}
