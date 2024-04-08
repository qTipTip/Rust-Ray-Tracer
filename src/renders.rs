use crate::canvas::Canvas;
use crate::color::Color;
use crate::image::write_to_file;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::tuple::Tuple;

pub fn render_sphere() {
    let ray_origin = Tuple::point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;

    let canvas_size = 100;
    let mut c = Canvas::new(canvas_size, canvas_size);

    let pixel_size = wall_size / canvas_size as f64;
    let half = wall_size / 2.0;

    let color = Color::new(1.0, 0.0, 0.0);
    let shape = Sphere::unit();

    for y in 0..c.height {
        let world_y = half - pixel_size * y as f64;
        for x in 0..c.width {
            let world_x = -half + pixel_size * x as f64;

            let position = Tuple::point(world_x, world_y, wall_z);

            let ray = Ray {
                origin: ray_origin,
                direction: (position - ray_origin).norm(),
            };

            let ix = ray.intersect(&shape);
            match ix.get_hit() {
                None => {}
                Some(_) => {
                    c.write_pixel(x, y, color);
                }
            }
        }
    }

    write_to_file(&c, "sphere.ppm");
}