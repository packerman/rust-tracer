
use crate::tuples::Vector;
use crate::tuples::Point;
use std::fmt::Debug;
use crate::intersections::Intersection;
use crate::rays::Ray;
use crate::materials::Material;
use crate::transformations::Transformation;

pub trait Shape: Debug {

    fn transform(&self) -> &Transformation;

    fn inversed_transform(&self) -> &Transformation;

    fn material(&self) -> &Material;

    fn set_material(&mut self, material: Material);

    fn local_intersect(&self, ray: &Ray) -> Vec<Intersection>;

    fn local_normal_at(&self, world_point: &Point) -> Vector;
}

impl dyn Shape {

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let local_ray = ray.transform(&self.inversed_transform());
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

// TODO remove
pub fn normal_at(shape: &dyn Shape, point: &Point) -> Vector {
    let local_point = *shape.inversed_transform() * *point;
    let local_normal = shape.local_normal_at(&local_point);
    let mut world_normal = shape.inversed_transform().transpose() * local_normal;
    world_normal.w = 0.;

    world_normal.normalize()
}

#[cfg(test)]
mod tests {

    use std::f64::consts::*;
    use crate::tuples::Tuple;
    use super::*;
    use approx::assert_abs_diff_eq;

    #[derive(Debug)]
    struct TestShape {
        transform: Transformation,
        inversed_transform: Transformation,
        material: Material,
    }

    impl TestShape {

        fn new() -> TestShape {
            TestShape {
                transform: Transformation::IDENTITY,
                inversed_transform: Transformation::IDENTITY,
                material: Material::new(),
            }
        }

        fn set_transform(&mut self, transform: Transformation) {
            self.transform = transform;
            self.inversed_transform = transform.inverse();
        }
    }

    impl Shape for TestShape {

        fn transform(&self) -> &Transformation {
            &self.transform
        }

        fn inversed_transform(&self) -> &Transformation {
            &self.inversed_transform
        }

        fn material(&self) -> &Material {
            &self.material
        }

        fn set_material(&mut self, material: Material) { todo!() }

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

            assert_eq!(s.transform(), &Transformation::IDENTITY);
        }

        #[test]
        fn assigning_a_transformation() {
            let mut s = TestShape::new();

            s.set_transform(Transformation::translation(2., 3., 4.));

            assert_eq!(s.transform(), &Transformation::translation(2., 3., 4.));
        }

        #[test]
        fn the_default_material() {
            let s = TestShape::new();

            assert_eq!(s.material(), &Material::new());
        }

        #[test]
        fn assigning_a_material() {
            let mut s = TestShape::new();
            let mut m = Material::new();
            m.ambient = 1.;

            s.material = m;

            assert_eq!(s.material(), &m);
        }
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let mut s = TestShape::new();

        s.set_transform(Transformation::scaling(2., 2., 2.));
        let xs = (&s as &dyn Shape).intersect(&r);

        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.);
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let mut s = TestShape::new();

        s.set_transform(Transformation::translation(5., 0., 0.));
        let xs = (&s as &dyn Shape).intersect(&r);

        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.);
    }

    #[test]
    fn computing_normal_on_a_translated_sphere() {
        let mut s = TestShape::new();
        s.set_transform(Transformation::translation(0., 1., 0.));

        let n = (&s as &dyn Shape).normal_at(&Tuple::point(0., 1.70711, - 0.70711));

        assert_abs_diff_eq!(n, Tuple::vector(0., 0.70711, - 0.70711), epsilon = 0.00001);
    }

    #[test]
    fn computing_normal_on_a_transformed_sphere() {
        let mut s = TestShape::new();
        let m = Transformation::scaling(1., 0.5, 1.) * Transformation::rotation_z(PI / 5.);
        s.set_transform(m);

        let n = (&s as &dyn Shape).normal_at(&Tuple::point(0., SQRT_2 / 2., - SQRT_2 / 2.));

        assert_abs_diff_eq!(n, Tuple::vector(0., 0.97014, - 0.24254), epsilon = 0.00001);
    }
}
