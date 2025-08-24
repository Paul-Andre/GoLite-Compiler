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
pub enum ExpressionVariant {
    Identifier { name: String, original_name: String },
    RawLiteral { value: String },
    BinaryOperation {
        op: BinaryOperator,
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    UnaryOperation {
        op: UnaryOperator,
        rhs: Box<Expression>,
    },
    Index {
        primary: Box<Expression>,
        index: Box<Expression>,
    },
    Selector {
        primary: Box<Expression>,
        name: String,
    },
    FunctionCall {
        primary: Box<Expression>,
        arguments: Vec<Expression>,
    },
    Append {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    TypeCast { name:String, expr: Box<Expression> },
}

#[derive(Debug)]
pub struct Expression {
    pub line_number: u32,
    pub kind: Kind,
    pub variant: ExpressionVariant,
}

#[derive(Debug)]
pub struct VarSpec {
    pub line_number: u32,
    pub names: Vec<String>,
    pub kind: Option<Box<AstKindNode>>,
    pub rhs: Option<Vec<Expression>>,
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
    Cases(Vec<Expression>),
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
    Expression(Box<Expression>),
    Assignment {
        lhs: Vec<Expression>,
        rhs: Vec<Expression>,
    },
    OpAssignment {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
        operator: BinaryOperator,
    },
    VarDeclarations { declarations: Vec<VarSpec> },
    TypeDeclarations { declarations: Vec<TypeSpec> },
    ShortVariableDeclaration {
        identifier_list: Vec<String>,
        expression_list: Vec<Expression>,
        is_assigning: Vec<bool>
    },
    IncDec {
        is_dec: bool,
        expr: Box<Expression>,
    },
    Print { exprs: Vec<Expression> },
    Println { exprs: Vec<Expression> },
    If {
        init: Box<StatementNode>,
        condition: Box<Expression>,
        if_branch: Vec<StatementNode>,
        else_branch: Option<Box<StatementNode>>,
    },
    For {
        init: Box<StatementNode>,
        condition: Option<Box<Expression>>,
        post: Box<StatementNode>,
        body: Vec<StatementNode>,
    },
    Switch {
        init: Box<StatementNode>,
        expr: Option<Box<Expression>>,
        body: Vec<CaseClause>,
    },
    Break,
    Continue,
    Return(Option<Box<Expression>>)
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
