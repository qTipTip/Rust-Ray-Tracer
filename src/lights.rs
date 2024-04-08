use crate::color::Color;
use crate::tuple::Tuple;

struct PointLight {
    intensity: Color,
    position: Tuple,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_light_has_intensity_and_position() {
        let l = PointLight {
            intensity: Color::new(0.0, 0.0, 0.0),
            position: Tuple::origin()
        };

        assert_eq!(l.intensity, Color::new(0.0, 0.0, 0.0));
        assert_eq!(l.position, Tuple::origin());
    }
}