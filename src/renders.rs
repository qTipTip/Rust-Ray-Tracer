use crate::canvas::Canvas;
use crate::color::Color;
use crate::image::write_to_file;
use crate::lights::{lighting, PointLight};
use crate::material::Material;
use crate::normals::Normal;
use crate::ray::{Intersect, Ray};
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
                    c.write_pixel(x, y, &color);
                }
            }
        }
    }

    write_to_file(&c, "sphere.ppm");
}

pub fn render_sphere_with_shading() {
    let ray_origin = Tuple::point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;

    let canvas_size = 500;
    let mut c = Canvas::new(canvas_size, canvas_size);

    let pixel_size = wall_size / canvas_size as f64;
    let half = wall_size / 2.0;

    let mut shape = Sphere::unit();
    let mut material = Material::default();
    material.color = Color::new(1.0, 0.0, 0.0);
    shape.set_material(material);

    let light_position = Tuple::point(-10.0, 10.0, -10.0);
    let light_color = Color::new(1.0, 1.0, 1.0);
    let light = PointLight::new(light_position, light_color);

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
                Some(hit) => {
                    let intersection_point = ray.position(hit.time);
                    let normal_vector = hit.object.normal_at(&intersection_point);
                    let eye_vector = -ray.direction;

                    let color = lighting(
                        &hit.object.get_material().unwrap(),
                        &light,
                        &intersection_point,
                        &eye_vector,
                        &normal_vector,
                    );

                    c.write_pixel(x, y, &color);
                }
            }
        }
    }

    write_to_file(&c, "../examples/ppm/sphere_shaded.ppm");
}
