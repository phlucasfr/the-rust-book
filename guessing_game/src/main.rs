use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number!");

    let secret_number: u8 = rand::thread_rng().gen_range(1..=100);

    println!("Please input a guess between 1 and 100!");

    loop {
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => {
                if !(1..=100).contains(&num) {
                    println!("Please enter a number between 1 and 100");
                    continue;
                }
                num
            }
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Wou win!");
                break;
            }
        }
    }
}
