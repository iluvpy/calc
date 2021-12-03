
pub struct SubExpr {
	pub expr: String,
	pub start: usize,
	pub operator: char,
	pub has_parenthesis: bool,
	pub last_expr: bool
}