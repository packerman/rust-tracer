use std::path::Path;
use std::fs::File;
use crate::tuples::Tuple;
use core::fmt::Error;
use std::fmt::Write;
use std::io::Write as IOWrite;

use crate::tuples::Color;

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

    fn write(&mut self, string: String) -> Result<(), Error> {
        if self.line.len() + self.separator.len() + string.len() > self.max_length {
            self.new_line()?;
        }
        if !self.line.is_empty() {
            write!(self.line, "{}", self.separator)?;
        }
        write!(self.line, "{}", string)?;
        Ok(())
    }

    fn new_line(&mut self) -> Result<(), Error> {
        write!(self.output, "{}\n", self.line)?;
        Ok(self.line.clear())
    }

    fn flush(&mut self) -> Result<(), Error> {
        if !self.line.is_empty() {
            self.new_line()?;
        }
        Ok(())
    }

    fn to_string(&mut self) -> Result<String, Error> {
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
        Canvas { width, height, pixels }
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.pixels[y][x]
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, c: Color) {
        self.pixels[y][x] = c;
    }

    fn f32_to_u8(value: f32) -> u8 {
        (value * 255.0).round() as u8
    }

    pub fn to_ppm(&self) -> Result<String, Error> {
        let mut formatter = PpmFormatter::new(69, String::from(" "));
        formatter.write(String::from("P3"))?;
        formatter.new_line()?;
        formatter.write(format!("{} {}", self.width, self.height))?;
        formatter.new_line()?;
        formatter.write(String::from("255"))?;
        formatter.new_line()?;
        for row in &self.pixels {
            for pixel in row {
                formatter.write(format!("{}", Self::f32_to_u8(pixel.red())))?;
                formatter.write(format!("{}", Self::f32_to_u8(pixel.green())))?;
                formatter.write(format!("{}", Self::f32_to_u8(pixel.blue())))?;
            }
            formatter.new_line()?;
        }
        let result = formatter.to_string()?;
        Ok(result)
    }

    pub fn save_to_file(&self, path: &Path) {
        let ppm = self.to_ppm().unwrap();
        let mut file = File::create(&path).unwrap();
        file.write_all(ppm.as_bytes()).unwrap();
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
        assert_eq!(header_lines,
            vec!["P3",
                "5 3",
                "255"]);
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
        assert_eq!(pixels_lines,
            vec!["255 0 0 0 0 0 0 0 0 0 0 0 0 0 0",
                "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0",
                "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255"]);
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
        assert_eq!(pixels_lines,
            vec!["255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
                "153 255 204 153 255 204 153 255 204 153 255 204 153",
                "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
                "153 255 204 153 255 204 153 255 204 153 255 204 153"]);
    }

    #[test]
    fn ppm_files_are_terminated_by_newline_character() {
        let c = Canvas::new(5, 3);
        let ppm = c.to_ppm().unwrap();
        assert_eq!(ppm.chars().last().unwrap(), '\n');
    }
}
