
use crate::matrices::Matrix4;

type Transformation = Matrix4;

impl Transformation {

    fn translation(x: f32, y: f32, z: f32) -> Transformation {
        Matrix4::new(1., 0., 0., x,
                    0., 1., 0., y,
                    0., 0., 1., z,
                    0., 0., 0., 1.)
    }

    fn scaling(x: f32, y: f32, z: f32) -> Transformation {
        Matrix4::new(x, 0., 0., 0.,
                    0., y, 0., 0.,
                    0., 0., z, 0.,
                    0., 0., 0., 1.)
    }
}

#[cfg(test)]
mod tests {

    use crate::tuples::Tuple;
    use super::*;

    #[test]
    fn multiplying_by_translation_matrix() {
        let transform = Transformation::translation(5., -3., 2.);
        let p = Tuple::point(-3., 4., 5.);

        assert_eq!(transform * p, Tuple::point(2., 1., 7.));
    }

    #[test]
    fn multiplying_by_translation_matrix_inverse() {
        let transform = Transformation::translation(5., -3., 2.);
        let inv = transform.inverse();
        let p = Tuple::point(-3., 4., 5.);

        assert_eq!(inv * p, Tuple::point(-8., 7., 3.));
    }

    #[test]
    fn translation_does_not_affect_vector() {
        let transform = Transformation::translation(5., -3., 2.);
        let v = Tuple::vector(-3., 4., 5.);

        assert_eq!(transform * v, v);
    }

    #[test]
    fn scaling_matrix_applied_to_point() {
        let transform = Transformation::scaling(2., 3., 4.);
        let p = Tuple::point(-4., 6., 8.);

        assert_eq!(transform * p, Tuple::point(-8., 18., 32.));
    }

    #[test]
    fn scaling_matrix_applied_to_vector() {
        let transform = Transformation::scaling(2., 3., 4.);
        let p = Tuple::vector(-4., 6., 8.);

        assert_eq!(transform * p, Tuple::vector(-8., 18., 32.));
    }

    #[test]
    fn multiplying_by_scaling_matrix_inverse() {
        let transform = Transformation::scaling(2., 3., 4.);
        let inv = transform.inverse();
        let p = Tuple::vector(-4., 6., 8.);

        assert_eq!(inv * p, Tuple::vector(-2., 2., 2.));
    }

    #[test]
    fn reflection_is_scaling() {
        let transform = Transformation::scaling(-1., 1., 1.);
        let p = Tuple::point(2., 3., 4.);

        assert_eq!(transform * p, Tuple::point(-2., 3., 4.));
    }
}
