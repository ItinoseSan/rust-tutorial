extern crate rand;

use rand::Rng;
use std::io;

fn main() {
    println!("Guess my number");

    let secret_number = rand::theard_rng().gen_range(1,101);
    println!("The secret num is: {}", secret_number);

    println!("Please input your guess");

    let mut guess_value = String::new();
    io::stdin().read_line(&mut guess_value).expect("Failed to read line");

    println!("You guessed: {}",guess_value);

    match guess.cmp(&secret_number) {
        Ordering::Less    => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal   => println!("You win!"),
    }
}
