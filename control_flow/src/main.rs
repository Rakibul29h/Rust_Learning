fn main() {
   
    simple_if(5);
    else_if_function(6);

    let condition = true;
    let result = if condition {5} else {3}; //This is an expression
    println!("{result}");
    
    /*
     * let result2 = if condition {5 } else {"siz"}
     * This make a error because  the values that have the potential to be results from each arm of the
    if must be the same typ
     */

    match_function();

    match_even_odd();
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

fn match_function(){
    let number = 100;
    match number{
        1 | 0=> println!("Number is one or zero"),
        2=> println!("Number is 2"),
        3=> println!("Number is 3"),
        4=> println!("Number is 4"),
        5=> println!("Number is 5"),
        _=>print!("Number is not in the range") // similar to default in switch case
    }
}

fn is_even(number:i32)->bool{
    if number % 2 ==0 {
        return true;
    }
    false
}

fn match_even_odd (){
    let number = 5;
    match number {
        x if is_even(x) => println!("Even"),
        x if !is_even(x) => println!("Odd"),
        _=>println!("not recognize")
    }
}