
use crate::tuples::Vector;
use crate::tuples::Point;
use crate::lights::PointLight;
use crate::tuples::Color;
use crate::tuples::Tuple;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Material {
    pub color: Color,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess:f32,
}

impl Material {

    pub const fn new() -> Material {
        Material {
            color: Tuple::color(1., 1., 1.),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.,
        }
    }

    pub fn lighting(&self, light: &PointLight, point: &Point, eyev: &Vector, normalv: &Vector, in_shadow: bool) -> Color {
        let effective_color = self.color * light.intensity;
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
            let reflectv = (- lightv).reflect(normalv);
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
        assert_eq!(m.color, Tuple::color(1., 1., 1.));
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.);
    }

    mod lighting_tests {

        use super::*;
        use approx::assert_abs_diff_eq;
        use crate::tuples::Point;
        use crate::materials::Tuple;
        use crate::materials::Material;

        const M: Material = Material::new();
        const POSITION: Point = Tuple::point(0., 0., 0.);

        #[test]
        fn ligthing_with_the_eye_between_the_light_and_the_surface() {
            let eyev = Tuple::vector(0., 0., -1.);
            let normalv = Tuple::vector(0., 0., -1.);
            let light = PointLight::new(Tuple::point(0., 0., -10.), Tuple::color(1., 1., 1.));

            let result = M.lighting(&light, &POSITION, &eyev, &normalv, false);
            assert_eq!(result, Tuple::color(1.9, 1.9, 1.9));
        }

        #[test]
        fn ligthing_with_the_eye_between_light_and_surface_eye_offset_45_deg() {
            let eyev = Tuple::vector(0., 2_f32.sqrt() / 2., - 2_f32.sqrt() / 2.);
            let normalv = Tuple::vector(0., 0., -1.);
            let light = PointLight::new(Tuple::point(0., 0., -10.), Tuple::color(1., 1., 1.));

            let result = M.lighting(&light, &POSITION, &eyev, &normalv, false);
            assert_eq!(result, Tuple::color(1., 1., 1.));
        }

        #[test]
        fn ligthing_with_the_eye_opposite_surface_light_offset_45_deg() {
            let eyev = Tuple::vector(0., 0., -1.);
            let normalv = Tuple::vector(0., 0., -1.);
            let light = PointLight::new(Tuple::point(0., 10., -10.), Tuple::color(1., 1., 1.));

            let result = M.lighting(&light, &POSITION, &eyev, &normalv, false);
            assert_abs_diff_eq!(result, Tuple::color(0.7364, 0.7364, 0.7364), epsilon = 0.00001);
        }

        #[test]
        fn ligthing_with_the_eye_in_the_path_of_the_reflection_vector() {
            let eyev = Tuple::vector(0., - 2_f32.sqrt() / 2., - 2_f32.sqrt() / 2.);
            let normalv = Tuple::vector(0., 0., -1.);
            let light = PointLight::new(Tuple::point(0., 10., -10.), Tuple::color(1., 1., 1.));

            let result = M.lighting(&light, &POSITION, &eyev, &normalv, false);
            assert_abs_diff_eq!(result, Tuple::color(1.6364, 1.6364, 1.6364), epsilon = 0.0001);
        }


        #[test]
        fn ligthing_with_the_light_behind_the_surface() {
            let eyev = Tuple::vector(0., 0., -1.);
            let normalv = Tuple::vector(0., 0., -1.);
            let light = PointLight::new(Tuple::point(0., 0., 10.), Tuple::color(1., 1., 1.));

            let result = M.lighting(&light, &POSITION, &eyev, &normalv, false);
            assert_eq!(result, Tuple::color(0.1, 0.1, 0.1));
        }

        #[test]
        fn ligthing_with_the_surface_in_shadow() {
            let eyev = Tuple::vector(0., 0., -1.);
            let normalv = Tuple::vector(0., 0., -1.);
            let light = PointLight::new(Tuple::point(0., 0., -10.), Tuple::color(1., 1., 1.));
            let in_shadow = true;

            let result = M.lighting(&light, &POSITION, &eyev, &normalv, in_shadow);
            assert_eq!(result, Tuple::color(0.1, 0.1, 0.1));
        }
    }
}
