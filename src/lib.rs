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

    for line in search(&config.query,&contents,true)
    {
    println!("{}", line);	
    }
	Ok(())
}

impl ConfigInfo {
pub fn new(mut args:std::env::Args)->Result<ConfigInfo,&'static str>{
	args.next();
    let query = match args.next() {
        Some(V)=>V,
        None=> return Err("Did not get a query string"),
    };

    let filename = match args.next() {
        Some(V)=>V,
        None=> return Err("Did not get a filename")
    };
	Ok(ConfigInfo{query,filename})
}
}

pub fn search<'a>(query: &str, contents: &'a str,caseSensitivity:bool) -> Vec<&'a str> { 
        if caseSensitivity {
    contents.lines().filter(|line| line.contains(&query)).collect()
    }
    else{
    contents.lines().filter(|line| line.to_lowercase().contains(&query)).collect()
    }
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
vec!["safe, fast, productive."], search(query, contents,true)
); 
assert_eq!(
vec!["safe, fast, productive."], search(query, contents,false)
); }
}
