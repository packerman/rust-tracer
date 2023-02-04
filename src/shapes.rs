use crate::intersections::Intersection;
use crate::materials::Material;
use crate::rays::Ray;
use crate::transformations::Transformation;
use crate::tuples::Point;
use crate::tuples::Scalar;
use crate::tuples::Tuple;
use crate::tuples::Vector;
use std::fmt::Debug;

pub trait ShapeType: Debug {
    fn local_intersect(&self, ray: &Ray) -> Vec<Scalar>;
    fn local_normal_at(&self, point: &Point) -> Vector;
}

#[derive(Debug)]
struct Sphere;

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

#[derive(Debug)]
struct Plane;

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

#[derive(Debug)]
pub struct Shape {
    transform: Transformation,
    inversed_transform: Transformation,
    pub material: Material,
    shape_type: Box<dyn ShapeType>,
}

// TODO
// impl PartialEq for Shape {
//     fn eq(&self, other: &Self) -> bool {
//         self.transform == other.transform
//             && self.material == other.material
//             && Rc::ptr_eq(&self.shape_type, &other.shape_type)
//     }
// }

impl Shape {
    pub fn sphere() -> Shape {
        Self::new(Box::new(Sphere))
    }

    pub fn plane() -> Shape {
        Self::new(Box::new(Plane))
    }

    fn new(shape_type: Box<dyn ShapeType>) -> Shape {
        Shape {
            transform: Transformation::IDENTITY,
            inversed_transform: Transformation::IDENTITY,
            material: Material::new(),
            shape_type,
        }
    }

    pub fn transform(&self) -> &Transformation {
        &self.transform
    }

    pub fn inversed_transform(&self) -> &Transformation {
        &self.inversed_transform
    }

    pub fn set_transform(&mut self, transform: Transformation) {
        self.transform = transform;
        self.inversed_transform = transform.inverse();
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let local_ray = ray.transform(&self.inversed_transform);
        self.shape_type
            .local_intersect(&local_ray)
            .iter()
            .map(|t| Intersection::new(*t, self))
            .collect()
    }

    pub fn normal_at(&self, world_point: &Point) -> Vector {
        let object_point = self.inversed_transform * *world_point;
        let object_normal = self.shape_type.local_normal_at(&object_point);
        let mut world_normal = self.inversed_transform.transpose() * object_normal;
        world_normal.w = 0.;
        world_normal.normalize()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tuples::Tuple;
    use approx::assert_abs_diff_eq;
    use std::f64::consts::PI;
    use std::f64::consts::SQRT_2;
    use std::ptr;

    #[test]
    fn a_shape_default_transformation() {
        let s = Shape::sphere();

        assert_eq!(s.transform(), &Transformation::IDENTITY);
    }

    #[test]
    fn changing_a_shape_transformation() {
        let mut s = Shape::sphere();
        let t = Transformation::translation(2., 3., 4.);

        s.set_transform(t);
        assert_eq!(s.transform(), &t);
    }

    #[test]
    fn a_shape_has_a_default_material() {
        let s = Shape::sphere();

        let m = s.material;

        assert_eq!(m, Material::new());
    }

    #[test]
    fn a_sphere_may_be_assigned_a_material() {
        let mut s = Shape::sphere();
        let mut m = Material::new();
        m.ambient = 1.;

        s.material = m;

        assert_eq!(s.material, m);
    }

    #[test]
    fn intersect_sets_the_object_on_the_intersection() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let s = Shape::sphere();

        let xs = s.intersect(&r);

        assert!(xs.len() > 0);
        for x in xs.iter() {
            assert!(ptr::eq(x.object, &s));
        }
    }

    #[test]
    fn intersecting_a_scaled_shape_with_a_ray() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let mut s = Shape::sphere();

        s.set_transform(Transformation::scaling(2., 2., 2.));
        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.);
        assert_eq!(xs[1].t, 7.);
    }

    #[test]
    fn intersecting_a_translated_shape_with_a_ray() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let mut s = Shape::sphere();

        s.set_transform(Transformation::translation(5., 0., 0.));
        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn computing_normal_on_a_translated_shape() {
        let mut s = Shape::sphere();
        s.set_transform(Transformation::translation(0., 1., 0.));

        let n = s.normal_at(&Tuple::point(0., 1.70711, -0.70711));

        assert_abs_diff_eq!(n, Tuple::vector(0., 0.70711, -0.70711), epsilon = 0.00001);
    }

    #[test]
    fn computing_normal_on_a_transformed_shape() {
        let mut s = Shape::sphere();
        let m = Transformation::scaling(1., 0.5, 1.) * Transformation::rotation_z(PI / 5.);
        s.set_transform(m);

        let n = s.normal_at(&Tuple::point(0., SQRT_2 / 2., -SQRT_2 / 2.));

        assert_abs_diff_eq!(n, Tuple::vector(0., 0.97014, -0.24254), epsilon = 0.00001);
    }

    mod spheres {

        use super::*;

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

    mod planes {

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
}
