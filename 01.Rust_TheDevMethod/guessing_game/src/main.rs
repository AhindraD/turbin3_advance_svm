use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let secret_num: u8 = rand::rng().random_range(1..=100);
    loop {
        let mut guess = String::new();
        println!("Enter your guess [0,100] :");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input...");
        println!("Your guess: {}", guess);
        let guess_num: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("Error parsing the guessed number: {}", e);
                continue;
            }
        };
        match guess_num.cmp(&secret_num) {
            Ordering::Equal => {
                println!("Correct!");
                break;
            }
            Ordering::Greater => println!("Too Big!"),
            Ordering::Less => println!("Too Less"),
        }
    }
}
