fn main() {
    //vector decleartion
    let mut v:Vec<i32> = Vec::new();

    //Adding vector
    v.push(7);
    v.push(8);
    println!("Vector value is {:?}",v);


    //another way of decleration
    let mut vector2= Vec::<i32>::new();
    vector2.push(3);
    vector2.push(4);
    println!("Vector of 2nd decleration is : {:?}",vector2);
}
