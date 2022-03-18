// bringing input and output library into scope (io) from the standard library (std).
use std::io; 

fn main() {
    println!("Guess the number!");

    println!("Please input your guess");

    //declaring a mutable varible (mut) and storing user's input (in this case it's the result of String::new() function - which returns a new instance of a string)
    // ::new() indicates that 'new' is the associate function of the String type, associate function is implemented on the type (also known as static method) rather then a particular instance of a type (eg. String). 
    // in short this creates a new empty string to a mutable variable 'guess'
    let mut guess = String::new();

    // below is an example of immutable varible. 
    // let guess = String::new();

    // std function returns an instance of std::io::stdin which is a type that represents a handle to the standard input of the terminal. 
    // .read_line(&mut guess) reads the users input handle in the terminal and saves it to the mutable variable 'guess' 
    // & means the variable reference, this prevents the code making multiple copies and wasting memory. 
    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("You guessed: {}", guess);
}
