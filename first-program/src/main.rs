use std::io;
fn main(){
    let mut age= String::new();
    io::stdin().read_line(&mut age).expect("Input failed");
    println!("Age is {}",age);
    
}