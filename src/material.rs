use crate::color::Color;

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Material {
    pub fn new(color: Color, ambient: f64, diffuse: f64, specular: f64, shininess: f64) -> Self {
        Self {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }
    pub fn default() -> Self {
        Self {
            color: Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::lights::{lighting, PointLight};
    use crate::tuple::Tuple;
    use super::*;

    #[test]
    fn lighting_eye_between_source_and_surface() {
        let m = Material::default();
        let pos = Tuple::origin();
        let eye = Tuple::vector(0.0, 0.0, -1.0);
        let normal = Tuple::vector(0.0, 0.0, -1.0);
        let l = PointLight::new(
            Tuple::point(0.0, 0.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        );

        let result = lighting(m, l, pos, eye, normal);

        assert_eq!(result, Color::new(1.9, 1.9, 1.9))
    }

    #[test]
    fn lighting_eye_offset_45_degrees() {
        let m = Material::default();
        let pos = Tuple::origin();
        let eye = Tuple::vector(0.0, f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0);
        let normal = Tuple::vector(0.0, 0.0, -1.0);
        let l = PointLight::new(
            Tuple::point(0.0, 0.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        );

        let result = lighting(m, l, pos, eye, normal);

        assert_eq!(result, Color::new(1.0, 1.0, 1.0))
    }

    #[test]
    fn lighting_light_offset_45_degrees() {
        let m = Material::default();
        let pos = Tuple::origin();
        let eye = Tuple::vector(0.0, 0.0, -1.0);
        let normal = Tuple::vector(0.0, 0.0, -1.0);
        let l = PointLight::new(
            Tuple::point(0.0, 10.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        );

        let result = lighting(m, l, pos, eye, normal);

        assert_eq!(result, Color::new(0.7363961030678927, 0.7363961030678927, 0.7363961030678927))
    }

    #[test]
    fn lighting_eye_in_reflection_vector() {
        let m = Material::default();
        let pos = Tuple::origin();
        let eye = Tuple::vector(0.0, -f64::sqrt(2.0) / 2.0, -f64::sqrt(2.0) / 2.0);
        let normal = Tuple::vector(0.0, 0.0, -1.0);
        let l = PointLight::new(
            Tuple::point(0.0, 10.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        );

        let result = lighting(m, l, pos, eye, normal);

        assert_eq!(result, Color::new(1.6363961030678928, 1.6363961030678928, 1.6363961030678928))
    }

    #[test]
    fn lighting_light_behind_surface() {
        let m = Material::default();
        let pos = Tuple::origin();
        let eye = Tuple::vector(0.0, 0.0, -1.0);
        let normal = Tuple::vector(0.0, 0.0, -1.0);
        let l = PointLight::new(
            Tuple::point(0.0, 0.0, 10.0),
            Color::new(1.0, 1.0, 1.0),
        );

        let result = lighting(m, l, pos, eye, normal);

        assert_eq!(result, Color::new(0.1, 0.1, 0.1))
    }
}

