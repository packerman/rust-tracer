
use crate::tuples::Point;
use crate::intersections::hit;
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

pub struct World {
    objects: Vec<Sphere>,
    lights: Vec<PointLight>,
}

impl World {

    fn new() -> World {
        World { objects: vec![], lights: vec![] }
    }

    pub fn with_objects_and_light<'a>(objects: Vec<Sphere>, light: PointLight) -> World {
        World { objects, lights: vec![light] }
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

    fn contains(&self, object: &Sphere) -> bool {
        self.objects.contains(object)
    }

    fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let mut result = vec![];
        for object in &self.objects {
            result.extend(object.intersect(ray));
        }
        intersections(result)
    }

    fn shade_hit(&self, comps: &Computations) -> Color {
        self.lights.iter()
            .map(|light| comps.object.material.lighting(light, &comps.point, &comps.eyev, &comps.normalv, false))
            .sum()
    }

    pub fn color_at(&self, ray: &Ray) -> Color {
        let intersections = self.intersect(&ray);
        let hit = hit(&intersections);
        match hit {
            None => Color::BLACK,
            Some(h) => {
                let comps = Computations::prepare(&h, &ray);
                self.shade_hit(&comps)
            }
        }
    }

    fn is_shadowed(&self, point: &Point, light: &PointLight) -> bool {
        let v = light.position - *point;
        let distance = v.magnitude();
        let direction = v.normalize();

        let r = Ray::new(*point, direction);
        let intersections = self.intersect(&r);
        match hit(&intersections) {
            Some(h) => h.t < distance,
            None => false,
        }
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
        assert!(w.lights.is_empty());
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

        assert_eq!(w.lights, vec![light]);
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
        w.lights[0] = PointLight::new(Tuple::point(0., 0.25, 0.), Tuple::color(1., 1., 1.));
        let r = Ray::new(Tuple::point(0., 0., 0.), Tuple::vector(0., 0., 1.));
        let shape = &w.objects[1];
        let i = Intersection::new(0.5, &shape);

        let comps = Computations::prepare(&i, &r);
        let c = w.shade_hit(&comps);

        assert_abs_diff_eq!(c, Tuple::color(0.90498, 0.90498, 0.90498), epsilon = 0.00001);
    }

    #[test]
    fn the_color_when_a_ray_misses() {
        let w = World::default();
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 1., 0.));

        let c = w.color_at(&r);

        assert_eq!(c, Tuple::color(0., 0., 0.));
    }

    #[test]
    fn the_color_when_a_ray_hits() {
        let w = World::default();
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));

        let c = w.color_at(&r);

        assert_abs_diff_eq!(c, Tuple::color(0.38066, 0.47583, 0.2855), epsilon = 0.00001);
    }

    #[test]
    fn the_color_with_an_intersection_behind_the_ray() {
        let mut w = World::default();
        let outer = &mut w.objects[0];
        outer.material.ambient = 1.;
        let inner = &mut w.objects[1];
        inner.material.ambient = 1.;
        let inner_color = inner.material.color;
        let r = Ray::new(Tuple::point(0., 0., 0.75), Tuple::vector(0., 0., -1.));

        let c = w.color_at(&r);
        assert_eq!(c, inner_color);
    }

    #[test]
    fn there_is_no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let w = World::default();
        let p = Tuple::point(0., 10., 0.);

        assert!(!w.is_shadowed(&p, &w.lights[0]));
    }

    #[test]
    fn the_shadow_when_an_object_is_between_the_point_and_the_light() {
        let w = World::default();
        let p = Tuple::point(10., -10., 10.);

        assert!(w.is_shadowed(&p, &w.lights[0]));
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_light() {
        let w = World::default();
        let p = Tuple::point(-20., 20., -20.);

        assert!(!w.is_shadowed(&p, &w.lights[0]));
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_point() {
        let w = World::default();
        let p = Tuple::point(-2., 2., -2.);

        assert!(!w.is_shadowed(&p, &w.lights[0]));
    }

    // #[test]
    // fn shade_hit_is_given_an_intersection_in_shadow() {
    //     let light = PointLight::new(Tuple::point(0., 0., -10.), Tuple::color(1., 1., 1.));
    //     let s1 = Sphere::new();
    //     let mut s2 = Sphere::new();
    //     s2.set_transform(Transformation::translation(0., 0., 10.));
    //     let i = Intersection::new(4., &s2);
    //     let w = World::with_objects_and_light(vec![s1, s2], light);
    //     let r = Ray::new(Tuple::point(0., 0., 5.), Tuple::vector(0., 0., 1.));

    //     let comps = Computations::prepare(&i, &r);
    //     let c = w.shade_hit(&comps);

    //     assert_eq!(c, Tuple::color(0.1, 0.1, 0.1));
    // }
}
