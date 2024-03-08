use std::collections::HashMap;

mod turtle;
mod lsystem;

fn main() {
    let mut rules = HashMap::new();

    rules.insert('F', "F+F-F-F+F".chars().collect());
    let mut koch_curve = lsystem::LSystem::new(vec!['F'], rules.clone());

    koch_curve.run(3);

    let mut turtle = turtle::Turtle::new(100, 100);
    for x in koch_curve.state {
        match x {
            'F' => turtle.forward(3),
            '+' => turtle.turn(90.0),
            '-' => turtle.turn(-90.0),
            _ => panic!("Invalid character")
        }
    }

    turtle.save("img.png".into()).unwrap()
}

