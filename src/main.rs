use std::collections::HashMap;

mod turtle;
mod lsystem;

fn main() {
    let mut rules = HashMap::new();

    rules.insert('F', "F+F-F-F+F".chars().collect());
    let mut koch_curve = lsystem::LSystem::new(vec!['F'], rules.clone());

    koch_curve.run(10);
    koch_curve.draw("img.png".into()).unwrap();


}

