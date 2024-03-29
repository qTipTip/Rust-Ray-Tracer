struct Color {
    red: f64,
    blue: f64,
    green: f64,
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Self { red, green, blue }
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
}