#[cfg(test)]
mod test;

use std::path::PathBuf;
use image::{DynamicImage, ImageResult, Rgb, RgbImage};
use core::f64::consts::TAU;

pub struct Turtle {
    buffer: RgbImage,
    pub x: i64,
    pub y: i64,
    pub angle: f64,
}

impl Turtle {
    pub fn new(width: u32, height: u32) -> Turtle {
        Turtle {
            buffer: RgbImage::new(width, height),
            x: 0,
            y: 0,
            angle: 0.0,
        }
    }

    pub fn forward(&mut self, distance: u32) {
        let new_x = self.x + (f64::sin(self.angle) * distance as f64)as i64;
        let new_y = self.y + (f64::cos(self.angle) * distance as f64) as i64;
        self.plot_line(self.x, self.y, new_x, new_y);
        self.x = new_x;
        self.y = new_y;
    }

    pub fn turn(&mut self, angle: f64) {
        self.angle = (self.angle + (angle / 360.0 * TAU)) % TAU;
    }

    // https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
    fn plot_line(&mut self, x0: i64, y0: i64, x1: i64, y1: i64) {
        let mut x0 = x0;
        let mut y0 = y0;

        let dx = i64::abs(x1 - x0);
        let dy = -i64::abs(y1 - y0);

        let mut error = dx + dy;

        let sx: i64 = if x0 < x1 { 1 } else { -1 };
        let sy: i64 = if y0 < y1 { 1 } else { -1 };

        loop {
            self.put_pixel(x0, y0);

            if x0 == y0 && y0 == y1 {
                break
            }

            let e2 = 2 * error;

            if e2 >= dy {
                if x0 == x1 { break }
                error = error + dy;
                x0 = x0  + sx;
            }

            if e2 <= dx {
                if y0 == y1 { break }
                error = error + dx;
                y0 = y0 + sy;
            }
        }
    }

    fn put_pixel(&mut self, x: i64, y: i64) {
        if x >= 0 && x < self.buffer.width() as i64 && y >= 0 && y < self.buffer.height() as i64 {
            self.buffer.put_pixel(x as u32, y as u32, Rgb([255, 0, 0]));
        }
    }

    pub fn save(&self, path: PathBuf) -> ImageResult<()> {
        DynamicImage::ImageRgb8(self.buffer.clone()).save(path)
    }
}