
pub const OPERATORS: &str = "^*/+-"; 

pub fn print_help() {
	println!("Usage: ./calc [PATH]");
	println!("Usage: ./calc [INPUT]");
	println!("PATH: path to file containing mathematical expression");
	println!("INPUT: directo input of a mathematical expression");
	println!("./calc help or ./calc h to print this text again");
}

pub fn help(input: &String) -> bool {
	if input == "h" || input == "help" {
		print_help();
		return true;
	}
	return false;
}

pub fn is_valid_expr(input: &String) -> bool {
	let chars: Vec<char> = input.chars().collect();
	for char_ in chars {
		if !char_.is_ascii_digit() && char_ != '.' && char_ != ',' && !OPERATORS.contains(char_) {
			return false;
		}
	}
	return true;
}