use crate::rays::Ray;
use crate::shapes::ShapeType;
use crate::tuples::Point;
use crate::tuples::Scalar;
use crate::tuples::Tuple;
use crate::tuples::Vector;

#[derive(PartialEq, Debug)]
pub struct Sphere;

impl ShapeType for Sphere {
    fn local_intersect(&self, ray: &Ray) -> Vec<Scalar> {
        let sphere_to_ray = ray.origin - Tuple::point(0., 0., 0.);

        let a = ray.direction.dot(&ray.direction);
        let b = 2. * ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.;

        let discriminant = b * b - 4. * a * c;

        if discriminant < 0. {
            vec![]
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2. * a);
            let t2 = (-b + discriminant.sqrt()) / (2. * a);

            vec![t1, t2]
        }
    }

    fn local_normal_at(&self, point: &Point) -> Vector {
        *point - Tuple::point(0., 0., 0.)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tuples::Tuple;
    use approx::assert_abs_diff_eq;

    #[test]
    fn ray_intersects_sphere_at_two_point() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let s = Sphere;
        let xs = s.local_intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], 4.);
        assert_eq!(xs[1], 6.);
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let r = Ray::new(Tuple::point(0., 1., -5.), Tuple::vector(0., 0., 1.));
        let s = Sphere;
        let xs = s.local_intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], 5.);
        assert_eq!(xs[1], 5.);
    }

    #[test]
    fn ray_misses_a_sphere() {
        let r = Ray::new(Tuple::point(0., 2., -5.), Tuple::vector(0., 0., 1.));
        let s = Sphere;
        let xs = s.local_intersect(&r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let r = Ray::new(Tuple::point(0., 0., 0.), Tuple::vector(0., 0., 1.));
        let s = Sphere;
        let xs = s.local_intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], -1.);
        assert_eq!(xs[1], 1.);
    }

    #[test]
    fn sphere_is_behind_a_ray() {
        let r = Ray::new(Tuple::point(0., 0., 5.), Tuple::vector(0., 0., 1.));
        let s = Sphere;
        let xs = s.local_intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], -6.);
        assert_eq!(xs[1], -4.);
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let s = Sphere;

        let n = s.local_normal_at(&Tuple::point(1., 0., 0.));

        assert_eq!(n, Tuple::vector(1., 0., 0.));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_y_axis() {
        let s = Sphere;

        let n = s.local_normal_at(&Tuple::point(0., 1., 0.));

        assert_eq!(n, Tuple::vector(0., 1., 0.));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_z_axis() {
        let s = Sphere;

        let n = s.local_normal_at(&Tuple::point(0., 0., 1.));

        assert_eq!(n, Tuple::vector(0., 0., 1.));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_nonaxial_point() {
        let s = Sphere;

        let n = s.local_normal_at(&Tuple::point(
            3_f64.sqrt() / 3.,
            3_f64.sqrt() / 3.,
            3_f64.sqrt() / 3.,
        ));

        assert_abs_diff_eq!(
            n,
            Tuple::vector(3_f64.sqrt() / 3., 3_f64.sqrt() / 3., 3_f64.sqrt() / 3.)
        );
    }

    #[test]
    fn the_normal_is_a_normalized_vector() {
        let s = Sphere;

        let n = s.local_normal_at(&Tuple::point(
            3_f64.sqrt() / 3.,
            3_f64.sqrt() / 3.,
            3_f64.sqrt() / 3.,
        ));

        assert_abs_diff_eq!(n, n.normalize());
    }
}
