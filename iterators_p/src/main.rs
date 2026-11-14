fn main() {

    let a = [2,3,4,5];
    let mut iter = a.into_iter(); // this give the ownership so the next can be done;
    assert_eq!(Some(2),iter.next()); // here iter.next is 2
    assert_eq!(Some(3),iter.next()) ;// here iter.next is 3
    assert_eq!(Some(4),iter.next()) ;// here iter.next is 4
    assert_eq!(Some(5),iter.next()) ;// here iter.next is 5
    assert_eq!(None,iter.next()) ;// here iter.next is End
    assert_eq!(None,iter.next()) ;// here iter.next is End
    let b = [2,3,4,5];
    let mut iter = b.iter(); // this give the reference so the next can be done with Some(&value );
    assert_eq!(Some(&2),iter.next()); // here iter.next is 2
    assert_eq!(Some(&3),iter.next()) ;// here iter.next is 3
    assert_eq!(Some(&4),iter.next()) ;// here iter.next is 4
    assert_eq!(Some(&5),iter.next()) ;// here iter.next is 5
   assert_eq!(None,iter.next()) ;// here iter.next is End
    assert_eq!(None,iter.next()) ;// here iter.next is End
    let vec: Vec<i32> = vec![1,2,3];

        
    
    for item in vec.iter(){// here the reference come not ownership this line is equal to (vec item
                           // in &vec 
                           // )
       println!("{}",item);

    }
    println!("{:?}",vec);// So the vector availabe here 

    for item in vec.into_iter(){ // here the ownership come the line is equal to (for item in vec)
        println!("{}",item);
    }
//
//println!("{:?}",vec) // this makes error because there is no ownership for that
                         //
                         

}

 
