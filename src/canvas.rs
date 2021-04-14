use crate::tuples::color;
use crate::tuples::Color;

struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Vec<Color>>,
}

impl Canvas {

    fn new(width: usize, height: usize) -> Canvas {
        let mut pixels = Vec::with_capacity(height);
        for _ in 0..height {
            let mut row = Vec::with_capacity(width);
            for _ in 0..width {
                row.push(color(0.0, 0.0, 0.0));
            }
            pixels.push(row);
        }
        Canvas { width, height, pixels }
    }

    fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.pixels[y][x]
    }

    fn write_pixel(&mut self, x: usize, y: usize, c: Color) {
        self.pixels[y][x] = c;
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
                assert_eq!(c.pixel_at(x, y), color(0.0, 0.0, 0.0));
            }
        }
    }

    #[test]
    fn writing_pixels() {
        let mut c = Canvas::new(10, 20);
        let red = color(1.0, 0.0, 0.0);
        c.write_pixel(2, 3, red);
        assert_eq!(c.pixel_at(2, 3), red);
    }
}
