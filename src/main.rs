use image::{DynamicImage, GrayImage, ImageResult, Rgb, RgbImage, RgbaImage};
use std::path::PathBuf;
use num_traits::float::FloatConst;

fn main() {
    let mut turtle = Turtle::new();
    turtle.x = 0;
    turtle.y = 0;
    for _ in 1..=4 {
        turtle.forward(10);
        turtle.turn(90.0);
    }

    turtle.save("img.png".into()).unwrap()
}

struct Turtle {
    buffer: RgbImage,
    x: u32,
    y: u32,
    angle: f64,
}

impl Turtle {
    fn new() -> Turtle {
        Turtle {
            buffer: RgbImage::new(32, 32),
            x: 0,
            y: 0,
            angle: 0.0,
        }
    }

    fn forward(&mut self, distance: u32) {
        let new_x = (self.x as i32 + (f64::sin(self.angle) * distance as f64) as i32) as u32;
        let new_y = (self.y as i32 + (f64::cos(self.angle) * distance as f64) as i32) as u32;
        self.plot_line(self.x, self.y, new_x, new_y);
        self.x = new_x;
        self.y = new_y;
    }

    fn turn(& mut self, angle: f64) {
        self.angle = (self.angle + (angle / 360.0 * f64::TAU())) % f64::TAU();
    }


    // https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
    fn plot_line(&mut self, x0: u32, y0: u32, x1: u32, y1: u32) {
        let mut x0 = x0;
        let mut y0 = y0;
        let dx = i32::abs(x1 as i32 - x0 as i32);
        let sx: i32 = if x0 < x1 { 1 } else { -1 };

        let dy = -i32::abs(y1 as i32 - y0 as i32);
        let sy: i32 = if y0 < y1 { 1 } else { -1 };

        let mut error = dx + dy;

        loop {
            self.buffer.put_pixel(x0, y0, Rgb([255, 0, 0]));
            if x0 == y0 && y0 == y1 {
                break
            }
            let e2 = 2 * error;
            if e2 >= dy {
                if x0 == x1 { break }
                error = error + dy;
                x0 = (x0 as i32 + sx) as u32;
            }
            if e2 <= dx {
                if y0 == y1 { break }
                error = error + dx;
                y0 = (y0 as i32 + sy) as u32;
            }
        }
    }

    fn save(&self, path: PathBuf) -> ImageResult<()> {
        DynamicImage::ImageRgb8(self.buffer.clone()).save(path)
    }
}
