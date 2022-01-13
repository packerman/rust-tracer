use crate::shapes::Shape;
use crate::transformations::Transformation;
use crate::tuples::Color;
use crate::tuples::Point;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum PatternType {
    Stripe { a: Color, b: Color },
}

impl PatternType {
    pub fn pattern_at(&self, point: &Point) -> Color {
        match self {
            PatternType::Stripe { a, b } => {
                if point.x.floor() % 2. == 0. {
                    *a
                } else {
                    *b
                }
            }
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Pattern {
    transform: Transformation,
    invered_transform: Transformation,
    pattern_type: PatternType,
}

impl Pattern {
    pub fn stripe(a: Color, b: Color) -> Pattern {
        Self::new(PatternType::Stripe { a, b })
    }

    pub fn new(pattern_type: PatternType) -> Pattern {
        Pattern {
            transform: Transformation::IDENTITY,
            invered_transform: Transformation::IDENTITY,
            pattern_type,
        }
    }

    pub fn transform(&self) -> &Transformation {
        &self.transform
    }

    pub fn set_transform(&mut self, transform: Transformation) {
        self.transform = transform;
        self.invered_transform = transform.inverse();
    }

    pub fn pattern_at_shape(&self, object: &Shape, world_point: &Point) -> Color {
        let object_point = *object.inversed_transform() * *world_point;
        let pattern_point = self.invered_transform * object_point;

        self.pattern_type.pattern_at(&pattern_point)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tuples::Tuple;

    const BLACK: Color = Tuple::color(0., 0., 0.);
    const WHITE: Color = Tuple::color(1., 1., 1.);

    #[test]
    fn default_pattern_transformation() {
        let pattern = Pattern::stripe(BLACK, WHITE);

        assert_eq!(pattern.transform(), &Transformation::IDENTITY);
    }

    #[test]
    fn assigning_a_transformation() {
        let mut pattern = Pattern::stripe(BLACK, WHITE);

        pattern.set_transform(Transformation::translation(1., 2., 3.));

        assert_eq!(
            pattern.transform(),
            &Transformation::translation(1., 2., 3.)
        );
    }

    #[test]
    fn pattern_with_object_transformation() {
        let mut object = Shape::sphere();
        object.set_transform(Transformation::scaling(2., 2., 2.));
        let pattern = Pattern::stripe(WHITE, BLACK);

        let c = pattern.pattern_at_shape(&object, &Tuple::point(1.5, 0., 0.));

        assert_eq!(c, WHITE);
    }

    #[test]
    fn pattern_with_pattern_transformation() {
        let object = Shape::sphere();
        let mut pattern = Pattern::stripe(WHITE, BLACK);
        pattern.set_transform(Transformation::scaling(2., 2., 2.));

        let c = pattern.pattern_at_shape(&object, &Tuple::point(1.5, 0., 0.));

        assert_eq!(c, WHITE);
    }

    #[test]
    fn pattern_with_both_an_object_and_pattern_transformation() {
        let mut object = Shape::sphere();
        object.set_transform(Transformation::scaling(2., 2., 2.));
        let mut pattern = Pattern::stripe(WHITE, BLACK);
        pattern.set_transform(Transformation::translation(0.5, 0., 0.));

        let c = pattern.pattern_at_shape(&object, &Tuple::point(2.5, 0., 0.));

        assert_eq!(c, WHITE);
    }

    mod stripe_pattern {

        use super::*;

        #[test]
        fn creating_a_stripe_pattern() {
            let pattern = PatternType::Stripe { a: WHITE, b: BLACK };

            assert!(matches!(pattern, PatternType::Stripe{a, b} if a == WHITE && b == BLACK))
        }

        #[test]
        fn a_stripe_pattern_is_constant_in_y() {
            let pattern = PatternType::Stripe { a: WHITE, b: BLACK };
            assert_eq!(pattern.pattern_at(&Tuple::point(0., 0., 0.)), WHITE);
            assert_eq!(pattern.pattern_at(&Tuple::point(0., 1., 0.)), WHITE);
            assert_eq!(pattern.pattern_at(&Tuple::point(0., 2., 0.)), WHITE);
        }

        #[test]
        fn a_stripe_pattern_is_constant_in_z() {
            let pattern = PatternType::Stripe { a: WHITE, b: BLACK };
            assert_eq!(pattern.pattern_at(&Tuple::point(0., 0., 0.)), WHITE);
            assert_eq!(pattern.pattern_at(&Tuple::point(0., 0., 1.)), WHITE);
            assert_eq!(pattern.pattern_at(&Tuple::point(0., 0., 2.)), WHITE);
        }

        #[test]
        fn a_stripe_pattern_alternates_in_x() {
            let pattern = PatternType::Stripe { a: WHITE, b: BLACK };
            assert_eq!(pattern.pattern_at(&Tuple::point(0., 0., 0.)), WHITE);
            assert_eq!(pattern.pattern_at(&Tuple::point(0.9, 0., 0.)), WHITE);
            assert_eq!(pattern.pattern_at(&Tuple::point(1., 0., 0.)), BLACK);
            assert_eq!(pattern.pattern_at(&Tuple::point(-0.1, 0., 0.)), BLACK);
            assert_eq!(pattern.pattern_at(&Tuple::point(-1., 0., 0.)), BLACK);
            assert_eq!(pattern.pattern_at(&Tuple::point(-1.1, 0., 0.)), WHITE);
        }
    }
}
