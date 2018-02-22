
enum AstKind{
    Identifier{name: String},
    Slice{base: Box<Kind>},
    Array{base: Box<Kind>, size: String},
    Struct{ fields: Vec<StructField> }
}

struct StructField {
    identifiers: Vec<String>,
    ast_kind: AstKind
}


struct SourceLocation{
    line_number: u32
}

enum Expression {
    Identifier{ name: String },
    RawLiteral{ value: String },
    BinOperation{  
        op: BinOperator,
        lhs: Box<ExpressionNode>,
        rhs: Box<ExpressionNode>
    },
    UnOperation{
        op: UnOperator,
        rhs: Box<ExpressionNode>,
    },
    Index{
        primary: Box<ExpressionNode>,
        index: Box<ExpressionNode>,
    },
    Selector{
        primary: Box<ExpressionNode>,
        name: String
    },
    FunctionCall{
        primary: Box<ExpressionNode>,
        arguments: Vec<ExpressionNode>
    },
    Append{
        lhs: Box<ExpressionNode>,
        rhs: Box<ExpressionNode>,
    },

    TypeCast{
        expr: Box<ExpressionNode>
    },

}

struct ExpressionNode {
    location: SourceLocation,
    kind: Kind,
    expression: Expression
}

    

