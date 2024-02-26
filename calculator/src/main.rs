use std::io;

fn main() {
    let mut num1: String = String::new();
    let mut num2: String = String::new();
    let mut choice: String = String::new();

    println!("Welcome to calculator! Please enter a number; 1. Addition, 2: Subtraction, 3: Multiplication, 4: Division");
    println!("Make a choice on the type of operation");

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    // Parse the input string into a u32
    let choice: u32 = match choice.trim().parse() {
        Ok(choice) => choice,
        Err(_) => {
            println!("Invalid input. Please enter a valid unsigned integer.");
            return;
        }
    };

    println!("Enter number 1");
    io::stdin()
        .read_line(&mut num1)
        .expect("Failed to read line");

    // Parse the input string into a u32
    let num1: u32 = match num1.trim().parse() {
        Ok(num1) => num1,
        Err(_) => {
            println!("Invalid input. Please enter a valid unsigned integer.");
            return;
        }
    };

    println!("Enter number 2");
    io::stdin()
        .read_line(&mut num2)
        .expect("Failed to read line");

    // Parse the input string into a u32
    let num2: u32 = match num2.trim().parse() {
        Ok(num2) => num2,
        Err(_) => {
            println!("Invalid input. Please enter a valid unsigned integer.");
            return;
        }
    };

    if choice == 1 {
        let result = num1 + num2;
        println!("Your result is: {result} ");
    }

    if choice == 2 {
        let result = num1 - num2;
        println!("Your result is: {result} ");
    }

    if choice == 3 {
        let result = num1 * num2;
        println!("Your result is: {result} ");
    }

    if choice == 4 {
        let result = num1 / num2;
        println!("Your result is: {result} ");
    }
}
