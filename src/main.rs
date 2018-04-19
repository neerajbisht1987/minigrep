extern crate minigrep;

use std::env;
use std::process;

use minigrep::ConfigInfo;

fn main() {
	let args: Vec<String> = env::args().collect();
	
	let config=ConfigInfo::new(&args).unwrap_or_else(|err|{
		println!("Problem parsing argument: {}",err);
		process::exit(1);
	});
		
	println!("Searching for :{}", config.query);
    println!("In file :{}", config.filename);	
	if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
	
	
}
