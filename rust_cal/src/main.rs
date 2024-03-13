mod basic_calculator;
use basic_calculator::MyBasicCalculator;
use basic_calculator::MathTrait; // Import the MathTrait trait into scope
use std::io;

fn main() {
    let mut expression = String::new();
    println!("Enter expression:");
    io::stdin().read_line(&mut expression).expect("Failed to read your expression.");

    let mut math = MyBasicCalculator::default();
    math.calculate_expression(&expression);
}