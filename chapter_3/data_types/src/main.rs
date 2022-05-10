fn main() {
    // every value in Rust has a data type 
    // there are 2 types of data type: scalar and compound
    // rust is a statically typed language which means it muyst know the types of all variables at compile time
    // .parse is used to convert a string into a numeric type 
    let guess: u32 = "42".parse().expect("Not a number!");

    // Scalar Types -> represents a single value, there are 4 sclar types: integers, floating-point, bumbers, Booleans and characters. 
    
    let signedInt: i32 // <- this variable is signed integer that takes up 32 bits of space (positive or negative sign) this is also the default
    let unsignedInt: u128 // <- this variable is unsigned and takes up 128 bits of space 

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32


}
