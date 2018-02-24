pub enum Kind {
    Undefined,
    Basic(BasicKind),
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BasicKind {
    Int = 0,
    Float = 1,
    Rune = 2,
    String = 3,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BinaryOperator {
    Or,
    And,

    Eq,
    Neq,
    Lt,
    Leq,
    Gt,
    Geq,

    Add,
    Sub,
    BwOr,
    BwXor,

    Mul,
    Div,
    Mod,
    LShift,
    RShift,
    BwAnd,
    BwAndNot,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UnaryOperator {
    Plus,
    Neg,
    BwCompl,
    Not,
}

pub enum AstKind {
    Identifier { name: String },
    Slice { base: Box<AstKind> },
    Array { base: Box<AstKind>, size: String },
    Struct { fields: Vec<StructField> },
}

pub struct StructField {
    pub identifiers: Vec<String>,
    pub ast_kind: AstKind,
}

pub enum Expression {
    Identifier { name: String },
    RawLiteral { value: String },
    BinaryOperation {
        op: BinaryOperator,
        lhs: Box<ExpressionNode>,
        rhs: Box<ExpressionNode>,
    },
    UnaryOperation {
        op: UnaryOperator,
        rhs: Box<ExpressionNode>,
    },
    Index {
        primary: Box<ExpressionNode>,
        index: Box<ExpressionNode>,
    },
    Selector {
        primary: Box<ExpressionNode>,
        name: String,
    },
    FunctionCall {
        primary: Box<ExpressionNode>,
        arguments: Vec<ExpressionNode>,
    },
    Append {
        lhs: Box<ExpressionNode>,
        rhs: Box<ExpressionNode>,
    },

    TypeCast { expr: Box<ExpressionNode> },
}

pub struct ExpressionNode {
    pub line_number: u32,
    pub kind: Kind,
    pub expression: Expression,
}

pub struct VarDeclaration {
    names: Vec<String>,
    kind: Option<AstKind>,
    rhs: Vec<ExpressionNode>,
}

pub struct TypeDeclaration {
    names: String,
    kind: AstKind,
}

pub enum CaseClauseTag {
    Default,
    Cases(Vec<ExpressionNode>),
}

pub struct CaseClause {
    tag: CaseClauseTag,
    statements: Vec<StatementNode>,
}


pub enum Statement {
    Empty,
    Block(Vec<StatementNode>),
    Expression(Box<ExpressionNode>),
    Assignment {
        lhs: Vec<ExpressionNode>,
        rhs: Vec<ExpressionNode>,
    },
    OpAssignment {
        lhs: ExpressionNode,
        rhs: ExpressionNode,
        operator: BinaryOperator,
    },
    VarDeclarations { declarations: Vec<VarDeclaration> },
    TypeDeclarations { declarations: Vec<TypeDeclaration> },
    ShortVariableDeclaration {
        IdentifierList: Vec<String>,
        ExpressionList: Vec<ExpressionNode>,
    },
    IncDec {
        is_dec: bool,
        expr: Box<ExpressionNode>,
    },
    Print { exprs: Vec<ExpressionNode> },
    Println { exprs: Vec<ExpressionNode> },
    If {
        init: Box<StatementNode>,
        condition: Box<ExpressionNode>,
        if_branch: Vec<StatementNode>,
        else_branch: Option<Box<StatementNode>>,
    },
    Loop { body: Vec<StatementNode> },
    While {
        condition: Box<ExpressionNode>,
        body: Vec<StatementNode>,
    },
    For {
        init: Box<StatementNode>,
        condition: Box<ExpressionNode>,
        post: Box<StatementNode>,
        body: Vec<StatementNode>,
    },
    Switch {
        init: Box<StatementNode>,
        expr: Option<Box<ExpressionNode>>,
        body: Vec<CaseClause>,
    },
    Break,
    Continue,
}

pub struct StatementNode {
    line_number: u32,
    statement: Statement,
}


pub struct Parameter {
    name: String,
    kind: Box<AstKind>,
}


pub enum TopLevelDeclaration {
    VarDeclarations { declarations: Vec<VarDeclaration> },
    TypeDeclarations { declarations: Vec<TypeDeclaration> },
    FunctionDeclaration {
        name: String,
        parameters: Vec<Parameter>,
        return_kind: Option<Box<AstKind>>,
        body: Vec<StatementNode>,
    },
}

struct Program {
    package_name: String,
    declarations: Vec<TopLevelDeclaration>,
}
