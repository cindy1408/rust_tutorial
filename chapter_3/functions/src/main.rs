// functions can be in an order, as long as it is written somewhere
fn first_function() {
    println!("this is the first function");
}

fn main() {
    println!("Hello, world!");
    first_function();
    second_function(12);
    third_function(100, 'h');
    fourth_function(100, "Hello");

    // let statements are used to perform an action, it does not return any values, hence they cannot be reassigned (there will be nothing to be bound to).
    let y = 6;
    println!("{}", y);


    let x = y;
    println!("{}", x);

    let y = {
        let x = 3 ;
        // expressions do not need a semi-colon, adding a semi-colon will turn it into a statement
        x + 1 
    };

    println!("value of y is {}", y);

    let j = return_function(); 
    println!("{}", j);

    let u = add_one_function(3); 
    println!("{}", u);
}

// argument must include the type that it is expecting
fn second_function(x: i32) {
    println!("the passed in argument is {}", x);
}

// multiple parameters are seperated using arguments
// char type is literally a character
fn third_function(x: i32, unit: char) {
    println!("the passed in arguments are {}{}", x, unit);
}

fn fourth_function(x: i32, sentence: &str) {
    println!("the passed in arguments are {} {}", x, sentence);
}

fn return_function() -> i32 {
    5
}

fn add_one_function(h : i32) -> i32 {
    h + 1 
}