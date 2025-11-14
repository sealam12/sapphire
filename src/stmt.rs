use crate::expr::Expr;
use crate::token::Token;
#[derive(Clone)]
pub enum Stmt {
	Expression {
		expression: Expr,
	},

	Print {
		expression: Expr,
	},

	Var {
		name: Token,
		initializer: Expr,
	},
}

pub trait Visitor {
	type Result;

	fn visit_expression(&mut self, stmt: &Stmt) -> Self::Result;
	fn visit_print(&mut self, stmt: &Stmt) -> Self::Result;
	fn visit_var(&mut self, stmt: &Stmt) -> Self::Result;
}

impl Stmt {
	pub fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Result {
		match self {
			Stmt::Expression {expression: _,  } => {
				visitor.visit_expression(self)
			}
			Stmt::Print {expression: _,  } => {
				visitor.visit_print(self)
			}
			Stmt::Var {name: _, initializer: _,  } => {
				visitor.visit_var(self)
			}
		}
	}
}