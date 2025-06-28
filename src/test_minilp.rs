// Simple test to understand minilp API
use minilp::{Problem, OptimizationDirection};

fn test_api() {
    let mut problem = Problem::new(OptimizationDirection::Minimize);
    
    // Test adding a variable
    let x1 = problem.add_var(1.0, (0.0, 1.0)); // coefficient 1.0, bounds (0,1)
    let x2 = problem.add_var(2.0, (0.0, f64::INFINITY)); // coefficient 2.0, bounds (0,inf)
    
    println!("Variable x1: {:?}", x1);
    println!("Variable x2: {:?}", x2);
}
