
use crate::tuples::Vector;
use crate::tuples::Point;
use std::fmt::Debug;
use crate::intersections::Intersection;
use crate::rays::Ray;
use crate::materials::Material;
use crate::transformations::Transformation;

#[derive(Debug)]
pub struct BaseShape {
    transform: Transformation,
    inverted_transform: Transformation,
    pub material: Material,
}

impl BaseShape {

    pub fn new() -> BaseShape {
        BaseShape {
            transform: Transformation::IDENTITY,
            inverted_transform: Transformation::IDENTITY,
            material: Material::new(),
        }
    }

    pub fn transform(&self) -> &Transformation {
        &self.transform
    }

    pub fn inversed_transform(&self) -> &Transformation {
        &self.inverted_transform
    }

    pub fn set_transform(&mut self, transform: Transformation) {
        self.transform = transform;
        self.inverted_transform = transform.inverse();
    }
}

pub trait Shape: Debug {

    fn base(&self) -> &BaseShape;

    fn base_mut(&mut self) -> &mut BaseShape;

    fn local_intersect(&self, ray: &Ray) -> Vec<Intersection>;

    fn local_normal_at(&self, world_point: &Point) -> Vector;
}

impl<'a> dyn Shape + 'a {

    pub fn transform(&self) -> &Transformation {
        self.base().transform()
    }

    pub fn inversed_transform(&self) -> &Transformation {
        self.base().inversed_transform()
    }

    pub fn set_transform(&mut self, transform: Transformation) {
        self.base_mut().set_transform(transform);
    }

    pub fn material(&self) -> &Material {
        &self.base().material
    }

    pub fn set_material(&mut self, material: Material) {
        self.base_mut().material = material;
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let local_ray = ray.transform(self.inversed_transform());
        self.local_intersect(&local_ray)
    }

    pub fn normal_at(&self, point: &Point) -> Vector {
        let local_point = *self.inversed_transform() * *point;
        let local_normal = self.local_normal_at(&local_point);
        let mut world_normal = self.inversed_transform().transpose() * local_normal;
        world_normal.w = 0.;

        world_normal.normalize()
    }
}

#[cfg(test)]
mod tests {

    use std::f64::consts::*;
    use crate::tuples::Tuple;
    use super::*;
    use approx::assert_abs_diff_eq;

    #[derive(Debug)]
    struct TestShape(BaseShape);

    impl TestShape {

        fn new() -> TestShape {
            TestShape {
                0: BaseShape::new()
            }
        }
    }

    impl Shape for TestShape {

        fn base(&self) -> &BaseShape {
            &self.0
        }

        fn base_mut(&mut self) -> &mut BaseShape {
            &mut self.0
        }

        fn local_intersect(&self, ray: &Ray) -> Vec<Intersection> {
            vec![Intersection::new(1., self)]
        }

        fn local_normal_at(&self, point: &Point) -> Vector {
            Tuple::vector(point.x, point.y, point.z)
        }
    }

    mod basic_tests {

        use super::*;

        #[test]
        fn the_default_transform() {
            let s = TestShape::new();

            assert_eq!((&s as &dyn Shape).transform(), &Transformation::IDENTITY);
        }

        #[test]
        fn assigning_a_transformation() {
            let mut s = TestShape::new();

            (&mut s as &mut dyn Shape).set_transform(Transformation::translation(2., 3., 4.));

            assert_eq!((&s as &dyn Shape).transform(), &Transformation::translation(2., 3., 4.));
        }

        #[test]
        fn the_default_material() {
            let s = TestShape::new();

            assert_eq!((&s as &dyn Shape).material(), &Material::new());
        }

        #[test]
        fn assigning_a_material() {
            let mut s = TestShape::new();
            let mut m = Material::new();
            m.ambient = 1.;

            (&mut s as &mut dyn Shape).set_material(m);

            assert_eq!((&s as &dyn Shape).material(), &m);
        }
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let mut s = TestShape::new();

        (&mut s as &mut dyn Shape).set_transform(Transformation::scaling(2., 2., 2.));
        let xs = (&s as &dyn Shape).intersect(&r);

        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.);
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let mut s = TestShape::new();

        (&mut s as &mut dyn Shape).set_transform(Transformation::translation(5., 0., 0.));
        let xs = (&s as &dyn Shape).intersect(&r);

        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.);
    }

    #[test]
    fn computing_normal_on_a_translated_sphere() {
        let mut s = TestShape::new();
        (&mut s as &mut dyn Shape).set_transform(Transformation::translation(0., 1., 0.));

        let n = (&s as &dyn Shape).normal_at(&Tuple::point(0., 1.70711, - 0.70711));

        assert_abs_diff_eq!(n, Tuple::vector(0., 0.70711, - 0.70711), epsilon = 0.00001);
    }

    #[test]
    fn computing_normal_on_a_transformed_sphere() {
        let mut s = TestShape::new();
        let m = Transformation::scaling(1., 0.5, 1.) * Transformation::rotation_z(PI / 5.);
        (&mut s as &mut dyn Shape).set_transform(m);

        let n = (&s as &dyn Shape).normal_at(&Tuple::point(0., SQRT_2 / 2., - SQRT_2 / 2.));

        assert_abs_diff_eq!(n, Tuple::vector(0., 0.97014, - 0.24254), epsilon = 0.00001);
    }
}
