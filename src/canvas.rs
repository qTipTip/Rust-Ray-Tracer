use std::ops::Index;
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
        self.pixels[x * self.height + y] = c
    }
}

impl Index<usize> for Canvas {
    type Output = [Color];

    fn index(&self, index: usize) -> &Self::Output {
        &self.pixels[index * self.height..(index + 1) * self.height]
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
    fn test_index_canvas() {
        let c = Canvas::new(2, 3);
        let o = Color::new(0.0, 0.0, 0.0);
        let row = [o, o, o];

        for i in 0..3 {
            assert_eq!(c[0][i], row[i]);
        }

        assert_eq!(c[0][0], o)
    }

    #[test]
    fn test_write_pixel() {
        let mut c = Canvas::new(10, 20);
        let o = Color::new(1.0, 1.0, 1.0);

        c.write_pixel(2, 4, o);

        assert_eq!(c[2][4], o);
    }
}