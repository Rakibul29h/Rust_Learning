fn main() {

    // Ownership is a set of rules that govern how a Rust program manages memory
    /*
     Rules of Ownership:
        * Each value in Rust has an owner
        * There can only be one owner at a time
        * When the owner goes out of scope, the value will be dropped.
     */
    

    // Scope ->A scope is the range within a program for which an item is valid

    // Defining an string (Strings are immutable)
     let s = String::from("Hello world");

     // The double colon :: operator allows us to namespace this particular  from function under the String type 
     // This type manages data allocated on the heap
     // 

     let mut str = String::from("Hello ");
     str.push_str(", world");
     println!("{str}");

     /*
        let s1 = String::from("Hello, ");
      let s2 = s1;
      println!("{s1} worlds
      */

     let mut s1 = String::from("hello");
     s1= String::from("ahoy");
     println!("{s1} , world!");

     /*
     If we do want to deeply copy the heap data of the String, not just the stack
     data, we can use a common method called clone. 
      */
     let p = String::from("clone data");
     let q= p.clone();
     println!(" p = {p} and q = {q}");

     // when a string pass to the funciton the full data move and ownership change but for primitive data 
     // It makes a copy of the content 

     // referencing  & -> reference operator
     // dereferencing * -> dereference operator

     // using reference opearator the ownership remain unchanged
     // A reference is like a pointer in that it’s an address we can follow to access the data stored at that address;
     let str1 = String::from("Rakibul");
     println!("{}",calculate_len(&str1));
     // we won’t need to return the values in order to give back ownership, because we never had ownership.
     //We call the action of creating a reference borrowing. 
     // Just as variables are immutable by default, so are references. We’re not allowed to modify something we
     //  have a reference to

     /*
      * Mutable reference
      fn main() {
      let mut s = String::from("hello");
      change(&mut s);
      }
      fn change(some_string: &mut String) {
      some_string.push_str(", world");
      }
      */

     let mut name = String::from("Rakibul ");
     change_name(&mut name);
     println!("{}",name);

     /*
     let mut s = String::from("hello");
     let r1 = &mut s;
     let r2 = &mut s;
     println!("{r1}, {r2}");
     This code is invalid because we cannot borrow s as mutable more than once time 
     The benefit of having this restriction is
     that Rust can prevent data races at compile time. A data race is similar to a
     race condition and happens when these three behaviors occur:
     ->Two or more pointers access the same data at the same time.
     ->At least one of the pointers is being used to write to the data.
     ->There’s no mechanism being used to synchronize access to the data.
      */

     let mut s2 = String::from("Hello");
     {
         let r1 = &mut s2;
     }
     let r2 = &mut s2;

     /*
      here r1 goes out of scope , so we can make a new reference with no problems;
      */

     /*
      *  We also cannot have a mutable reference while we have an immutable one to the same value.
      let mut s = String::from("hello");
      let r1 = &s; // no problem
      let r2 = &s; // no problem
      let r3 = &mut s; // BIG PROBLEM
      println!("{r1}, {r2}, and {r3}");
      See more about referencing and browing
      */

     
     
}

fn calculate_len( s: &String) -> usize{
    s.len()
}

fn change_name(name: &mut String){
    name.push_str(" islam");
}
