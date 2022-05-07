# rust_tutorial

Summaries

Chapter 1 

cargo build will create an executable file
cargo.lock keep tracks of the exact version of the dependencies in this project (Cargo.lock is a self sufficient file that maintains itself)

cargo new project_name will create the filename in the directory.

cargo run will run the binary file directly if no changes were made, if changes were made, cargo run will build the binary file before running it. 

cargo check will check if the code written will compile but doesn't execute it. 

result of the build is saved in target/debug directory when cargo build is used. 

use cargo build --release to compile it with optimisation (creates a build in target/release instead of target/debug)

rustc main.rs and then run ./main


 Chapter 2

   Guessing project 


 Chapter 3 

 VARIABLES and MUTABILITY
 
 constants are declared using the keyword const and a data type is also required 

 mut keywords allows you to reassign the variable (shadowing), that means the data type will still have to remain the same and as it's reassigned, inner scopes will still reassign declared mut variables in outer scope 

 let keyword will allow you to redeclare the variable, which means they can have different types of data. let variables assigned in inner scopes will not effect the same let variables declared in outscope as the inner scope variable is a new variable with the same used name. 

 FUNCTIONS 
 
 functions are denoted with fn, parameters must include the argument name and type, multiple ones requires commas for seperations 
 return functions does not need any 'return' for value but it must be declared after the parameters... denoted by (->) then followed by the type
 char type is for one character where as a str is for a collection of chars

 CONTROL FLOW 

 In rusts, if statements and for loops are used for controlling the execution flow 
 the if statement conditions are bools so everything must be true or false 
 as 'if' is an expression, declaring a let variable with an if statement is possible.. 
 let x = true 
 let x = if conditon 'hi' else 'hello'

there are many types of loops, with the common as loops {} this is infinite
this loop can be paired with continue and break in order to know when to stop and continue
with the break keyword, we are able to return a value with the statement after break; 
While loop is used when there is condition to stop the loop (requires bools)
a for loop is better for looping over an array... use for element in a {}
.rev() is used to reverse an array and (1..4) is used as range 1 up to 4... 
