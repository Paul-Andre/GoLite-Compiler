use ast::*;
use std::ffi::CStr;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn exp_identifier(line: u32, string: *const c_char) -> *mut ExpressionNode {
    let copied_string = unsafe { CStr::from_ptr(string) }.to_str().unwrap().into();
    Box::into_raw(Box::new(ExpressionNode {
        location: SourceLocation { line_number: line },
        expression: Expression::Identifier { name: copied_string },
        kind: Kind::Undefined,
    }))
}

/*
fn exp_rawliteral(line: u32, str: String) -> Box<ExpressionNode> {
    Box::new(ExpressionNode {location: line, expression: Expression::RawLiteral{ value: str }})
}

fn exp_binoperation(line: u32, str: operator, left: Box<ExpressionNode>, right: Box<ExpressionNode>) -> Box<ExpressionNode> {
    Box::new(
        ExpressionNode {
            location: line, 
            expression: Expression::BinOperation{ op: str, lhs: left, rhs: right }
        }
    )
}

fn exp_unoperation(line: u32, str: operator, right: Box<ExpressionNode>) -> Box<ExpressionNode> {
    Box::new(
        ExpressionNode {
            location: line, 
            expression: Expression::UnOperation{ op: str, rhs: right }
        }
    )
}

fn exp_index(line: u32, p: Box<ExpressionNode>, i: Box<ExpressionNode>) -> Box<ExpressionNode> {
    Box::new(
        ExpressionNode {
            location: line, 
            expression: Expression::Index{ primary: p, index: i }
        }
    )
}

fn exp_selector(line: u32, p: Box<ExpressionNode>, str: String) -> Box<ExpressionNode> {
    Box::new(
        ExpressionNode {
            location: line, 
            expression: Expression::Selector{ primary: p, name: str }
        }
    )
}

fn exp_functioncall(line: u32, p: Box<ExpressionNode>, args: Vec<ExpressionNode>) -> Box<ExpressionNode> {
    Box::new(
        ExpressionNode {
            location: line, 
            expression: Expression::FunctionCall{ primary: p, arguments: args }
        }
    )
}

fn exp_append(line: u32, left: Box<ExpressionNode>, right: Box<ExpressionNode>) -> Box<ExpressionNode> {
    Box::new(
        ExpressionNode {
            location: line, 
            expression: Expression::Append{ lhs: left, rhs: right }
        }
    )
}

fn exp_typecast(line: u32, exp: Box<ExpressionNode>) -> Box<ExpressionNode> {
    Box::new(
        ExpressionNode {
            location: line, 
            expression: Expression::TypeCast{ expr: exp }
        }
    )
}
*/
