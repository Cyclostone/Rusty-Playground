use std::io;

fn main() {
    println!("Simple Calculator");
    println!("Available Operations: +, -, *, /");
    println!("Enter your expression (e.g. 5 + 3):");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let tokens: Vec<&str> = input.trim().split_whitespace().collect();

    if tokens.len() != 3 {
        println!("Invalid Input. Please follow the format: number operator number");
        return;
    }

    let num1: f64 = match tokens[0].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid First Number");
            return;
        }
    };

    let operator = tokens[1];

    let num2: f64 = match tokens[2].parse() {
        Ok(num) => num, 
        Err(_) => {
            println!("Invalid Second Number");
            return;
        }
    };

    let result = match operator {
        "+" => add(num1, num2),
        "-" => subtract(num1, num2),
        "*" => multiply(num1, num2),
        "/" => divide(num1, num2),
        _ => {
            println!("Invalid Operator: Use +, -, *, /");
            return;
        }
    };  

    println!("Result: {:.2}", result);
}

fn add(num1: f64, num2: f64) -> f64 {
    num1 + num2
}

fn subtract(num1: f64, num2: f64) -> f64 {
    num1 - num2
}

fn multiply(num1: f64, num2: f64) -> f64 {
    num1 * num2
}

fn divide(num1: f64, num2: f64) -> f64 {
    if num2 == 0.0 {
        println!("Error: Division by zero is not allowed");
        return 0.0;
    }
    num1 / num2
}