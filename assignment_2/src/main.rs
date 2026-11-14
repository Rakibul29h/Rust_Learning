// This is the assignment no 2: It is nothing 
struct Counter{
    count:u32
}

impl Counter {
    fn new()->Self{
        Self{count :0 }
    }
}
impl Iterator for Counter{
    type Item=u32;
    fn next(&mut self) -> Option<Self::Item>{
        if self.count <5 {
            self.count = self.count+1;
            Some(self.count)
        }else{
            None
        }
    }

}
fn main() {


let count = Counter::new();

for val in count{
    println!("{}",val)
}


}



