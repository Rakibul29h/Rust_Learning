
#[derive(Debug)]
struct Ractangle{
    width:u32,
    height:u32
}

fn main() {
   /*
    * Struct
        To define a struct, we enter the keyword struct and name the entire struct.
        A struct's name should describe the significance of the pieces of data being groped together.
        Inside curly brackets we define the names and types of the pieces of data. Which call fields
    */

    #[derive(Debug)]
   struct User{
       active:bool,
       username:String,
       email:String,
       sign_in_count:u64
   }
   /*
    To used the struct we create an instanc eof that struct by specifying concrete values for each of the fields.
    We create an instance by stating the name of the struct and then add curly brackets containing key:value pairs
    where keys are the name of the fields and the values are the data we want to store in those feild.
    */

   let user1 = User{
       active:true,
       username: String::from("Rakibul29hr"),
       email:String::from("rakibul2170@gmail.com"),
       sign_in_count:1
   };

   /*
     To get a specific value from a struct , we use dot notation. If the instance is mutable then we can change the value.
    */

   println!("Username of user1 {}",user1.username);

   let mut user2 = User{
       active:true,
       username: String::from("hi"),
       email:String::from("hi@gmail.com"),
       sign_in_count:2
   };
   println!("Before change the signin count {}",user2.sign_in_count);

   user2.sign_in_count = 3;

      println!("After change the signin count {}",user2.sign_in_count);


      //creating a function that make a user
      // 
    /*
    fn build_user (email:String, username:String)->User{
        User{
            active:true,
            username:username,
            email:email,
            sign_in_count:1,
        }
    }

    We can short this function by using the feild init shorthand
     */
    fn build_user (email:String, username:String)->User{
        User{
            active:true,
            username,
            email,
            sign_in_count:1,
        }
    }

    let user3 = build_user(String::from("jodu@kodu.com"), String::from("jodu"));
    println!("User 3 is {}",user3.username);

    /*
     * Creating Instance with struct update syntax
        It's often useful to create a new instance of a struct that includes most of the values from another instance of
        the same type but changes some of then . there help update syntax
        -> The syntax .. specifies that the remaining fields not explicitly set should have the same value as the fields in the 
        given instance.
     */

    let _user5 = User{
        email:String::from("kana@bana.com"),
        ..user3
    };

    //tuple structs
    /*
        Tuple structs have the added meaning the struct name provides but don't have names associated with their fields
        ;rather they just have the ypes of the fields.
     */
    struct Color (i32,i32,i32);

    /*
        Defining Unit like structs:
        We can also define structs that don't have ant fields. These are called unit-like structs because they behave 
        similar to (), 
     */

    let rec1 = Ractangle{
        width:20,
        height:20
    };
    println!("Area of the rectangle is {} square unit", area(&rec1));

    /*
     Printing a full struct we need to add #[derive(Debug)] before the struct decleration
     then println!("{variablename:?}");
     */

    println!("Full rec is {rec1:?}");
}

fn area(rectange:&Ractangle)->u32{
    rectange.height*rectange.width
}


