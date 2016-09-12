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
            &Statement::Expression(ref expr) => self.eval_expression(expr),
            // _ => panic!("fixme: unexpected Statement"),
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

    fn eval_expression(&mut self, expr: &SuperExpression) -> Value {
        match expr {
            &SuperExpression::NumberLiteral(num) => Value::Number(num),
            &SuperExpression::Plus(ref left_expr, ref right_expr) => {
                let left = self.eval_expression(left_expr.as_ref());
                let right = self.eval_expression(right_expr.as_ref());
                left.plus(&right)
            }
            &SuperExpression::Minus(ref left_expr, ref right_expr) => {
                let left = self.eval_expression(left_expr.as_ref());
                let right = self.eval_expression(right_expr.as_ref());
                left.minus(&right)
            }
            &SuperExpression::Mult(ref left_expr, ref right_expr) => {
                let left = self.eval_expression(left_expr.as_ref());
                let right = self.eval_expression(right_expr.as_ref());
                left.mult(&right)
            }
            &SuperExpression::Div(ref left_expr, ref right_expr) => {
                let left = self.eval_expression(left_expr.as_ref());
                let right = self.eval_expression(right_expr.as_ref());
                left.div(&right)
            }
            &SuperExpression::Expression(ref expr) => self.eval_expression(expr.as_ref()),
            _ => panic!("Not implemented: {:#?}", expr)
        }
    }
}
