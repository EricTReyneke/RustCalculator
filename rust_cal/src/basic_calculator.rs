pub struct MyBasicCalculator {
    pub number1: i32,
    pub number2: i32,
}

impl Default for MyBasicCalculator {
    fn default() -> Self {
        Self { number1: 0, number2: 0 }
    }
}

pub trait MathTrait {
    fn calculate_expression(&mut self, expression: &str);
}

impl MathTrait for MyBasicCalculator {
    fn calculate_expression(&mut self, expression: &str) {
        let mut expression_without_spaces: Vec<char> = expression.chars().filter(|c| !c.is_whitespace()).collect();
        let mut current_numbers = String::new();
        let mut last_operator: Option<char> = None;

        for expression_char in expression_without_spaces {
            if expression_char.is_numeric() {
                current_numbers.push(expression_char);
            } else {
                if let Some(op) = last_operator {
                    let num1 = self.number1;
                    let num2 = current_numbers.parse().expect("Failed to parse number string into i32");
                    let result = self.execute_operation(op, num1, num2);
                    self.number1 = result;
                    current_numbers.clear();
                } else {
                    self.number1 = current_numbers.parse().expect("Failed to parse number string into i32");
                    current_numbers.clear();
                }
                last_operator = Some(expression_char);
            }
        }

        // Handle the last number and operation
        if !current_numbers.is_empty() && last_operator.is_some() {
            self.number2 = current_numbers.parse().expect("Failed to parse number string into i32");
            let result = self.execute_operation(last_operator.unwrap(), self.number1, self.number2);
            println!("Final Result: {}", result);
        }
    }
}

impl MyBasicCalculator{
    fn execute_operation(&mut self, operation: char, num1: i32, num2: i32) -> i32 {
        match operation {
            '+' => self.addition(num1, num2),
            '-' => self.subtraction(num1, num2),
            '*' | 'x' => self.multiply(num1, num2),
            _ => {
                println!("Error: Unknown operation!");
                0
            }
        }
    }

    
    fn addition(&self, num1: i32, num2: i32) -> i32 {
        num1 + num2
    }

    fn subtraction(&self, num1: i32, num2: i32) -> i32 {
        num1 - num2
    }

    fn multiply(&self, num1: i32, num2: i32) -> i32 {
        num1 * num2
    }
}