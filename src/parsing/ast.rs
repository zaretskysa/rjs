
#[derive(Debug, PartialEq, Clone)]
pub enum Program {
    Program(Vec<SourceElement>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum SourceElement {
    StatementSE(Statement),
    FunctionDeclSE(FunctionDeclaration),
}

#[derive(Debug, PartialEq, Clone)]
pub enum FunctionDeclaration {
    FunctionDeclaration(String, Vec<String>, Vec<Statement>),
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
    UnaryMult(PAccessExpr),
    Mult(PMultExpr, PAccessExpr),
    Div(PMultExpr, PAccessExpr),
}

pub type PMultExpr = Box<MultExpr>;

#[derive(Debug, PartialEq, Clone)]
pub enum AccessExpr {
    NumberLiteral(i32),
//    BoolLiteral(bool),
    Identifier(String),
    Call(String, Vec<AssignmentExpr>),
}

pub type PAccessExpr = Box<AccessExpr>;


#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Null,
    Number(i32),
    String(String),
    Bool(bool),
}


#[derive(Debug, PartialEq, Clone)]
pub enum PrimaryExpression {
    This,
    Identifier(String),
    Literal(Literal),
    ArrayLiteral,
    ObjectLiteral(Vec<PropertyAssignment>),
    Expression,
}


#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Expression,
}

pub type PExpression = Box<Expression>;

#[derive(Debug, PartialEq, Clone)]
pub enum PropertyAssignment {
    FieldValue,
    Getter,
    Setter,
}

#[derive(Debug, PartialEq, Clone)]
pub enum MathOp {
    Plus,
    Minus,
    Mult,
    Div,
    Modulus,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    InstanceOf,
    In,
    Equal,
    NotEqual,
    StrictEqual,
    StrictNotEqual,
    LeftShift,
    SignedRightShift,
    UnsignedRightShift,
}

#[derive(Debug, PartialEq, Clone)]
pub enum SuperExpression {
    Stub, // TODO: remove

    LogicalOr(PSuperExpression, PSuperExpression),
    LogicalAnd(PSuperExpression, PSuperExpression),
    BitwiseOr(PSuperExpression, PSuperExpression),
    BitwiseAnd(PSuperExpression, PSuperExpression),
    BitwiseXor(PSuperExpression, PSuperExpression),
    Less(PSuperExpression, PSuperExpression),
    Greater(PSuperExpression, PSuperExpression),
    LessEqual(PSuperExpression, PSuperExpression),
    GreaterEqual(PSuperExpression, PSuperExpression),
    InstanceOf(PSuperExpression, PSuperExpression),
    In(PSuperExpression, PSuperExpression),
    This,
    New(PSuperExpression),
    Identifier(String),
    LeftShift(PSuperExpression, PSuperExpression),
    SignedRightShift(PSuperExpression, PSuperExpression),
    UnsignedRightShift(PSuperExpression, PSuperExpression),
    Plus(PSuperExpression, PSuperExpression),
    Minus(PSuperExpression, PSuperExpression),
    Mult(PSuperExpression, PSuperExpression),
    Div(PSuperExpression, PSuperExpression),
    Modulus(PSuperExpression, PSuperExpression),
    Delete,
    Void,
    TypeOf,
    PrefixIncrement,
    PrefixDecrement,
    UnaryPlus,
    UnaryMinus,
    UnaryTilde,
    UnaryExclamation,
    PostfixIncrement(PSuperExpression),
    PostfixDecrement(PSuperExpression),
    Literal(Literal),
    ArrayLiteral(ArrayLiteral),
    ObjectLiteral(ObjectLiteral),
    Expression(PSuperExpression),
    Sequence,
    Ternary(PSuperExpression, PSuperExpression, PSuperExpression),
    Call(PSuperExpression),
    DotAccess(PSuperExpression),
    BracketsAccess(PSuperExpression),
}

pub type PSuperExpression = Box<SuperExpression>;

pub type ArrayLiteral = Vec<Option<()>>;

pub type ObjectLiteral = Vec<PropertyAssignment>;

#[derive(Debug, PartialEq, Clone)]
pub enum MemberExpression {
    Primary,
    Function,
    AccessByBrackets,
    AccessByDot,
    New,
}

#[derive(Debug, PartialEq, Clone)]
pub enum CallAction {
    Call,
    DotAccess,
    BracketsAccess,
}

#[derive(Debug, PartialEq, Clone)]
pub enum AccessAction {
    Dot,
    Brackets,
}