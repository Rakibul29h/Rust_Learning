fn main() {

    //shadowing
   //  
   // let x = 10;
   // let x = 10+2;
   // println!("value of x outer scope {x}");
   // {
   //     let x= 5;
   //     println!("value of x inner scope {x}")
   // }
   // println!("value of x outer scope {x}");

   // constant
   // const MINUTES_IN_HOURS : i32=60;
   // println!("{MINUTES_IN_HOURS}")

   /*
    Shadowing is different from marking a variable as mut because we’ll
    get a compile-time error if we accidentally try to reassign to this variable
    without using the let keyword. 
    */ 
   
   /*   The other difference between mut and shadowing is that because we’re
   effectively creating a new variable when we use the let keyword again, we
   can change the type of the value but reuse the same name.
   let spaces = "   ";
   let spaces = spaces.len();
   println!("{spaces}")
   */

   
}
