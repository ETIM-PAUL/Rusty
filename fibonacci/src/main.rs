use std::io;

fn main() {
    let mut number = String::new();
    let mut old_value = 0;
    let mut result = 1;
    let mut next = 0;
    let mut count = 0;

    println!("Enter the number you want to generate the nth Fibonacci number");

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    // Parse the input string into a f32
    let number: i32 = match number.trim().parse() {
        Ok(number) => number,
        Err(_) => {
            println!("Invalid input. Please enter an unsigned integer");
            return;
        }
    };

    if number == 0 {
        println!("The nth Fibonacci of {number} is 0");
    } else if number == 1 {
        println!("The nth Fibonacci of {number} is 1");
    } else {
        while count < (number - 1) {
            next = old_value + result;
            old_value = result;
            result = next;
            count += 1;
        }
        println!("The nth Fibonacci of {number} is {result}");
    }
}
