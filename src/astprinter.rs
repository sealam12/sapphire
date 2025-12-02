use crate::expr;

pub struct AstPrinter;

impl expr::Visitor for AstPrinter {
    type Result = String;

    fn visit_binary(&mut self, expr: &expr::Expr) -> Self::Result {
        if let expr::Expr::Binary { left, operator, right } = expr {
            self.parenthesize(&operator.lexeme, &[left, right])
        } else {
            unreachable!()
        }
    }

    fn visit_grouping(&mut self, expr: &expr::Expr) -> Self::Result {
        if let expr::Expr::Grouping { expression } = expr {
            self.parenthesize("group", &[expression])
        } else {
            unreachable!()
        }
    }

    fn visit_literal(&mut self, expr: &expr::Expr) -> Self::Result {
        if let expr::Expr::Literal { value } = expr {
            value.to_string()
        } else {
            unreachable!()
        }
    }

    fn visit_unary(&mut self, expr: &expr::Expr) -> Self::Result {
        if let expr::Expr::Unary { operator, right } = expr {
            self.parenthesize(&operator.lexeme, &[right])
        } else {
            unreachable!()
        }
    }

    fn visit_variable(&mut self, expr: &expr::Expr) -> Self::Result {
        if let expr::Expr::Variable { name } = expr {
            format!("var {}", name)
        } else {
            unreachable!()
        }
    }

    fn visit_assign(&mut self, expr: &expr::Expr) -> Self::Result {
        if let expr::Expr::Assign { name, value } = expr {
            let formatted_str: &str = &value.accept(self);
            format!("(= {} {})", name, formatted_str)
        } else {
            unreachable!()
        }
    }

    fn visit_ternary(&mut self, expr: &expr::Expr) -> Self::Result {
        if let expr::Expr::Ternary { condition, if_true, if_false } = expr {
            let fmt_condition: &str = &condition.accept(self);
            let fmt_true: &str = &if_true.accept(self);
            let fmt_false: &str = &if_false.accept(self);

            format!("({} ? {} : {})", fmt_condition, fmt_true, fmt_false)
        } else {
            unreachable!()
        }
    }

    fn visit_logical(&mut self, expr: &expr::Expr) -> Self::Result {
        if let expr::Expr::Logical { left, operator, right } = expr {
            self.parenthesize(&operator.lexeme, &[left, right])
        } else {
            unreachable!()
        }
    }

    fn visit_list(&mut self, expr: &expr::Expr) -> Self::Result {
        if let expr::Expr::List { expressions } = expr {
            "List".to_owned()
        } else {
            unreachable!()
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