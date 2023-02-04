use crate::tuples::{Color, Scalar, Tuple};
use std::{
    error,
    ffi::OsStr,
    fmt::Write,
    fs::File,
    io::{BufWriter, Write as IOWrite},
    path::Path,
};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

struct PpmFormatter {
    max_length: usize,
    separator: String,
    output: String,
    line: String,
}

impl PpmFormatter {
    fn new(max_length: usize, separator: String) -> PpmFormatter {
        PpmFormatter {
            max_length,
            separator,
            output: String::new(),
            line: String::new(),
        }
    }

    fn write(&mut self, string: String) -> Result<()> {
        if self.line.len() + self.separator.len() + string.len() > self.max_length {
            self.new_line()?;
        }
        if !self.line.is_empty() {
            write!(self.line, "{}", self.separator)?;
        }
        write!(self.line, "{}", string)?;
        Ok(())
    }

    fn new_line(&mut self) -> Result<()> {
        write!(self.output, "{}\n", self.line)?;
        Ok(self.line.clear())
    }

    fn flush(&mut self) -> Result<()> {
        if !self.line.is_empty() {
            self.new_line()?;
        }
        Ok(())
    }

    fn to_string(&mut self) -> Result<String> {
        self.flush()?;
        Ok(self.output.clone())
    }
}

pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Vec<Color>>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        let mut pixels = Vec::with_capacity(height);
        for _ in 0..height {
            let mut row = Vec::with_capacity(width);
            for _ in 0..width {
                row.push(Tuple::color(0.0, 0.0, 0.0));
            }
            pixels.push(row);
        }
        Canvas {
            width,
            height,
            pixels,
        }
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.pixels[y][x]
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, c: Color) {
        self.pixels[y][x] = c;
    }

    fn scalar_to_u8(value: Scalar) -> u8 {
        (value * 255.0).round() as u8
    }

    pub fn to_ppm(&self) -> Result<String> {
        let mut formatter = PpmFormatter::new(69, String::from(" "));
        formatter.write(String::from("P3"))?;
        formatter.new_line()?;
        formatter.write(format!("{} {}", self.width, self.height))?;
        formatter.new_line()?;
        formatter.write(String::from("255"))?;
        formatter.new_line()?;
        for row in &self.pixels {
            for pixel in row {
                formatter.write(format!("{}", Self::scalar_to_u8(pixel.red())))?;
                formatter.write(format!("{}", Self::scalar_to_u8(pixel.green())))?;
                formatter.write(format!("{}", Self::scalar_to_u8(pixel.blue())))?;
            }
            formatter.new_line()?;
        }
        let result = formatter.to_string()?;
        Ok(result)
    }

    pub fn to_rgb_vec(&self) -> Vec<u8> {
        let mut result = Vec::with_capacity(self.width * self.height * 3);
        for row in &self.pixels {
            for pixel in row {
                result.push(Self::scalar_to_u8(pixel.red()));
                result.push(Self::scalar_to_u8(pixel.green()));
                result.push(Self::scalar_to_u8(pixel.blue()));
            }
        }
        result
    }

    fn save_to_ppm(&self, path: &Path) -> Result<()> {
        let ppm = self.to_ppm()?;
        let mut file = File::create(&path)?;
        file.write_all(ppm.as_bytes())?;
        Ok(())
    }

    fn save_to_png(&self, path: &Path) -> Result<()> {
        let file = File::create(path)?;
        let ref mut w = BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, self.width as u32, self.height as u32);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header()?;

        let data = self.to_rgb_vec();
        writer.write_image_data(&data)?;
        Ok(())
    }

    pub fn save_to_file(&self, path: &Path) -> Result<()> {
        match path.extension() {
            Some(ext) => {
                if ext == OsStr::new("ppm") {
                    self.save_to_ppm(path)
                } else if ext == OsStr::new("png") {
                    self.save_to_png(path)
                } else {
                    Err(format!("Unsupported extension: {:?}", ext).into())
                }
            }
            None => Err("Unspecified extension".into()),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn creating_canvas() {
        let width = 10;
        let height = 20;
        let c = Canvas::new(width, height);
        for x in 0..width {
            for y in 0..height {
                assert_eq!(c.pixel_at(x, y), Tuple::color(0.0, 0.0, 0.0));
            }
        }
    }

    #[test]
    fn writing_pixels() {
        let mut c = Canvas::new(10, 20);
        let red = Tuple::color(1.0, 0.0, 0.0);
        c.write_pixel(2, 3, red);
        assert_eq!(c.pixel_at(2, 3), red);
    }

    #[test]
    fn constructing_ppm_header() {
        let c = Canvas::new(5, 3);
        let ppm = c.to_ppm().unwrap();
        let header_lines: Vec<&str> = ppm.lines().take(3).collect();
        assert_eq!(header_lines, vec!["P3", "5 3", "255"]);
    }

    #[test]
    fn construsting_ppm_pixel_data() {
        let mut c = Canvas::new(5, 3);
        let c1 = Tuple::color(1.5, 0.0, 0.0);
        let c2 = Tuple::color(0.0, 0.5, 0.0);
        let c3 = Tuple::color(-0.5, 0.0, 1.0);

        c.write_pixel(0, 0, c1);
        c.write_pixel(2, 1, c2);
        c.write_pixel(4, 2, c3);
        let ppm = c.to_ppm().unwrap();

        let pixels_lines: Vec<&str> = ppm.lines().skip(3).take(3).collect();
        assert_eq!(
            pixels_lines,
            vec![
                "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0",
                "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0",
                "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255"
            ]
        );
    }

    #[test]
    fn splitting_long_lines_in_ppm_files() {
        let mut c = Canvas::new(10, 2);
        for x in 0..10 {
            for y in 0..2 {
                c.write_pixel(x, y, Tuple::color(1.0, 0.8, 0.6))
            }
        }

        let ppm = c.to_ppm().unwrap();

        let pixels_lines: Vec<&str> = ppm.lines().skip(3).take(4).collect();
        assert_eq!(
            pixels_lines,
            vec![
                "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
                "153 255 204 153 255 204 153 255 204 153 255 204 153",
                "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
                "153 255 204 153 255 204 153 255 204 153 255 204 153"
            ]
        );
    }

    #[test]
    fn ppm_files_are_terminated_by_newline_character() {
        let c = Canvas::new(5, 3);
        let ppm = c.to_ppm().unwrap();
        assert_eq!(ppm.chars().last().unwrap(), '\n');
    }

    #[test]
    fn convert_to_u8_array() {
        let mut c = Canvas::new(3, 2);
        c.write_pixel(0, 0, Tuple::color(1., 0., 0.));
        c.write_pixel(1, 0, Tuple::color(0., 1., 0.));
        c.write_pixel(2, 0, Tuple::color(1., 1., 1.));
        c.write_pixel(0, 1, Tuple::color(0., 0., 1.));
        c.write_pixel(1, 1, Tuple::color(1., 1., 0.));
        c.write_pixel(2, 1, Tuple::color(0., 0., 0.));

        let data = c.to_rgb_vec();

        assert_eq!(
            data,
            vec![255, 0, 0, 0, 255, 0, 255, 255, 255, 0, 0, 255, 255, 255, 0, 0, 0, 0]
        );
    }
}
