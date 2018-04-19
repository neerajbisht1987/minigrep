use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct ConfigInfo {
pub query:String,
pub filename:String
}



pub fn run(config:ConfigInfo)->Result<(),Box<Error>>
{
	let mut f = File::open(config.filename).expect("file not found");
	let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    println!("With text:\n{}", contents);	 
	Ok(())
}

impl ConfigInfo {
pub fn new(args:&[String])->Result<ConfigInfo,&'static str>{
	if args.len() < 3 {
	return Err("not enough argument");
	}
	let query = args[1].clone();
    let filename = args[2].clone();
	Ok(ConfigInfo{query,filename})
}
}