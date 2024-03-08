mod turtle;
mod lsystem;

fn main() {
    let mut turtle = turtle::Turtle::new(100, 100);
    for _ in 1..=4 {
        turtle.forward(200);
        turtle.turn(90.0);
    }

    turtle.save("img.png".into()).unwrap()
}

