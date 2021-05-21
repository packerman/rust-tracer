use lib::shapes::Shape;
use lib::tuples::Scalar;
use lib::intersections::hit;
use std::io::Write;
use std::fs::File;
use std::path::Path;
use lib::rays::Ray;
use lib::spheres::Sphere;
use lib::canvas::Canvas;
use lib::tuples::Tuple;

fn main() {
    let ray_origin = Tuple::point(0., 0., -5.);
    let wall_z = 10.;
    let wall_size = 7.;
    let canvas_pixels = 800;
    let pixel_size = wall_size / canvas_pixels as Scalar;
    let half = wall_size / 2.;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let color = Tuple::color(1., 0., 0.);
    let shape = Sphere::new();

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * (y as Scalar);
        for x in 0..canvas_pixels {
            let world_x = - half + pixel_size * (x as Scalar);
            let position = Tuple::point(world_x, world_y, wall_z);
            let r = Ray::new(ray_origin, (position - ray_origin).normalize());
            let xs = (&shape as &dyn Shape).intersect(&r);

            if hit(&xs).is_some() {
                canvas.write_pixel(x, y, color);
            }
        }
    }

    let ppm = canvas.to_ppm().unwrap();
    let path = Path::new("silhouette.ppm");
    let mut file = File::create(&path).unwrap();
    file.write_all(ppm.as_bytes()).unwrap();
}
