use rust_tracer::{
    canvas::Canvas,
    tuples::{Point, Tuple, Vector},
};
use std::{fs::File, io::Write, path::Path};

struct Projectile {
    position: Point,
    velocity: Vector,
}

struct Environment {
    gravity: Vector,
    wind: Vector,
}

impl Environment {
    fn tick(&self, proj: &Projectile) -> Projectile {
        Projectile {
            position: proj.position + proj.velocity,
            velocity: proj.velocity + self.gravity + self.wind,
        }
    }
}

fn main() {
    let mut p = Projectile {
        position: Tuple::point(0.0, 1.0, 0.0),
        velocity: Tuple::vector(1.0, 1.8, 0.0).normalize() * 11.25,
    };
    let e = Environment {
        gravity: Tuple::vector(0.0, -0.1, 0.0),
        wind: Tuple::vector(-0.01, 0.0, 0.0),
    };
    let mut c = Canvas::new(900, 550);
    let red = Tuple::color(1.0, 0.0, 0.0);
    loop {
        c.write_pixel(p.position.x as usize, 550 - 1 - p.position.y as usize, red);
        p = e.tick(&p);
        if p.position.y < 0.0 {
            break;
        }
    }
    let ppm = c.to_ppm().unwrap();
    let path = Path::new("projectile.ppm");
    let mut file = File::create(&path).unwrap();
    file.write_all(ppm.as_bytes()).unwrap();
}
