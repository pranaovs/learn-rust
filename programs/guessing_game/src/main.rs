use rand::random_range;

use std::cmp::Ordering;
use std::io;

fn main() {
    let secret = random_range(1..=100);

    println!("Secret number is: {}", secret);

    println!("Welcome to the Guessing Game!");
    println!("Input a guess from 1 to 100: ");

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a valid number!");
                continue;
            }
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
