
use crate::tuples::Color;
use crate::intersections::Computations;
use crate::intersections::intersections;
use crate::intersections::Intersection;
use crate::rays::Ray;
use crate::transformations::Transformation;
use crate::materials::Material;
use crate::tuples::Tuple;
use crate::lights::PointLight;
use crate::spheres::Sphere;

struct World {
    objects: Vec<Sphere>,
    light: Option<PointLight>,
}

impl World {

    pub fn new() -> World {
        World { objects: vec![], light: None }
    }

    pub fn with_objects_and_light<'a>(objects: Vec<Sphere>, light: PointLight) -> World {
        World { objects, light: Some(light) }
    }

    pub fn default() -> World {
        let light = PointLight::new(Tuple::point(-10., 10., -10.), Tuple::color(1., 1., 1.));

        let mut s1 = Sphere::new();
        let mut m1 = Material::new();
        m1.color = Tuple::color(0.8, 1., 0.6);
        m1.diffuse = 0.7;
        m1.specular = 0.2;
        s1.material = m1;

        let mut s2 = Sphere::new();
        s2.set_transform(Transformation::scaling(0.5, 0.5, 0.5));

        World::with_objects_and_light(vec![s1, s2], light)
    }

    pub fn contains(&self, object: &Sphere) -> bool {
        self.objects.contains(object)
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let mut result = vec![];
        for object in &self.objects {
            result.extend(object.intersect(ray));
        }
        intersections(result)
    }

    fn shade_hit(&self, comps: &Computations) -> Color {
        comps.object.material.lighting(&self.light.as_ref().unwrap(), &comps.point, &comps.eyev, &comps.normalv)
    }
}

#[cfg(test)]
mod tests {

    use crate::intersections::Computations;
    use crate::rays::Ray;
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn creating_a_world() {
        let w = World::new();

        assert!(w.objects.is_empty());
        assert!(w.light.is_none());
    }

    #[test]
    fn the_default_world() {
        let light = PointLight::new(Tuple::point(-10., 10., -10.), Tuple::color(1., 1., 1.));
        let mut s1 = Sphere::new();
        let mut m1 = Material::new();
        m1.color = Tuple::color(0.8, 1., 0.6);
        m1.diffuse = 0.7;
        m1.specular = 0.2;
        s1.material = m1;

        let mut s2 = Sphere::new();
        s2.set_transform(Transformation::scaling(0.5, 0.5, 0.5));

        let w = World::default();

        assert_eq!(w.light, Some(light));
        assert!(w.contains(&s1));
        assert!(w.contains(&s2));
    }

    #[test]
    fn intersect_a_world_with_a_ray() {
        let w = World::default();
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));

        let xs = w.intersect(&r);

        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].t, 4.);
        assert_eq!(xs[1].t, 4.5);
        assert_eq!(xs[2].t, 5.5);
        assert_eq!(xs[3].t, 6.);
    }

    #[test]
    fn shading_an_intersection() {
        let w = World::default();
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let shape = &w.objects[0];
        let i = Intersection::new(4., &shape);

        let comps = Computations::prepare(&i, &r);
        let c = w.shade_hit(&comps);

        assert_abs_diff_eq!(c, Tuple::color(0.38066, 0.47583, 0.2855), epsilon = 0.00001);
    }

    #[test]
    fn shading_an_intersection_from_an_inside() {
        let mut w = World::default();
        w.light = Some(PointLight::new(Tuple::point(0., 0.25, 0.), Tuple::color(1., 1., 1.)));
        let r = Ray::new(Tuple::point(0., 0., 0.), Tuple::vector(0., 0., 1.));
        let shape = &w.objects[1];
        let i = Intersection::new(0.5, &shape);

        let comps = Computations::prepare(&i, &r);
        let c = w.shade_hit(&comps);

        assert_abs_diff_eq!(c, Tuple::color(0.90498, 0.90498, 0.90498), epsilon = 0.00001);
    }
}
