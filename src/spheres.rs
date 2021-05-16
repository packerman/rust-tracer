use crate::shapes::Shape;
use crate::materials::Material;
use crate::tuples::Vector;
use crate::tuples::Point;
use crate::intersections::Intersection;
use crate::tuples::Tuple;
use crate::rays::Ray;
use crate::transformations::Transformation;

#[derive(PartialEq, Debug)]
pub struct Sphere {
    transform: Transformation,
    inversed_transform: Transformation,
    pub material: Material,
}

impl Sphere {

    pub fn new() -> Sphere {
        Sphere {
            transform: Transformation::IDENTITY,
            inversed_transform: Transformation::IDENTITY,
            material: Material::new(),
        }
    }

    pub fn set_transform(&mut self, transform: Transformation) {
        self.transform = transform;
        self.inversed_transform = transform.inverse();
    }
}

impl Shape for Sphere {

    fn transform(&self) -> &Transformation {
        &self.transform
    }

    fn inversed_transform(&self) -> &Transformation {
        &self.inversed_transform
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    fn local_intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let sphere_to_ray = ray.origin - Tuple::point(0., 0., 0.);

        let a = ray.direction.dot(&ray.direction);
        let b = 2. * ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.;

        let discriminant = b * b - 4. * a * c;

        if discriminant < 0. {
            vec![]
        } else {
            let t1 = (- b - discriminant.sqrt()) / (2. * a);
            let t2 = (- b + discriminant.sqrt()) / (2. * a);

            vec![Intersection::new(t1, self),
                Intersection::new(t2, self)]
        }
     }

    fn local_normal_at(&self, point: &Point) -> Vector {
        *point - Tuple::point(0., 0., 0.)
    }
}

#[cfg(test)]
mod tests {

    use crate::tuples::Tuple;
    use super::*;
    use approx::assert_abs_diff_eq;
    use std::ptr;

    #[test]
    fn ray_intersects_sphere_at_two_point() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let s = Sphere::new();
        let xs = s.local_intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.);
        assert_eq!(xs[1].t, 6.);
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let r = Ray::new(Tuple::point(0., 1., -5.), Tuple::vector(0., 0., 1.));
        let s = Sphere::new();
        let xs = s.local_intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.);
        assert_eq!(xs[1].t, 5.);
    }

    #[test]
    fn ray_misses_a_sphere() {
        let r = Ray::new(Tuple::point(0., 2., -5.), Tuple::vector(0., 0., 1.));
        let s = Sphere::new();
        let xs = s.local_intersect(&r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let r = Ray::new(Tuple::point(0., 0., 0.), Tuple::vector(0., 0., 1.));
        let s = Sphere::new();
        let xs = s.local_intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.);
        assert_eq!(xs[1].t, 1.);
    }

    #[test]
    fn sphere_is_behind_a_ray() {
        let r = Ray::new(Tuple::point(0., 0., 5.), Tuple::vector(0., 0., 1.));
        let s = Sphere::new();
        let xs = s.local_intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.);
        assert_eq!(xs[1].t, -4.);
    }

    #[test]
    fn intersect_sets_the_object_on_the_intersection() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let s = Sphere::new();

        let xs = s.local_intersect(&r);

        assert_eq!(xs.len(), 2);
        assert!(ptr::eq(xs[0].object as *const dyn Shape as *const u8, &s as *const dyn Shape as *const u8));
        assert!(ptr::eq(xs[1].object as *const dyn Shape as *const u8, &s as *const dyn Shape as *const u8));
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
    fn the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let s = Sphere::new();

        let n = s.local_normal_at(&Tuple::point(1., 0., 0.));

        assert_eq!(n, Tuple::vector(1., 0., 0.));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_y_axis() {
        let s = Sphere::new();

        let n = s.local_normal_at(&Tuple::point(0., 1., 0.));

        assert_eq!(n, Tuple::vector(0., 1., 0.));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_z_axis() {
        let s = Sphere::new();

        let n = s.local_normal_at(&Tuple::point(0., 0., 1.));

        assert_eq!(n, Tuple::vector(0., 0., 1.));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_nonaxial_point() {
        let s = Sphere::new();

        let n = s.local_normal_at(&Tuple::point(3_f64.sqrt() / 3., 3_f64.sqrt() / 3., 3_f64.sqrt() / 3.));

        assert_abs_diff_eq!(n, Tuple::vector(3_f64.sqrt() / 3., 3_f64.sqrt() / 3., 3_f64.sqrt() / 3.));
    }

    #[test]
    fn the_normal_is_a_normalized_vector() {
        let s = Sphere::new();

        let n = s.local_normal_at(&Tuple::point(3_f64.sqrt() / 3., 3_f64.sqrt() / 3., 3_f64.sqrt() / 3.));

        assert_abs_diff_eq!(n, n.normalize());
    }

    #[test]
    fn a_sphere_has_a_default_material() {
        let s = Sphere::new();

        let m = s.material;

        assert_eq!(m, Material::new());
    }

    #[test]
    fn a_sphere_may_be_assigned_a_material() {
        let mut s = Sphere::new();
        let mut m = Material::new();
        m.ambient = 1.;

        s.material =m;

        assert_eq!(s.material, m);
    }
}
