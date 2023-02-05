use crate::{
    intersections::Intersection,
    materials::Material,
    rays::Ray,
    transformations::Transformation,
    tuples::{Point, Scalar, Vector},
};
use std::fmt::Debug;

use self::{planes::Plane, spheres::Sphere};

pub mod planes;
pub mod spheres;

pub trait ShapeType: Debug {
    fn local_intersect(&self, ray: &Ray) -> Vec<Scalar>;
    fn local_normal_at(&self, point: &Point) -> Vector;
}

#[derive(Debug)]
pub struct Shape {
    transform: Transformation,
    inversed_transform: Transformation,
    material: Material,
    shape_type: Box<dyn ShapeType>,
}

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
            material: Material::default(),
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

    pub fn material(&self) -> &Material {
        &self.material
    }

    pub fn material_mut(&mut self) -> &mut Material {
        &mut self.material
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
    use std::{
        f64::consts::{PI, SQRT_2},
        ptr,
    };

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

        assert_eq!(m.diffuse, Material::default().diffuse);
    }

    #[test]
    fn a_sphere_may_be_assigned_a_material() {
        let mut s = Shape::sphere();
        let mut m = Material::default();
        m.ambient = 1.;

        s.material = m;

        assert_eq!(s.material.ambient, 1.);
    }

    #[test]
    fn intersect_sets_the_object_on_the_intersection() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let s = Shape::sphere();

        let xs = s.intersect(&r);

        assert!(!xs.is_empty());
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
}
