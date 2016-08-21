use parsing::ast::*;


#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Function(FunctionDeclaration),
    Number(i32),
    Undefined,
}

impl Value {
    pub fn plus(&self, other: &Value) -> Value {
        match (self, other) {
            (&Value::Number(left), &Value::Number(right)) => Value::Number(left + right),
            _ => Value::Undefined,
        }
    }

    pub fn minus(&self, other: &Value) -> Value {
        match (self, other) {
            (&Value::Number(left), &Value::Number(right)) => Value::Number(left - right),
            _ => Value::Undefined,
        }
    }

    pub fn mult(&self, other: &Value) -> Value {
        match (self, other) {
            (&Value::Number(left), &Value::Number(right)) => Value::Number(left * right),
            _ => Value::Undefined,
        }
    }

    pub fn div(&self, other: &Value) -> Value {
        match (self, other) {
            (&Value::Number(left), &Value::Number(right)) => Value::Number(left / right),
            _ => Value::Undefined,
        }
    }
}