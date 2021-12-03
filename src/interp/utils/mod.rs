use super::structs;
use super::{SYMBOLS, OPERATORS};

pub fn print_help() {
	println!("Usage: ./calc [INPUT]");
	println!("INPUT: directo input of a mathematical expression");
	println!("./calc help or ./calc h to print this text again");
}

pub fn help(input: &String) -> bool {
	return input == "h" || input == "help";
}

// returns a vector of 2 numbers and the operator that where extracted from the expr param 
// where expr means 2 numbers and 1 operator like x+y or x*y
// returns Vec<String>[num1, num2, operator]
pub fn break_subexpr(expr: &String) -> Vec<String>{
	let mut broken: Vec<String> = vec![];
	// number 1
	let mut num1 = String::from("");
	let chars: Vec<char> = expr.chars().collect();
	let mut i = 0;
	loop {
		if !chars[i].is_numeric() {
			let operator = String::from(chars[i]); // get operator
			i += 1; // start of second number (jumped over mathematical operator)
			broken.push(num1);
			let mut num2 = String::from("");
			for j in i..chars.len() {
				num2.push(chars[j]);
			}
			broken.push(num2);
			broken.push(operator);
			break;
		}
		num1.push(chars[i]);
		i += 1;
	}
	return broken;

}

pub fn is_valid_expr(input: &String) -> bool {
	let chars: Vec<char> = input.chars().collect();
	for char_ in chars {
		if !is_number(char_) && !OPERATORS.contains(char_) && !SYMBOLS.contains(char_) {
			return false;
		}
	}
	return true;
}

// returns true if char_ is part of a number (can be part of a number)
// i.e all digits, ',' and '.'
pub fn is_number(char_: char) -> bool {
	return char_.is_ascii_digit() || char_ == '.' || char_ == ',';
}

pub fn is_int(strn: &String) -> bool {
	let chars: Vec<char> = strn.chars().collect();
	for char_ in chars {
		if !is_number(char_) {
			return false;
		}
	}
	return true;
}

pub fn is_float(strn: &String) -> bool {
	return is_int(&strn) && strn.contains('.') || strn.contains(',');
}

pub fn find_subexpr(input: &String) -> structs::SubExpr {
	let mut expr = String::from("");
	let mut start: usize = 0;
	let mut operator_char: char = ' ';
	let mut has_parenthesis = false;
	let mut last_expr = false;
	let chars: Vec<char> = input.chars().collect();
	let operators: Vec<char> = OPERATORS.chars().collect();
	let index = input.find('(');
	if index != None {
		let index_ = index.unwrap();
		start = index_;
		for i in index_+1..chars.len() {
			if chars[i] == ')' {
				break;
			}
			expr.push(chars[i]);
		}
		for operator in operators {
			let index_ = input.find(operator);
			if index_ != None {
				operator_char = operator;
				break;
			}
		}
		has_parenthesis = true;
	}
	else { // find normal sub expr without brackets (sub exrp means something like x+y, y-x, x*y, etc)
		// find operator 
		let mut op_index: Option<usize> = None;
		for operator in operators {
			let index_ = input.find(operator);
			if index_ != None {
				op_index = index_;
				operator_char = operator;
				break;
			}
		}
		// no operators where found and that means that the input contains only 1 number
		// find that number
		if op_index == None {
			for char_ in &chars {
				expr.push(*char_);
			}
			last_expr = true;
		} 
		else {
			// found index and thus i need to go back to the start of the expression
			let mut index_ = op_index.unwrap()-1;
			loop {
				if is_number(chars[index_]) {
					expr.push(chars[index_]);
				}
				else {
					break;
				}
				if index_ == 0  {
					break;
				} 
				else {
					index_ -= 1;
				}
				
			}
			println!("set start to {}", start);
			start = index_;
			expr = expr.chars().rev().collect::<String>(); // invert
			expr.push(chars[op_index.unwrap()]);
			index_ = op_index.unwrap()+1;
			for i in index_..chars.len() {
				if is_number(chars[i]) {
					expr.push(chars[i]);
				}
				else {
					break;
				}
			}
			
		}
		
	}

	return structs::SubExpr {
		expr: expr,
		start: start,
		operator: operator_char,
		has_parenthesis: has_parenthesis,
		last_expr: last_expr
	};
}