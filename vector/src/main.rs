fn main() {
    //vector decleartion
    // Vector is heap allocate
    // So there need to focus on ownership
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

    //vector initalize and decleartion
    let mut vector = vec![1,2,3,4,5,6];
    println!("Vector initiallize and decleration: {:?}",vector);

    vector.pop();
    vector.pop();
    println!("After pop the vector array is : {:?}",vector);

    
}
 