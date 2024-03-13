mod basic_calculator;
use basic_calculator::{MathTrait, MyBasicCalculator};
use std::io;

fn main() {
    let mut expression = String::new();
    println!("Enter expression:");
    io::stdin().read_line(&mut expression).expect("Failed to read your expression.");

    let mut calculator: Box<dyn MathTrait> = Box::new(MyBasicCalculator::default());
    calculator.calculate_expression(&expression);
}