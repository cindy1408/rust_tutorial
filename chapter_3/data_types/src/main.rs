fn main() {
    // every value in Rust has a data type 
    // there are 2 types of data type: scalar and compound
    // rust is a statically typed language which means it muyst know the types of all variables at compile time
    let guess: u32 = "42".parse().expect("Not a number!");
}
