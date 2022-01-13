use crate::shapes::Shape;
use crate::transformations::Transformation;
use crate::tuples::Color;
use crate::tuples::Point;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct StripePattern {
    transform: Transformation,
    invered_transform: Transformation,
    a: Color,
    b: Color,
}

impl StripePattern {
    pub fn new(a: Color, b: Color) -> StripePattern {
        StripePattern {
            transform: Transformation::IDENTITY,
            invered_transform: Transformation::IDENTITY,
            a,
            b,
        }
    }

    pub fn set_transform(&mut self, transform: Transformation) {
        self.transform = transform;
        self.invered_transform = transform.inverse();
    }

    pub fn stripe_at(&self, point: &Point) -> Color {
        if point.x.floor() % 2. == 0. {
            self.a
        } else {
            self.b
        }
    }

    pub fn stripe_at_object(&self, object: &Shape, world_point: &Point) -> Color {
        let object_point = *object.inversed_transform() * *world_point;
        let pattern_point = self.invered_transform * object_point;

        self.stripe_at(&pattern_point)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tuples::Tuple;

    const BLACK: Color = Tuple::color(0., 0., 0.);
    const WHITE: Color = Tuple::color(1., 1., 1.);

    #[test]
    fn stripes_with_object_transformation() {
        let mut object = Shape::sphere();
        object.set_transform(Transformation::scaling(2., 2., 2.));
        let pattern = StripePattern::new(WHITE, BLACK);

        let c = pattern.stripe_at_object(&object, &Tuple::point(1.5, 0., 0.));

        assert_eq!(c, WHITE);
    }

    #[test]
    fn stripes_with_pattern_transformation() {
        let object = Shape::sphere();
        let mut pattern = StripePattern::new(WHITE, BLACK);
        pattern.set_transform(Transformation::scaling(2., 2., 2.));

        let c = pattern.stripe_at_object(&object, &Tuple::point(1.5, 0., 0.));

        assert_eq!(c, WHITE);
    }

    #[test]
    fn stripes_with_both_an_object_and_pattern_transformation() {
        let mut object = Shape::sphere();
        object.set_transform(Transformation::scaling(2., 2., 2.));
        let mut pattern = StripePattern::new(WHITE, BLACK);
        pattern.set_transform(Transformation::translation(0.5, 0., 0.));

        let c = pattern.stripe_at_object(&object, &Tuple::point(2.5, 0., 0.));

        assert_eq!(c, WHITE);
    }

    mod stripe_pattern {

        use super::*;

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
}
