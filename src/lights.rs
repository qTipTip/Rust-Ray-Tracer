use crate::color::Color;
use crate::material::Material;
use crate::tuple::Tuple;

#[derive(Clone, Copy)]
pub struct PointLight {
    position: Tuple,
    intensity: Color,

}

impl PointLight {
    pub fn new(position: Tuple, intensity: Color) -> Self {
        Self { position, intensity }
    }
}

pub fn lighting(material: Material, light: PointLight, point: Tuple, eye_vector: Tuple, normal_vector: Tuple) -> Color {
    let effective_color = material.color * light.intensity;
    let light_vector = (light.position - point).norm();
    let ambient = effective_color * material.ambient;

    let light_dot_normal = light_vector.dot(normal_vector);

    let (diffuse, specular) = {
        if light_dot_normal < 0.0 {
            let diffuse = 0.0;
            let specular = 0.0;

            (Color::black(), Color::black())
        } else {
            let diffuse = effective_color * material.diffuse * light_dot_normal;
            let reflect_vector = (-light_vector).reflect(normal_vector);
            let reflect_dot_eye = reflect_vector.dot(eye_vector);

            let specular = {
                if reflect_dot_eye <= 0.0 {
                    Color::black()
                } else {
                    let factor = reflect_dot_eye.powf(material.shininess);
                    light.intensity * material.specular * factor
                }
            };
            (diffuse, specular)
        }
    };
    ambient + diffuse + specular
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_light_has_intensity_and_position() {
        let l = PointLight {
            intensity: Color::new(0.0, 0.0, 0.0),
            position: Tuple::origin(),
        };

        assert_eq!(l.intensity, Color::new(0.0, 0.0, 0.0));
        assert_eq!(l.position, Tuple::origin());
    }
}