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

Chapter 4 
Ownership is a set of rules that determines how Rust program manages memeory. 
Go have garbage collectors that looks for un-used memory, in other programs, programmers must explicityly allocate and free the memory. 

In Rust, memory is managed through a system of ownership with a set of rules that the compiler checks, if any rules are broken, the program wont compile.

The Stack and the Heap 
stack and heap are both parts of the memory available to your code to use at runtime but structured differently.

The Stack stores values using 'last in, first out', adding data is called pushing onto the stack and removing data is called popping off the stack. all data stored on the stack must be known (fix size)

The Heap Stack stores any data with unknown size or mutuable length/size at compile time. when a space for a data with a certain amount of spaces are requested, the memory allocator finds an empty spot in the heap thats big enough, marks it as used and then returns a pointer (address of that location), this is called allocating on the heap or allocating because the location of the data is not yet known whereas the Stack, the location is already known when we're pushing data onto the stack, the pointer is already stored on the stack. If you want the actual data, we must follow the pointer
pushing to the stack is faster than allocating on the heap and accessign data in the heap is sloower than accessing data on the stack because you have to follow a pointer to get there.

the values (including pointers to the data) and the functions local variables from when the function is being called, gets pushed onto the stack. When the function is over, those values are popped off the stack. 

Ownership needs to keep track of the code data that are on the heap, remove duplicates data and cleaning up unused data on the heap. 
