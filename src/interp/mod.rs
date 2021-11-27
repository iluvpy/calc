
pub mod utils;


pub struct InterpData {
	pub input: String,
	pub terminate: bool
}

pub fn interpret(data: &mut InterpData) {
	println!("interpreter reached!");
}

pub fn argument(input: &String) {
	let mut data = InterpData {
		input: input.clone(),
		terminate: false
	};
	
	if utils::help(&data.input) {
		return;
	}
	if utils::is_valid_expr(&data.input) {
		interpret(&mut data);
	} 
	else {
		println!("invalid input");
	}
}