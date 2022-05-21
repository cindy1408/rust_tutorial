// The RULES

// Each value in Rust has a variable that’s called its owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped.

fn main() {
    //variable scope 
    // s refers to a string literal 
    let _s = "hello"; 

    {
        // s is only valid in this scope
        let _s = "hello";
    }
    // :: operator allows us to namespace from the from function undethe String type (instead of using string_from)
    let _s = String::from("hello");     // this is requesting the memory it needs

    // this type of string can be mutated
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a string

    println!("{}", s);

    // during compile time, the size of memory must be known to be allocated (stack)
    // as mutable string has a unknown size, this is stored in the heap, unknown at compile time, to hold the contents. 
    // - the memory must be requested from the memory allocator at runtime 
    // - need a way of returning this memory to the allocator when we're done with the string

    // in Rust, the memory is automatically returned once the variable that owns it goes out of scope
    {
        let _s = String::from("hello"); // s is valid from this point forward 
    }; // this scope is now over so variable s is no longer valid, Rust calls the function 'drop' where the auther of String can put the code to return memory.
    
    // essentially y makes a copy in the value of x and bind it to y 
    let x = 5; // creates variable x and assigns 5 to it 
    let _y = x;  // create variable y and assigned x to it, x is equal to 5 so y is equal to 5.

    let s1 = String::from("hello"); 
    let _s2 = s1; // this does NOT make a copy, a copy of the stack is created which is s2, but that only contains the name, ptr, len and capacity, it does however point to the value (hello) in the heap. so s1 and s2 both points to the heap 'hello', only the copy of the varible is done but not the value. 
    // This can also cause issues when cleaning up memory, as s1 and s2 goes out of scope, Rust will attempt to 'drop' on both s1 and s2 but as there values are pointed to the same heap location, it will cause a 'double free error' hence, once Rust assigns s1 to s2, s1 is considered no longer valid to prevent this error. 
    // assigning a copy of a string is called 'MOVE' when we MOVE the pointer from one stack to another stack but the heap location stays the same, and the first stack is no longer valid. 

    // DEEPLY copy is when the heap data of the string is cloned along with the stack data, this is called CLONE. 
    let s1 = String::from("hello");
    let s2 = s1.clone(); 

    println!("s1 = {}, s2 = {}", s1, s2);

    // Below works perfectly fine because the whole value of the variable is stored on the stack so when the stack is copied, it has everything. (integers has a known compile time)
    let x = 5; 
    let y = x; 

    println!("x = {}. y = {}", x, y);

    // Rust wont let us annotate a type with Copy if the type or any of it's part implements the Drop trait, if we use 'COPY' we'll get a compule time error. 

    // these all implement copy: 
        // All the integer types, such as u32.
        // The Boolean type, bool, with values true and false.
        // All the floating point types, such as f64.
        // The character type, char.
        // Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.

    // passing a value to a function is the same as assigning a value to a variable, depending on the variable, it will copy or move. 

    let s = String::from("hello");

    takes_ownership(s); // variable s is no longer valid.. as it is moved
    println!("{}", s); // not valid! 

    let x = 5; 
    
    makes_copy(x); // variable x is still valid as it is copied
    println!("{}", x);

    // returning values can also transfer ownership

    let _s1 = gives_ownship(); // moves it return value into s1

    let _s2 = String::from("hello"); // s2 comes into scope

    let _s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back, which also moves it return value to s3

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1); 

    println!("The length of '{}' is {}.", s2, len);


}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer)
}

fn gives_ownship() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string 
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}