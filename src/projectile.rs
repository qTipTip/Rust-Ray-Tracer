use crate::canvas::Canvas;
use crate::color::Color;
use crate::image::write_to_file;
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
pub fn projectile() {
    let e = Environment {
        gravity: Tuple::vector(0.0, -0.1, 0.0),
        wind: Tuple::vector(-0.02, 0.0, 0.0),
    };
    let mut p = Projectile {
        position: Tuple::vector(0.0, 1.0, 0.0),
        velocity: Tuple::vector(1.0, 1.8, 0.0).norm() * 11.25,
    };

    let mut c = Canvas::new(900, 550);
    let color = Color::new(1.0, 1.0, 1.0);


    while p.position.y > 0.0 {
        p = tick(&e, &p);

        let pixel_x = p.position.x.round().clamp(0.0, c.width as f64) as usize;
        let pixel_y = c.height - p.position.y.round().clamp(0.0, c.height as f64) as usize;

        c.write_pixel(pixel_x, pixel_y, color);
    }

    write_to_file(&c, "./examples/projectile.ppm");
}