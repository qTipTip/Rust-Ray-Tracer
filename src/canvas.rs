

use crate::color::Color;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height, pixels: vec![Color::new(0.0, 0.0, 0.0); width * height] }
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, c: Color) {
        if x >= self.width || y >= self.height {
            return;
        }

        self.pixels[self.width * y + x] = c
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        self.pixels[self.width * y + x]
    }
}

#[cfg(test)]
mod tests {
    use crate::canvas::Canvas;
    use crate::color::Color;

    #[test]
    fn test_create_canvas() {
        let c = Canvas::new(10, 20);
        let o = Color::new(0.0, 0.0, 0.0);

        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);

        for color in c.pixels {
            assert_eq!(color, o);
        }
    }

    #[test]
    fn test_write_pixel() {
        let mut c = Canvas::new(3, 5);
        let o = Color::new(1.0, 1.0, 1.0);

        c.write_pixel(1, 0, o);
        println!("{:?}", c.pixels);
        assert_eq!(c.get_pixel(1, 0), o);
    }
}