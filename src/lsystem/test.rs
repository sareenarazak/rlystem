use std::collections::HashMap;
use crate::lsystem::LSystem;

#[test]
fn test_koch_curve() {
    let mut rules = HashMap::new();

    rules.insert('F', "F+F-F-F+F".chars().collect());
    let mut koch_curve = LSystem::new(vec!['F'], rules.clone());
    koch_curve.update();
    assert_eq!(koch_curve.state,  "F+F-F-F+F".chars().collect::<Vec<_>>());

    koch_curve.update();
    assert_eq!(koch_curve.state, "F+F-F-F+F+F+F-F-F+F-F+F-F-F+F-F+F-F-F+F+F+F-F-F+F".chars().collect::<Vec<_>>());

    let mut koch_curve = LSystem::new(vec!['F'], rules);
    koch_curve.run(1);
    assert_eq!(koch_curve.state,  "F+F-F-F+F".chars().collect::<Vec<_>>());
}