fn main() {
   /*
    * Every value in Rust is of a certain data type, which tells Rust what kind of
    data is being specified so that it knows how to work with that data. We’ll
    look at two data type subsets: scalar and compound.
    Keep in mind that Rust is a statically typed language, which means that
    it must know the types of all variables at compile time. The compiler can
    usually infer what type we want to use based on the value and how we use
    it. In cases when many types are possible, we must use type annotation (:u32)
    */
   // Parge the string to a number
   let guess: u32 = "42".parse().expect("Not a number!");
   print!("{guess}");

   /*
    ## Scalar Types
        A scalar type represents a single value. Rust has four primary scalar types:
            * integers (An integer is a number without a fractional component.)
                ->signed (i8,i16,i32, i64, i128,isize)
                ->unsigned integer (u8, u16, u32, u64, u128 , usize)
                    ->Number literals can also use _ as
                    a visual separator to make the number easier to read, such as 1_000, which
                    will have the same value as if you had specified 1000
                    
                    Decimal             98_222
                    Hex                 0xff
                    Octal               0o77
                    Binary              0b1111_0000
                    Byte (u8 only)      b'A'
                      
            * floating-point  numbers( Rust’s floating-point types are f32 and f64,which are 32 bits and 64 bits in size, respectively.)
            * Booleans (Boolean type in Rust has two possible values: true and false. Booleans are 1 byte in size)
            * characters. (Rust’s char type is the language’s most primitive alphabetic type)
    */

   let age:i8 = 23; // integer decleration

   //floating point numbers
   let coldDrinksPrice:f32 = 45.50;
   let acPrice :f64 = 10000000.234;

   let isPrime:bool = false;

   let z:char = 'z';

   /*
    * Numeric Operations
    Rust supports the basic mathematical operations you’d expect for all the number types: 
    * addition (+)
    * subtraction (-)
    * multiplication (*)
    * division (/)
    * remainder. (%)
    Integer division truncates toward zero to the nearest integer
    */

   
   
   
}
