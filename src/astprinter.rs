use crate::expr;
use crate::token::Token;
use crate::value::Value;

pub struct AstPrinter;

impl expr::Visitor for AstPrinter {
    type Result = String;

    fn visit_binary(&mut self, expr: &expr::Expr) -> Self::Result {
        if let expr::Expr::Binary { left, operator, right } = expr {
            self.parenthesize(&operator.lexeme, &[left, right])
        } else {
            unreachable!() // Should not happen with correct usage
        }
    }

    fn visit_grouping(&mut self, expr: &expr::Expr) -> Self::Result {
        if let expr::Expr::Grouping { expression } = expr {
            self.parenthesize("group", &[expression])
        } else {
            unreachable!() // Should not happen
        }
    }

    fn visit_literal(&mut self, expr: &expr::Expr) -> Self::Result {
        if let expr::Expr::Literal { value } = expr {
            match value {
                Value::Number(n) => n.to_string(),
                Value::Str(s) => format!("\"{}\"", s),
                Value::Bool(b) => b.to_string(),
                Value::Null => "null".to_string(),
            }
        } else {
            unreachable!() // Should not happen
        }
    }

    fn visit_unary(&mut self, expr: &expr::Expr) -> Self::Result {
        if let expr::Expr::Unary { operator, right } = expr {
            self.parenthesize(&operator.lexeme, &[right])
        } else {
            unreachable!() // Should not happen
        }
    }
}

impl AstPrinter {
    pub fn print(&mut self, expr: &expr::Expr) -> String {
        expr.accept(self)
    }

    fn parenthesize(&mut self, name: &str, exprs: &[&Box<expr::Expr>]) -> String {
        let mut builder = String::new();
        builder.push('(');
        builder.push_str(name);
        for expr in exprs {
            builder.push(' ');
            builder.push_str(&expr.accept(self));
        }
        builder.push(')');
        builder
    }
}