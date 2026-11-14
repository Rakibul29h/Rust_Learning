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

fn main() {

    let circle = Circle{
        radius:5.0
    };

    circle.area_circle();
}
