use crate::canvas::Canvas;
use crate::color::Color;
use crate::image::write_to_file;
use crate::transformations::rotation_z;
use crate::tuple::Tuple;
use std::f64::consts::PI;

pub fn clock() {
    let mut p = Tuple::point(30.0, 0.0, 0.0);
    let mut c = Canvas::new(100, 100);
    let q = Tuple::point(50.0, 50.0, 0.0);

    let rotation_one_hour = rotation_z(2.0 * PI / 12.0);

    for i in 0..12 {
        let z = p + q;
        let color = Color::new(
            1.0 - 1.0 / (i + 1) as f64,
            1.0 - 1.0 / (i + 1) as f64,
            1.0 / (i + 1) as f64,
        );

        c.write_pixel(z.x as usize, c.height - z.y as usize, &color);
        p = &rotation_one_hour * &p;
    }

    write_to_file(&c, "./examples/clock.ppm");
}
