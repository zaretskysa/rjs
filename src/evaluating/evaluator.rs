use parsing::ast::*;
use evaluating::value::*;
use evaluating::lexical_stack::*;

pub struct Evaluator {
    env: LexicalStack,
}

impl Evaluator {
    pub fn new() -> Evaluator {
        Evaluator{env: LexicalStack::new()}
    }

    pub fn eval(&mut self, prog: &Program) -> Value {
        return self.eval_program(prog)
    }

    pub fn eval_program(&mut self, prog: &Program) -> Value {
        let &Program::Program(ref src_elements) = prog;
        let mut last_result = Value::Undefined;
        for el in src_elements {
            last_result = self.eval_source_element(el);
        }
        return last_result
    }

    fn eval_source_element(&mut self, el: &SourceElement) -> Value {
        match el {
            &SourceElement::Statement(ref stmt) => self.eval_statement(stmt),
            &SourceElement::FunctionDecl(ref decl) => self.eval_func_decl(decl),
        }
    }

    fn eval_func_decl(&mut self, decl: &FunctionDeclaration) -> Value {
        let &FunctionDeclaration::FunctionDeclaration(ref id, _, _) = decl;
        self.env.insert(id.clone(), Value::Function(decl.clone()));
        Value::Undefined
    }

    fn eval_statement(&mut self, st: &Statement) -> Value {
        match st {
            &Statement::Stub => Value::Undefined,
            &Statement::Empty => Value::Undefined,
            &Statement::Block(ref stmts) => self.eval_statements(stmts),
            // &Statement::Expression(ref expr) => self.eval_expression(expr),
            &Statement::VarDecl(ref id, ref expr) => self.eval_var_decl(id, expr),
            &Statement::If(ref _expr, ref _st, ref _else_st) => Value::Undefined,
            _ => panic!("fixme: unexpected Statement"),
        }
    }

    fn eval_source_elements(&mut self, _ses: &Vec<SourceElement>) -> Value {
        Value::Undefined
    }

    fn eval_statements(&mut self, stmts: &Vec<Statement>) -> Value {
        let mut result = Value::Undefined;
        for st in stmts {
            result = self.eval_statement(st);
        }
        return result;
    }

    fn eval_expression(&mut self, expr: &AssignmentExpr) -> Value {
        match expr {
            &AssignmentExpr::UnaryAssignment(ref logical_or) => self.eval_logical_or_expr(logical_or.as_ref()),
            &AssignmentExpr::BinaryAssignment(ref _id, ref _logical_or) => Value::Undefined,
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
            &LogicalOrExpr::BinaryOr(ref _logical_or, ref _logical_and) => Value::Undefined,
        }
    }

    fn eval_logical_and_expr(&mut self, expr: &LogicalAndExpr) -> Value {
        match expr {
            &LogicalAndExpr::UnaryAnd(ref eq_expr) => self.eval_equality_expr(eq_expr.as_ref()),
            &LogicalAndExpr::BinaryAnd(ref _logical_and, ref _eq) => Value::Undefined,
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
            &AccessExpr::Identifier(ref id) => self.env.get(id),
            &AccessExpr::Call(ref id, ref params) => self.eval_func_call(id, params),
        }
    }

    fn eval_func_call(&mut self, func_name: &String, params: &Vec<AssignmentExpr>) -> Value {
        let func = self.env.get(func_name);
        self.env.enter_scope();

        let result = match func {
            Value::Function(FunctionDeclaration::FunctionDeclaration(_, formal_params, body)) => {
                assert!(params.len() == formal_params.len());
                for i in 0..params.len() {
                    let ref param_name = formal_params[i];
                    let param_value = self.eval_expression(&params[i]);
                    self.env.insert(param_name.clone(), param_value);
                }
                self.eval_source_elements(&body)
            },
            _ => panic!("Not a function"),
        };

        self.env.leave_scope();
        return result;
    }
}
