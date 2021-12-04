use super::structs;

// calculates for example y*x or y+x etc
pub fn calc_subexpression(subexpr: &structs::SubExpr) -> String {
	let numbers: Vec<&str> = subexpr.expr.split(subexpr.operator).collect();

	if subexpr.expr == "" {
		return String::from("");
	}
	if numbers.len() == 1 {
		return String::from(numbers[0]);
	}

	let num1 = match numbers[0].parse::<f32>() {
		Ok(res) => res,
		Err(why) => panic!("couldnt convert number, error: {}", why),
	};
	let num2 = match numbers[1].parse::<f32>() {
		Ok(res) => res,
		Err(why) => panic!("coudnt convert number, error {}", why),
	};

	if subexpr.operator == '*' {
		println!("executing: {}*{}", num1, num2);
		return (num1*num2).to_string(); 
	}
	if subexpr.operator == '/' {
		println!("executing: {}/{}", num1, num2);
		return (num1/num2).to_string();
	}
	if subexpr.operator == '^' {
		println!("executing: {}^{}", num1, num2);
		return f32::powf(num1, num2).to_string();
	}
	if subexpr.operator == '+' {
		println!("executing: {}+{}", num1, num2);
		return (num1+num2).to_string();
	}
	if subexpr.operator == '-' {
		println!("executing: {}-{}", num1, num2);
		return (num1-num2).to_string();
	}

	return String::from("");
}