use std::time::Instant;
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
	let start_time = Instant::now();
	loop {
		let subexpr = utils::find_subexpr(&data.input);
		if subexpr.last_expr { // this was the last expression
			break;
		}
		let start = subexpr.start;
		let result = math::calc_subexpression(&subexpr);
		println!("result: {}", result);		
		println!("before: {}", data.input);
		let len = subexpr.start + subexpr.expr.len();
		println!("start: {} and finish: {}", start, len);
		if subexpr.has_parenthesis {
			data.input.replace_range(start..len+2, &result); // remove the closing parenthesis ')'
		} 
		else {
			data.input.replace_range(start..len, &result);
		}
		println!("after insert: {}", data.input);
	}

	println!("result: {}", data.input);
	
	println!("interpreter finished in {:?}", start_time.elapsed());
}

