
mod math;
mod structs;
pub mod utils;

// mathematical operators in right order
pub const OPERATORS: &str = "^*/+-"; 
pub const SYMBOLS: &str = "()";


pub struct InterpData {
	pub input: String,
	pub terminate: bool
}


pub fn argument(input: &String) {
	let mut data = InterpData {
		input: input.clone(),
		terminate: false
	};
	
	if utils::help(&data.input) {
		utils::print_help();
		return;
	}
	if utils::is_valid_expr(&data.input) {
		interpret(&mut data);
	} 
	else {
		println!("invalid input");
	}
}


pub fn interpret(data: &mut InterpData) {
	println!("interpreter started");
	loop {
		let subexpr = utils::find_subexpr(&data.input);
		
		let start = subexpr.start;
		let result = math::calc_subexpression(&subexpr);
		//println!("result: {}", result);
		if subexpr.last_expr { // this was the last expression
			break;
		}
		//println!("input: {}", data.input);
		let len = subexpr.start + subexpr.expr.len();
		if subexpr.has_parenthesis {
			data.input.replace_range(start..len+1, ""); // remove the closing parenthesis ')'
		} 
		else { 
			data.input.replace_range(start..len, "");
		}
		data.input.insert_str(start, &result);
	}

	println!("result: {}", data.input);
	
	println!("interpreter finished");
}

