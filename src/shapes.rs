use crate::intersections::Intersection;
use crate::materials::Material;
use crate::rays::Ray;
use crate::spheres::Sphere;
use crate::transformations::Transformation;
use crate::tuples::Point;
use crate::tuples::Scalar;
use crate::tuples::Vector;

pub trait ShapeType {
    fn local_intersect(&self, ray: &Ray) -> Vec<Scalar>;

    fn local_normal_at(&self, point: &Point) -> Vector;
}

pub struct Shape {
    transform: Transformation,
    inversed_transform: Transformation,
    pub material: Material,
    shape_type: Box<dyn ShapeType>,
}

impl Shape {
    pub fn sphere() -> Shape {
        Self::new(Box::new(Sphere))
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

    struct TestShape;

    impl ShapeType for TestShape {
        fn local_intersect(&self, ray: &Ray) -> Vec<Scalar> {
            let origin = ray.origin;
            let direction = ray.direction;
            vec![
                origin.x,
                origin.y,
                origin.z,
                direction.x,
                direction.y,
                direction.z,
            ]
        }
        fn local_normal_at(&self, point: &Point) -> Vector {
            Tuple::vector(point.x, point.y, point.z)
        }
    }

    pub fn test_shape() -> Shape {
        Shape::new(Box::new(TestShape))
    }

    #[test]
    fn a_shape_default_transformation() {
        let s = test_shape();

        assert_eq!(s.transform(), &Transformation::IDENTITY);
    }

    #[test]
    fn changing_a_shape_transformation() {
        let mut s = test_shape();
        let t = Transformation::translation(2., 3., 4.);

        s.set_transform(t);
        assert_eq!(s.transform(), &t);
    }

    #[test]
    fn a_shape_has_a_default_material() {
        let s = test_shape();

        let m = s.material;

        assert_eq!(m, Material::new());
    }

    #[test]
    fn a_sphere_may_be_assigned_a_material() {
        let mut s = test_shape();
        let mut m = Material::new();
        m.ambient = 1.;

        s.material = m;

        assert_eq!(s.material, m);
    }

    #[test]
    fn intersect_sets_the_object_on_the_intersection() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let s = test_shape();

        let xs = s.intersect(&r);

        assert!(xs.len() > 0);
        for x in xs.iter() {
            assert!(ptr::eq(xs[0].object, &s));
        }
    }

    #[test]
    fn intersecting_a_scaled_shape_with_a_ray() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let mut s = test_shape();

        s.set_transform(Transformation::scaling(2., 2., 2.));
        let xs = s.intersect(&r);

        let saved_ray = Ray::new(
            Tuple::point(xs[0].t, xs[1].t, xs[2].t),
            Tuple::vector(xs[3].t, xs[4].t, xs[5].t),
        );
        assert_eq!(saved_ray.origin, Tuple::point(0., 0., -2.5));
        assert_eq!(saved_ray.direction, Tuple::vector(0., 0., 0.5));
    }

    #[test]
    fn intersecting_a_translated_shape_with_a_ray() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let mut s = test_shape();

        s.set_transform(Transformation::translation(5., 0., 0.));
        let xs = s.intersect(&r);

        let saved_ray = Ray::new(
            Tuple::point(xs[0].t, xs[1].t, xs[2].t),
            Tuple::vector(xs[3].t, xs[4].t, xs[5].t),
        );
        assert_eq!(saved_ray.origin, Tuple::point(-5., 0., -5.));
        assert_eq!(saved_ray.direction, Tuple::vector(0., 0., 1.));
    }

    #[test]
    fn computing_normal_on_a_translated_shape() {
        let mut s = test_shape();
        s.set_transform(Transformation::translation(0., 1., 0.));

        let n = s.normal_at(&Tuple::point(0., 1.70711, -0.70711));

        assert_abs_diff_eq!(n, Tuple::vector(0., 0.70711, -0.70711), epsilon = 0.00001);
    }

    #[test]
    fn computing_normal_on_a_transformed_shape() {
        let mut s = test_shape();
        let m = Transformation::scaling(1., 0.5, 1.) * Transformation::rotation_z(PI / 5.);
        s.set_transform(m);

        let n = s.normal_at(&Tuple::point(0., SQRT_2 / 2., -SQRT_2 / 2.));

        assert_abs_diff_eq!(n, Tuple::vector(0., 0.97014, -0.24254), epsilon = 0.00001);
    }
}
