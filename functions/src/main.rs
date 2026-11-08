fn main() {
    // the keyword to define a function is fn
    hello_world();
    para_function(23); // the concrete value passed to the function is called argument

    /*
    # Expression and Statement
    * Statements are instructions that perform some action and do not return a value.
    * Expressions evaluate to a resultant value
        x=y=6 is not valid in Rust
        5+6 is an expression
        * Expressions can be part of statements
        *  Calling a function is an expression.
        * Calling a macro is an expression
        * A new scope block created with curly brackets is an expression
    */
    let y = {
        let x = 3;
        x+1 //a block can return a value, but only if its final expression does not end with a semicolon. so there is no semiclone
    };
    println!("The value of y is : {y} ");
}

fn hello_world() {
    println!("Hello World")
}

//a parameterized function
// In function signatures must declare the type of each parameter
fn para_function(x: i32) {
    println!("The value of x is {x}");
}
