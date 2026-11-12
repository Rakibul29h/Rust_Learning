fn main() {

    // Array declearation
    let mut  arr : [&str;3]=["Hello", "Guys", "Thanks you"];

    // write_arr(arr); 
    //for printing array the system is {:?}
    write_arr_ref(&mut arr);
    println!("Arr = {:?}",arr);
    // to know th esize of an array
    print!("Size of the array is {}",arr.len());

    //passing reference
}

fn write_arr( mut arr1:[ &str;3]){ //here arr1 is a copy of arr this is the fact of
    arr1[0]="Hi";
    println!("arr1 = {:?}", arr1);
}
// look at the reference
fn write_arr_ref( arr2:&mut [&str;3]){
    arr2[0]="Hi";
    println!("arr2  = {:?}", arr2);
}