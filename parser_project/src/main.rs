use clap::Parser;

#[derive(Parser,Debug)]
struct Args{
    #[arg(long,short)]
    name:String,
}

fn main() {
   let arg= Args::parse();
    println!("Hello {:?}",arg.name);

}


