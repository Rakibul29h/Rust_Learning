/*
 * Enum
    => There are three type of enums
        -> Simple
        -> Optional
        -> Result
 */


#[derive(Debug)]
enum Shape{
    Circle(f64),
    Rectangle(f64,f64)
}

impl Shape{
    fn new_circle(radius:f64)->Self{
        Self::Circle(radius)
    }

    fn new_rectangle(height:f64, width:f64)->Self{
        Self::Rectangle(height, width)
    }

    fn area(&self)->f64{
        match self {
            Shape::Circle(radius)=> 3.1416*radius*radius,
            Shape::Rectangle(height, width)=> height*width
        }
    }
}

fn main() {
    let circle = Shape::new_circle(5.0);
    println!("Circle 1 is {:?}",circle);
    let rec1 = Shape::new_rectangle(3.2, 2.3);
    println!("Rectangle of rec1 = {:?}",rec1);
    eprintln!("Area of rectange is {}",rec1.area());

    //Optinal enum example
    match get_user_phone_number(1){
        Some(data)=> println!("Data is {}",data),
        None => println!("None data ")
    }
    //Result <T, X> type
    match divide(3, 0) {
        Ok(data) => println!("Result is {}",data),
        Err(err) => println!("Error is {}",err)
    }
    
}

fn get_user_phone_number(userid:i32)->Option<i32>{ // Optional Enum
    let mob_num = 12345;

    if userid ==1 {
        Some(mob_num)
    }else{
        None
    }
}

fn divide(x:i32,y:i32)-> Result <i32, String>{ // Result enum
    if y==0 {
        Err("Y is zero".to_owned())
    }else{
        Ok(x/y)
    }
}