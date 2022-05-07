fn main() {
    // control flow is controlling the flow of executions (if expressions and loops)
    let number = 7; 

    // blocks of codes inside the curly brackets are sometimes called arms, if statements are based on conditions which are bools (true or false)
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number = 3;
    if number != 0 {
        println!("number was something other than zero");
    }

    // handling mutliple conditions, ifs and else ifs (if there are more than one case, match is a better way around it, in chapter 6)
    let number = 6; 
    
    if number % 4 == 0 {
        println!("number is divisible by 4"); 
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2"); 
    }

    // "if" is an expression meaning we can use it on the right side of a let statement 
    let condition = true; 
    // condition is similar to a tenary statement, if true then first value, if false, second value
    // value of 5 and 6 must be the same type as 'number' variable as during compile time, the compiler will know the type for variable number and hence makes sure everywhere else is also that same type
    let number = if condition {5} else {6};

    println!("The value of number is: {}", number);

    if_statement(32);

    loop {
        println!("again!");
        break
    }
    // continue to skip over any remaining code in the iteration of the loop
    // break helps to stop any remaining code in the iteration of the loop
    
    // mut is used to mutate the variable count not NOT redeclare like using let 
    let mut count = 0; 
    'counting_up: loop {
        println!("count = {}", count); 
        let mut remaining = 10; 

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break; 
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -=1; 
        }

        count += 1; 
    }
    println!("End count = {}", count);


// returning from a loop after break  (used to retry until successful)
    let mut counter = 0; 

    let result = loop {
        counter += 1; 

        if counter == 10 {
            break counter * 2; 
        }
    };

    println!("The result is {}, and counter is {}", result, counter); 

// Conditional Loops with While -> evaluating a condition within a loop 
    let mut number = 3;

    while number != 0 {
        println!("{}!", number); 

        number -=1; 
    }

    println!("LIFT OFF!!");

// Looping through a collection with for -> can be used to loop over elements of a collection, such as an array 
    let a = [10, 20, 30, 40, 50];
    let mut index = 0; 
// essentially having a while loop over a variable that represents the index of the array, this is prone to error due to any changes in the length of the array 
    while index < 5 {
        println!("the value is: {}", a[index]);

        index +=1; 
    }

    // better approach, element is a variable that represents the value in each index of the array
    for element in a {
        println!("the value is: {}", element);
    }

    // (1..4) means range from 1 up to 4, .rev() means in reverse 
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFT OFF!");
}

fn if_statement(x : i32) {
    if x % 2 == 0 {
        println!("It's even!");
    } else {
        println!("It's odd!")
    }
}