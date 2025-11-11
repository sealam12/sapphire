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
}

pub trait Visitor {
	type Result;

	fn visit_binary(&mut self, binary: &Expr) -> Self::Result;
	fn visit_grouping(&mut self, grouping: &Expr) -> Self::Result;
	fn visit_literal(&mut self, literal: &Expr) -> Self::Result;
	fn visit_unary(&mut self, unary: &Expr) -> Self::Result;
}

impl Expr {
	pub fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Result {
		match self {
			Expr::Binary {left, operator, right,  } => {
				visitor.visit_binary(self)
			}
			Expr::Grouping {expression,  } => {
				visitor.visit_grouping(self)
			}
			Expr::Literal {value,  } => {
				visitor.visit_literal(self)
			}
			Expr::Unary {operator, right,  } => {
				visitor.visit_unary(self)
			}
		}
	}
}