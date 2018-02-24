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
    Slice { base: Box<AstKindNode> },
    Array { base: Box<AstKindNode>, size: String },
    Struct { fields: Vec<Field> },
}

pub struct AstKindNode {
    pub line_number: u32,
    pub ast_kind: AstKind
}

// This is either the field of a struct or a list of parameters declared with the same type for a
// function
pub struct Field {
    pub line_number: u32,
    pub identifiers: Vec<String>,
    pub kind: Box<AstKindNode>,
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
    pub line_number: u32,
    pub names: Vec<String>,
    pub kind: Option<Box<AstKindNode>>,
    pub rhs: Option<Vec<ExpressionNode>>,
}

pub struct TypeDeclaration {
    pub names: String,
    pub kind: Box<AstKindNode>,
}

pub enum SwitchCase {
    Default,
    Cases(Vec<ExpressionNode>),
}

pub struct CaseClause {
    pub line_number: u32,
    pub switch_case: SwitchCase,
    pub statements: Vec<StatementNode>,
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
        lhs: Box<ExpressionNode>,
        rhs: Box<ExpressionNode>,
        operator: BinaryOperator,
    },
    VarDeclarations { declarations: Vec<VarDeclaration> },
    TypeDeclarations { declarations: Vec<TypeDeclaration> },
    ShortVariableDeclaration {
        identifier_list: Vec<String>,
        expression_list: Vec<ExpressionNode>,
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
    Return(Option<Box<ExpressionNode>>)
}

pub struct StatementNode {
    pub line_number: u32,
    pub statement: Statement,
}

pub struct VarDeclaration {
    pub line_number: u32,
    pub names: Vec<String>,
    pub kind: Option<Box<AstKindNode>>,
    pub rhs: Vec<ExpressionNode>,
}

pub struct TypeDeclaration {
    pub names: String,
    pub kind: Box<AstKindNode>,
}

pub enum TopLevelDeclaration {
    VarDeclarations { declarations: Vec<VarDeclaration> },
    TypeDeclarations { declarations: Vec<TypeDeclaration> },
    FunctionDeclaration {
        name: String,
        parameters: Vec<Field>,
        return_kind: Option<Box<AstKindNode>>,
        body: Vec<StatementNode>,
    },
}

pub struct TopLevelDeclarationNode {
    pub line_number: u32,
    pub top_level_declaration: TopLevelDeclaration
}

pub struct Program {
    pub package_name: String,
    pub declarations: Vec<TopLevelDeclaration>,
}
