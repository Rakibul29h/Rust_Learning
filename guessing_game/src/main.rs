use std::io;
use rand::prelude::*;
fn main() {
    let guess_list = ["grapes","banana","orange","mango","goyava"];
    let mut rng = rand::rng();

    let index = rng.random_range(0..guess_list.len());
    let random_fruit = guess_list[index];
    println!("Random fruit is {random_fruit}");
   
    
    }
}

fn guess_checker (selected_fruit: &str, random_fruit:&str)->bool{
    selected_fruit== random_fruit
}
