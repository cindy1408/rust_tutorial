use std::io; 

fn main() {
    // every value in Rust has a data type 
    // there are 2 types of data type: scalar and compound
    // rust is a statically typed language which means it muyst know the types of all variables at compile time
    // .parse is used to convert a string into a numeric type 
    let guess: u32 = "42".parse().expect("Not a number!");

    // Scalar Types -> represents a single value, there are 4 sclar types: integers, floating-point, bumbers, Booleans and characters. 
    
    let signedInt: i32; // <- this variable is signed integer that takes up 32 bits of space (positive or negative sign) this is also the default
    let unsignedInt: u128; // <- this variable is unsigned and takes up 128 bits of space 

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    let su = 5 + 10; 

    let difference = 95.5 - 4.3; 

    let product = 4 * 30; 

    let quotient = 56.7 / 32.2; 
    let floored = 2/3;  // this will round down

    let remainder = 43 % 5; 

    // THE BOOLEAN TYPE - one byte in size 

    let t = true; 

    let f: bool = false; // this include explicit type notation 

    // THE CHARACTER TYPE - most primitive alphabetic type, these are in single quotation marks
    // they are 4 bytes in size and represents a Unicode Scalar Value (can represent more than just ASCII)
    // valid in accented letters, Chinese, Japanese, and Korean characters, emoji and zero-width spaces

    let c = 'z'; 
    let z = 'Z'; 
    let heart_eted_cat = '😻'; 

    // THE COMPOUND TYPE 
    // compound types can group muktuples valyes into one type, in Rust there are 2 types: 

        // 1. THE TUPLE TYPE - grouping together a number of values with a variety of types into one compound type, they have a fixed length and size. 

        let tup: (i32, f64, u8) = (500, 6.5, 1); // the variable tup binds to the entire tuple as a single compound element. 
        // use pattern matching to destructure a tuple value: 

        let (x, y, z) = tup; 

        println!("The value of y is: {}", y); 

        // Tuple can also be accessed directly by using a period '.' followed by the index of the value: 

        let x: (i32, f64, u8) = (500, 6.4 ,1); 

        let five_hundred = x.0; 
        let six_point_four = x.1; 
        let one = x.2; 

        // tuples without any values contains one value, written (). It's called a unit type and the value is called the unit value. 

        // THE ARRAY TYPE - every element must have the same value and have a fix length

        let a = [1, 2, 3, 4, 5]; 

        // Arrays are allocated in the stack instead of the heap 

        let months = ["January", "Feburary", "March", "April", "May"]; 

        let a: [i32; 5] = [1, 2, 3, 4, 5];  // this specifies the array is i32 type with a length of 5 elements

        let a = [3; 5]; // means : initalising the array with 5 3s in the array

        let a = [3, 3, 3, 3, 3, 3]; 

        let first = a[0]; 
        let second = a[1]; 
        
        // trying to access an array that does not exist eg a[10]; 

        let a = [1, 2, 3, 4, 5];
        
        println!("Please enter an array index."); 

        let mut index = String::new(); 

        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");

        let index: usize = index 
            .trim()
            .parse()
            .expect("Index entered was not a number");

        let element = a[index]; 

        println!(
            "The value of the element at index {} is: {}", 
            index, element
        );
}
