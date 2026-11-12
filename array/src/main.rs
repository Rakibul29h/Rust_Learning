fn main() {

    // Array declearation
    let  arr : [&str;3]=["Hello", "Guys", "Thanks you"];

    write_arr(arr);
    //for printing array the system is {:?}
    println!("Arr = {:?}",arr);
    // to know th esize of an array
    print!("Size of the array is {}",arr.len());
}

fn write_arr(mut arr1:[ &str;3]){ //here arr1 is a copy of arr this is the fact of
    arr1[0]="Hi";
    println!("arr1 = {:?}", arr1);
}