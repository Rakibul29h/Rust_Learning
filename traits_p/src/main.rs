struct Circle{
    radius:f64,
}

trait Shape {
    fn area_circle(&self)->f64;
}

impl Shape for Circle{
    fn area_circle(&self)->f64 {
        3.1416*self.radius*self.radius
    }
}


struct Person;
trait Greet{
    fn greet(&self){
        println!("Hello from default greet!");
    }
}
impl Greet for Person{}
fn main() {

    let circle = Circle{
        radius:5.0
    };

    circle.area_circle();

    let person1 = Person;
    person1.greet();
}
