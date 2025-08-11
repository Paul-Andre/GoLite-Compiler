use kind::*;

#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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
    Mul,
    Div,

    BwOr,
    BwXor,
    Mod,
    BwAnd,
    BwAndNot,

    LShift,
    RShift,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum UnaryOperator {
    Plus,
    Neg,
    BwCompl,
    Not,
}

#[derive(Debug)]
pub enum AstKind {
    Identifier { name: String },
    Slice { base: Box<AstKindNode> },
    Array { base: Box<AstKindNode>, size: String },
    Struct { fields: Vec<Field> },
}

#[derive(Debug)]
pub struct AstKindNode {
    pub line_number: u32,
    pub ast_kind: AstKind
}

// This is either the field of a struct or a list of parameters declared with the same type for a
// function
#[derive(Debug)]
pub struct Field {
    pub line_number: u32,
    pub identifiers: Vec<String>,
    pub kind: Box<AstKindNode>,
}

#[derive(Debug)]
pub enum Expression {
    Identifier { name: String, original_name: String },
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
    TypeCast { name:String, expr: Box<ExpressionNode> },
}

#[derive(Debug)]
pub struct ExpressionNode {
    pub line_number: u32,
    pub kind: Kind,
    pub expression: Expression,
}

#[derive(Debug)]
pub struct VarSpec {
    pub line_number: u32,
    pub names: Vec<String>,
    pub kind: Option<Box<AstKindNode>>,
    pub rhs: Option<Vec<ExpressionNode>>,
    pub evaluated_kind: Kind
}

#[derive(Debug)]
pub struct TypeSpec {
    pub line_number: u32,
    pub name: String,
    pub kind: Box<AstKindNode>,
}

#[derive(Debug)]
pub enum SwitchCase {
    Default,
    Cases(Vec<ExpressionNode>),
}

#[derive(Debug)]
pub struct CaseClause {
    pub line_number: u32,
    pub switch_case: SwitchCase,
    pub statements: Vec<StatementNode>,
}

#[derive(Debug)]
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
    VarDeclarations { declarations: Vec<VarSpec> },
    TypeDeclarations { declarations: Vec<TypeSpec> },
    ShortVariableDeclaration {
        identifier_list: Vec<String>,
        expression_list: Vec<ExpressionNode>,
        is_assigning: Vec<bool>
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
    For {
        init: Box<StatementNode>,
        condition: Option<Box<ExpressionNode>>,
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

#[derive(Debug)]
pub struct StatementNode {
    pub line_number: u32,
    pub statement: Statement,
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<Field>,
    pub return_kind: Option<Box<AstKindNode>>,
    pub body: Vec<StatementNode>,
}

#[derive(Debug)]
pub enum TopLevelDeclaration {
    VarDeclarations { declarations: Vec<VarSpec> },
    TypeDeclarations { declarations: Vec<TypeSpec> },
    FunctionDeclaration (Function),
}

#[derive(Debug)]
pub struct TopLevelDeclarationNode {
    pub line_number: u32,
    pub top_level_declaration: TopLevelDeclaration
}

#[repr(C)]
#[derive(Debug)]
pub struct Program {
    pub package_name: String,
    pub declarations: Vec<TopLevelDeclarationNode>,
}
