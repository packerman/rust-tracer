use crate::tuples::Vector;
use crate::tuples::Point;
use crate::intersections::Intersection;
use crate::tuples::Tuple;
use crate::rays::Ray;
use crate::transformations::Transformation;

#[derive(Debug)]
pub struct Sphere {
    transform: Transformation,
    inversed_transform: Transformation,
}

impl Sphere {

    pub fn new() -> Sphere {
        Sphere {
            transform: Transformation::IDENTITY,
            inversed_transform: Transformation::IDENTITY,
        }
    }

    pub fn transform(&self) -> &Transformation {
        &self.transform
    }

    pub fn set_transform(&mut self, transform: Transformation) {
        self.transform = transform;
        self.inversed_transform = transform.inverse();
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let ray2 = ray.transform(&self.inversed_transform);

        let sphere_to_ray = ray2.origin() - Tuple::point(0., 0., 0.);

        let a = ray2.direction().dot(&ray2.direction());
        let b = 2. * ray2.direction().dot(&sphere_to_ray);
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

    pub fn normal_at(&self, p: Point) -> Vector {
        (p - Tuple::point(0., 0., 0.)).normalize()
    }
}

#[cfg(test)]
mod tests {

    use crate::tuples::Tuple;
    use super::*;
    use std::ptr;
    use approx::assert_abs_diff_eq;

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

    #[test]
    fn a_sphere_default_transformation() {
        let s = Sphere::new();

        assert_eq!(s.transform(), &Transformation::IDENTITY);
    }

    #[test]
    fn changing_a_sphere_transformation() {
        let mut s = Sphere::new();
        let t = Transformation::translation(2., 3., 4.);

        s.set_transform(t);
        assert_eq!(s.transform(), &t);
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let mut s = Sphere::new();

        s.set_transform(Transformation::scaling(2., 2., 2.));
        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t(), 3.);
        assert_eq!(xs[1].t(), 7.);
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let mut s = Sphere::new();

        s.set_transform(Transformation::translation(5., 0., 0.));
        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let s = Sphere::new();

        let n = s.normal_at(Tuple::point(1., 0., 0.));

        assert_eq!(n, Tuple::vector(1., 0., 0.));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_y_axis() {
        let s = Sphere::new();

        let n = s.normal_at(Tuple::point(0., 1., 0.));

        assert_eq!(n, Tuple::vector(0., 1., 0.));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_z_axis() {
        let s = Sphere::new();

        let n = s.normal_at(Tuple::point(0., 0., 1.));

        assert_eq!(n, Tuple::vector(0., 0., 1.));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_nonaxial_point() {
        let s = Sphere::new();

        let n = s.normal_at(Tuple::point(3_f32.sqrt() / 3., 3_f32.sqrt() / 3., 3_f32.sqrt() / 3.));

        assert_abs_diff_eq!(n, Tuple::vector(3_f32.sqrt() / 3., 3_f32.sqrt() / 3., 3_f32.sqrt() / 3.));
    }

    #[test]
    fn the_normal_is_a_normalized_vector() {
        let s = Sphere::new();

        let n = s.normal_at(Tuple::point(3_f32.sqrt() / 3., 3_f32.sqrt() / 3., 3_f32.sqrt() / 3.));

        assert_abs_diff_eq!(n, n.normalize());
    }
}
