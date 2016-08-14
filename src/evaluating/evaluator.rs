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

    pub fn eval(&self, prog: &Program) -> Value {
        return self.eval_program(prog)
    }

    pub fn eval_program(&self, prog: &Program) -> Value {
        let &Program::Program(ref stmts) = prog;
        return self.eval_statement(&stmts[0])
    }

    fn eval_statement(&self, st: &Statement) -> Value {
        match st {
            &Statement::EmptySt => Value::Undefined,
            &Statement::BlockSt(ref stmts) => self.eval_statements(stmts),
            &Statement::ExpressionSt(ref expr) => self.eval_expression(expr),
            &Statement::VarDeclSt(ref id, ref expr) => self.eval_var_decl(id, expr),
            &Statement::IfSt(ref expr, ref st, ref else_st) => Value::Undefined,
        }
    }

    fn eval_statements(&self, stmts: &Vec<Statement>) -> Value {
        Value::Undefined
    }

    fn eval_expression(&self, expr: &AssignmentExpr) -> Value {
        match expr {
            &AssignmentExpr::UnaryAssignment(ref logical_or) => self.eval_logical_or_expr(logical_or.as_ref()),
            &AssignmentExpr::BinaryAssignment(ref id, ref logical_or) => Value::Undefined,
        }
    }

    fn eval_var_decl(&self, id: &String, expr: &AssignmentExpr) -> Value {
        Value::Undefined
    }

    fn eval_logical_or_expr(&self, expr: &LogicalOrExpr) -> Value {
        match expr {
            &LogicalOrExpr::UnaryOr(ref logical_and) => self.eval_logical_and_expr(logical_and.as_ref()),
            &LogicalOrExpr::BinaryOr(ref logical_or, ref logicalAnd) => Value::Undefined,
        }
    }

    fn eval_logical_and_expr(&self, expr: &LogicalAndExpr) -> Value {
        match expr {
            &LogicalAndExpr::UnaryAnd(ref eq_expr) => self.eval_equality_expr(eq_expr.as_ref()),
            &LogicalAndExpr::BinaryAnd(ref logical_and, ref eq) => Value::Undefined,
        }
    }

    fn eval_equality_expr(&self, expr: &EqualityExpr) -> Value {
        match expr {
            &EqualityExpr::UnaryEquality(ref add_expr) => self.eval_additive_expr(add_expr.as_ref()),
            &EqualityExpr::Equal(..) => Value::Undefined,
            &EqualityExpr::NotEqual(..) => Value::Undefined,
        }
    }

    fn eval_additive_expr(&self, expr: &AdditiveExpr) -> Value {
        match expr {
            &AdditiveExpr::UnaryAdditive(ref left_expr) => self.eval_mult_expr(left_expr.as_ref()),
            &AdditiveExpr::Plus(ref left_expr, ref right_expr) => {
                let left = self.eval_additive_expr(left_expr.as_ref());
                let right = self.eval_mult_expr(right_expr.as_ref());
                match (left, right) {
                    (Value::Number(left_num), Value::Number(right_num)) => Value::Number(left_num + right_num),
                    _ => Value::Undefined,
                }
            },
            &AdditiveExpr::Minus(ref left_expr, ref right_expr) => {
                let left = self.eval_additive_expr(left_expr.as_ref());
                let right = self.eval_mult_expr(right_expr.as_ref());
                match (left, right) {
                    (Value::Number(left_num), Value::Number(right_num)) => Value::Number(left_num - right_num),
                    _ => Value::Undefined,
                }
            }
        }
    }

    fn eval_mult_expr(&self, expr: &MultExpr) -> Value {
        match expr {
            &MultExpr::UnaryMult(num) => Value::Number(num),
            &MultExpr::Mult(ref mult_expr, num) => {
                match self.eval_mult_expr(mult_expr) {
                    Value::Number(left) => Value::Number(left * num),
                    Value::Undefined => Value::Undefined,
                }
            },
            &MultExpr::Div(ref mult_expr, num) => {
                match self.eval_mult_expr(mult_expr) {
                    Value::Number(left) => Value::Number(left / num),
                    Value::Undefined => Value::Undefined,
                }
            }
        }
    }

}

