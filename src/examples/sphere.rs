use lib::{
    canvas::Canvas,
    intersections::hit,
    lights::PointLight,
    materials::Material,
    rays::Ray,
    shapes::Shape,
    tuples::{Scalar, Tuple},
};
use std::{fs::File, io::Write, path::Path};

fn main() {
    let ray_origin = Tuple::point(0., 0., -5.);
    let wall_z = 10.;
    let wall_size = 7.;
    let canvas_pixels = 800;
    let pixel_size = wall_size / canvas_pixels as Scalar;
    let half = wall_size / 2.;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let mut shape = Shape::sphere();

    let mut material = Material::default();
    material.set_color(Tuple::color(1., 0.2, 1.));
    *shape.material_mut() = material;

    let light = PointLight::new(Tuple::point(-10., 10., -10.), Tuple::color(1., 1., 1.));

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * (y as Scalar);
        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * (x as Scalar);
            let position = Tuple::point(world_x, world_y, wall_z);
            let r = Ray::new(ray_origin, (position - ray_origin).normalize());
            let xs = shape.intersect(&r);

            for hit in hit(&xs) {
                let point = r.position(hit.t);
                let normal = hit.object.normal_at(&point);
                let eye = -r.direction;
                let color = hit
                    .object
                    .material()
                    .lighting(&shape, &light, &point, &eye, &normal, false);
                canvas.write_pixel(x, y, color);
            }
        }
    }

    let ppm = canvas.to_ppm().unwrap();
    let path = Path::new("sphere.ppm");
    let mut file = File::create(&path).unwrap();
    file.write_all(ppm.as_bytes()).unwrap();
}
