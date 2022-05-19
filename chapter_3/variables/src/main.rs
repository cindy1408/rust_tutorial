fn main() {
    // by default all variables are immutatable, the code below will error during compile time as variable x is assigned twice
    // let x = 5; 
    // println!("The value of x is: {}", x); 
    // x = 6; 
    // println!("The value of x is: {}",x);

    // below shows the same but with a mutatble variable denoted with 'mut'
    let mut x = 5; 
    println!("The value of x is: {}", x); 
    x = 6; 
    {
        // in this {} is the inner scope but as we dont have let keyword, it's reassigned variable x with 6 
        x = x * 2; 
        println!("The value of y in the inner scope if {}", x);
    }
    // this value of x is now equal to 12
    println!("The value of x is: {}", x); 

    // constants vs variables 
    // - constants cannot be mutatable, it's denoted with the keyword const and the Data Type must be declared. They can be declared in any scope including global scope
    // - constants can only be set to a constant expreession and not a result of a value through a computed runtime. --> https://doc.rust-lang.org/reference/const_eval.html 
    // constants are written in all uppercase with underscores between each word
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("This is a constant: {}", THREE_HOURS_IN_SECONDS);

    // shadowing - when a new variable is declared with th same name as a previous variable (the first is shadowed by the second)
    // let keyword will allow us to shadow 
    let y = 5; 

    let y = y + 1; 

    {
        // in this {} is the inner scope, we've used the keyword let and so a new variable with the same name is created within that scope so the variable y outside that scope is not effected
        let y = y * 2; 
        println!("The value of y in the inner scope if {}", y);
    }
    // this value of y is still equal to 6
    println!("The value of y is {}", y); 


    let spaces = "   "; // string type
    let spaces = spaces.len(); // number type
    println!("{}", spaces);

    // with the same spaces example, we cannot use mut keyword as the value type are still required to be the same (below will error)
    let mut spaces = "   "; // string type
    // spaces = spaces.len(); // number type

}