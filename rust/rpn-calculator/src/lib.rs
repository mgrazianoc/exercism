#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    if inputs.is_empty(){
        return None
    }
    
    let mut stack: Vec<i32> = Vec::new();
    let mut valid: bool = true;

    /*
        input: // 4 8 + 7 5 - /
        stack:
            [], [4], [4, 8], [4, 8, +], [12], [12, 7], [12, 7, 5], 
            [12, 7, 5, -], [12, 2], [12, 2, /], [6]
    */

    for input in inputs.iter(){
        match input {
            CalculatorInput::Add => {
                let second = stack.pop();
                let first = stack.pop();

                match (first, second) {
                    (Some(a), Some(b)) => stack.push(a + b),
                    _ => {
                        valid = false;
                        break
                    }
                }
                
            },
            CalculatorInput::Subtract => {
                let second = stack.pop();
                let first = stack.pop();

                match (first, second) {
                    (Some(a), Some(b)) => stack.push(a - b),
                    _ => {
                        valid = false;
                        break
                    }
                }
            },
            CalculatorInput::Multiply => {
                let second = stack.pop();
                let first = stack.pop();

                match (first, second) {
                    (Some(a), Some(b)) => stack.push(a * b),
                    _ => {
                        valid = false;
                        break
                    }
                }
            },
            CalculatorInput::Divide => {
                let second = stack.pop();
                let first = stack.pop();

                match (first, second) {
                    (Some(a), Some(b)) => stack.push(a / b),
                    _ => {
                        valid = false;
                        break
                    }
                }
            },
            CalculatorInput::Value(y) => stack.push(*y)
        }
    }

    if valid && stack.len() == 1 {
        Some(stack.pop().unwrap())
    }else{
        None
    }

}
