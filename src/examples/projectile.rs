use std::io::Write;
use std::fs::File;
use std::path::Path;
use lib::tuples::color;
use lib::tuples::vector;
use lib::tuples::Point;
use lib::tuples::Vector;
use lib::tuples::point;
use lib::canvas::Canvas;

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
        position: point(0.0, 1.0, 0.0),
        velocity: vector(1.0, 1.8, 0.0).normalize() * 11.25,
    };
    let e = Environment {
        gravity: vector(0.0, -0.1, 0.0),
        wind: vector(-0.01, 0.0, 0.0),
    };
    let mut c = Canvas::new(900, 550);
    let red = color(1.0, 0.0, 0.0);
    loop {
        c.write_pixel(p.position.x() as usize, 550 - 1 - p.position.y() as usize, red);
        p = e.tick(&p);
        if p.position.y() < 0.0 {
            break;
        }
    }
    let ppm = c.to_ppm().unwrap();
    let path = Path::new("projectile.ppm");
    let mut file = File::create(&path).unwrap();
    file.write_all(ppm.as_bytes()).unwrap();
}
