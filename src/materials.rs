use crate::lights::PointLight;
use crate::patterns::Pattern;
use crate::shapes::Shape;
use crate::tuples::Color;
use crate::tuples::Point;
use crate::tuples::Scalar;
use crate::tuples::Tuple;
use crate::tuples::Vector;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Material {
    pub pattern: Pattern,
    pub ambient: Scalar,
    pub diffuse: Scalar,
    pub specular: Scalar,
    pub shininess: Scalar,
}

impl Material {
    pub const fn new() -> Material {
        Material {
            pattern: Pattern::solid(Tuple::color(1., 1., 1.)),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.,
        }
    }

    pub fn set_color(&mut self, color: Color) {
        self.pattern = Pattern::solid(color);
    }

    pub fn lighting(
        &self,
        object: &Shape,
        light: &PointLight,
        point: &Point,
        eyev: &Vector,
        normalv: &Vector,
        in_shadow: bool,
    ) -> Color {
        let color = self.pattern.pattern_at_shape(object, point);
        let effective_color = color * light.intensity;
        let lightv = (light.position - *point).normalize();
        let ambient = effective_color * self.ambient;
        let light_dot_normal = lightv.dot(normalv);

        let diffuse: Color;
        let specular: Color;
        if in_shadow || light_dot_normal < 0. {
            diffuse = Color::BLACK;
            specular = Color::BLACK;
        } else {
            diffuse = effective_color * self.diffuse * light_dot_normal;
            let reflectv = (-lightv).reflect(normalv);
            let reflect_dot_eye = reflectv.dot(eyev);
            if reflect_dot_eye < 0. {
                specular = Color::BLACK;
            } else {
                let factor = reflect_dot_eye.powf(self.shininess);
                specular = light.intensity * self.specular * factor;
            }
        }

        ambient + diffuse + specular
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn default_material() {
        let m = Material::new();
        assert_eq!(m.pattern, Pattern::solid(Tuple::color(1., 1., 1.)));
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.);
    }

    mod lighting_tests {

        use super::*;
        use crate::materials::Material;
        use crate::materials::Tuple;
        use crate::tuples::Point;
        use approx::assert_abs_diff_eq;
        use std::f64::consts::*;

        const M: Material = Material::new();
        const POSITION: Point = Tuple::point(0., 0., 0.);

        #[test]
        fn ligthing_with_the_eye_between_the_light_and_the_surface() {
            let eyev = Tuple::vector(0., 0., -1.);
            let normalv = Tuple::vector(0., 0., -1.);
            let light = PointLight::new(Tuple::point(0., 0., -10.), Tuple::color(1., 1., 1.));

            let result = M.lighting(&Shape::sphere(), &light, &POSITION, &eyev, &normalv, false);
            assert_eq!(result, Tuple::color(1.9, 1.9, 1.9));
        }

        #[test]
        fn ligthing_with_the_eye_between_light_and_surface_eye_offset_45_deg() {
            let eyev = Tuple::vector(0., SQRT_2 / 2., -SQRT_2 / 2.);
            let normalv = Tuple::vector(0., 0., -1.);
            let light = PointLight::new(Tuple::point(0., 0., -10.), Tuple::color(1., 1., 1.));

            let result = M.lighting(&Shape::sphere(), &light, &POSITION, &eyev, &normalv, false);
            assert_eq!(result, Tuple::color(1., 1., 1.));
        }

        #[test]
        fn ligthing_with_the_eye_opposite_surface_light_offset_45_deg() {
            let eyev = Tuple::vector(0., 0., -1.);
            let normalv = Tuple::vector(0., 0., -1.);
            let light = PointLight::new(Tuple::point(0., 10., -10.), Tuple::color(1., 1., 1.));

            let result = M.lighting(&Shape::sphere(), &light, &POSITION, &eyev, &normalv, false);
            assert_abs_diff_eq!(
                result,
                Tuple::color(0.7364, 0.7364, 0.7364),
                epsilon = 0.00001
            );
        }

        #[test]
        fn ligthing_with_the_eye_in_the_path_of_the_reflection_vector() {
            let eyev = Tuple::vector(0., -SQRT_2 / 2., -SQRT_2 / 2.);
            let normalv = Tuple::vector(0., 0., -1.);
            let light = PointLight::new(Tuple::point(0., 10., -10.), Tuple::color(1., 1., 1.));

            let result = M.lighting(&Shape::sphere(), &light, &POSITION, &eyev, &normalv, false);
            assert_abs_diff_eq!(
                result,
                Tuple::color(1.6364, 1.6364, 1.6364),
                epsilon = 0.0001
            );
        }

        #[test]
        fn ligthing_with_the_light_behind_the_surface() {
            let eyev = Tuple::vector(0., 0., -1.);
            let normalv = Tuple::vector(0., 0., -1.);
            let light = PointLight::new(Tuple::point(0., 0., 10.), Tuple::color(1., 1., 1.));

            let result = M.lighting(&Shape::sphere(), &light, &POSITION, &eyev, &normalv, false);
            assert_eq!(result, Tuple::color(0.1, 0.1, 0.1));
        }

        #[test]
        fn ligthing_with_the_surface_in_shadow() {
            let eyev = Tuple::vector(0., 0., -1.);
            let normalv = Tuple::vector(0., 0., -1.);
            let light = PointLight::new(Tuple::point(0., 0., -10.), Tuple::color(1., 1., 1.));
            let in_shadow = true;

            let result = M.lighting(
                &Shape::sphere(),
                &light,
                &POSITION,
                &eyev,
                &normalv,
                in_shadow,
            );
            assert_eq!(result, Tuple::color(0.1, 0.1, 0.1));
        }

        #[test]
        fn lighting_with_a_pattern_applied() {
            let mut m = Material::new();
            m.pattern = Pattern::stripe(Tuple::color(1., 1., 1.), Tuple::color(0., 0., 0.));
            m.ambient = 1.;
            m.diffuse = 0.;
            m.specular = 0.;
            let eyev = Tuple::vector(0., 0., -1.);
            let normalv = Tuple::vector(0., 0., -1.);
            let light = PointLight::new(Tuple::point(0., 0., -10.), Tuple::color(1., 1., 1.));

            let c1 = m.lighting(
                &Shape::sphere(),
                &light,
                &Tuple::point(0.9, 0., 0.),
                &eyev,
                &normalv,
                false,
            );
            let c2 = m.lighting(
                &Shape::sphere(),
                &light,
                &Tuple::point(1.1, 0., 0.),
                &eyev,
                &normalv,
                false,
            );

            assert_eq!(c1, Tuple::color(1., 1., 1.));
            assert_eq!(c2, Tuple::color(0., 0., 0.));
        }
    }
}
