use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() < 3 {
	panic!("not enough argument");
		}
	let config=ConfigInfo::new(&args);
	println!("Searching for :{}", config.query);
    println!("In file :{}", config.filename);	
	
	let mut f = File::open(config.filename).expect("file not found");
	let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    println!("With text:\n{}", contents);	 
	 
	
}
struct ConfigInfo {
query:String,
filename:String
}

impl ConfigInfo {
fn new(args:&[String])->ConfigInfo{
	let query = args[1].clone();
    let filename = args[2].clone();
	ConfigInfo{query,filename}
}
}