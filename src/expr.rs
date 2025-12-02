use crate::token::Token;
use crate::value::Value;
#[derive(Clone)]
pub enum Expr {
	List {
		expressions: Vec<Expr>,
	},

	Ternary {
		condition: Box<Expr>,
		if_true: Box<Expr>,
		if_false: Box<Expr>,
	},

	Assign {
		name: Token,
		value: Box<Expr>,
	},

	Binary {
		left: Box<Expr>,
		operator: Token,
		right: Box<Expr>,
	},

	Logical {
		left: Box<Expr>,
		operator: Token,
		right: Box<Expr>,
	},

	Grouping {
		expression: Box<Expr>,
	},

	Literal {
		value: Value,
	},

	Unary {
		operator: Token,
		right: Box<Expr>,
	},

	Variable {
		name: Token,
	},
}

pub trait Visitor {
	type Result;

	fn visit_list(&mut self, expr: &Expr) -> Self::Result;
	fn visit_ternary(&mut self, expr: &Expr) -> Self::Result;
	fn visit_assign(&mut self, expr: &Expr) -> Self::Result;
	fn visit_binary(&mut self, expr: &Expr) -> Self::Result;
	fn visit_logical(&mut self, expr: &Expr) -> Self::Result;
	fn visit_grouping(&mut self, expr: &Expr) -> Self::Result;
	fn visit_literal(&mut self, expr: &Expr) -> Self::Result;
	fn visit_unary(&mut self, expr: &Expr) -> Self::Result;
	fn visit_variable(&mut self, expr: &Expr) -> Self::Result;
}

impl Expr {
	pub fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Result {
		match self {
			Expr::List {expressions: _,  } => {
				visitor.visit_list(self)
			}
			Expr::Ternary {condition: _, if_true: _, if_false: _,  } => {
				visitor.visit_ternary(self)
			}
			Expr::Assign {name: _, value: _,  } => {
				visitor.visit_assign(self)
			}
			Expr::Binary {left: _, operator: _, right: _,  } => {
				visitor.visit_binary(self)
			}
			Expr::Logical {left: _, operator: _, right: _,  } => {
				visitor.visit_logical(self)
			}
			Expr::Grouping {expression: _,  } => {
				visitor.visit_grouping(self)
			}
			Expr::Literal {value: _,  } => {
				visitor.visit_literal(self)
			}
			Expr::Unary {operator: _, right: _,  } => {
				visitor.visit_unary(self)
			}
			Expr::Variable {name: _,  } => {
				visitor.visit_variable(self)
			}
		}
	}
}