fn main() {
   
    simple_if(5);
    else_if_function(6);

    let condition = true;
    let result = if condition {5} else {3}; //This is an expression
    println!("{result}")
    
    /*
     * let result2 = if condition {5 } else {"siz"}
     * This make a error because  the values that have the potential to be results from each arm of the
    if must be the same typ
     */
}

fn simple_if(x:i32){
    if x < 5 {
        println!("Condition was true")
    }else{
        println!("nothing")
    }
}

fn else_if_function(x:i32){
    if x % 4 ==0 {
        println!("number is divisible by 4");
    } else if  x%3 == 0 {
        println!("number is divisible by 3");
    } else if x %2 ==0 {
        println!("number is divisible by 2");
    }else {
        println!("number is not divisible by 4,3,or 2");
    }
}