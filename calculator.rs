use std::fs::OpenOptions;
use std::io::{self, Write};
use std::str::FromStr;

// Function to print the border for the CLI
fn print_border() {
    println!("******************************");
    println!("*   CLI Calculator v1.0     *");
    println!("******************************");
}

// Function to log the calculation history to a file
fn log_history(history: &str) {
    let mut history_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("history.txt")
        .expect("Unable to open history file for writing.");
    
    if let Err(e) = writeln!(history_file, "{}", history) {
        eprintln!("Error writing to history file: {}", e);
    }
}

// Function to perform the calculation
fn calculate(num1: f64, num2: f64, op: char) -> Result<f64, &'static str> {
    match op {
        '+' => Ok(num1 + num2),
        '-' => Ok(num1 - num2),
        '*' => Ok(num1 * num2),
        '/' => {
            if num2 == 0.0 {
                Err("Error: Division by zero!")
            } else {
                Ok(num1 / num2)
            }
        }
        _ => Err("Error: Invalid operator!"),
    }
}

fn main() {
    loop {
        print_border();
        
        println!("Enter calculation (or type 'exit' to quit): ");
        let mut expression = String::new();
        io::stdin().read_line(&mut expression).expect("Failed to read line");
        let expression = expression.trim();
        
        if expression.eq_ignore_ascii_case("exit") {
            break;
        }
        
        let tokens: Vec<&str> = expression.split_whitespace().collect();
        if tokens.len() != 3 {
            eprintln!("Error: Invalid input format. Use 'number operator number'.");
            continue;
        }
        
        let num1 = match f64::from_str(tokens[0]) {
            Ok(n) => n,
            Err(_) => {
                eprintln!("Error: Invalid number format.");
                continue;
            }
        };
        
        let op = tokens[1].chars().next().unwrap_or(' ');
        
        let num2 = match f64::from_str(tokens[2]) {
            Ok(n) => n,
            Err(_) => {
                eprintln!("Error: Invalid number format.");
                continue;
            }
        };
        
        match calculate(num1, num2, op) {
            Ok(result) => {
                println!("Result: {}", result);
                let history = format!("{} {} {} = {}", num1, op, num2, result);
                log_history(&history);
            }
            Err(e) => eprintln!("{}", e),
        }
    }
}
