use std::env;

mod interp;


fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() > 1 {
		interp::argument(&args[1]);
	}	
    else {
		interp::utils::print_help();
	}
}
