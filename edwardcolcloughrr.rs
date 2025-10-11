use std::io;
use rand::Rng;

fn main() {
    let number = rand::thread_rng().gen_range(1..=5);
    
    println!("Guess 1-5:");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    
    let guess: u32 = guess.trim().parse().unwrap_or(0);
    
    if guess == number {
        println!("Correct!");
    } else {
        println!("Wrong! It was {}", number);
    }
}