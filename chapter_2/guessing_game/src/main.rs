// bringing input and output library into scope (io) from the standard library (std).
use std::io; 

fn main() {
    println!("Guess the number!");

    println!("Please input your guess");

    //declaring a mutable varible (mut).
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("You guessed: {}", guess);
}
