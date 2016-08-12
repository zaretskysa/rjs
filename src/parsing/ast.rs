
#[derive(Debug, PartialEq, Clone)]
pub enum Program {
    Program(Vec<Statement>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum SourceElement {
    StatementSE(Statement),
    // FunctionDeclSourceElement(FunctionDeclaration),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    EmptySt,
    BlockSt(Vec<Statement>),
    ExpressionSt(AssignmentExpr),
    VarDeclSt(String, AssignmentExpr),
    IfSt(LogicalOrExpr, PStatement, Option<PStatement>),
}

pub type PStatement = Box<Statement>;

#[derive(Debug, PartialEq, Clone)]
pub enum AssignmentExpr {
    UnaryAssignment(PLogicalOrExpr),
    BinaryAssignment(String, PLogicalOrExpr)
}

pub type PAssignmentExpr = Box<AssignmentExpr>;

#[derive(Debug, PartialEq, Clone)]
pub enum LogicalOrExpr {
    UnaryOr(PLogicalAndExpr),
    BinaryOr(PLogicalOrExpr, PLogicalAndExpr)
}

pub type PLogicalOrExpr = Box<LogicalOrExpr>;

#[derive(Debug, PartialEq, Clone)]
pub enum LogicalAndExpr {
    UnaryAnd(PEqualityExpr),
    BinaryAnd(PLogicalAndExpr, PEqualityExpr)
}

pub type PLogicalAndExpr = Box<LogicalAndExpr>;

#[derive(Debug, PartialEq, Clone)]
pub enum EqualityExpr {
    UnaryEquality(PAdditiveExpr),
    Equal(PEqualityExpr, PAdditiveExpr),
    NotEqual(PEqualityExpr, PAdditiveExpr),
}

pub type PEqualityExpr = Box<EqualityExpr>;

#[derive(Debug, PartialEq, Clone)]
pub enum AdditiveExpr {
    UnaryAdditive(PMultExpr),
    Plus(PAdditiveExpr, PMultExpr),
    Minus(PAdditiveExpr, PMultExpr),
}

pub type PAdditiveExpr = Box<AdditiveExpr>;

#[derive(Debug, PartialEq, Clone)]
pub enum MultExpr {
    UnaryMult(i32),
    Mult(PMultExpr, i32),
    Div(PMultExpr, i32),
}

pub type PMultExpr = Box<MultExpr>;
