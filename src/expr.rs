use crate::token::Token;
use crate::value::Value;
#[derive(Clone)]
pub enum Expr {
	Binary {
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

	fn visit_binary(&mut self, expr: &Expr) -> Self::Result;
	fn visit_grouping(&mut self, expr: &Expr) -> Self::Result;
	fn visit_literal(&mut self, expr: &Expr) -> Self::Result;
	fn visit_unary(&mut self, expr: &Expr) -> Self::Result;
	fn visit_variable(&mut self, expr: &Expr) -> Self::Result;
}

impl Expr {
	pub fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Result {
		match self {
			Expr::Binary {left: _, operator: _, right: _,  } => {
				visitor.visit_binary(self)
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