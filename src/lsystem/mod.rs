#[cfg(test)]
mod test;

use std::collections::HashMap;
use std::hash::Hash;
use std::path::PathBuf;
use image::ImageError;
use crate::turtle;
use crate::turtle::Turtle;

trait LSystemSymbol {
    fn run_turtle(&self, turtle: &mut Turtle);
}

impl LSystemSymbol for char {
    fn run_turtle(&self, turtle: &mut Turtle) {
        match self {
            'F' => turtle.forward(4),
            '+' => turtle.turn(90.0),
            '-' => turtle.turn(-90.0),
            _ => ()
        }
    }
}

pub struct LSystem<T>
where T: PartialEq + Eq + Hash + Clone + LSystemSymbol {
    pub state: Vec<T>,
    rules: HashMap<T, Vec<T>>,
}

impl<T> LSystem<T>
where T: PartialEq + Eq + Hash + Clone + LSystemSymbol {
    pub fn new(state: Vec<T>, rules: HashMap<T, Vec<T>>) -> LSystem<T> {
        LSystem{state, rules}
    }

    pub fn update(&mut self) {
        let mut new_state = vec![];
        for x in &self.state {
            let mut part = match self.rules.get(&x) {
                Some(part) => part.to_vec(),
                None => vec![x.clone()],
            };
            new_state.append(&mut part);
        }
        self.state = new_state;
    }

    pub fn run(&mut self, n: usize) {
        for _ in 1..=n {
            self.update();
        }
    }

    pub fn draw(&self, path: PathBuf) -> Result<(), ImageError> {
        let mut turtle = turtle::Turtle::new(1000, 1000);
        for x in &self.state {
            x.run_turtle(&mut turtle);
        }

        turtle.save(path)
    }
}

