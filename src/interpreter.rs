use crate::expr::{self, Expr};
use crate::value::Value;

pub struct Interpreter;

impl Interpreter {
    
}

impl expr::Visitor for Interpreter {
    type Result = Value;

    fn visit_binary(&mut self, expr: &Expr) -> Self::Result {
        if let Expr::Binary { left, operator, right } = expr {
            
        } else {
            unreachable!()
        }
    }

    fn visit_grouping(&mut self, expr: &Expr) -> Self::Result {
        if let Expr::Grouping { expression } = expr {
            
        } else {
            unreachable!()
        }
    }

    fn visit_literal(&mut self, expr: &Expr) -> Self::Result {
        if let Expr::Literal { value } = expr {

        } else {
            unreachable!()
        }
    }

    fn visit_unary(&mut self, expr: &Expr) -> Self::Result {
        if let Expr::Unary { operator, right } = expr {

        } else {
            unreachable!()
        }
    }
}