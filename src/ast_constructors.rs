use std::env;
mod ast;

fn exp_identifier(line: i32, str: String) -> Box<ExpressionNode> {
    Box::new(ExpressionNode {location: line, expression: Expression::Identifier{ name: str }})
}

fn exp_rawliteral(line: i32, str: String) -> Box<ExpressionNode> {
    Box::new(ExpressionNode {location: line, expression: Expression::RawLiteral{ value: str }})
}

fn exp_binoperation(line: i32, str: operator, left: Box<ExpressionNode, right: Box<ExpressionNode>) -> Box<ExpressionNode> {
    Box::new(
        ExpressionNode {
            location: line, 
            expression: Expression::BinOperation{ op: str, lhs: left, rhs: right }
        }
    )
}

fn exp_unoperation(line: i32, str: operator, right: Box<ExpressionNode>) -> Box<ExpressionNode> {
    Box::new(
        ExpressionNode {
            location: line, 
            expression: Expression::UnOperation{ op: str, rhs: right }
        }
    )
}

fn exp_index(line: i32, p: Box<ExpressionNode>, i: Box<ExpressionNode>) -> Box<ExpressionNode> {
    Box::new(
        ExpressionNode {
            location: line, 
            expression: Expression::Index{ primary: p, index: i }
        }
    )
}

fn exp_selector(line: i32, p: Box<ExpressionNode>, str: String) -> Box<ExpressionNode> {
    Box::new(
        ExpressionNode {
            location: line, 
            expression: Expression::Selector{ primary: p, name: str }
        }
    )
}

fn exp_functioncall(line: i32, p: Box<ExpressionNode>, args: Vec<ExpressionNode>) -> Box<ExpressionNode> {
    Box::new(
        ExpressionNode {
            location: line, 
            expression: Expression::FunctionCall{ primary: p, arguments: args }
        }
    )
}

fn exp_append(line: i32, left: Box<ExpressionNode>, right: Box<ExpressionNode>) -> Box<ExpressionNode> {
    Box::new(
        ExpressionNode {
            location: line, 
            expression: Expression::Append{ lhs: left, right: rhs }
        }
    )
}

fn exp_typecast(line: i32, exp: Box<ExpressionNode>) -> Box<ExpressionNode> {
    Box::new(
        ExpressionNode {
            location: line, 
            expression: Expression::TypeCast{ exp: expr }
        }
    )
}
