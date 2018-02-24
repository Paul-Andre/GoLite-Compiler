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

pub enum BinOperator {

}

pub enum UnOperator {

}

pub enum AstKind {
    Identifier { name: String },
    Slice { base: Box<Kind> },
    Array { base: Box<Kind>, size: String },
    Struct { fields: Vec<StructField> },
}

pub struct StructField {
    pub identifiers: Vec<String>,
    pub ast_kind: AstKind,
}


pub struct SourceLocation {
    pub line_number: u32,
}

pub enum Expression {
    Identifier { name: String },
    RawLiteral { value: String },
    BinOperation {
        op: BinOperator,
        lhs: Box<ExpressionNode>,
        rhs: Box<ExpressionNode>,
    },
    UnOperation {
        op: UnOperator,
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
    pub location: SourceLocation,
    pub kind: Kind,
    pub expression: Expression,
}


