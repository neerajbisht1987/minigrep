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

    for line in search(&config.query,&contents)
    {
    println!("{}", line);	
    }
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

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> { 
    let mut results = Vec::new();
    for line in contents.lines(){
        if line.contains(query){
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
fn one_result() {
let query = "fast"; let contents = "\
Rust:
safe, fast, productive.
Pick three.";
assert_eq!(
vec!["safe, fast, productive."], search(query, contents)
); }
}
