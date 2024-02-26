use rand::Rng;
use std::io;

fn main() {
    let mut number = String::new();
    println!("Enter a value to check if its a prime number or not");

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    // Parse the input string into a u32
    let number: u64 = match number.trim().parse() {
        Ok(number) => number,
        Err(_) => {
            println!("Invalid input. Please enter a valid unsigned integer.");
            return;
        }
    };

    if is_prime(number, 5) {
        println!("{} is likely prime", number);
    } else {
        println!("{} is composite", number);
    }
}

fn is_prime(n: u64, k: u64) -> bool {
    if n <= 1 {
        return false;
    } else if n <= 3 {
        return true;
    } else if n % 2 == 0 {
        return false;
    }

    let mut d = n - 1;
    while d % 2 == 0 {
        d /= 2;
    }

    let mut rng = rand::thread_rng();
    for _ in 0..k {
        let a = rng.gen_range(2..n - 1);
        let mut x = mod_pow(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        while d != n - 1 {
            x = (x * x) % n;
            d *= 2;
            if x == 1 {
                return false;
            }
            if x == n - 1 {
                break;
            }
        }
        if x != n - 1 {
            return false;
        }
    }
    true
}

fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp >>= 1;
        base = (base * base) % modulus;
    }
    result
}
