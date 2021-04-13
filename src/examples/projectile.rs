use lib::tuples::vector;
use lib::tuples::Point;
use lib::tuples::Vector;
use lib::tuples::point;

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
        velocity: vector(1.0, 1.0, 0.0).normalize(),
    };
    let e = Environment {
        gravity: vector(0.0, -0.1, 0.0),
        wind: vector(-0.01, 0.0, 0.0),
    };
    let mut t = 0;
    loop {
        println!("{}: {:?}", t, p.position);
        p = e.tick(&p);
        if p.position.y() < 0.0 {
            break;
        }
        t += 1;
    }
}
