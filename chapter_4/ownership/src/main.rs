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

    // The String Type - stored in the heap because they are mutable
    let _s = String::from("hello");


}
