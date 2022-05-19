// The RULES

// Each value in Rust has a variable that’s called its owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped.

fn main() {
    //variable scope 
    // s refers to a string literal 
    let s = "hello"; 

    {
        // s is only valid in this scope
        let s = "hello";
    }
    // :: operator allows us to namespace from the from function undethe String type (instead of using string_from)
    let _s = String::from("hello");

    // this type of string can be mutated
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a string

    println!("{}", s);

    // during compile time, the size of memory must be known to be allocated (stack)
    // as mutable string has a unknown size, this is stored in the heap, unknown at compile time, to hold the contents. 
    // - the memory must be requested from the memory allocator at runtime 
    // - need a way of returning this memory to the allocator when we're done with the string

    String::from // this is requesting the memory it needs
    // in Rust, the memory is automatically returned once the variable that owns it goes out of scope
    {
        let s = String::from("hello"); // s is valid from this point forward 
    } // this scope is now over so variable s is no longer valid, Rust calls the function 'drop' where the auther of String can put the code to return memory.
    // essentially y makes a copy in the value of x and bind it to y 
    let x = 5; // creates variable x and assigns 5 to it 
    let y = x;  // create variable y and assigned x to it, x is equal to 5 so y is equal to 5.

    let s1 = String::from("hello"); 
    let s2 = s1; 


}
