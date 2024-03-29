mod tuple;
mod color;
mod canvas;
mod image;

use crate::tuple::Tuple;

struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

fn tick(environment: &Environment, projectile: &Projectile) -> Projectile {
    Projectile {
        position: projectile.position + projectile.velocity,
        velocity: projectile.velocity + environment.gravity + environment.wind,
    }
}

fn main() {
    let e = Environment {
        gravity: Tuple::vector(0.0, -0.1, 0.0),
        wind: Tuple::vector(-0.01, 0.0, 0.0),
    };
    let mut p = Projectile {
        position: Tuple::vector(0.0, 1.0, 0.0),
        velocity: Tuple::vector(1.0, 1.0, 0.0).norm(),
    };

    while p.position.y > 0.0 {
        println!("{:?}", p.position);
        p = tick(&e, &p);
    }
}
