use rust_tracer::{
    canvas::Canvas,
    transformations::Transformation,
    tuples::{Point, Scalar, Tuple},
};
use std::{f64::consts::*, fs::File, io::Write, path::Path};

fn main() {
    let size = 800;
    let radius = 3. * (size as Scalar) / 8.;
    let twelve = Tuple::point(0., 0., 1.);
    let hours: Vec<Point> = (0..12)
        .map(|i| Transformation::rotation_y((i as Scalar) * FRAC_PI_6) * twelve)
        .collect();

    let mut c = Canvas::new(size, size);
    let white = Tuple::color(1., 1., 1.);
    let half = (size as Scalar) / 2.;
    for hour in &hours {
        c.write_pixel(
            (hour.x * radius + half) as usize,
            (hour.z * radius + half) as usize,
            white,
        );
    }

    let ppm = c.to_ppm().unwrap();
    let path = Path::new("clock.ppm");
    let mut file = File::create(&path).unwrap();
    file.write_all(ppm.as_bytes()).unwrap();
}
