// bringing input and output library into scope (io) from the standard library (std).
use std::io; 
use std::cmp::Ordering;
use rand::Rng; 

// prelude are items that are defined in th standard libary that are bought into the scope of every program. items that are not prelude will need to use the keywork "use"

fn main() {
    println!("Guess the number!");
    println!("Please input your guess");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {

        //declaring a mutable varible (mut) and storing user's input (in this case it's the result of String::new() function - which returns a new instance of a string)
        // ::new() indicates that 'new' is the associate function of the String type, associate function is implemented on the type (also known as static method) rather then a particular instance of a type (eg. String). 
        // in short this creates a new empty string to a mutable variable 'guess'
        let mut guess = String::new();
    
        // below is an example of immutable varible. 
        // let guess = String::new();
    
        // std function returns an instance of std::io::stdin which is a type that represents a handle to the standard input of the terminal. 
        // .read_line(&mut guess) reads the users input handle in the terminal and saves it to the mutable variable 'guess'. .read_line() also returns an io::Result (specific version of a sub-module), they are enums (Ok or Err)

        // & means the variable reference, this prevents the code making multiple copies and wasting memory. 
        io::stdin().read_line(&mut guess)
        // .expect() is handling the error that may return from .read_line(). This is the Result.
        .expect("Failed to read line");

        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num, 
            Err(_) => continue,
        };

        // prints the users saved input into {}
        println!("You guessed: {}", guess);

        // match guess and secret_number
        match guess.cmp(&secret_number) {
            // Less, Greater and Equal are Enums of Ordering library.
            Ordering::Less => println!("Too Small!"), 
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!"); 
                break;
            }
        }

        // println() is a macro that prints a string to the screen 
        // println!("The secret number is : {}", secret_number);
    }

}
