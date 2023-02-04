use crate::{
    shapes::Shape,
    transformations::Transformation,
    tuples::{Color, Point},
};

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum PatternType {
    Solid { a: Color },
    Stripe { a: Color, b: Color },
    Gradient { a: Color, b: Color },
    Ring { a: Color, b: Color },
    Checker { a: Color, b: Color },
}

impl PatternType {
    pub fn pattern_at(&self, point: &Point) -> Color {
        match self {
            PatternType::Solid { a } => *a,
            PatternType::Stripe { a, b } => {
                if point.x.floor() % 2. == 0. {
                    *a
                } else {
                    *b
                }
            }
            PatternType::Gradient { a, b } => *a + (*b - *a) * (point.x - point.x.floor()),
            PatternType::Ring { a, b } => {
                if (point.x * point.x + point.z * point.z).sqrt() % 2. == 0. {
                    *a
                } else {
                    *b
                }
            }
            PatternType::Checker { a, b } => {
                if (point.x.floor() + point.y.floor() + point.z.floor()) % 2. == 0. {
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
    pub const fn solid(a: Color) -> Pattern {
        Self::new(PatternType::Solid { a })
    }

    pub fn stripe(a: Color, b: Color) -> Pattern {
        Self::new(PatternType::Stripe { a, b })
    }

    pub fn gradient(a: Color, b: Color) -> Pattern {
        Self::new(PatternType::Gradient { a, b })
    }

    pub fn ring(a: Color, b: Color) -> Pattern {
        Self::new(PatternType::Ring { a, b })
    }

    pub fn checker(a: Color, b: Color) -> Pattern {
        Self::new(PatternType::Checker { a, b })
    }

    pub const fn new(pattern_type: PatternType) -> Pattern {
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

    mod solid_pattern {

        use super::*;

        #[test]
        fn a_stripe_pattern_is_constant_in_x() {
            let pattern = PatternType::Solid { a: WHITE };
            assert_eq!(pattern.pattern_at(&Tuple::point(0., 0., 0.)), WHITE);
            assert_eq!(pattern.pattern_at(&Tuple::point(1., 0., 0.)), WHITE);
            assert_eq!(pattern.pattern_at(&Tuple::point(2., 0., 0.)), WHITE);
        }

        #[test]
        fn a_stripe_pattern_is_constant_in_y() {
            let pattern = PatternType::Solid { a: WHITE };
            assert_eq!(pattern.pattern_at(&Tuple::point(0., 0., 0.)), WHITE);
            assert_eq!(pattern.pattern_at(&Tuple::point(0., 1., 0.)), WHITE);
            assert_eq!(pattern.pattern_at(&Tuple::point(0., 2., 0.)), WHITE);
        }

        #[test]
        fn a_stripe_pattern_is_constant_in_z() {
            let pattern = PatternType::Solid { a: WHITE };
            assert_eq!(pattern.pattern_at(&Tuple::point(0., 0., 0.)), WHITE);
            assert_eq!(pattern.pattern_at(&Tuple::point(0., 0., 1.)), WHITE);
            assert_eq!(pattern.pattern_at(&Tuple::point(0., 0., 2.)), WHITE);
        }
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

    mod gradient_pattern {

        use super::*;

        #[test]
        fn a_gradient_linearly_interpolates_between_colors() {
            let pattern = PatternType::Gradient { a: WHITE, b: BLACK };

            assert_eq!(pattern.pattern_at(&Tuple::point(0., 0., 0.)), WHITE);
            assert_eq!(
                pattern.pattern_at(&Tuple::point(0.25, 0., 0.)),
                Tuple::color(0.75, 0.75, 0.75)
            );
            assert_eq!(
                pattern.pattern_at(&Tuple::point(0.5, 0., 0.)),
                Tuple::color(0.5, 0.5, 0.5)
            );
            assert_eq!(
                pattern.pattern_at(&Tuple::point(0.75, 0., 0.)),
                Tuple::color(0.25, 0.25, 0.25)
            );
        }
    }

    mod ring_pattern {

        use super::*;

        #[test]
        fn a_ring_should_extend_in_both_x_and_z() {
            let pattern = PatternType::Ring { a: WHITE, b: BLACK };

            assert_eq!(pattern.pattern_at(&Tuple::point(0., 0., 0.)), WHITE);
            assert_eq!(pattern.pattern_at(&Tuple::point(1., 0., 0.)), BLACK);
            assert_eq!(pattern.pattern_at(&Tuple::point(0., 0., 1.)), BLACK);
            assert_eq!(pattern.pattern_at(&Tuple::point(0.708, 0., 0.708)), BLACK);
        }
    }

    mod checker_pattern {

        use super::*;

        #[test]
        fn checker_pattern_should_repeat_in_x() {
            let pattern = PatternType::Checker { a: WHITE, b: BLACK };

            assert_eq!(pattern.pattern_at(&Tuple::point(0., 0., 0.)), WHITE);
            assert_eq!(pattern.pattern_at(&Tuple::point(0.99, 0., 0.)), WHITE);
            assert_eq!(pattern.pattern_at(&Tuple::point(1.01, 0., 0.)), BLACK);
        }

        #[test]
        fn checker_pattern_should_repeat_in_y() {
            let pattern = PatternType::Checker { a: WHITE, b: BLACK };

            assert_eq!(pattern.pattern_at(&Tuple::point(0., 0., 0.)), WHITE);
            assert_eq!(pattern.pattern_at(&Tuple::point(0., 0.99, 0.)), WHITE);
            assert_eq!(pattern.pattern_at(&Tuple::point(0., 1.01, 0.)), BLACK);
        }

        #[test]
        fn checker_pattern_should_repeat_in_z() {
            let pattern = PatternType::Checker { a: WHITE, b: BLACK };

            assert_eq!(pattern.pattern_at(&Tuple::point(0., 0., 0.)), WHITE);
            assert_eq!(pattern.pattern_at(&Tuple::point(0., 0., 0.99)), WHITE);
            assert_eq!(pattern.pattern_at(&Tuple::point(0., 0., 1.01)), BLACK);
        }
    }
}
