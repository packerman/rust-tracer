
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

    fn rotation_x(r: f32) -> Transformation {
        Matrix4::new(1., 0., 0., 0.,
                        0., r.cos(), - r.sin(), 0.,
                        0., r.sin(), r.cos(), 0.,
                        0., 0., 0., 1.)
    }

    fn rotation_y(r: f32) -> Transformation {
        Matrix4::new(r.cos(), 0., r.sin(), 0.,
                        0., 1., 0., 0.,
                        - r.sin(), 0., r.cos(), 0.,
                        0., 0., 0., 1.)
    }

    fn rotation_z(r: f32) -> Transformation {
        Matrix4::new(r.cos(), - r.sin(), 0., 0.,
                        r.sin(), r.cos(), 0., 0.,
                        0., 0., 1., 0.,
                        0., 0., 0., 1.)
    }

    fn shearing(x_y: f32, x_z: f32, y_x: f32, y_z: f32, z_x: f32, z_y: f32) -> Transformation {
        Matrix4::new(1., x_y, x_z, 0.,
            y_x, 1., y_z, 0.,
            z_x, z_y, 1., 0.,
            0., 0., 0., 1.)
    }
}

#[cfg(test)]
mod tests {

    use std::f32::consts::*;
    use crate::tuples::Tuple;
    use super::*;
    use approx::assert_abs_diff_eq;

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

    #[test]
    fn rotating_point_around_x_axis() {
        let p = Tuple::point(0., 1., 0.);
        let half_quarter = Transformation::rotation_x(FRAC_PI_4);
        let full_quarter = Transformation::rotation_x(FRAC_PI_2);

        assert_abs_diff_eq!(half_quarter * p, Tuple::point(0., SQRT_2/2., SQRT_2/2.));
        assert_abs_diff_eq!(full_quarter * p, Tuple::point(0., 0., 1.));
    }

    #[test]
    fn rotating_point_around_x_axis_reverse() {
        let p = Tuple::point(0., 1., 0.);
        let half_quarter = Transformation::rotation_x(FRAC_PI_4);
        let inv = half_quarter.inverse();

        assert_abs_diff_eq!(inv * p, Tuple::point(0., SQRT_2 / 2., - SQRT_2 / 2.));
    }

    #[test]
    fn rotating_around_y_axis() {
        let p = Tuple::point(0., 0., 1.);
        let half_quarter = Transformation::rotation_y(FRAC_PI_4);
        let full_quarter = Transformation::rotation_y(FRAC_PI_2);

        assert_abs_diff_eq!(half_quarter * p, Tuple::point(SQRT_2 / 2., 0., SQRT_2 / 2.));
        assert_abs_diff_eq!(full_quarter * p, Tuple::point(1., 0., 0.));
    }

    #[test]
    fn rotating_around_z_axis() {
        let p = Tuple::point(0., 1., 0.);
        let half_quarter = Transformation::rotation_z(FRAC_PI_4);
        let full_quarter = Transformation::rotation_z(FRAC_PI_2);

        assert_abs_diff_eq!(half_quarter * p, Tuple::point(- SQRT_2 / 2., SQRT_2 / 2., 0.));
        assert_abs_diff_eq!(full_quarter * p, Tuple::point(- 1., 0., 0.));
    }

    #[test]
    fn shearing_transformation_moves_x_in_proportion_to_y() {
        let transform = Transformation::shearing(1., 0., 0., 0., 0., 0.);
        let p = Tuple::point(2., 3., 4.);
        assert_eq!(transform * p, Tuple::point(5., 3., 4.));
    }

    #[test]
    fn shearing_transformation_moves_x_in_proportion_to_z() {
        let transform = Transformation::shearing(0., 1., 0., 0., 0., 0.);
        let p = Tuple::point(2., 3., 4.);
        assert_eq!(transform * p, Tuple::point(6., 3., 4.));
    }

    #[test]
    fn shearing_transformation_moves_y_in_proportion_to_x() {
        let transform = Transformation::shearing(0., 0., 1., 0., 0., 0.);
        let p = Tuple::point(2., 3., 4.);
        assert_eq!(transform * p, Tuple::point(2., 5., 4.));
    }

    #[test]
    fn shearing_transformation_moves_y_in_proportion_to_z() {
        let transform = Transformation::shearing(0., 0., 0., 1., 0., 0.);
        let p = Tuple::point(2., 3., 4.);
        assert_eq!(transform * p, Tuple::point(2., 7., 4.));
    }

    #[test]
    fn shearing_transformation_moves_z_in_proportion_to_x() {
        let transform = Transformation::shearing(0., 0., 0., 0., 1., 0.);
        let p = Tuple::point(2., 3., 4.);
        assert_eq!(transform * p, Tuple::point(2., 3., 6.));
    }

    #[test]
    fn shearing_transformation_moves_z_in_proportion_to_y() {
        let transform = Transformation::shearing(0., 0., 0., 0., 0., 1.);
        let p = Tuple::point(2., 3., 4.);
        assert_eq!(transform * p, Tuple::point(2., 3., 7.));
    }

    #[test]
    fn individual_transformations_are_applied_in_sequence() {
        let p = Tuple::point(1., 0., 1.);
        let a = Transformation::rotation_x(FRAC_PI_2);
        let b = Transformation::scaling(5., 5., 5.);
        let c = Transformation::translation(10., 5., 7.);

        let p2 = a * p;
        assert_abs_diff_eq!(p2, Tuple::point(1., -1., 0.));
        let p3 = b * p2;
        assert_abs_diff_eq!(p3, Tuple::point(5., -5., 0.), epsilon = 0.000001);
        let p4 = c * p3;
        assert_eq!(p4, Tuple::point(15., 0., 7.));
    }

    #[test]
    fn chained_transformations_must_be_applied_in_reverse_order() {
        let p = Tuple::point(1., 0., 1.);
        let a = Transformation::rotation_x(FRAC_PI_2);
        let b = Transformation::scaling(5., 5., 5.);
        let c = Transformation::translation(10., 5., 7.);

        let t = c * b * a;

        assert_eq!(t * p, Tuple::point(15., 0., 7.));
    }
}
