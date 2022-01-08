use crate::tuples::Color;
use crate::tuples::Point;

#[derive(PartialEq, Debug)]
pub struct PointLight {
    pub intensity: Color,
    pub position: Point,
}

impl PointLight {
    pub fn new(position: Point, intensity: Color) -> PointLight {
        PointLight {
            position,
            intensity,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tuples::Tuple;

    #[test]
    fn a_point_light_has_a_position_and_intensity() {
        let intensity = Tuple::color(1., 1., 1.);
        let position = Tuple::point(0., 0., 0.);

        let light = PointLight::new(position, intensity);

        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity)
    }
}
