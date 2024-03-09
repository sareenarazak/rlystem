use crate::turtle;

#[test]
fn test_turtle() {
    let mut turtle = turtle::Turtle::new(100, 100);
    turtle.forward(10);

    assert_eq!(turtle.x, 0);
    assert_eq!(turtle.y, 10);

    turtle.turn(180.0);
    assert_eq!(turtle.angle, std::f64::consts::PI);
}