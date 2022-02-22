// Compare the guess to the secret number.

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    // some code here

    println!("You guessed: {}", guess);

    match guess.cmp(secret_number){
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => println!("You win!"),
    }

}
