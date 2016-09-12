
#[derive(Debug, PartialEq, Clone)]
pub enum Program {
    Program(Vec<SourceElement>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum SourceElement {
    Statement(Statement),
    FunctionDecl(FunctionDeclaration),
}

#[derive(Debug, PartialEq, Clone)]
pub enum FunctionDeclaration {
    FunctionDeclaration(String, Vec<String>, Vec<SourceElement>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Stub,
    Empty,
    Block(Vec<Statement>),
    Expression(SuperExpression),
}

pub type PStatement = Box<Statement>;

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Null,
    Number(i32),
    String(String),
    Bool(bool),
}


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

    Assign,
    MultAssign,
    DivAssign,
    ModulusAssign,
    PlusAssign,
    MinusAssign,
    LeftShiftAssign,
    SignedRightShiftAssign,
    UnsignedRightShiftAssign,
    BitwiseAndAssign,
    BitwiseXorAssign,
    BitwiseOrAssign,
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

    Assign,
    MultAssign,
    DivAssign,
    ModulusAssign,
    PlusAssign,
    MinusAssign,
    LeftShiftAssign,
    SignedRightShiftAssign,
    UnsignedRightShiftAssign,
    BitwiseAndAssign,
    BitwiseXorAssign,
    BitwiseOrAssign,

    Delete,
    Void,
    TypeOf,
    PrefixIncrement,
    PrefixDecrement,
    UnaryPlus,
    UnaryMinus,
    BitwiseNot,
    LogicalNot,
    PostfixIncrement(PSuperExpression),
    PostfixDecrement(PSuperExpression),
    Literal(Literal),
    NullLiteral,
    NumberLiteral(i32),
    StringLiteral(String),
    BoolLiteral(bool),
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