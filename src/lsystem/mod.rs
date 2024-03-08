#[cfg(test)]
mod test;

use std::collections::HashMap;
use std::hash::Hash;

pub struct LSystem<T>
where T: PartialEq + Eq + Hash + Clone {
    pub state: Vec<T>,
    rules: HashMap<T, Vec<T>>,
}

impl<T> LSystem<T>
where T: PartialEq + Eq + Hash + Clone {
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
}

