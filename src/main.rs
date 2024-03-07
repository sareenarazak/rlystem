use image::{DynamicImage, GrayImage, ImageResult, Rgb, RgbImage, RgbaImage};
use std::path::PathBuf;

fn main() {
    let mut turtle = Turtle::new();
    turtle.angle = Direction::Down;
    for _ in 1..=4 {
        turtle.forward(10);
        turtle.turn_left();
    }

    turtle.save("img.png".into()).unwrap()
}

enum Direction {
    Up,
    Down,
    Right,
    Left,
}

struct Turtle {
    buffer: RgbImage,
    x: u32,
    y: u32,
    angle: Direction,
}

impl Turtle {
    fn new() -> Turtle {
        Turtle {
            buffer: RgbImage::new(32, 32),
            x: 0,
            y: 0,
            angle: Direction::Up,
        }
    }

    fn forward(&mut self, distance: u32) {
        let (dx, dy): (i32, i32) = match self.angle {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        for _ in 0..distance {
            self.buffer
                .put_pixel((self.x as i32 + dx) as u32, (self.y as i32 + dy) as u32, Rgb([255, 0, 0]));
            self.x = (self.x as i32 + dx) as u32;
            self.y = (self.y as i32 + dy) as u32;
        }
    }

    fn turn_left(&mut self) {
        self.angle = match self.angle {
            Direction::Right => Direction::Up,
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
        }
    }

    fn turn_right(&mut self) {
        self.angle = match self.angle {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn save(&self, path: PathBuf) -> ImageResult<()> {
        DynamicImage::ImageRgb8(self.buffer.clone()).save(path)
    }
}
