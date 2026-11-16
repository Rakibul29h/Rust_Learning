use clap::Parser;

#[derive(Parser,Debug)]
struct Args{
    #[arg(long,short)]
    name:String,
}

fn main() {
   let arg= Args::parse();
    println!("Hello {:?}",arg.name);
	"This is a temporary change for that I can change a commit and make a stike fo
	that and;
}


