use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Guess the number");

    println!("Please enter your guess: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too Low!!!"),
        Ordering::Greater => println!("Too Big!!!"),
        Ordering::Equal => println!("Equal"),
    }

    println!("You guessed: {guess}");
    println!("Secret number was: {}", secret_number);
}
