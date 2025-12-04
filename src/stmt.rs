use crate::expr::Expr;
use crate::token::Token;
#[derive(Clone, Debug)]
pub enum Stmt {
	Block {
		statements: Vec<Stmt>,
	},

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

	Function {
		name: Token,
		params: Vec<Token>,
		body: Box<Stmt>,
	},

	If {
		condition: Expr,
		then_branch: Box<Stmt>,
		else_branch: Option<Box<Stmt>>,
	},

	While {
		condition: Expr,
		body: Box<Stmt>,
	},
}

pub trait Visitor {
	type Result;

	fn visit_block(&mut self, stmt: &Stmt) -> Self::Result;
	fn visit_expression(&mut self, stmt: &Stmt) -> Self::Result;
	fn visit_print(&mut self, stmt: &Stmt) -> Self::Result;
	fn visit_var(&mut self, stmt: &Stmt) -> Self::Result;
	fn visit_function(&mut self, stmt: &Stmt) -> Self::Result;
	fn visit_if(&mut self, stmt: &Stmt) -> Self::Result;
	fn visit_while(&mut self, stmt: &Stmt) -> Self::Result;
}

impl Stmt {
	pub fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Result {
		match self {
			Stmt::Block {statements: _,  } => {
				visitor.visit_block(self)
			}
			Stmt::Expression {expression: _,  } => {
				visitor.visit_expression(self)
			}
			Stmt::Print {expression: _,  } => {
				visitor.visit_print(self)
			}
			Stmt::Var {name: _, initializer: _,  } => {
				visitor.visit_var(self)
			}
			Stmt::Function {name: _, params: _, body: _,  } => {
				visitor.visit_function(self)
			}
			Stmt::If {condition: _, then_branch: _, else_branch: _,  } => {
				visitor.visit_if(self)
			}
			Stmt::While {condition: _, body: _,  } => {
				visitor.visit_while(self)
			}
		}
	}
}