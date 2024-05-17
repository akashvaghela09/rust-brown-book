use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("=== Welcome to Guessing Game ===");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=1000);

    loop {
        println!("guess any number : ");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("your guessed number was less"),
            Ordering::Greater => println!("your guessed number was greater"),
            Ordering::Equal => {
                println!("*** You WON ***");
                break;
            }
        }
    }
}
