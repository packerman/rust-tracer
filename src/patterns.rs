use crate::tuples::Color;
use crate::tuples::Point;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct StripePattern {
    a: Color,
    b: Color,
}

impl StripePattern {
    pub fn new(a: Color, b: Color) -> StripePattern {
        StripePattern { a, b }
    }

    pub fn stripe_at(&self, point: &Point) -> Color {
        if point.x.floor() % 2. == 0. {
            self.a
        } else {
            self.b
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tuples::Tuple;

    const BLACK: Color = Tuple::color(0., 0., 0.);
    const WHITE: Color = Tuple::color(1., 1., 1.);

    #[test]
    fn creating_a_stripe_pattern() {
        let pattern = StripePattern::new(WHITE, BLACK);
        assert_eq!(pattern.a, WHITE);
        assert_eq!(pattern.b, BLACK);
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_y() {
        let pattern = StripePattern::new(WHITE, BLACK);
        assert_eq!(pattern.stripe_at(&Tuple::point(0., 0., 0.)), WHITE);
        assert_eq!(pattern.stripe_at(&Tuple::point(0., 1., 0.)), WHITE);
        assert_eq!(pattern.stripe_at(&Tuple::point(0., 2., 0.)), WHITE);
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_z() {
        let pattern = StripePattern::new(WHITE, BLACK);
        assert_eq!(pattern.stripe_at(&Tuple::point(0., 0., 0.)), WHITE);
        assert_eq!(pattern.stripe_at(&Tuple::point(0., 0., 1.)), WHITE);
        assert_eq!(pattern.stripe_at(&Tuple::point(0., 0., 2.)), WHITE);
    }

    #[test]
    fn a_stripe_pattern_alternates_in_x() {
        let pattern = StripePattern::new(WHITE, BLACK);
        assert_eq!(pattern.stripe_at(&Tuple::point(0., 0., 0.)), WHITE);
        assert_eq!(pattern.stripe_at(&Tuple::point(0.9, 0., 0.)), WHITE);
        assert_eq!(pattern.stripe_at(&Tuple::point(1., 0., 0.)), BLACK);
        assert_eq!(pattern.stripe_at(&Tuple::point(-0.1, 0., 0.)), BLACK);
        assert_eq!(pattern.stripe_at(&Tuple::point(-1., 0., 0.)), BLACK);
        assert_eq!(pattern.stripe_at(&Tuple::point(-1.1, 0., 0.)), WHITE);
    }
}
