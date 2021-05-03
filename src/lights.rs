use crate::tuples::Point;
use crate::tuples::Color;

pub struct PointLight {
    intensity: Color,
    position: Point,
}

impl PointLight {

    pub fn new(position: Point, intensity: Color) -> PointLight {
        PointLight { position, intensity }
    }

    pub fn position(&self) -> &Point {
        &self.position
    }

    pub fn intensity(&self) -> &Color {
        &self.intensity
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

        assert_eq!(light.position(), &position);
        assert_eq!(light.intensity(), &intensity)
    }
}
