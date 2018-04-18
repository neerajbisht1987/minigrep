use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;
fn main() {
	let args: Vec<String> = env::args().collect();
	
	let config=ConfigInfo::new(&args).unwrap_or_else(|err|{
		println!("Problem parsing argument: {}",err);
		process::exit(1);
	});
		
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
fn new(args:&[String])->Result<ConfigInfo,&'static str>{
	if args.len() < 3 {
	return Err("not enough argument");
	}
	let query = args[1].clone();
    let filename = args[2].clone();
	Ok(ConfigInfo{query,filename})
}
}