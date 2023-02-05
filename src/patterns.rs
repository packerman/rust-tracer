use std::{fmt::Debug, rc::Rc};

use crate::{
    shapes::Shape,
    transformations::Transformation,
    tuples::{Color, Point},
};

pub trait PatternType: Debug {
    fn pattern_at(&self, point: &Point) -> Color;
}

#[derive(Debug, Clone)]
pub struct Solid {
    a: Color,
}

impl Solid {
    pub fn new(a: Color) -> Rc<Self> {
        Rc::new(Self { a })
    }
}

impl PatternType for Solid {
    fn pattern_at(&self, _point: &Point) -> Color {
        self.a
    }
}

#[derive(Debug, Clone)]
pub struct Stripe {
    a: Rc<dyn PatternType>,
    b: Rc<dyn PatternType>,
}

impl Stripe {
    pub fn new(a: Rc<dyn PatternType>, b: Rc<dyn PatternType>) -> Rc<Self> {
        Rc::new(Self { a, b })
    }

    pub fn new_solid(a: Color, b: Color) -> Rc<Self> {
        Self::new(Solid::new(a), Solid::new(b))
    }
}

impl PatternType for Stripe {
    fn pattern_at(&self, point: &Point) -> Color {
        if point.x.floor() % 2. == 0. {
            self.a.pattern_at(point)
        } else {
            self.b.pattern_at(point)
        }
    }
}

#[derive(Debug, Clone)]
pub struct Gradient {
    a: Rc<dyn PatternType>,
    b: Rc<dyn PatternType>,
}

impl Gradient {
    pub fn new(a: Rc<dyn PatternType>, b: Rc<dyn PatternType>) -> Rc<Self> {
        Rc::new(Self { a, b })
    }

    pub fn new_solid(a: Color, b: Color) -> Rc<Self> {
        Self::new(Solid::new(a), Solid::new(b))
    }
}

impl PatternType for Gradient {
    fn pattern_at(&self, point: &Point) -> Color {
        let a = self.a.pattern_at(point);
        let b = self.b.pattern_at(point);
        a + (b - a) * (point.x - point.x.floor())
    }
}

#[derive(Debug, Clone)]
pub struct Ring {
    a: Rc<dyn PatternType>,
    b: Rc<dyn PatternType>,
}

impl Ring {
    pub fn new(a: Rc<dyn PatternType>, b: Rc<dyn PatternType>) -> Rc<Self> {
        Rc::new(Self { a, b })
    }

    pub fn new_solid(a: Color, b: Color) -> Rc<Self> {
        Self::new(Solid::new(a), Solid::new(b))
    }
}

impl PatternType for Ring {
    fn pattern_at(&self, point: &Point) -> Color {
        if (point.x * point.x + point.z * point.z).sqrt() % 2. == 0. {
            self.a.pattern_at(point)
        } else {
            self.b.pattern_at(point)
        }
    }
}

#[derive(Debug, Clone)]
pub struct Checker {
    a: Rc<dyn PatternType>,
    b: Rc<dyn PatternType>,
}

impl Checker {
    pub fn new(a: Rc<dyn PatternType>, b: Rc<dyn PatternType>) -> Rc<Self> {
        Rc::new(Self { a, b })
    }

    pub fn new_solid(a: Color, b: Color) -> Rc<Self> {
        Self::new(Solid::new(a), Solid::new(b))
    }
}

impl PatternType for Checker {
    fn pattern_at(&self, point: &Point) -> Color {
        if (point.x.floor() + point.y.floor() + point.z.floor()) % 2. == 0. {
            self.a.pattern_at(point)
        } else {
            self.b.pattern_at(point)
        }
    }
}

#[derive(Debug, Clone)]
pub struct Pattern {
    transform: Transformation,
    invered_transform: Transformation,
    pattern_type: Rc<dyn PatternType>,
}

impl Pattern {
    pub fn solid(a: Color) -> Pattern {
        Self::new(Solid::new(a))
    }

    pub fn stripe(a: Color, b: Color) -> Pattern {
        Self::new(Stripe::new_solid(a, b))
    }

    pub fn gradient(a: Color, b: Color) -> Pattern {
        Self::new(Gradient::new_solid(a, b))
    }

    pub fn ring(a: Color, b: Color) -> Pattern {
        Self::new(Ring::new_solid(a, b))
    }

    pub fn checker(a: Color, b: Color) -> Pattern {
        Self::new(Checker::new_solid(a, b))
    }

    pub const fn new(pattern_type: Rc<dyn PatternType>) -> Pattern {
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

    mod solid {

        use super::*;

        #[test]
        fn a_stripe_pattern_is_constant_in_x() {
            let pattern = Solid::new(WHITE);
            assert_eq!(pattern.pattern_at(&Tuple::point(0., 0., 0.)), WHITE);
            assert_eq!(pattern.pattern_at(&Tuple::point(1., 0., 0.)), WHITE);
            assert_eq!(pattern.pattern_at(&Tuple::point(2., 0., 0.)), WHITE);
        }

        #[test]
        fn a_stripe_pattern_is_constant_in_y() {
            let pattern = Solid::new(WHITE);
            assert_eq!(pattern.pattern_at(&Tuple::point(0., 0., 0.)), WHITE);
            assert_eq!(pattern.pattern_at(&Tuple::point(0., 1., 0.)), WHITE);
            assert_eq!(pattern.pattern_at(&Tuple::point(0., 2., 0.)), WHITE);
        }

        #[test]
        fn a_stripe_pattern_is_constant_in_z() {
            let pattern = Solid::new(WHITE);
            assert_eq!(pattern.pattern_at(&Tuple::point(0., 0., 0.)), WHITE);
            assert_eq!(pattern.pattern_at(&Tuple::point(0., 0., 1.)), WHITE);
            assert_eq!(pattern.pattern_at(&Tuple::point(0., 0., 2.)), WHITE);
        }
    }

    mod stripe {

        use super::*;

        #[test]
        fn a_stripe_pattern_is_constant_in_y() {
            let pattern = Stripe::new_solid(WHITE, BLACK);
            assert_eq!(pattern.pattern_at(&Tuple::point(0., 0., 0.)), WHITE);
            assert_eq!(pattern.pattern_at(&Tuple::point(0., 1., 0.)), WHITE);
            assert_eq!(pattern.pattern_at(&Tuple::point(0., 2., 0.)), WHITE);
        }

        #[test]
        fn a_stripe_pattern_is_constant_in_z() {
            let pattern = Stripe::new_solid(WHITE, BLACK);
            assert_eq!(pattern.pattern_at(&Tuple::point(0., 0., 0.)), WHITE);
            assert_eq!(pattern.pattern_at(&Tuple::point(0., 0., 1.)), WHITE);
            assert_eq!(pattern.pattern_at(&Tuple::point(0., 0., 2.)), WHITE);
        }

        #[test]
        fn a_stripe_pattern_alternates_in_x() {
            let pattern = Stripe::new_solid(WHITE, BLACK);
            assert_eq!(pattern.pattern_at(&Tuple::point(0., 0., 0.)), WHITE);
            assert_eq!(pattern.pattern_at(&Tuple::point(0.9, 0., 0.)), WHITE);
            assert_eq!(pattern.pattern_at(&Tuple::point(1., 0., 0.)), BLACK);
            assert_eq!(pattern.pattern_at(&Tuple::point(-0.1, 0., 0.)), BLACK);
            assert_eq!(pattern.pattern_at(&Tuple::point(-1., 0., 0.)), BLACK);
            assert_eq!(pattern.pattern_at(&Tuple::point(-1.1, 0., 0.)), WHITE);
        }
    }

    mod gradient {

        use super::*;

        #[test]
        fn a_gradient_linearly_interpolates_between_colors() {
            let pattern = Gradient::new_solid(WHITE, BLACK);

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

    mod ring {

        use super::*;

        #[test]
        fn a_ring_should_extend_in_both_x_and_z() {
            let pattern = Ring::new_solid(WHITE, BLACK);

            assert_eq!(pattern.pattern_at(&Tuple::point(0., 0., 0.)), WHITE);
            assert_eq!(pattern.pattern_at(&Tuple::point(1., 0., 0.)), BLACK);
            assert_eq!(pattern.pattern_at(&Tuple::point(0., 0., 1.)), BLACK);
            assert_eq!(pattern.pattern_at(&Tuple::point(0.708, 0., 0.708)), BLACK);
        }
    }

    mod checker {

        use super::*;

        #[test]
        fn checker_pattern_should_repeat_in_x() {
            let pattern = Checker::new_solid(WHITE, BLACK);

            assert_eq!(pattern.pattern_at(&Tuple::point(0., 0., 0.)), WHITE);
            assert_eq!(pattern.pattern_at(&Tuple::point(0.99, 0., 0.)), WHITE);
            assert_eq!(pattern.pattern_at(&Tuple::point(1.01, 0., 0.)), BLACK);
        }

        #[test]
        fn checker_pattern_should_repeat_in_y() {
            let pattern = Checker::new_solid(WHITE, BLACK);

            assert_eq!(pattern.pattern_at(&Tuple::point(0., 0., 0.)), WHITE);
            assert_eq!(pattern.pattern_at(&Tuple::point(0., 0.99, 0.)), WHITE);
            assert_eq!(pattern.pattern_at(&Tuple::point(0., 1.01, 0.)), BLACK);
        }

        #[test]
        fn checker_pattern_should_repeat_in_z() {
            let pattern = Checker::new_solid(WHITE, BLACK);

            assert_eq!(pattern.pattern_at(&Tuple::point(0., 0., 0.)), WHITE);
            assert_eq!(pattern.pattern_at(&Tuple::point(0., 0., 0.99)), WHITE);
            assert_eq!(pattern.pattern_at(&Tuple::point(0., 0., 1.01)), BLACK);
        }
    }
}
