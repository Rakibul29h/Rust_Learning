

trait Course {
    fn get_overview(&self)->String;
}

struct Workshop{
    title:String,
    instructor:String,
    duration:String,
}

struct Seminar{
    title:String,
    speaker:String,
    location:String
}

impl Course for Workshop{
    fn get_overview(&self)->String{
    format!("Workshop title:{} \n Wokshop instructor:{}\n Workshop duration{}",self.title,self.instructor,self.duration)
    }
}
impl Course for Seminar{
    fn get_overview(&self)->String{
       format!("Seminar title:{} \n Seminar Speaker:{}\n seminar location{}",self.title,self.speaker,self.location)
    }
}

fn main() {
    let workshop = Workshop{
        title:"Blockchain".to_owned(),
        instructor:"Code Eater".to_owned(),
        duration:"10".to_owned()
    };

    let seminar = Seminar{
        title:"Blockchain Seimnar".to_owned(),
        speaker:"Ali hasan".to_owned(),
        location:"Dhaka".to_owned()
    };

    print_course_overview(seminar);
    print_course_overview(workshop);
}

fn print_course_overview<T:Course>(course:T){
    println!("{}",course.get_overview());
}
