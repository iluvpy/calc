use std::env;
use std::path::Path;
use std::fs;
mod interp;

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() > 1 {
		let mut content = args[1].clone();
		let file = Path::new(&content).is_file();
		if file {
			content = fs::read_to_string(content).expect("Error while reading file!");
		}
		interp::argument(&content);
	}	
    else {
		interp::utils::print_help();
	}
}