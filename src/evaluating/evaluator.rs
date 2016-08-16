use std::collections::HashMap;

use parsing::ast::*;
use evaluating::value::*;

pub struct Evaluator {
    env: HashMap<String, Value>,
}

impl Evaluator {
    pub fn new() -> Evaluator {
        Evaluator{env: HashMap::new()}
    }

    pub fn eval(&mut self, prog: &Program) -> Value {
        return self.eval_program(prog)
    }

    pub fn eval_program(&mut self, prog: &Program) -> Value {
        let &Program::Program(ref stmts) = prog;
        let mut last_result = Value::Undefined;
        for st in stmts {
            last_result = self.eval_statement(st);
        }
        return last_result
    }

    fn eval_statement(&mut self, st: &Statement) -> Value {
        match st {
            &Statement::EmptySt => Value::Undefined,
            &Statement::BlockSt(ref stmts) => self.eval_statements(stmts),
            &Statement::ExpressionSt(ref expr) => self.eval_expression(expr),
            &Statement::VarDeclSt(ref id, ref expr) => self.eval_var_decl(id, expr),
            &Statement::IfSt(ref expr, ref st, ref else_st) => Value::Undefined,
        }
    }

    fn eval_statements(&mut self, stmts: &Vec<Statement>) -> Value {
        Value::Undefined
    }

    fn eval_expression(&mut self, expr: &AssignmentExpr) -> Value {
        match expr {
            &AssignmentExpr::UnaryAssignment(ref logical_or) => self.eval_logical_or_expr(logical_or.as_ref()),
            &AssignmentExpr::BinaryAssignment(ref id, ref logical_or) => Value::Undefined,
        }
    }

    fn eval_var_decl(&mut self, id: &String, expr: &AssignmentExpr) -> Value {
        let val = self.eval_expression(expr);
        self.env.insert(id.clone(), val.clone());
        val.clone()
    }

    fn eval_logical_or_expr(&mut self, expr: &LogicalOrExpr) -> Value {
        match expr {
            &LogicalOrExpr::UnaryOr(ref logical_and) => self.eval_logical_and_expr(logical_and.as_ref()),
            &LogicalOrExpr::BinaryOr(ref logical_or, ref logicalAnd) => Value::Undefined,
        }
    }

    fn eval_logical_and_expr(&mut self, expr: &LogicalAndExpr) -> Value {
        match expr {
            &LogicalAndExpr::UnaryAnd(ref eq_expr) => self.eval_equality_expr(eq_expr.as_ref()),
            &LogicalAndExpr::BinaryAnd(ref logical_and, ref eq) => Value::Undefined,
        }
    }

    fn eval_equality_expr(&mut self, expr: &EqualityExpr) -> Value {
        match expr {
            &EqualityExpr::UnaryEquality(ref add_expr) => self.eval_additive_expr(add_expr.as_ref()),
            &EqualityExpr::Equal(..) => Value::Undefined,
            &EqualityExpr::NotEqual(..) => Value::Undefined,
        }
    }

    fn eval_additive_expr(&mut self, expr: &AdditiveExpr) -> Value {
        match expr {
            &AdditiveExpr::UnaryAdditive(ref left_expr) => self.eval_mult_expr(left_expr.as_ref()),
            &AdditiveExpr::Plus(ref left_expr, ref right_expr) => {
                let left = self.eval_additive_expr(left_expr.as_ref());
                let right = self.eval_mult_expr(right_expr.as_ref());
                left.plus(&right)
            },
            &AdditiveExpr::Minus(ref left_expr, ref right_expr) => {
                let left = self.eval_additive_expr(left_expr.as_ref());
                let right = self.eval_mult_expr(right_expr.as_ref());
                left.minus(&right)
            }
        }
    }

    fn eval_mult_expr(&mut self, expr: &MultExpr) -> Value {
        match expr {
            &MultExpr::UnaryMult(ref left_expr) => self.eval_access_expr(left_expr.as_ref()),
            &MultExpr::Mult(ref left_expr, ref right_expr) => {
                let left = self.eval_mult_expr(left_expr.as_ref());
                let right = self.eval_access_expr(right_expr.as_ref());
                left.mult(&right)
            },
            &MultExpr::Div(ref left_expr, ref right_expr) => {
                let left = self.eval_mult_expr(left_expr.as_ref());
                let right = self.eval_access_expr(right_expr.as_ref());
                left.div(&right)
            },
        }
    }

    fn eval_access_expr(&mut self, expr: &AccessExpr) -> Value {
        match expr {
            &AccessExpr::NumberLiteral(num) => Value::Number(num),
            &AccessExpr::Identifier(ref id) => match self.env.get(id) {
                Some(ref val) => {
                    (*val).clone()
                },
                None => Value::Undefined,
            }
        }
    }

}
