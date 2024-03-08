use std::collections::HashMap;

mod turtle;
mod lsystem;

fn main() {
    let mut rules = HashMap::new();

    rules.insert('F', "F+F-F-F+F".chars().collect());
    let mut koch_curve = lsystem::LSystem::new(vec!['F'], rules.clone());

    koch_curve.run(10);
    koch_curve.draw("img.png".into()).unwrap();

    let mut rules = HashMap::new();

    rules.insert('F', "FF".chars().collect());
    rules.insert('X', "F+[[X]-X]-F[-FX]+X".chars().collect());

    let mut plant = lsystem::LSystem::new(vec!['X'], rules.clone());
    plant.run(6);

    let mut turtle = turtle::Turtle::new(1000, 1000);
    turtle.angle = 45.0;

    let mut stack = vec![];


    for x in &plant.state {
        match x {
            'F' => turtle.forward(8),
            '+' => turtle.turn(25.0),
            '-' => turtle.turn(-25.0),
            '[' => stack.push((turtle.angle, turtle.x, turtle.y)),
            ']' => {
                let (angle, x, y) = stack.pop().unwrap();
                turtle.angle = angle;
                turtle.x = x;
                turtle.y = y;
            },
            _ => ()
        }
    }

    turtle.save("img.png".into()).unwrap()


}

