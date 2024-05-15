use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Hello from guessing Game...");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("=== Guess a new number ===");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("failed to read");
        println!("You guessed {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("your guess value was less"),
            Ordering::Greater => println!("your guess value was greater"),
            Ordering::Equal => {
                println!("You WON !!!");
                break;
            }
        }
    }
}
