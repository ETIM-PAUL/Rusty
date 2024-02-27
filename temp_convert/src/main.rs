use std::io;

fn main() {
    let mut choice = String::new();
    let mut temp = String::new();

    println!("Welcome to Temperature Converter");
    println!("You can convert from Fahrenheit to Celsius and vice versa. Enter 1 for Fahrenheit to Celsius. 2 for Celsius to Fahrenheit");

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    // Parse the input string into a u32
    let choice: i32 = match choice.trim().parse() {
        Ok(choice) => choice,
        Err(_) => {
            println!("Invalid input. Please enter a valid choice (number).");
            return;
        }
    };

    println!("Enter the temperature value");

    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");

    // Parse the input string into a f32
    let temp: f32 = match temp.trim().parse() {
        Ok(temp) => temp,
        Err(_) => {
            println!("Invalid input. Please enter a valid value.");
            return;
        }
    };

    if choice == 1 {
        let result: f32 = (temp - 32 as f32) * (5 as f32 / 9 as f32);
        println!("The temp value in celsius is: {result} ");
    } else if choice == 2 {
        let result: f32 = (temp * (9 as f32 / 5 as f32)) + 32 as f32;
        println!("The temp value in fahrenheit is: {result} ");
    }
}
