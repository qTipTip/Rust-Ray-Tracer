use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub red: f64,
    pub blue: f64,
    pub green: f64,
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Self { red, green, blue }
    }
}

impl PartialEq for Color {
    fn eq(&self, rhs: &Self) -> bool {
        (self.blue - rhs.blue).abs() < f64::EPSILON
            && (self.red - rhs.red).abs() < f64::EPSILON
            && (self.green - rhs.green).abs() < f64::EPSILON
    }
}


impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Color::new(self.red + rhs.red, self.green + rhs.green, self.blue + rhs.blue)
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Color::new(self.red - rhs.red, self.green - rhs.green, self.blue - rhs.blue)
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Color::new(self.red * rhs.red, self.green * rhs.green, self.blue * rhs.blue)
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Color::new(self.red * rhs, self.green * rhs, self.blue * rhs)
    }
}

#[cfg(test)]
mod tests {
    use crate::color::Color;

    #[test]
    fn colors_are_tuples() {
        let c = Color::new(-0.5, 0.4, 1.7);

        assert_eq!(c.red, -0.5);
        assert_eq!(c.green, 0.4);
        assert_eq!(c.blue, 1.7);
    }


    #[test]
    fn add_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        assert_eq!(c1 + c2, Color::new(1.6, 0.7, 1.0));
    }

    #[test]
    fn subtract_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        assert_eq!(c1 - c2, Color::new(0.2, 0.5, 0.5));
    }

    #[test]
    fn scalar_multiplication() {
        let c1 = Color::new(0.9, 0.6, 0.75);

        assert_eq!(c1 * 2.0, Color::new(1.8, 1.2, 1.5));
    }

    #[test]
    fn color_multiplication() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        assert_eq!(c1 * c2, Color::new(0.63, 0.06, 0.1875))
    }
}