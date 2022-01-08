use crate::canvas::Canvas;
use crate::rays::Ray;
use crate::transformations::Transformation;
use crate::tuples::Scalar;
use crate::tuples::Tuple;
use crate::world::World;

pub struct Camera {
    hsize: usize,
    vsize: usize,
    field_of_view: Scalar,
    transform: Transformation,
    inversed_transform: Transformation,
    pixel_size: Scalar,
    half_width: Scalar,
    half_height: Scalar,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: Scalar) -> Camera {
        let half_view = (field_of_view / 2.).tan();
        let aspect = (hsize as Scalar) / (vsize as Scalar);

        let half_width: Scalar;
        let half_height: Scalar;
        if aspect >= 1. {
            half_width = half_view;
            half_height = half_view / aspect;
        } else {
            half_width = half_view * aspect;
            half_height = half_view;
        }

        Camera {
            hsize,
            vsize,
            field_of_view,
            transform: Transformation::IDENTITY,
            inversed_transform: Transformation::IDENTITY,
            pixel_size: (half_width * 2.) / (hsize as Scalar),
            half_width,
            half_height,
        }
    }

    fn transform(&self) -> &Transformation {
        &self.transform
    }

    pub fn set_transform(&mut self, transform: Transformation) {
        self.transform = transform;
        self.inversed_transform = transform.inverse();
    }

    fn ray_for_pixel(&self, px: usize, py: usize) -> Ray {
        let xoffset = (px as Scalar + 0.5) * self.pixel_size;
        let yoffset = (py as Scalar + 0.5) * self.pixel_size;

        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        let pixel = self.inversed_transform * Tuple::point(world_x, world_y, -1.);
        let origin = self.inversed_transform * Tuple::point(0., 0., 0.);
        let direction = (pixel - origin).normalize();

        Ray::new(origin, direction)
    }

    pub fn render(&self, world: &World) -> Canvas {
        let mut image = Canvas::new(self.hsize, self.vsize);

        for y in 0..self.vsize {
            for x in 0..self.hsize {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(&ray);
                image.write_pixel(x, y, color);
            }
        }

        image
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tuples::Tuple;
    use approx::assert_abs_diff_eq;
    use std::f64::consts::*;

    #[test]
    fn constructing_a_camera() {
        let hsize = 160;
        let vsize = 120;
        let field_of_view = FRAC_PI_2;

        let c = Camera::new(hsize, vsize, field_of_view);

        assert_eq!(c.hsize, 160);
        assert_eq!(c.vsize, 120);
        assert_eq!(c.field_of_view, FRAC_PI_2);
        assert_eq!(c.transform(), &Transformation::IDENTITY);
    }

    #[test]
    fn the_pixel_size_for_a_horizontal_canvas() {
        let c = Camera::new(200, 125, FRAC_PI_2);

        assert_abs_diff_eq!(c.pixel_size, 0.01);
    }

    #[test]
    fn the_pixel_size_for_a_vertical_canvas() {
        let c = Camera::new(125, 200, FRAC_PI_2);

        assert_abs_diff_eq!(c.pixel_size, 0.01);
    }

    #[test]
    fn constructing_a_ray_through_the_center_of_the_canvas() {
        let c = Camera::new(201, 101, FRAC_PI_2);

        let r = c.ray_for_pixel(100, 50);

        assert_eq!(r.origin, Tuple::point(0., 0., 0.));
        assert_abs_diff_eq!(r.direction, Tuple::vector(0., 0., -1.));
    }

    #[test]
    fn constructing_a_ray_through_the_corner_of_the_canvas() {
        let c = Camera::new(201, 101, FRAC_PI_2);

        let r = c.ray_for_pixel(0, 0);

        assert_eq!(r.origin, Tuple::point(0., 0., 0.));
        assert_abs_diff_eq!(
            r.direction,
            Tuple::vector(0.66519, 0.33259, -0.66851),
            epsilon = 0.00001
        );
    }

    #[test]
    fn constructing_a_ray_when_the_camera_is_transformed() {
        let mut c = Camera::new(201, 101, FRAC_PI_2);
        c.set_transform(
            Transformation::rotation_y(FRAC_PI_4) * Transformation::translation(0., -2., 5.),
        );

        let r = c.ray_for_pixel(100, 50);

        assert_abs_diff_eq!(r.origin, Tuple::point(0., 2., -5.), epsilon = 0.000001);
        assert_abs_diff_eq!(r.direction, Tuple::vector(SQRT_2 / 2., 0., -SQRT_2 / 2.));
    }

    #[test]
    fn render_a_world_with_camera() {
        let w = World::default();
        let mut c = Camera::new(11, 11, FRAC_PI_2);
        let from = Tuple::point(0., 0., -5.);
        let to = Tuple::point(0., 0., 0.);
        let up = Tuple::vector(0., 1., 0.);
        c.set_transform(Transformation::view(&from, &to, &up));

        let image = c.render(&w);
        assert_abs_diff_eq!(
            image.pixel_at(5, 5),
            Tuple::color(0.38066, 0.47583, 0.2855),
            epsilon = 0.00001
        );
    }
}
