use crate::canvas::Canvas;
use std::fs::File;
use std::io::Write;

pub fn write_to_file(canvas: &Canvas, path: &str) {
    let mut file = File::create(path).unwrap();
    let ppm_header = create_ppm_header(canvas);
    file.write_all(ppm_header.as_bytes()).unwrap();

    let mut full_ppm_file = "".to_owned();
    for p in &canvas.pixels {
        let rgb_u8 = p.to_tuple();
        full_ppm_file.push_str(&format!("{} {} {} ", rgb_u8[0], rgb_u8[1], rgb_u8[2]))
    }

    let text_wrapped_ppm_file = textwrap::wrap(&full_ppm_file, 70);
    for line in text_wrapped_ppm_file {
        file.write_all(line.as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();
    }
    file.write_all(b"\n").unwrap();
}

fn create_ppm_header(canvas: &Canvas) -> String {
    format!(
        r#"
P3
{} {}
255
"#,
        canvas.width, canvas.height
    )
}

#[cfg(test)]
mod tests {
    use crate::canvas::Canvas;
    use crate::image::create_ppm_header;

    #[test]
    fn test_create_ppm_header() {
        let c = Canvas::new(10, 20);
        let header = create_ppm_header(&c);

        assert_eq!(
            header,
            r#"
P3
10 20
255
"#
        )
    }
}
