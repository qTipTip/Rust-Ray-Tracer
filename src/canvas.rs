use std::fs::File;
use std::io::Write;
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

    pub fn write_to_file(&self, path: &str) {
        let mut file = File::create(path).unwrap();
        let ppm_header = self.create_ppm_header();
        file.write_all(ppm_header.as_bytes()).unwrap();

        let mut full_ppm_file = "".to_owned();
        for p in &self.pixels {
            let rgb_u8 = p.to_tuple();
            full_ppm_file.push_str(
                &format!("{} {} {} ", rgb_u8[0], rgb_u8[1], rgb_u8[2])
            )
        }

        let text_wrapped_ppm_file = textwrap::wrap(&full_ppm_file, 70);
        for line in text_wrapped_ppm_file {
            file.write_all(line.as_bytes()).unwrap();
            file.write_all(b"\n").unwrap();
        }
        file.write_all(b"\n").unwrap();
    }

    fn create_ppm_header(&self) -> String {
        format!(r#"
P3
{} {}
255
"#, self.width, self.height)
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

    #[test]
    fn test_write_to_file() {
        let mut c = Canvas::new(10, 20);
        c.write_to_file("./test.ppm");
    }

    #[test]
    fn test_create_ppm_header() {
        let c = Canvas::new(10, 20);
        let header = c.create_ppm_header();

        assert_eq!(header, r#"
P3
10 20
255
"#)
    }
}